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
- Many traits from Rust `std`, see [`newer_type_std`](https://docs.rs/newer-type-std/latest/newer_type-std/index.html) crate documentation

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

// `crate = ..` is needed when your crate does not depend on `announce` itself.
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
of the traits of `std` are in `newer-type-std`.

### Renamed dependencies and facades

The generated code finds `newer-type` under the name your `Cargo.toml` gives it, so a renamed
dependency (`nt = { package = "newer-type", .. }`, then `#[nt::implement(..)]`) works as is. If
your crate reaches newer-type only through another crate that re-exports it, pass the path:

```rust,ignore
#[facade::newer_type::implement(SayHello, newer_type = ::facade::newer_type)]
pub struct MyName(String);
```

## Examples using `newer_type::traits`

In order to implement traits defined in Rust's standard library for your newtype, there are definitions
in [`newer_type_std`](https://docs.rs/newer-type-std/latest/newer_type-std/index.html) crate (the std trait
copies of [`announce-std`](https://docs.rs/announce-std)). You can pick up traits to be implemented from it.

```rust,ignore
# use newer_type::implement;
use newer_type_std::{iter::IntoIterator, iter::Extend, cmp::PartialEq, cmp::Eq};

#[implement(IntoIterator, Extend<T>, PartialEq, Eq)]
struct MyVec<T>(Vec<T>);

// now `MyVec` implements std::iter::IntoIterator, std::ops::Extend, std::cmp::{PartialEq, Eq}
```

# Use Cases
This crate is particularly useful when:

- You want to create safe wrappers for existing types (e.g. for validation or domain modeling)
- You're working with abstract data structures like ASTs or instruction sets and want ergonomic iteration
- You frequently use the newtype pattern and want to avoid repetitive code

# Installation

Add this to your Cargo.toml:

```Cargo.toml
[dependencies]
newer-type = "0.1"
```

# Changes

`#[target]` is removed: a trait for `#[implement]` is now announced with
[`#[announce]`](https://docs.rs/announce), whose trampolines `#[implement]` reads. Every option
of `#[target(..)]` has a replacement, except the marker trait of `alternative`:

| `#[target(..)]` | now |
| --- | --- |
| `#[target]` | `#[announce]`, or `#[newer_type::announce::announce(crate = ::newer_type::announce)]` in a crate that does not depend on `announce` |
| `repeater = Repeater` (a hand-written `trait Repeater<T, const TRAIT_ID: u64, const NTH: usize>`) | `repeater = crate::Repeater`, with `announce::define_repeater!(pub Repeater)`. It is needed only for types that do not resolve outside the defining crate; absolute paths and primitive types need none, and `allow_std_prelude = true` accepts the bare names of the std prelude |
| no `repeater` (one was made up, so any relative type worked) | relative types are an error unless there is a `repeater` or `allow_std_prelude = true`, or the paths are written absolute (`crate::Local`, `::std::string::String`) |
| `alternative = Path` | `mock = Path`. The empty `unsafe` marker trait that `#[target]` emitted under the trait's name is gone: `#[implement]` implements the real trait only |
| `newer_type = Path` | the `announce` runtime is `crate = Path` (`::newer_type::announce`); `#[implement(.., newer_type = Path)]` is unchanged |
| `newer_type_std::Repeater` | removed (the traits of `newer-type-std` are those of `announce-std`) |

# License

MIT
