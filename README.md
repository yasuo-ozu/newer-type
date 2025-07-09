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

- User-defined traits annotated with `#[target]`
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
# use newer_type::{implement, target};
#[target]
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

## Examples using `newer_type::traits`

In order to implement traits defined in Rust's standard library for your newtype, there are empty definitions
in `newer_type::traits` namespace. You can pick up traits to be implemented from it.

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

# License

MIT
