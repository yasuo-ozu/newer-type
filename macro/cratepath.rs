//! Lookup of `newer-type` among the dependencies that a manifest inherits from
//! its workspace.
//!
//! `proc-macro-crate` 1.x reads the dependency tables of the caller's
//! `Cargo.toml` only. A renamed dependency that the member takes from the
//! workspace has no `package` field there:
//!
//! ```toml
//! # workspace root
//! [workspace.dependencies]
//! nt = { package = "newer-type", version = "0.4" }
//!
//! # member
//! [dependencies]
//! nt.workspace = true   # or: nt = { workspace = true }
//! ```
//!
//! so it is not found as `newer-type`. This module follows `workspace = true`
//! to `[workspace.dependencies]` of the workspace root (the nearest manifest
//! with a `[workspace]` table above the member, or the one that
//! `package.workspace` names) and returns the key under which `newer-type` is
//! reached. The dependency tables are `dependencies`, `dev-dependencies` and
//! their `[target.'cfg'.*]` forms, the ones proc-macro-crate reads too.
//! `build-dependencies` are left out: they are linked into `build.rs` only, not
//! into the crate whose code the macro expands in.
//!
//! The result is cached per thread, keyed by the manifest directory and the
//! package, and valid while the modification times of the manifests it was
//! computed from stay the same (like the cache of proc-macro-crate): a
//! rust-analyzer session expands `#[implement]` many times in one process.

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use toml_edit::{Document, Item, TableLike};

/// A manifest that a lookup read, with its modification time from just before
/// it was read (`None`: the file does not exist).
type Stamp = (PathBuf, Option<SystemTime>);

struct Cached {
    stamps: Vec<Stamp>,
    name: Option<String>,
}

thread_local! {
    // `Mutex::new` is not `const` before Rust 1.63 and the MSRV is 1.60, so no
    // `static Mutex<HashMap<..>>`; a proc macro runs in the compiler's thread.
    static CACHE: RefCell<HashMap<(PathBuf, String), Cached>> = RefCell::new(HashMap::new());
    /// Manifests parsed by this thread (tests count them to see cache hits).
    #[cfg(test)]
    static PARSED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// `Ok(None)` if `path` does not exist, `Err` if it does but the platform does
/// not tell its modification time (then nothing is cached).
fn mtime(path: &Path) -> Result<Option<SystemTime>, ()> {
    match std::fs::metadata(path) {
        Ok(meta) => meta.modified().map(Some).map_err(|_| ()),
        Err(_) => Ok(None),
    }
}

/// Reads manifests and remembers which ones, as of when.
struct Manifests {
    stamps: Vec<Stamp>,
    cacheable: bool,
}

impl Manifests {
    fn new() -> Self {
        Manifests {
            stamps: Vec::new(),
            cacheable: true,
        }
    }

    /// The stamp is taken before the read, so that a change in between makes
    /// the entry stale rather than silently current.
    fn read(&mut self, path: &Path) -> Option<Document> {
        match mtime(path) {
            Ok(time) => self.stamps.push((path.to_path_buf(), time)),
            Err(()) => self.cacheable = false,
        }
        #[cfg(test)]
        PARSED.with(|n| n.set(n.get() + 1));
        std::fs::read_to_string(path).ok()?.parse().ok()
    }
}

fn is_fresh(stamps: &[Stamp]) -> bool {
    stamps.iter().all(|(path, time)| mtime(path) == Ok(*time))
}

/// The name (as an identifier) under which the crate at `manifest_dir` depends
/// on `package` through `workspace = true`, if it does.
pub fn inherited_dependency_name(manifest_dir: &Path, package: &str) -> Option<String> {
    let key = (manifest_dir.to_path_buf(), package.to_string());
    let hit = CACHE.with(|cache| {
        cache
            .borrow()
            .get(&key)
            .filter(|cached| is_fresh(&cached.stamps))
            .map(|cached| cached.name.clone())
    });
    if let Some(name) = hit {
        return name;
    }
    let mut manifests = Manifests::new();
    let name = lookup(&mut manifests, manifest_dir, package);
    if manifests.cacheable {
        let stamps = manifests.stamps;
        CACHE.with(|cache| {
            cache.borrow_mut().insert(
                key,
                Cached {
                    stamps,
                    name: name.clone(),
                },
            )
        });
    }
    name
}

fn lookup(manifests: &mut Manifests, manifest_dir: &Path, package: &str) -> Option<String> {
    let manifest = manifests.read(&manifest_dir.join("Cargo.toml"))?;
    let inherited = inherited_keys(&manifest);
    if inherited.is_empty() {
        return None;
    }
    let workspace = workspace_root(manifests, manifest_dir, &manifest)?;
    let table = workspace
        .get("workspace")?
        .as_table_like()?
        .get("dependencies")?
        .as_table_like()?;
    inherited
        .into_iter()
        .find(|key| {
            table
                .get(key)
                .map_or(false, |dep| package_of(key, dep) == package)
        })
        .map(|key| key.replace('-', "_"))
}

/// The package that the dependency `key = dep` refers to: its `package` field,
/// or the key itself.
fn package_of<'a>(key: &'a str, dep: &'a Item) -> &'a str {
    dep.as_table_like()
        .and_then(|t| t.get("package"))
        .and_then(Item::as_str)
        .unwrap_or(key)
}

/// Adds the dependency tables of `parent` (a manifest, or a `[target.'cfg']`
/// table) to `out`.
fn push_dependency_tables<'a>(parent: &'a dyn TableLike, out: &mut Vec<&'a dyn TableLike>) {
    // Not "build-dependencies": see the module documentation.
    for kind in ["dependencies", "dev-dependencies"] {
        if let Some(table) = parent.get(kind).and_then(Item::as_table_like) {
            out.push(table);
        }
    }
}

