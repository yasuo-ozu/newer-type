pub use newer_type_macro::{__implement_internal, implement, target};

pub mod traits {
    use super::*;

    #[target]
    pub trait IntoIterator {
        type Item;
        type IntoIter: Iterator<Item = Self::Item>;
        fn into_iter(self) -> Self::IntoIter;
    }

    #[target]
    pub trait Extend<A> {
        fn extend<T>(&mut self, iter: T)
        where
            T: IntoIterator<Item = A>;
    }

    #[target]
    trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
        // fn size_hint(&self) -> (usize, Option<usize>);
        // fn count(self) -> usize
        //    where Self: Sized;
        // fn last(self) -> Option<Self::Item>
        //    where Self: Sized;
        // fn nth(&mut self, n: usize) -> Option<Self::Item>;
    }

    #[target]
    pub trait FusedIterator: Iterator {}

    #[target]
    pub trait ExactSizeIterator: Iterator {
        // fn len(&self) -> usize;
    }

    #[target]
    pub trait DoubleEndedIterator: Iterator {
        fn next_back(&mut self) -> Option<Self::Item>;

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
    #[target]
    pub unsafe trait GlobalAlloc {
        unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8;
        unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout);
        // unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8;
        // unsafe fn realloc(
        //     &self,
        //     ptr: *mut u8,
        //     layout: Layout,
        //     new_size: usize,
        // ) -> *mut u8;
    }

    // std::borrow
    #[target]
    pub trait Borrow<Borrowed>
    where
        Borrowed: ?Sized,
    {
        // Required method
        fn borrow(&self) -> &Borrowed;
    }

    #[target]
    pub trait BorrowMut<Borrowed>: Borrow<Borrowed>
    where
        Borrowed: ?Sized,
    {
        fn borrow_mut(&mut self) -> &mut Borrowed;
    }

    #[target]
    pub trait ToOwned {
        type Owned: Borrow<Self>;
        fn to_owned(&self) -> Self::Owned;
        // fn clone_into(&self, target: &mut Self::Owned);
    }

    // std::cmp
    pub trait PartialEq<Rhs = Self>
    where
        Rhs: ?Sized,
    {
        fn eq(&self, other: &Rhs) -> bool;
        // fn ne(&self, other: &Rhs) -> bool;
    }

    pub trait Eq: PartialEq {}

    pub trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
    where
        Rhs: ?Sized,
    {
        fn partial_cmp(&self, other: &Rhs) -> Option<std::cmp::Ordering>;

        // fn lt(&self, other: &Rhs) -> bool;
        // fn le(&self, other: &Rhs) -> bool;
        // fn gt(&self, other: &Rhs) -> bool;
        // fn ge(&self, other: &Rhs) -> bool;
    }

    pub trait Ord: Eq + PartialOrd {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering;
        // fn max(self, other: Self) -> Self
        //    where Self: Sized;
        // fn min(self, other: Self) -> Self
        //    where Self: Sized;
        // fn clamp(self, min: Self, max: Self) -> Self
        //    where Self: Sized;
    }
}
