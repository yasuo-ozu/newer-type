#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("./README.md")]

// internal
pub use newer_type_macro::__implement_internal;

/// Implement a trait for given enum or struct. The trait should be defined with
/// [`target`] attribute.
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
/// - `newer_type` ... Set path to `newer_type` crate. Defaults to
///   `::newer_type`. Example: `::your_crate::_export::newer_type`.
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
/// pub trait Repeater<const TRAIT_ID : u64, const NTH : usize, T: ?Sized> {
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
/// pub trait Repeater<const TRAIT_ID : u64, const NTH : usize, T: ?Sized> {
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
/// pub trait Repeater<const TRAIT_ID : u64, const NTH : usize, T: ?Sized> {
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
