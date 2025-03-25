#![doc = include_str!("./README.md")]

pub use newer_type_macro::{__implement_internal, implement, target};

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
            /// ```ignore
            #[doc = concat!(
                " #[implement(",
                    $(
                        "for<",
                        $(stringify!($trait_params),)+
                        "> newer_type::traits::",
                    )?
                        stringify!($trait_name),
                    $(
                        "<",
                        $(stringify!($trait_params),)+
                        ">",
                    )?
                ")]")]
            /// struct MyStruct {
            #[doc = concat!("     slot: TypeWhichAlreadyImplements", stringify!($trait_name), ",")]
            /// }
            /// ```
            pub trait $trait_name $($trait_contents)*
            emit_traits!{ $($t)* }
        };
    }

    emit_traits! {
        {
            #[target(alternative = ::core::iter::IntoIterator)]
            pub trait IntoIterator {
                type Item;
                type IntoIter: ::core::iter::Iterator<Item = Self::Item>;
                fn into_iter(self) -> Self::IntoIter;
            }
        }

        [A]{
            #[target(alternative = ::core::iter::Extend)]
            pub trait Extend<A> {
                fn extend<T>(&mut self, iter: T)
                where
                    T: ::core::iter::IntoIterator<Item = A>;
            }
        }

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
                fn nth(&mut self, n: ::core::primitive::usize) -> Option<Self::Item>;
            }
        }

        {
            #[target(alternative = ::core::iter::FusedIterator)]
            pub trait FusedIterator: ::core::iter::Iterator {}
        }

        {
            #[target(alternative = ::core::iter::ExactSizeIterator)]
            pub trait ExactSizeIterator: ::core::iter::Iterator {
                fn len(&self) -> ::core::primitive::usize;
            }
        }

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
                    P: ::core::ops::FnMut(&Self::Item) -> bool;
            }
        }

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

        [Borrowed]{
            #[target(alternative = ::core::borrow::BorrowMut)]
            pub trait BorrowMut<Borrowed>: ::core::borrow::Borrow<Borrowed>
            where
                Borrowed: ?::core::marker::Sized,
            {
                fn borrow_mut(&mut self) -> &mut Borrowed;
            }
        }

        {
            #[target(alternative = ::std::borrow::ToOwned)]
            pub trait ToOwned {
                type Owned: ::core::borrow::Borrow<Self>;
                fn to_owned(&self) -> Self::Owned;
                fn clone_into(&self, target: &mut Self::Owned);
            }
        }

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

        {
            #[target(alternative = ::core::cmp::Eq)]
            pub trait Eq: ::core::cmp::PartialEq {}
        }

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
    }
}
