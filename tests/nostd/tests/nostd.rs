//! The `#![no_std]` crate works when it is used (the build for a target without
//! std is checked by CI).
use newer_type_test_nostd::{Either, Measure, Width};

#[test]
fn struct_forwards() {
    assert_eq!(Width(3).measure().0, 3);
    assert_eq!(Width(3).scaled(4), 12);
}

#[test]
fn enum_forwards() {
    assert_eq!(Either::Left(2).measure().0, 2);
    assert_eq!(Either::Right(5).scaled(2), 10);
}
