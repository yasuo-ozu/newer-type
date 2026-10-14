# Changelog

## [0.4.0] — Unreleased

### Removed

- The `#[target]` attribute. A trait for `#[implement]` is announced with the `announce` attribute
  of the [announce](https://docs.rs/announce) crate instead, which newer-type re-exports:
  `#[newer_type::announce::announce(crate = ::newer_type::announce)]` (just `#[announce]` in a
  crate that depends on `announce`). `#[target]` did not validate the trait beyond its type
  analysis, which `#[announce]` does itself, so there is no replacement check macro. The options
  map as follows:

  | `#[target(..)]` | now |
  | --- | --- |
  | `#[target]` | `#[announce]` |
  | `repeater = Repeater` (a hand-written trait) | `repeater = crate::Repeater`, with `announce::define_repeater!(pub Repeater)`; needed only for relative types |
  | no `repeater` (one was made up) | relative types need a `repeater` or `allow_std_prelude = true`, or an absolute path |
  | `alternative = Path` | `mock = Path`; the empty `unsafe` marker trait of `alternative` is gone |
  | `newer_type = Path` | `crate = Path` names the `announce` runtime. `#[implement(.., newer_type = Path)]` is new in 0.4 (see Added) |

- The crate `newer-type-std`, which only re-exported the std trait mocks of
  [announce-std](https://docs.rs/announce-std) (version-gated by the rustc in use). Name the
  traits through `announce-std` directly, with `announce-std = "0.1"` in `Cargo.toml`:
  `#[implement(::announce_std::fmt::Display)]` instead of
  `#[implement(newer_type_std::fmt::Display)]` (the modules and names are the same).
- `newer_type_std::Repeater`, which only served `#[target(repeater = ..)]`.
- The `type-leak` dependency of `newer-type-macro` (relative types are analysed by announce, which
  newer-type reads through `announce::type_leak`).

### Changed

- Traits travel to `#[implement]` through [announce](https://docs.rs/announce) 0.1
  trampolines instead of newer-type's own `macro_rules!`. The macro `Trait!` keeps its name.
  Traits defined with newer-type 0.3 cannot be used by 0.4 and vice versa.
- A trait announced with `repeater = ..` has its relative types rewritten to the repeater's
  projections with the arguments of the implementor (`#[implement(Trait<u8>)]`,
  `#[implement(for<U> Trait<U>)]`, omitted defaults), as announce's `type_leak` table describes.
- `newer-type` depends on `announce` without its default feature `proc-macro-api` (syn,
  proc-macro2 and type-leak, for proc-macro crates), so it builds for `#![no_std]` crates and
  targets without std (`default-features = false`; this needs resolver 2, the default of
  edition 2021). `newer-type-macro` runs on the host and keeps the feature.
- MSRV is 1.60 (was 1.56): announce needs 1.57, and `proc-macro-crate` (renamed-dependency lookup)
  needs 1.60. The GAT-based tests still need 1.65. Building on 1.60 needs an MSRV-aware
  resolver (or a lockfile written with one): a plain `cargo generate-lockfile` picks versions that
  need a newer rustc (`toml_edit 0.19.15` needs 1.66; `proc-macro2`, `syn` and `unicode-ident` need
  1.71). Resolve with `CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=fallback cargo generate-lockfile`
  (cargo 1.84 or later), which settles on `toml_edit 0.19.8` among others. Pinning only
  `toml_edit` (`cargo update -p toml_edit --precise 0.19.8`) is not enough.
- `#[implement]` without a dependency on `newer-type` in the crate's `Cargo.toml` (a facade,
  or a manifest that does not describe the real build, such as those generated for Bazel or
  Buck2) generates code with `::newer_type`, as 0.3 did, instead of aborting. Pass
  `newer_type = ..` when that path does not resolve.
- The lookup of an inherited rename (`nt.workspace = true`) reads `[dependencies]` and
  `[dev-dependencies]` (and their `[target]` forms), like proc-macro-crate, not
  `[build-dependencies]`, and is cached per manifest modification time.

### Added

- `newer_type::announce`, a re-export of the `announce` crate. A crate that does not depend on
  `announce` itself must pass `crate = ::newer_type::announce` to `#[announce]`. The re-export is
  public API: `announce` 0.2 would be a breaking change of `newer-type` as well.
- `#[implement(.., newer_type = PATH)]` sets the path to newer-type for the generated code, in any
  position (new in 0.4: `#[implement]` did not take it before). It defaults to the name under
  which the implementing crate's `Cargo.toml` depends on `newer-type`: renamed dependencies work,
  also those inherited from `[workspace.dependencies]` with `workspace = true`. A crate with no
  such dependency gets `::newer_type`; one that reaches newer-type only through a facade must pass
  `newer_type = ..`.
- Any `#[announce]` trait can be an `#[implement]` target, including one announced as a copy in an
  inline module of the same name (`emit_item = false`), whose paths resolve where `#[implement]`
  is used.

### Fixed

- Nested `#[cfg]`s in an announced trait are evaluated in the defining crate.
- `#[implement]` on an enum for a trait whose method takes `Self` in several parameters
  (`#[implement(::announce_std::cmp::PartialEq)] enum E { A(u8), B(u8) }`) generated code that did
  not compile (`expected &u8, found &E`). It now reports that such a trait cannot be forwarded for
  an enum, because the arguments can be of different variants. A struct is not affected.
