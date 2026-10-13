//! One compile test per std trait (and per combination that is commonly
//! implemented together): `#[implement(..)]` on a struct with one field of a
//! type that implements the trait.
//!
//! `compiles! { name: FieldType => [implement args], [more args], ..; }`

// `ExitCode` is newer than the MSRV, and `ToString` is the trait under test.
#![allow(clippy::incompatible_msrv, clippy::to_string_trait_impl)]

macro_rules! compiles {
    ($($name:ident: $slot:ty => $([$($imp:tt)*]),+;)*) => {$(
        #[allow(dead_code)]
        mod $name {
            use newer_type::implement;
            $(#[implement($($imp)*)])+
            struct MyStruct {
                slot: $slot,
            }
            #[test]
            fn compiles() {}
        }
    )*};
}

compiles! {
    borrow: u8 => [for<Borrowed> newer_type_std::borrow::Borrow<Borrowed>];
    borrow_mut: u8 =>
        [for<Borrowed> newer_type_std::borrow::Borrow<Borrowed>],
        [for<Borrowed> newer_type_std::borrow::BorrowMut<Borrowed>];
    partial_eq: u8 => [newer_type_std::cmp::PartialEq];
    eq: u8 => [newer_type_std::cmp::PartialEq], [newer_type_std::cmp::Eq];
    partial_ord: u8 => [newer_type_std::cmp::PartialEq], [newer_type_std::cmp::PartialOrd];
    ord: u8 =>
        [newer_type_std::cmp::PartialEq],
        [newer_type_std::cmp::Eq],
        [newer_type_std::cmp::PartialOrd],
        [newer_type_std::cmp::Ord];
    as_ref: u8 => [for<T> newer_type_std::convert::AsRef<T>];
    as_mut: u8 => [for<T> newer_type_std::convert::AsMut<T>];
    into: char => [newer_type_std::convert::Into<u32>];
    try_into: i32 => [newer_type_std::convert::TryInto<u8>];
    error: std::io::Error =>
        [newer_type_std::fmt::Display, newer_type_std::fmt::Debug, newer_type_std::error::Error];
    display: u8 => [newer_type_std::fmt::Display];
    debug: u8 => [newer_type_std::fmt::Debug];
    binary: u8 => [newer_type_std::fmt::Binary];
    octal: u8 => [newer_type_std::fmt::Octal];
    lower_hex: u8 => [newer_type_std::fmt::LowerHex];
    upper_hex: u8 => [newer_type_std::fmt::UpperHex];
    lower_exp: u8 => [newer_type_std::fmt::LowerExp];
    upper_exp: u8 => [newer_type_std::fmt::UpperExp];
    pointer: Box<u8> => [newer_type_std::fmt::Pointer];
    fmt_write: String => [newer_type_std::fmt::Write];
    hash: u8 => [newer_type_std::hash::Hash];
    hasher: std::collections::hash_map::DefaultHasher => [newer_type_std::hash::Hasher];
    build_hasher: std::collections::hash_map::RandomState => [newer_type_std::hash::BuildHasher];
    read: std::io::Cursor<Vec<u8>> => [newer_type_std::io::Read];
    buf_read: std::io::Cursor<Vec<u8>> =>
        [newer_type_std::io::Read], [newer_type_std::io::BufRead];
    seek: std::fs::File => [newer_type_std::io::Seek];
    into_iterator: Vec<u8> => [newer_type_std::iter::IntoIterator];
    extend: Vec<u8> => [for<A> newer_type_std::iter::Extend<A>];
    iterator: std::vec::IntoIter<u8> => [newer_type_std::iter::Iterator];
    fused_iterator: std::vec::IntoIter<u8> =>
        [newer_type_std::iter::Iterator], [newer_type_std::iter::FusedIterator];
    exact_size_iterator: std::vec::IntoIter<u8> =>
        [newer_type_std::iter::Iterator], [newer_type_std::iter::ExactSizeIterator];
    double_ended_iterator: std::vec::IntoIter<u8> =>
        [newer_type_std::iter::Iterator], [newer_type_std::iter::DoubleEndedIterator];
    to_socket_addrs: String => [newer_type_std::net::ToSocketAddrs];
    deref: Box<usize> => [newer_type_std::ops::Deref];
    deref_mut: Box<u8> => [newer_type_std::ops::Deref], [newer_type_std::ops::DerefMut];
    index: Vec<u8> => [for<Idx> newer_type_std::ops::Index<Idx>];
    index_mut: Vec<u8> =>
        [for<Idx> newer_type_std::ops::Index<Idx>], [for<Idx> newer_type_std::ops::IndexMut<Idx>];
    not: bool => [newer_type_std::ops::Not];
    neg: isize => [newer_type_std::ops::Neg];
    bit_and: usize => [for<Rhs> newer_type_std::ops::BitAnd<Rhs>];
    bit_or: usize => [for<Rhs> newer_type_std::ops::BitOr<Rhs>];
    bit_xor: usize => [for<Rhs> newer_type_std::ops::BitXor<Rhs>];
    shl: usize => [for<Rhs> newer_type_std::ops::Shl<Rhs>];
    shr: usize => [for<Rhs> newer_type_std::ops::Shr<Rhs>];
    bit_and_assign: usize => [for<Rhs> newer_type_std::ops::BitAndAssign<Rhs>];
    bit_or_assign: usize => [for<Rhs> newer_type_std::ops::BitOrAssign<Rhs>];
    bit_xor_assign: usize => [for<Rhs> newer_type_std::ops::BitXorAssign<Rhs>];
    shl_assign: usize => [for<Rhs> newer_type_std::ops::ShlAssign<Rhs>];
    shr_assign: usize => [for<Rhs> newer_type_std::ops::ShrAssign<Rhs>];
    add: usize => [for<Rhs> newer_type_std::ops::Add<Rhs>];
    sub: usize => [for<Rhs> newer_type_std::ops::Sub<Rhs>];
    mul: usize => [for<Rhs> newer_type_std::ops::Mul<Rhs>];
    div: usize => [for<Rhs> newer_type_std::ops::Div<Rhs>];
    rem: usize => [for<Rhs> newer_type_std::ops::Rem<Rhs>];
    add_assign: usize => [for<Rhs> newer_type_std::ops::AddAssign<Rhs>];
    sub_assign: usize => [for<Rhs> newer_type_std::ops::SubAssign<Rhs>];
    mul_assign: usize => [for<Rhs> newer_type_std::ops::MulAssign<Rhs>];
    div_assign: usize => [for<Rhs> newer_type_std::ops::DivAssign<Rhs>];
    rem_assign: usize => [for<Rhs> newer_type_std::ops::RemAssign<Rhs>];
    termination: std::process::ExitCode => [newer_type_std::process::Termination];
    to_string: u8 => [newer_type_std::string::ToString];
}
