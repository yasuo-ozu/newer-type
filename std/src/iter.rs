use crate::emit_traits;
use newer_type::target;

emit_traits! {
    #[implement_of(newer_type_std::iter::IntoIterator)]
    #[slot(Vec<u8>)]
    #[target(alternative = ::core::iter::IntoIterator)]
    pub trait IntoIterator {
        type Item;
        type IntoIter: ::core::iter::Iterator<Item = Self::Item>;
        fn into_iter(self) -> Self::IntoIter;
    }

    #[implement_of(for<A> newer_type_std::iter::Extend<A>)]
    #[slot(Vec<u8>)]
    #[target(alternative = ::core::iter::Extend)]
    pub trait Extend[A] {
        fn extend<T>(&mut self, iter: T)
        where
            T: ::core::iter::IntoIterator<Item = A>;
    }

    #[implement_of(newer_type_std::iter::Iterator)]
    #[slot(std::vec::IntoIter<u8>)]
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

    #[implement_of(newer_type_std::iter::Iterator, newer_type_std::iter::FusedIterator)]
    #[slot(std::vec::IntoIter<u8>)]
    #[target(alternative = ::core::iter::FusedIterator)]
    pub trait FusedIterator: [::core::iter::Iterator] {}


    #[implement_of(newer_type_std::iter::Iterator, newer_type_std::iter::ExactSizeIterator)]
    #[slot(std::vec::IntoIter<u8>)]
    #[target(alternative = ::core::iter::ExactSizeIterator)]
    pub trait ExactSizeIterator: [::core::iter::Iterator] {
        fn len(&self) -> ::core::primitive::usize;
    }


    #[implement_of(newer_type_std::iter::Iterator, newer_type_std::iter::DoubleEndedIterator)]
    #[slot(std::vec::IntoIter<u8>)]
    #[target(alternative = ::core::iter::DoubleEndedIterator)]
    pub trait DoubleEndedIterator: [::core::iter::Iterator] {
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
