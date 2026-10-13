#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("./README.md")]

// internal
pub use newer_type_macro::__implement_internal;

#[doc(hidden)]
pub mod __private {
    pub use ::announce;
}

/// Implement a trait for given enum or struct. The trait should be defined with
/// [`target`] attribute.
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
/// ```ignore
/// use newer_type::implement;
/// use newer_type_std::{ops::Extend, cmp::PartialEq};
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
pub use newer_type_macro::implement;

/// Define a trait for use of [`implement`] macro.
///
/// # Arguments (all optional)
///
/// - `alternative` ... Trait. If specified, implement this trait instead of the
///   target trait itself. The target trait is used only for an argument of
///   [`implement`] macro.
/// - `newer_type` ... Set path to `newer_type` crate. Defaults to the name
///   under which the defining crate's `Cargo.toml` depends on `newer-type`
///   (`::newer_type` unless renamed). Example:
///   `::your_crate::_export::newer_type`. The path is resolved **in the
///   defining crate** (it locates the trampoline machinery for the trait); it
///   no longer tells [`implement`] where newer-type is — users of the trait
///   choose that with `#[implement(newer_type = ..)]`. A path that does not
///   resolve gives an error at the attribute, followed by a spurious `E0428`
///   ("the name `Trait` is defined multiple times") for the hidden wrapper
///   module; fix the first error.
/// - `repeater` ... Absolute path to the `Repeater` crate. see the example
///   section. The `Repeater` trait is defined in the same crate that the target
///   trait is defined, and should be visible from the users, which refer to the
///   trait with `#[implement]` macro.
///
/// # Example
///
/// ```
/// use newer_type::target;
///
/// pub trait Repeater<T: ?Sized, const TRAIT_ID : u64, const NTH : usize> {
///     type Type;
/// }
///
/// #[target(repeater = Repeater)]
/// trait MyTrait {
///     fn my_fn(&self) -> ::core::primitive::usize;
/// }
/// ```
///
/// ```
/// use newer_type::target;
/// type TypeFromContext = usize;
///
/// pub trait Repeater<T: ?Sized, const TRAIT_ID : u64, const NTH : usize> {
///     type Type;
/// }
///
/// #[target(repeater = Repeater)]
/// trait MyTrait {
///     fn my_fn(&self, t: TypeFromContext) -> Box<usize>;
/// }
/// ```
///
/// We recomend this pattern to set `repeater` path correctly.
///
/// ```ignore
/// use newer_type::target;
/// type TypeFromContext = usize;
///
/// // placed in crate root
/// pub trait Repeater<T: ?Sized, const TRAIT_ID : u64, const NTH : usize> {
///     type Type;
/// }
///
/// macro_rules! emit_trait {
///     () => {
///         #[target(repeater = $crate::Repeater)]
///         trait MyTrait {
///             fn my_fn(&self, t: TypeFromContext) -> Box<usize>;
///         }
///     };
/// }
/// emit_trait!();
/// ```
pub use newer_type_macro::target;
