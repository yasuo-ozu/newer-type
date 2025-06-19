use newer_type::implement;
use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::io;

#[implement(
    newer_type_std::error::Error,
    newer_type_std::fmt::Display,
    newer_type_std::fmt::Debug
)]
pub struct MyErrorStruct {
    inner: io::Error,
}

#[implement(
    newer_type_std::error::Error,
    newer_type_std::fmt::Display,
    newer_type_std::fmt::Debug
)]
pub enum MyErrorEnum {
    Io(io::Error),
    Fmt(fmt::Error),
}

#[implement(
    newer_type_std::error::Error,
    newer_type_std::fmt::Display,
    newer_type_std::fmt::Debug
)]
pub struct MyGenericErrorStruct<T: Error + Display + Debug> {
    inner: T,
}

#[implement(
    newer_type_std::error::Error,
    newer_type_std::fmt::Display,
    newer_type_std::fmt::Debug
)]
pub enum MyGenericErrorEnum<T: Error + Display + Debug> {
    Single(T),
    Nested(MyErrorEnum),
}

impl MyErrorStruct {
    fn new() -> Self {
        Self {
            inner: io::Error::new(io::ErrorKind::Other, "custom io error"),
        }
    }
}

impl MyErrorEnum {
    fn from_io() -> Self {
        MyErrorEnum::Io(io::Error::new(io::ErrorKind::NotFound, "file not found"))
    }

    fn from_fmt() -> Self {
        MyErrorEnum::Fmt(fmt::Error)
    }
}

#[test]
fn test_error_struct_trait() {
    let err = MyErrorStruct::new();
    assert_eq!(format!("{}", err), "custom io error");
    assert!(err.source().is_none());
}

#[test]
fn test_error_enum_trait() {
    let e1 = MyErrorEnum::from_io();
    let e2 = MyErrorEnum::from_fmt();
    assert!(e1.to_string().contains("file not found"));
    assert!(format!("{:?}", e2).contains("Error"));
    assert!(e1.source().is_none());
}

#[test]
fn test_generic_error_struct() {
    let inner = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
    let err = MyGenericErrorStruct { inner };
    assert!(format!("{}", err).contains("permission denied"));
    assert!(err.source().is_none());
}

#[test]
fn test_generic_error_enum() {
    let e1 = MyGenericErrorEnum::Single(fmt::Error);
    let e2: MyGenericErrorEnum<fmt::Error> = MyGenericErrorEnum::Nested(MyErrorEnum::from_io());
    assert!(format!("{}", e1).contains("error"));
    assert!(format!("{:?}", e2).contains("file not found"));
    assert!(e1.source().is_none());
}
