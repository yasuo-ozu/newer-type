pub use newer_type_macro::{__implement_internal, implement, target};

#[doc(hidden)]
pub struct Alternate(::core::convert::Infallible);

#[doc(hidden)]
pub trait Repeater<const TRAIT_NUM: u64, const N: usize> {
    type Type;
}

pub mod traits {
    use super::*;

    pub struct Implementor<T>(core::marker::PhantomData<T>, core::convert::Infallible);

    macro_rules! emit_traits {
        () => {
            #[target(alternative = ::core::iter::IntoIterator, newer_type = $crate, implementor = $crate::traits::Implementor)]
            pub trait IntoIterator {
                type Item;
                type IntoIter: ::core::iter::Iterator<Item = Self::Item>;
                fn into_iter(self) -> Self::IntoIter;
            }

            #[target(alternative = ::core::iter::Extend, newer_type = $crate, implementor = $crate::traits::Implementor)]
            pub trait Extend<A> {
                fn extend<T>(&mut self, iter: T)
                where
                    T: ::core::iter::IntoIterator<Item = A>;
            }

            #[target(alternative = ::core::iter::Iterator, newer_type = $crate, implementor = $crate::traits::Implementor)]
            pub trait Iterator {
                type Item;
                fn next(&mut self) -> ::core::option::Option<Self::Item>;
                // fn size_hint(&self) -> (usize, Option<usize>);
                // fn count(self) -> usize
                //    where Self: Sized;
                // fn last(self) -> Option<Self::Item>
                //    where Self: Sized;
                // fn nth(&mut self, n: usize) -> Option<Self::Item>;
            }

            #[target(alternative = ::core::iter::FusedIterator, newer_type = $crate, implementor = $crate::traits::Implementor)]
            pub trait FusedIterator: ::core::iter::Iterator {}

            #[target(alternative = ::core::iter::ExactSizeIterator, newer_type = $crate, implementor = $crate::traits::Implementor)]
            pub trait ExactSizeIterator: ::core::iter::Iterator {
                // fn len(&self) -> usize;
            }

            #[target(alternative = ::core::iter::DoubleEndedIterator, newer_type = $crate, implementor = $crate::traits::Implementor)]
            pub trait DoubleEndedIterator: ::core::iter::Iterator {
                fn next_back(&mut self) -> ::core::option::Option<Self::Item>;

                // fn nth_back(&mut self, n: usize) -> Option<Self::Item>;
                // fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> R
                //    where Self: Sized,
                //          F: FnMut(B, Self::Item) -> R,
                //          R: Try<Output = B>;
                // fn rfold<B, F>(self, init: B, f: F) -> B
                //    where Self: Sized,
                //          F: FnMut(B, Self::Item) -> B;
                // fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
                //    where Self: Sized,
                //          P: FnMut(&Self::Item) -> bool;
            }

            // std::alloc
            #[target(alternative = ::core::alloc::GlobalAlloc, newer_type = $crate, implementor = $crate::traits::Implementor)]
            pub unsafe trait GlobalAlloc {
                unsafe fn alloc(&self, layout: ::std::alloc::Layout) -> *mut ::core::primitive::u8;
                unsafe fn dealloc(
                    &self,
                    ptr: *mut ::core::primitive::u8,
                    layout: ::std::alloc::Layout,
                );
                // unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8;
                // unsafe fn realloc(
                //     &self,
                //     ptr: *mut u8,
                //     layout: Layout,
                //     new_size: usize,
                // ) -> *mut u8;
            }

            // std::borrow
            #[target(alternative = ::core::borrow::Borrow, newer_type = $crate, implementor = $crate::traits::Implementor)]
            pub trait Borrow<Borrowed>
            where
                Borrowed: ?::core::marker::Sized,
            {
                // Required method
                fn borrow(&self) -> &Borrowed;
            }

            #[target(alternative = ::core::borrow::BorrowMut, newer_type = $crate, implementor = $crate::traits::Implementor)]
            pub trait BorrowMut<Borrowed>: ::core::borrow::Borrow<Borrowed>
            where
                Borrowed: ?::core::marker::Sized,
            {
                fn borrow_mut(&mut self) -> &mut Borrowed;
            }

            #[target(alternative = ::core::borrow::ToOwned, newer_type = $crate, implementor = $crate::traits::Implementor)]
            pub trait ToOwned {
                type Owned: ::core::borrow::Borrow<Self>;
                fn to_owned(&self) -> Self::Owned;
                // fn clone_into(&self, target: &mut Self::Owned);
            }

            // std::cmp
            #[target(alternative = ::core::cmp::PartialEq, newer_type = $crate, implementor = $crate::traits::Implementor)]
            pub trait PartialEq<Rhs = Self>
            where
                Rhs: ?::core::marker::Sized,
            {
                fn eq(&self, other: &Rhs) -> bool;
                // fn ne(&self, other: &Rhs) -> bool;
            }

            #[target(alternative = ::core::cmp::Eq, newer_type = $crate, implementor = $crate::traits::Implementor)]
            pub trait Eq: ::core::cmp::PartialEq {}

            #[target(alternative = ::core::cmp::PartialOrd, newer_type = $crate, implementor = $crate::traits::Implementor)]
            pub trait PartialOrd<Rhs = Self>: ::core::cmp::PartialEq<Rhs>
            where
                Rhs: ?::core::marker::Sized,
            {
                fn partial_cmp(&self, other: &Rhs)
                    -> ::core::option::Option<::core::cmp::Ordering>;

                // fn lt(&self, other: &Rhs) -> bool;
                // fn le(&self, other: &Rhs) -> bool;
                // fn gt(&self, other: &Rhs) -> bool;
                // fn ge(&self, other: &Rhs) -> bool;
            }

            #[target(alternative = ::core::cmp::Ord, newer_type = $crate, implementor = $crate::traits::Implementor)]
            pub trait Ord: ::core::cmp::Eq + ::core::cmp::PartialOrd {
                fn cmp(&self, other: &Self) -> ::core::cmp::Ordering;
                // fn max(self, other: Self) -> Self
                //    where Self: Sized;
                // fn min(self, other: Self) -> Self
                //    where Self: Sized;
                // fn clamp(self, min: Self, max: Self) -> Self
                //    where Self: Sized;
            }
        };
    }

    emit_traits!();
}
