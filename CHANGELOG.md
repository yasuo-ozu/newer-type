# Changelog

## [0.4.0] — Unreleased

### Changed

- `#[target]` traits travel to `#[implement]` through [announce](https://docs.rs/announce) 0.1
  trampolines instead of newer-type's own `macro_rules!`. `#[target]` emits the trait plus
  `#[announce(emit_item = false, meta = __target(..))] <vis> mod Trait { <trait> }`; the macro
  `Trait!` keeps its name. Traits defined with newer-type 0.3 cannot be used by 0.4 and vice
  versa.
- `newer-type-std` re-exports the std trait mocks of `announce-std` (version-gated by the rustc in
  use) instead of its own copies.
- The path to newer-type used by `#[implement]` is chosen by the implementing crate:
  `#[implement(.., newer_type = PATH)]`, defaulting to the name under which that crate's
  `Cargo.toml` depends on `newer-type` (renamed dependencies work). A crate that reaches
  newer-type only through a facade must pass `newer_type = ..`; otherwise the macro reports an
  error with that hint. `newer_type = ..` may appear in any position.
- `#[target(newer_type = P)]` is resolved in the defining crate (default: resolved from its
  `Cargo.toml`, as for `#[implement]`).
- MSRV is 1.60 (was 1.56): announce needs 1.57, and `proc-macro-crate` (renamed-dependency lookup)
  needs 1.60. The GAT-based tests still need 1.65.

### Added

- Any `#[announce]` trait (with absolute signatures) can be an `#[implement]` target.

### Fixed

- Nested `#[cfg]`s in a `#[target]` trait are evaluated in the defining crate.
