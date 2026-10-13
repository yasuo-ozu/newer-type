#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("./README.md")]

// internal
#[doc(hidden)]
pub use newer_type_macro::__implement_internal;

/// Re-export of the [`announce`](https://docs.rs/announce) crate, so that a crate can announce
/// its traits for [`implement`] without depending on `announce` itself:
///
/// ```text
/// #[newer_type::announce::announce(crate = ::newer_type::announce)]
/// pub trait MyTrait { /* .. */ }
/// ```
///
/// **The option `crate = ::newer_type::announce` is required** in a crate that
/// has no dependency on `announce` of its own: `#[announce]` takes the path to
/// its runtime from the dependency named `announce` in the crate's
/// `Cargo.toml`, and a crate that has none fails with errors that do not name
/// the cause (``cannot find `announce` in the list of imported crates``,
/// ``cannot find attribute `__announce` in this scope``). A crate that depends
/// on `announce` directly can omit the option. See [`implement`] for the rules
/// of an announced trait.
pub use announce;

/// Implement a trait for given enum or struct. The trait should be announced
/// with the [`announce`](macro@announce::announce) attribute.
///
/// `newer_type = PATH` (in any position) sets the path to this crate for the
/// generated code. By default it is the name under which the calling crate's
/// `Cargo.toml` depends on `newer-type` (so a renamed dependency
/// `nt = { package = "newer-type", .. }` works). A crate that reaches
/// newer-type only through a facade has no such dependency and must pass the
/// path: `#[implement(Trait, newer_type = ::facade::newer_type)]`; without it,
/// the macro reports an error with this hint.
///
/// # Example
///
/// The traits of `std` are named through [`announce-std`](https://docs.rs/announce-std):
///
/// ```
/// use newer_type::implement;
/// use announce_std::{cmp::PartialEq, iter::Extend};
///
/// #[implement(Extend<usize>)]
/// struct Example1(Vec<usize>);
///
/// #[implement(Extend<T>)]
/// struct Example2<T>(Vec<T>);
///
/// #[implement(for<T> PartialEq<T>)]
/// struct Example3(String);
///
/// #[implement(for<T: std::fmt::Debug> PartialEq<T>)]
/// struct Example4<U>(U);
/// ```
///
/// # Announcing a trait
///
/// `#[implement]` reads the trait through the trampoline that
/// [`#[announce]`](macro@announce::announce) emits next to it. Announce the
/// trait where it is defined; the crate that uses the trait needs nothing else.
///
/// ```
/// use newer_type::{announce::announce, implement};
///
/// // `crate = ..` is needed when the crate does not depend on `announce` itself.
/// // `allow_std_prelude` lets the signatures name `String` without a path.
/// #[announce(crate = ::newer_type::announce, allow_std_prelude = true)]
/// trait SayHello {
///     fn say_hello(&self) -> String;
/// }
///
/// impl SayHello for String {
///     fn say_hello(&self) -> String {
///         format!("Hello, {}!", self)
///     }
/// }
///
/// #[implement(SayHello)]
/// struct MyName(String);
///
/// assert_eq!(MyName("world".into()).say_hello(), "Hello, world!");
/// ```
///
/// The options of `#[announce]` that matter to `#[implement]`:
///
/// - `mock = path`: implement the trait at `path` instead of the announced one.
///   The announced trait mirrors the real one (this is how the traits of `std` are
///   made available: see [`announce-std`](https://docs.rs/announce-std)).
/// - `repeater = crate::Repeater`: the signatures may name types that do not
///   resolve outside the defining crate (see below).
/// - `crate = path`: the path to the `announce` crate; `::newer_type::announce`
///   for a crate without a direct dependency on `announce`.
/// - `allow_std_prelude = true`, `allow_primitive = false`: which bare names
///   are accepted without a repeater.
///
/// # Paths in an announced trait
///
/// `#[announce]` hands the trait to other crates, so it requires every path in
/// it to mean the same there; it reports a path that would not resolve at the
/// trait. This is the rule of announce, not of newer-type:
///
/// - Traits (supertraits, bounds of generics) are written with absolute paths:
///   `crate::Base`, `::core::fmt::Debug`.
/// - Types are primitive, absolute (`::std::string::String`, `crate::Local`),
///   or the trait is announced with `allow_std_prelude = true` (then `String`,
///   `Vec<u8>`, `Option<T>` and the like are fine) or with a `repeater`, which
///   carries the relative types to the other crate:
///
/// ```
/// use newer_type::{announce::{announce, define_repeater}, implement};
///
/// // At a path that the users of the trait can reach.
/// define_repeater!(pub Repeater);
///
/// type TypeFromContext = usize;
///
/// #[announce(crate = ::newer_type::announce, repeater = crate::Repeater)]
/// trait MyTrait {
///     fn my_fn(&self, t: TypeFromContext) -> Box<usize>;
/// }
///
/// impl MyTrait for u8 {
///     fn my_fn(&self, t: TypeFromContext) -> Box<usize> {
///         Box::new(*self as usize + t)
///     }
/// }
///
/// #[implement(MyTrait)]
/// struct Wrap(u8);
///
/// fn main() {
///     assert_eq!(*Wrap(1).my_fn(2), 3);
/// }
/// ```
///
/// A trait that wants its paths resolved where `#[implement]` is used can be
/// announced as a copy in an inline module of the same name
/// (`#[announce(emit_item = false)] mod T { trait T { .. } }`),
/// which `#[implement]` unwraps. The relative paths of the copy are then those
/// of the user's scope.
pub use newer_type_macro::implement;
