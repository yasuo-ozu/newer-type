use newer_type::implement;
use newer_type::traits::{
    Borrow, BorrowMut, Debug, Display, Eq, Extend, Hash, IntoIterator, Ord, PartialEq, PartialOrd,
};

#[implement(
    IntoIterator,
    Extend<T>,
    Borrow<T>,
    BorrowMut<T>,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Display,
    Debug,
    Hash
)]
pub struct MyVec<T>(Vec<T>);

impl<T> MyVec<T> {
    pub fn new(inner: Vec<T>) -> Self {
        Self(inner)
    }
}

#[test]
fn test_into_iterator() {
    let my_vec = MyVec::new(vec![1, 2, 3]);
    let collected: Vec<_> = my_vec.into_iter().collect();
    assert_eq!(collected, vec![1, 2, 3]);
}

#[test]
fn test_extend() {
    let mut my_vec = MyVec::new(vec![1, 2]);
    my_vec.extend(vec![3, 4]);
    assert_eq!(my_vec.0, vec![1, 2, 3, 4]);
}

#[test]
fn test_partial_eq() {
    let a = MyVec::new(vec![1, 2, 3]);
    let b = MyVec::new(vec![1, 2, 3]);
    let c = MyVec::new(vec![1, 2, 4]);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_partial_ord() {
    let a = MyVec::new(vec![1, 2, 3]);
    let b = MyVec::new(vec![1, 2, 4]);
    assert!(a < b);
}

#[implement(Borrow<str>, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Debug)]
pub struct MyString(String);

impl MyString {
    pub fn new(inner: &str) -> Self {
        Self(inner.to_owned())
    }
}

#[test]
fn test_borrow_str() {
    use std::borrow::Borrow;
    let s = MyString::new("hello");
    fn f<T: newer_type::traits::Borrow<str>>(t: &T) {
        t.borrow();
    }
    f(&s);
    let borrowed: &str = s.borrow();
    assert_eq!(borrowed, "hello");
}

#[test]
fn test_partial_eq2() {
    let a = MyString::new("abc");
    let b = MyString::new("abc");
    let c = MyString::new("xyz");
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_ord() {
    let a = MyString::new("apple");
    let b = MyString::new("banana");
    assert!(a < b);
}
