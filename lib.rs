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
pub struct Alternate(::core::convert::Infallible);

#[doc(hidden)]
pub trait Repeater<const TRAIT_NUM: u64, const N: usize> {
    type Type;
}

pub mod traits {
    use super::*;
    #[cfg(doc)]
    use crate as newer_type;

    macro_rules! emit_traits {
        () => {};
        (
            $(#[doc = $doc_example:literal])*
            $([$($trait_params:ident),*$(,)?])?{
                #[target(alternative = $alternative:path)]
                pub trait $trait_name:ident $($trait_contents:tt)*
            }
            $($t:tt)*
        ) => {
            #[target(alternative = $alternative, newer_type = $crate)]
            #[doc = concat!("This trait is empty declaration of [`", stringify!($alternative), "`] to be used")]
            #[doc = "with [`newer_type::implement`]."]
            ///
            /// # Example
            ///
            $(#[doc = $doc_example])*
            pub trait $trait_name $($trait_contents)*
            emit_traits!{ $($t)* }
        };
    }

    emit_traits! {
        /// ```
        /// # use newer_type::implement;
        /// #[implement(newer_type::traits::IntoIterator)]
        /// struct MyStruct {
        ///     slot: Vec<u8>,
        /// }
        /// ```
        {
            #[target(alternative = ::core::iter::IntoIterator)]
            pub trait IntoIterator {
                type Item;
                type IntoIter: ::core::iter::Iterator<Item = Self::Item>;
                fn into_iter(self) -> Self::IntoIter;
            }
        }

        /// ```
        /// # use newer_type::implement;
        /// #[implement(for<A> newer_type::traits::Extend<A>)]
        /// struct MyStruct {
        ///     slot: Vec<u8>,
        /// }
        /// ```
        [A]{
            #[target(alternative = ::core::iter::Extend)]
            pub trait Extend<A> {
                fn extend<T>(&mut self, iter: T)
                where
                    T: ::core::iter::IntoIterator<Item = A>;
            }
        }

        /// ```
        /// # use newer_type::implement;
        /// #[implement(newer_type::traits::Iterator)]
        /// struct MyStruct {
        ///     slot: std::vec::IntoIter<u8>,
        /// }
        /// ```
        {
            #[target(alternative = ::core::iter::Iterator)]
            pub trait Iterator {
                type Item;
                fn next(&mut self) -> ::core::option::Option<Self::Item>;
                fn size_hint(
                    &self,
                ) -> (
                ::core::primitive::usize,
                ::core::option::Option<::core::primitive::usize>,
                );
                fn count(self) -> ::core::primitive::usize
                where
                    Self: ::core::marker::Sized;
                fn last(self) -> ::core::option::Option<Self::Item>
                where
                    Self: ::core::marker::Sized;
                fn nth(&mut self, n: ::core::primitive::usize) -> ::core::option::Option<Self::Item>;
            }
        }

        /// ```
        /// # use newer_type::implement;
        /// #[implement(newer_type::traits::Iterator, newer_type::traits::FusedIterator)]
        /// struct MyStruct {
        ///     slot: std::vec::IntoIter<u8>,
        /// }
        /// ```
        {
            #[target(alternative = ::core::iter::FusedIterator)]
            pub trait FusedIterator: ::core::iter::Iterator {}
        }

        /// ```
        /// # use newer_type::implement;
        /// #[implement(newer_type::traits::Iterator, newer_type::traits::ExactSizeIterator)]
        /// struct MyStruct {
        ///     slot: std::vec::IntoIter<u8>,
        /// }
        /// ```
        {
            #[target(alternative = ::core::iter::ExactSizeIterator)]
            pub trait ExactSizeIterator: ::core::iter::Iterator {
                fn len(&self) -> ::core::primitive::usize;
            }
        }

        /// ```
        /// # use newer_type::implement;
        /// #[implement(newer_type::traits::Iterator, newer_type::traits::DoubleEndedIterator)]
        /// struct MyStruct {
        ///     slot: std::vec::IntoIter<u8>,
        /// }
        /// ```
        {
            #[target(alternative = ::core::iter::DoubleEndedIterator)]
            pub trait DoubleEndedIterator: ::core::iter::Iterator {
                fn next_back(&mut self) -> ::core::option::Option<Self::Item>;

                fn nth_back(
                    &mut self,
                    n: ::core::primitive::usize,
                ) -> ::core::option::Option<Self::Item>;
                // fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> R
                // where
                //     Self: ::core::marker::Sized,
                //     F: ::core::ops::FnMut(B, Self::Item) -> R,
                //     R: ::core::ops::Try<Output = B>;
                fn rfold<B, F>(self, init: B, f: F) -> B
                where
                    Self: ::core::marker::Sized,
                    F: ::core::ops::FnMut(B, Self::Item) -> B;
                fn rfind<P>(&mut self, predicate: P) -> ::core::option::Option<Self::Item>
                where
                    Self: ::core::marker::Sized,
                    P: ::core::ops::FnMut(&Self::Item) -> ::core::primitive::bool;
            }
        }

        /// ```
        /// # use newer_type::implement;
        /// #[implement(for<Borrowed> newer_type::traits::Borrow<Borrowed>)]
        /// struct MyStruct {
        ///     slot: u8
        /// }
        /// ```
        [Borrowed]{
            #[target(alternative = ::core::borrow::Borrow)]
            pub trait Borrow<Borrowed>
            where
                Borrowed: ?::core::marker::Sized,
            {
                // Required method
                fn borrow(&self) -> &Borrowed;
            }
        }

        /// ```
        /// # use newer_type::implement;
        /// #[implement(for<Borrowed> newer_type::traits::Borrow<Borrowed>)]
        /// #[implement(for<Borrowed> newer_type::traits::BorrowMut<Borrowed>)]
        /// struct MyStruct {
        ///     slot: u8,
        /// }
        /// ```
        [Borrowed]{
            #[target(alternative = ::core::borrow::BorrowMut)]
            pub trait BorrowMut<Borrowed>: ::core::borrow::Borrow<Borrowed>
            where
                Borrowed: ?::core::marker::Sized,
            {
                fn borrow_mut(&mut self) -> &mut Borrowed;
            }
        }

        /// ```ignore
        /// # use newer_type::implement;
        /// #[implement(newer_type::traits::ToOwned)]
        /// struct MyStruct {
        ///     slot: String,
        /// }
        /// ```
        {
            #[target(alternative = ::std::borrow::ToOwned)]
            pub trait ToOwned {
                type Owned: ::core::borrow::Borrow<Self>;
                fn to_owned(&self) -> Self::Owned;
                fn clone_into(&self, target: &mut Self::Owned);
            }
        }

        /// ```
        /// # use newer_type::implement;
        /// #[implement(for<Rhs> newer_type::traits::PartialEq<Rhs>)]
        /// struct MyStruct {
        ///     slot: u8,
        /// }
        /// ```
        [Rhs]{
            #[target(alternative = ::core::cmp::PartialEq)]
            pub trait PartialEq<Rhs = Self>
            where
                Rhs: ?::core::marker::Sized,
            {
                fn eq(&self, other: &Rhs) -> ::core::primitive::bool;
                fn ne(&self, other: &Rhs) -> ::core::primitive::bool;
            }
        }

        /// ```ignore
        /// # use newer_type::implement;
        /// #[implement(newer_type::traits::PartialEq<Self>)]
        /// #[implement(newer_type::traits::Eq)]
        /// struct MyStruct {
        ///     slot: u8
        /// }
        /// ```
        {
            #[target(alternative = ::core::cmp::Eq)]
            pub trait Eq: ::core::cmp::PartialEq {}
        }

        /// ```ignore
        /// # use newer_type::implement;
        /// #[implement(newer_type::traits::PartialOrd<u8>)]
        /// struct MyStruct {
        ///     slot: u8
        /// }
        /// ```
        [Rhs]{
            #[target(alternative = ::core::cmp::PartialOrd)]
            pub trait PartialOrd<Rhs = Self>: ::core::cmp::PartialEq<Rhs>
            where
                Rhs: ?::core::marker::Sized,
            {
                fn partial_cmp(&self, other: &Rhs)
                    -> ::core::option::Option<::core::cmp::Ordering>;

                fn lt(&self, other: &Rhs) -> ::core::primitive::bool;
                fn le(&self, other: &Rhs) -> ::core::primitive::bool;
                fn gt(&self, other: &Rhs) -> ::core::primitive::bool;
                fn ge(&self, other: &Rhs) -> ::core::primitive::bool;
            }
        }

        /// ```ignore
        /// # use newer_type::implement;
        /// #[implement(newer_type::traits::PartialEq<MyStruct>)]
        /// #[implement(newer_type::traits::Eq)]
        /// #[implement(newer_type::traits::PartialOrd<MyStruct>)]
        /// #[implement(newer_type::traits::Ord)]
        /// struct MyStruct {
        ///     slot: u8
        /// }
        /// ```
        {
            #[target(alternative = ::core::cmp::Ord)]
            pub trait Ord: ::core::cmp::Eq + ::core::cmp::PartialOrd {
                fn cmp(&self, other: &Self) -> ::core::cmp::Ordering;
                // fn max(self, other: Self) -> Self
                // where
                //     Self: ::core::marker::Sized;
                // fn min(self, other: Self) -> Self
                // where
                //     Self: ::core::marker::Sized;
                // fn clamp(self, min: Self, max: Self) -> Self
                // where
                //     Self: ::core::marker::Sized;
            }
        }
        /// ```
        /// # use newer_type::implement;
        /// #[implement(newer_type::traits::Hash)]
        /// struct MyStruct {
        ///     slot: u8
        /// }
        /// ```
        {
            #[target(alternative = ::core::hash::Hash)]
            pub trait Hash {
                fn hash<H>(&self, state: &mut H)
                   where H: ::core::hash::Hasher;
            }
        }
        /// ```
        /// # use newer_type::implement;
        /// #[implement(newer_type::traits::Display)]
        /// struct MyStruct {
        ///     slot: u8
        /// }
        /// ```
        {
            #[target(alternative = ::core::fmt::Display)]
            pub trait Display {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result;
            }
        }
        /// ```
        /// # use newer_type::implement;
        /// #[implement(newer_type::traits::Debug)]
        /// struct MyStruct {
        ///     slot: u8
        /// }
        /// ```
        {
            #[target(alternative = ::core::fmt::Debug)]
            pub trait Debug {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result;
            }
        }
    }
}
