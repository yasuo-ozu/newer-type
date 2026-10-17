# Changelog

## [Unreleased]

### Changed

- Built on [announce](https://docs.rs/announce) 0.2 (and `announce-std = "0.2"` for the traits of
  `std`). `newer_type::announce` is the `announce` 0.2 crate, so its changes apply to a trait
  announced through the re-export: several `#[announce]`s on one trait merge, and
  `#[derive(Announce)]` is gone (see the announce changelog). `#[implement]` is unchanged, and the
  MSRV stays 1.60.

## [0.4.1] — 2026-10-15

### Added

- Associated type bindings in `#[implement]`:
  `#[implement(::announce_std::iter::Iterator<Item = u8>)]`, `#[implement(Conv<u8, Out = String>)]`,
  `#[implement(for<T> Conv<T, Out = T>)]`. The binding is not part of the trait in the header of
  the impl: it constrains the wrapped types (`Inner: Trait<Item = u8>`) and sets the associated
  type of the impl. For an enum it is used instead of the type parameter that is made up for the
  variants to share. Bounds (`Item: Bound`), const bindings and bindings of generic associated
  types are rejected with an explanatory message, as are a binding of an associated type that the
  trait does not have, and a type bound twice.
- `async fn` in an announced trait is forwarded to the wrapped type with `.await` (a trait with
  `async fn` needs Rust 1.75). The generated impl did not compile before.

## [0.4.0] — 2026-10-15

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
- MSRV is 1.60 (was 1.56): announce needs 1.57, and its `define_repeater!` needs 1.59. The
  GAT-based tests still need 1.65. Building on 1.60 needs an MSRV-aware resolver (or a lockfile
  written with one): a plain `cargo generate-lockfile` picks versions that need a newer rustc
  (`proc-macro2`, `syn` and `unicode-ident` need 1.71). Resolve with
  `CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=fallback cargo generate-lockfile` (cargo 1.84 or
  later), which settles on older versions of them.

### Added

- `newer_type::announce`, a re-export of the `announce` crate. A crate that does not depend on
  `announce` itself must pass `crate = ::newer_type::announce` to `#[announce]`. The re-export is
  public API: `announce` 0.2 would be a breaking change of `newer-type` as well.
- `#[implement(.., newer_type = PATH)]` sets the path to newer-type for the generated code, in any
  position (new in 0.4: `#[implement]` did not take it before). The default is `::newer_type`, as
  in 0.3, which does not resolve in a crate that depends on `newer-type` under another name
  (`nt = { package = "newer-type", .. }`) or reaches it only through a facade: such a crate must
  pass `newer_type = ::nt` or `newer_type = ::facade::newer_type`.
- Any `#[announce]` trait can be an `#[implement]` target, including one announced as a copy in an
  inline module of the same name (`emit_item = false`), whose paths resolve where `#[implement]`
  is used.

### Fixed

- Nested `#[cfg]`s in an announced trait are evaluated in the defining crate.
- `#[implement]` on an enum for a trait whose method takes `Self` in several parameters
  (`#[implement(::announce_std::cmp::PartialEq)] enum E { A(u8), B(u8) }`) generated code that did
  not compile (`expected &u8, found &E`). It now reports that such a trait cannot be forwarded for
  an enum, because the arguments can be of different variants. A struct is not affected.
