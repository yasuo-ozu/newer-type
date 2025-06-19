use newer_type::implement;
use newer_type_std::borrow::{Borrow, BorrowMut};

#[implement(
    Borrow<T>,
    BorrowMut<T>,
)]
pub struct MyVec<T>(Vec<T>);

impl<T> MyVec<T> {
    pub fn new(inner: Vec<T>) -> Self {
        Self(inner)
    }
}

#[implement(Borrow<str>)]
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
    fn f<T: std::borrow::Borrow<str>>(t: &T) {
        t.borrow();
    }
    f(&s);
    let borrowed: &str = s.borrow();
    assert_eq!(borrowed, "hello");
}
