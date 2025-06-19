use newer_type::implement;
use std::fmt::Write;

#[implement(
    newer_type_std::fmt::Display,
    newer_type_std::fmt::Debug,
    newer_type_std::fmt::Binary,
    newer_type_std::fmt::Octal,
    newer_type_std::fmt::LowerHex,
    newer_type_std::fmt::UpperHex,
    newer_type_std::fmt::LowerExp,
    newer_type_std::fmt::UpperExp,
    // newer_type_std::fmt::Pointer,
    // newer_type_std::fmt::Write
)]
pub struct MyFmtStruct {
    inner: u32,
}

#[implement(newer_type_std::fmt::Display, newer_type_std::fmt::Debug)]
pub struct MyGenericFmtStruct<T> {
    inner: T,
}

#[implement(newer_type_std::fmt::Display, newer_type_std::fmt::Debug)]
pub enum MyFmtEnum {
    Text(String),
    Number(i32),
}

#[implement(newer_type_std::fmt::Debug)]
pub enum MyGenericFmtEnum<T> {
    One(T),
    Many(Vec<T>),
}

#[implement(newer_type_std::fmt::Write)]
pub struct MyWriteStruct {
    inner: String,
}

#[implement(newer_type_std::fmt::Write)]
pub enum MyWriteEnum {
    A(String),
    B(std::ffi::OsString),
}

#[test]
fn test_display_debug() {
    let value = MyFmtStruct { inner: 42 };
    assert_eq!(format!("{}", value), "42");
    assert_eq!(format!("{:?}", value), "42");
}

#[test]
fn test_binary_octal_hex() {
    let value = MyFmtStruct { inner: 42 };
    assert_eq!(format!("{:b}", value), "101010");
    assert_eq!(format!("{:o}", value), "52");
    assert_eq!(format!("{:x}", value), "2a");
    assert_eq!(format!("{:X}", value), "2A");
}

#[test]
fn test_exponential_and_pointer() {
    let value = MyFmtStruct { inner: 1_000_000 };
    let lower = format!("{:e}", value);
    let upper = format!("{:E}", value);
    assert!(lower.starts_with("1e") || lower.starts_with("1.0e"));
    assert!(upper.starts_with("1E") || upper.starts_with("1.0E"));

    // Just ensure it runs without panic
    let _ = format!("{:p}", &value);
}

#[test]
fn test_fmt_write_trait() {
    let value = MyFmtStruct { inner: 123 };
    let mut output = String::new();
    write!(&mut output, "value: {}", value).unwrap();
    assert_eq!(output, "value: 123");
}

#[test]
fn test_generic_fmt_struct() {
    let value = MyGenericFmtStruct { inner: 3.14f64 };
    assert_eq!(format!("{}", value), "3.14");
    assert_eq!(format!("{:?}", value), "3.14");
}

#[test]
fn test_fmt_enum() {
    let a = MyFmtEnum::Text("hello".to_string());
    let b = MyFmtEnum::Number(123);
    assert_eq!(format!("{}", a), "hello");
    assert_eq!(format!("{}", b), "123");
    assert_eq!(format!("{:?}", a), "\"hello\"");
    assert_eq!(format!("{:?}", b), "123");
}

#[test]
fn test_generic_fmt_enum() {
    let e1 = MyGenericFmtEnum::One(3.14);
    let e2 = MyGenericFmtEnum::Many(vec![1, 2, 3]);
    assert_eq!(format!("{:?}", e1), "3.14");
    assert_eq!(format!("{:?}", e2), "[1, 2, 3]");
}

#[test]
fn test_write_struct_trait() {
    let mut wrapper = MyWriteStruct {
        inner: String::new(),
    };
    write!(&mut wrapper, "abc{}", 123).unwrap();
    assert_eq!(wrapper.inner, "abc123");
}

#[test]
fn test_write_enum_trait() {
    let mut wrapper = MyWriteEnum::A(String::new());
    write!(&mut wrapper, "hello {}", "world").unwrap();
    match wrapper {
        MyWriteEnum::A(ref s) => assert_eq!(s, "hello world"),
        _ => panic!("unexpected variant"),
    }

    let mut wrapper = MyWriteEnum::B(std::ffi::OsString::new());
    write!(&mut wrapper, "abc{}", 123).unwrap();
    match wrapper {
        MyWriteEnum::B(ref v) => assert_eq!(&v.clone().into_string().unwrap(), &"abc123"),
        _ => panic!("unexpected variant"),
    }
}