/// The keys of the dependencies with `workspace = true`, in the dependency
/// tables of the manifest that its code can use.
fn inherited_keys(manifest: &Document) -> Vec<String> {
    let mut tables: Vec<&dyn TableLike> = Vec::new();
    push_dependency_tables(manifest.as_table(), &mut tables);
    if let Some(targets) = manifest.get("target").and_then(Item::as_table_like) {
        for (_, cfg) in targets.iter() {
            if let Some(cfg) = cfg.as_table_like() {
                push_dependency_tables(cfg, &mut tables);
            }
        }
    }
    let mut keys = Vec::new();
    for table in tables {
        for (key, dep) in table.iter() {
            let inherits = dep
                .as_table_like()
                .and_then(|t| t.get("workspace"))
                .and_then(Item::as_bool)
                == Some(true);
            if inherits {
                keys.push(key.to_string());
            }
        }
    }
    keys
}

/// The manifest of the workspace root of the member at `manifest_dir`: the one
/// `package.workspace` names, else the nearest one (the member's own included)
/// that has a `[workspace]` table.
fn workspace_root(
    manifests: &mut Manifests,
    manifest_dir: &Path,
    manifest: &Document,
) -> Option<Document> {
    if let Some(root) = manifest
        .get("package")
        .and_then(Item::as_table_like)
        .and_then(|p| p.get("workspace"))
        .and_then(Item::as_str)
    {
        return manifests.read(&manifest_dir.join(root).join("Cargo.toml"));
    }
    manifest_dir
        .ancestors()
        .map(|dir| dir.join("Cargo.toml"))
        .find_map(|path| {
            manifests
                .read(&path)
                .filter(|doc| doc.get("workspace").is_some())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys(manifest: &str) -> Vec<String> {
        inherited_keys(&manifest.parse().unwrap())
    }

    #[test]
    fn inherited_keys_of_every_spelling() {
        let manifest = r#"
[package]
name = "member"

[dependencies]
a = { workspace = true }
b.workspace = true
c = "1"
d = { version = "1", workspace = false }

[dev-dependencies.e]
workspace = true

[target.'cfg(unix)'.dependencies]
g = { workspace = true }

[target.'cfg(unix)'.dev-dependencies]
h.workspace = true
"#;
        let mut found = keys(manifest);
        found.sort();
        assert_eq!(found, ["a", "b", "e", "g", "h"]);
    }

    #[test]
    fn build_dependencies_are_not_inherited_keys() {
        // A build-dependency is not linked into the crate's code.
        let manifest = r#"
[package]
name = "member"

[build-dependencies]
f = { workspace = true, features = ["x"] }

[build-dependencies.i]
workspace = true

[target.'cfg(unix)'.build-dependencies]
j.workspace = true
"#;
        assert_eq!(keys(manifest), Vec::<String>::new());
    }

    #[test]
    fn package_field_or_key() {
        let ws: Document = r#"
[workspace.dependencies]
nt = { package = "newer-type", version = "0.4" }
plain = "1"
other = { version = "1" }
"#
        .parse()
        .unwrap();
        let t = ws["workspace"]["dependencies"].as_table_like().unwrap();
        assert_eq!(package_of("nt", t.get("nt").unwrap()), "newer-type");
        assert_eq!(package_of("plain", t.get("plain").unwrap()), "plain");
        assert_eq!(package_of("other", t.get("other").unwrap()), "other");
    }

    /// A workspace in a scratch directory: the root manifest has the
    /// `[workspace.dependencies]`, the member inherits from it.
    fn scratch_workspace(name: &str, member: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "newer-type-cratepath-{}-{}",
            name,
            std::process::id()
        ));
        let dir = root.join("member");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            root.join("Cargo.toml"),
            r#"
[workspace]
members = ["member"]

[workspace.dependencies]
nt-renamed = { package = "newer-type", version = "0.4" }
other = "1"
"#,
        )
        .unwrap();
        std::fs::write(dir.join("Cargo.toml"), member).unwrap();
        dir
    }

    #[test]
    fn follows_workspace_inheritance_to_the_root() {
        let dir = scratch_workspace(
            "found",
            "[package]\nname = \"m\"\n[dependencies]\nother.workspace = true\nnt-renamed = { workspace = true }\n",
        );
        assert_eq!(
            inherited_dependency_name(&dir, "newer-type").as_deref(),
            Some("nt_renamed")
        );
        assert_eq!(inherited_dependency_name(&dir, "absent"), None);
        std::fs::remove_dir_all(dir.parent().unwrap()).unwrap();
    }

    #[test]
    fn ignores_what_the_member_does_not_inherit() {
        // `nt-renamed` is in the workspace, but the member does not use it.
        let dir = scratch_workspace(
            "unused",
            "[package]\nname = \"m\"\n[dependencies]\nother.workspace = true\n",
        );
        assert_eq!(inherited_dependency_name(&dir, "newer-type"), None);
        std::fs::remove_dir_all(dir.parent().unwrap()).unwrap();
    }

    #[test]
    fn a_build_dependency_only_rename_is_not_picked() {
        // `nt-renamed` is in the workspace, but the member takes it as a
        // build-dependency only: `::nt_renamed` would not resolve in its code.
        let dir = scratch_workspace(
            "build-only",
            "[package]\nname = \"m\"\n[build-dependencies]\nnt-renamed = { workspace = true }\n\
             [target.'cfg(unix)'.build-dependencies]\nnt-renamed = { workspace = true }\n",
        );
        assert_eq!(inherited_dependency_name(&dir, "newer-type"), None);
        std::fs::remove_dir_all(dir.parent().unwrap()).unwrap();
    }

    #[test]
    fn a_dev_dependency_rename_is_picked() {
        let dir = scratch_workspace(
            "dev",
            "[package]\nname = \"m\"\n[dev-dependencies]\nnt-renamed.workspace = true\n",
        );
        assert_eq!(
            inherited_dependency_name(&dir, "newer-type").as_deref(),
            Some("nt_renamed")
        );
        std::fs::remove_dir_all(dir.parent().unwrap()).unwrap();
    }

    /// Rewrites `path` until its modification time differs from `old`
    /// (timestamps can be coarse).
    fn rewrite_until_newer(path: &Path, content: &str, old: SystemTime) {
        for _ in 0..200 {
            std::fs::write(path, content).unwrap();
            if std::fs::metadata(path).unwrap().modified().unwrap() != old {
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        panic!("the modification time of {:?} did not change", path);
    }

    fn parsed() -> usize {
        PARSED.with(|n| n.get())
    }

    #[test]
    fn caches_until_a_manifest_changes() {
        let member = "[package]\nname = \"m\"\n[dependencies]\nnt-renamed.workspace = true\n";
        let dir = scratch_workspace("cache", member);
        let found = || inherited_dependency_name(&dir, "newer-type");

        assert_eq!(found().as_deref(), Some("nt_renamed"));
        let after_first = parsed();
        assert!(after_first > 0);
        // Unchanged manifests: answered from the cache, nothing is parsed.
        assert_eq!(found().as_deref(), Some("nt_renamed"));
        assert_eq!(found().as_deref(), Some("nt_renamed"));
        assert_eq!(parsed(), after_first);
        // Another package is another entry.
        assert_eq!(inherited_dependency_name(&dir, "absent"), None);
        assert!(parsed() > after_first);

        // The member stops inheriting: the entry is stale.
        let member_path = dir.join("Cargo.toml");
        let old = std::fs::metadata(&member_path).unwrap().modified().unwrap();
        rewrite_until_newer(&member_path, "[package]\nname = \"m\"\n", old);
        let before = parsed();
        assert_eq!(found(), None);
        assert!(parsed() > before);

        // The member inherits again, then the workspace root drops the
        // rename: stale again.
        let old = std::fs::metadata(&member_path).unwrap().modified().unwrap();
        rewrite_until_newer(&member_path, member, old);
        assert_eq!(found().as_deref(), Some("nt_renamed"));
        let root_path = dir.parent().unwrap().join("Cargo.toml");
        let old = std::fs::metadata(&root_path).unwrap().modified().unwrap();
        rewrite_until_newer(
            &root_path,
            "[workspace]\nmembers = [\"member\"]\n[workspace.dependencies]\nnt-renamed = \"1\"\n",
            old,
        );
        assert_eq!(found(), None);

        // A missing manifest is not an error, and is not cached as present.
        std::fs::remove_dir_all(dir.parent().unwrap()).unwrap();
        assert_eq!(found(), None);
    }
}
