#![doc = include_str!("./README.md")]

// internal
pub use newer_type_macro::__implement_internal;

/// Implement a trait for given enum or struct. The trait should be defined with
/// [`target`] attribute.
///
/// # Example
///
/// ```
/// use newer_type::implement;
/// use newer_type::traits::{Extend, PartialEq};
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
///   [`implement`] macro. See implementation of [`traits`].
/// - `newer_type` ... Set path to `newer_type` crate. Defaults to
///   `::newer_type`. Example: `::your_crate::_export::newer_type`.
///
///   # Example
///
///   ```
///   use newer_type::target;
///
///   #[target]
///   trait MyTrait {
///       fn my_fn(&self) -> ::core::primitive::usize;
///   }
///   ```
///
///   ```
///   use newer_type::target;
///   type TypeFromContext = usize;
///   #[target]
///   trait MyTrait {
///       fn my_fn(&self, t: TypeFromContext) -> Box<usize>;
///   }
///   ```
pub use newer_type_macro::target;

#[doc(hidden)]
pub trait Repeater<const TRAIT_NUM: u64, const N: usize> {
    type Type;
}
