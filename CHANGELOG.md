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
  | `newer_type = Path` | `crate = Path` names the `announce` runtime; `#[implement(.., newer_type = Path)]` is unchanged |

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
- `newer-type-std` re-exports the std trait mocks of `announce-std` (version-gated by the rustc in
  use) instead of its own copies.
- The path to newer-type used by `#[implement]` is chosen by the implementing crate:
  `#[implement(.., newer_type = PATH)]`, defaulting to the name under which that crate's
  `Cargo.toml` depends on `newer-type` (renamed dependencies work). A crate that reaches
  newer-type only through a facade must pass `newer_type = ..`; otherwise the macro reports an
  error with that hint. `newer_type = ..` may appear in any position.
- MSRV is 1.60 (was 1.56): announce needs 1.57, and `proc-macro-crate` (renamed-dependency lookup)
  needs 1.60. The GAT-based tests still need 1.65.

### Added

- `newer_type::announce`, a re-export of the `announce` crate.
- Any `#[announce]` trait can be an `#[implement]` target, including one announced as a copy in an
  inline module of the same name (`emit_item = false`), whose paths resolve where `#[implement]`
  is used.

### Fixed

- Nested `#[cfg]`s in an announced trait are evaluated in the defining crate.
