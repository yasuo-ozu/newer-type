use newer_type::implement;
use newer_type_std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[implement(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Debug)]
pub struct MyVec<T>(Vec<T>);

impl<T> MyVec<T> {
    pub fn new(inner: Vec<T>) -> Self {
        Self(inner)
    }
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

#[implement(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Debug)]
pub struct MyString(String);

impl MyString {
    pub fn new(inner: &str) -> Self {
        Self(inner.to_owned())
    }
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
