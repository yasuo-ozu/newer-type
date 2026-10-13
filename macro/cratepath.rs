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
//! reached. The dependency tables are `dependencies`, `dev-dependencies`,
//! `build-dependencies` and their `[target.'cfg'.*]` forms.

use std::path::{Path, PathBuf};
use toml_edit::{Document, Item, TableLike};

/// The name (as an identifier) under which the crate at `manifest_dir` depends
/// on `package` through `workspace = true`, if it does.
pub fn inherited_dependency_name(manifest_dir: &Path, package: &str) -> Option<String> {
    let manifest_path = manifest_dir.join("Cargo.toml");
    let manifest = read(&manifest_path)?;
    let inherited = inherited_keys(&manifest);
    if inherited.is_empty() {
        return None;
    }
    let workspace = read(&workspace_root(manifest_dir, &manifest)?)?;
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

fn read(path: &Path) -> Option<Document> {
    std::fs::read_to_string(path).ok()?.parse().ok()
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
    for kind in ["dependencies", "dev-dependencies", "build-dependencies"] {
        if let Some(table) = parent.get(kind).and_then(Item::as_table_like) {
            out.push(table);
        }
    }
}

/// The keys of the dependencies with `workspace = true`, in all dependency
/// tables of the manifest.
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
fn workspace_root(manifest_dir: &Path, manifest: &Document) -> Option<PathBuf> {
    if let Some(root) = manifest
        .get("package")
        .and_then(Item::as_table_like)
        .and_then(|p| p.get("workspace"))
        .and_then(Item::as_str)
    {
        return Some(manifest_dir.join(root).join("Cargo.toml"));
    }
    manifest_dir
        .ancestors()
        .map(|dir| dir.join("Cargo.toml"))
        .find(|path| read(path).map_or(false, |doc| doc.get("workspace").is_some()))
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

[build-dependencies]
f = { workspace = true, features = ["x"] }

[target.'cfg(unix)'.dependencies]
g = { workspace = true }

[target.'cfg(unix)'.dev-dependencies]
h.workspace = true
"#;
        let mut found = keys(manifest);
        found.sort();
        assert_eq!(found, ["a", "b", "e", "f", "g", "h"]);
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
}
