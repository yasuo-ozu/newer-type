# newer-type crate [![Latest Version]][crates.io] [![Documentation]][docs.rs] [![GitHub Actions]][actions]

[Latest Version]: https://img.shields.io/crates/v/newer-type.svg
[crates.io]: https://crates.io/crates/newer-type
[Documentation]: https://img.shields.io/docsrs/newer-type
[docs.rs]: https://docs.rs/newer-type/latest/
[GitHub Actions]: https://github.com/yasuo-ozu/newer-type/actions/workflows/rust.yml/badge.svg
[actions]: https://github.com/yasuo-ozu/newer-type/actions/workflows/rust.yml

**Ergonomic support for the newtype pattern in Rust, with automatic trait implementations.**

The newtype pattern in Rust is useful for creating distinct types without runtime overhead. However, it often requires boilerplate code to re-implement traits of the inner type.

The `newer-type` crate provides a procedural macro `#[implement(...)]` to reduce that boilerplate by automatically implementing traits for your wrapper types.

## Features

The `#[implement(...)]` macro currently supports automatic implementations for:

- User-defined traits announced with `#[announce]` (re-exported as `newer_type::announce`)
- Many traits from Rust `std`, named through the [`announce-std`](https://docs.rs/announce-std) crate
  (`#[implement(::announce_std::fmt::Display)]`, see below)

## Example

### Without `newer-type` (manual newtype definition)

```rust
trait SayHello {
    fn say_hello(&self) -> String;
}

impl SayHello for String {
    fn say_hello(&self) -> String {
        format!("Hello, {}!", self)
    }
}

pub struct MyName(String);

impl SayHello for MyName {
    fn say_hello(&self) -> String {
        self.0.say_hello()
    }
}
```

### With `newer-type` crate

```rust
use newer_type::{announce::announce, implement};

// `crate = ::newer_type::announce` is required when your crate does not depend
// on `announce` itself (see the note below).
// `allow_std_prelude` lets the signatures name `String` without a path.
#[announce(crate = ::newer_type::announce, allow_std_prelude = true)]
trait SayHello {
    fn say_hello(&self) -> String;
}

impl SayHello for String {
    fn say_hello(&self) -> String {
        format!("Hello, {}!", self)
    }
}

#[implement(SayHello)]
pub struct MyName(String);
```

That's it! The selected traits are automatically implemented for you.

> **Note: `crate = ::newer_type::announce`.** `newer-type` re-exports `announce`, so a crate needs
> no dependency on it, but then `#[announce]` must be told where its runtime is:
> `#[newer_type::announce::announce(crate = ::newer_type::announce)]`. Without the option the
> macro looks for a dependency named `announce` in your `Cargo.toml`, and a crate that has none
> fails with errors that do not name the cause: ``cannot find `announce` in the list of imported
> crates`` and ``cannot find attribute `__announce` in this scope``. If you see either, add
> `crate = ::newer_type::announce` (or depend on `announce` directly and drop the option).

### Announcing a trait

A trait is made available to `#[implement]` by announcing it with the
[`announce`](https://docs.rs/announce) attribute, which this crate re-exports so that a crate
without a dependency on `announce` can write
`#[newer_type::announce::announce(crate = ::newer_type::announce)]`. `#[announce]` emits a
`macro_rules!` with the name of the trait next to it; `#[implement(SayHello)]` calls it to read
the trait, in any crate.

`#[announce]` hands the signatures to other crates, so every path in the trait must mean the same
there, and it reports one that would not, at the trait:

- Traits (supertraits, bounds) are written with absolute paths: `crate::Base`, `::core::fmt::Debug`.
- Types are primitive, absolute (`::std::string::String`, `crate::Local`), or the trait is
  announced with `allow_std_prelude = true` (then `String`, `Vec<u8>`, `Option<T>` and the like are
  fine) or with a `repeater`, which carries the types of the defining crate:

```rust
use newer_type::{announce::{announce, define_repeater}, implement};

define_repeater!(pub Repeater); // at a path that the users of the trait can reach

pub struct Meters(pub u32);

// `Meters` is relative: it means something only in this crate.
#[announce(crate = ::newer_type::announce, repeater = crate::Repeater)]
trait Measure {
    fn measure(&self) -> Meters;
}

impl Measure for u32 {
    fn measure(&self) -> Meters {
        Meters(*self)
    }
}

#[implement(Measure)]
struct Width(u32);

fn main() {
    assert_eq!(Width(3).measure().0, 3);
}
```

To implement another trait instead of the announced one, mirror the real trait and name it with
`mock = path`: `#[announce(mock = ::std::string::ToString)] trait ToString { .. }`. The mirrors
of the traits of `std` are in [`announce-std`](https://docs.rs/announce-std).

### Renamed dependencies and facades

The generated code refers to `newer-type` as `::newer_type`. If your crate depends on it under
another name (`nt = { package = "newer-type", .. }`), or reaches it through another crate that
re-exports it, `::newer_type` does not resolve there: pass the path to `newer-type` with
`newer_type = path` (in any position):

```rust,ignore
// `nt = { package = "newer-type", .. }` in Cargo.toml
#[nt::implement(SayHello, newer_type = ::nt)]
pub struct MyName(String);

// newer-type reached through the crate `facade`
#[facade::newer_type::implement(SayHello, newer_type = ::facade::newer_type)]
pub struct YourName(String);
```

### `no_std`

With `default-features = false`, `newer-type` builds for `#![no_std]` crates and for targets
without std (`cargo build --target x86_64-unknown-none`). It uses `announce` without its default
feature `proc-macro-api` (syn and the like, which need std); with resolver 2, the default of
edition 2021, that feature is enabled for the macro crate only, which runs on the host.

## Implementing the traits of `std`

The traits of Rust's standard library are mirrored by the
[`announce-std`](https://docs.rs/announce-std) crate (add `announce-std = "0.1"` to your
`Cargo.toml`). Name a trait through it, and `#[implement]` implements the real `std`/`core`
trait:

```rust
use announce_std::{cmp::{Eq, PartialEq}, iter::{Extend, IntoIterator}};
use newer_type::implement;

#[implement(IntoIterator, Extend<T>, PartialEq, Eq)]
struct MyVec<T>(Vec<T>);

// now `MyVec` implements std::iter::IntoIterator, std::iter::Extend, std::cmp::{PartialEq, Eq}
```

Or by full path, without `use`: `#[implement(::announce_std::fmt::Display)]`.

# Use Cases
This crate is particularly useful when:

- You want to create safe wrappers for existing types (e.g. for validation or domain modeling)
- You're working with abstract data structures like ASTs or instruction sets and want ergonomic iteration
- You frequently use the newtype pattern and want to avoid repetitive code

# Installation

Add this to your Cargo.toml:

```Cargo.toml
[dependencies]
newer-type = "0.4"
announce-std = "0.1" # only to implement the traits of `std`
```

# License

MIT
