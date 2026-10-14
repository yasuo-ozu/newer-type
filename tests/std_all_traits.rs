//! One compile test per std trait (and per combination that is commonly
//! implemented together): `#[implement(..)]` on a struct with one field of a
//! type that implements the trait.
//!
//! `compiles! { name: FieldType => [implement args], [more args], ..; }`

// `ExitCode` is newer than the MSRV, and `ToString` is the trait under test.
// (`unknown_lints`: older clippy does not know these two lints.)
#![allow(unknown_lints)]
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
    borrow: u8 => [for<Borrowed> ::announce_std::borrow::Borrow<Borrowed>];
    borrow_mut: u8 =>
        [for<Borrowed> ::announce_std::borrow::Borrow<Borrowed>],
        [for<Borrowed> ::announce_std::borrow::BorrowMut<Borrowed>];
    partial_eq: u8 => [::announce_std::cmp::PartialEq];
    eq: u8 => [::announce_std::cmp::PartialEq], [::announce_std::cmp::Eq];
    partial_ord: u8 => [::announce_std::cmp::PartialEq], [::announce_std::cmp::PartialOrd];
    ord: u8 =>
        [::announce_std::cmp::PartialEq],
        [::announce_std::cmp::Eq],
        [::announce_std::cmp::PartialOrd],
        [::announce_std::cmp::Ord];
    as_ref: u8 => [for<T> ::announce_std::convert::AsRef<T>];
    as_mut: u8 => [for<T> ::announce_std::convert::AsMut<T>];
    into: char => [::announce_std::convert::Into<u32>];
    try_into: i32 => [::announce_std::convert::TryInto<u8>];
    error: std::io::Error =>
        [::announce_std::fmt::Display, ::announce_std::fmt::Debug, ::announce_std::error::Error];
    display: u8 => [::announce_std::fmt::Display];
    debug: u8 => [::announce_std::fmt::Debug];
    binary: u8 => [::announce_std::fmt::Binary];
    octal: u8 => [::announce_std::fmt::Octal];
    lower_hex: u8 => [::announce_std::fmt::LowerHex];
    upper_hex: u8 => [::announce_std::fmt::UpperHex];
    lower_exp: u8 => [::announce_std::fmt::LowerExp];
    upper_exp: u8 => [::announce_std::fmt::UpperExp];
    pointer: Box<u8> => [::announce_std::fmt::Pointer];
    fmt_write: String => [::announce_std::fmt::Write];
    hash: u8 => [::announce_std::hash::Hash];
    hasher: std::collections::hash_map::DefaultHasher => [::announce_std::hash::Hasher];
    build_hasher: std::collections::hash_map::RandomState => [::announce_std::hash::BuildHasher];
    read: std::io::Cursor<Vec<u8>> => [::announce_std::io::Read];
    buf_read: std::io::Cursor<Vec<u8>> =>
        [::announce_std::io::Read], [::announce_std::io::BufRead];
    seek: std::fs::File => [::announce_std::io::Seek];
    into_iterator: Vec<u8> => [::announce_std::iter::IntoIterator];
    extend: Vec<u8> => [for<A> ::announce_std::iter::Extend<A>];
    iterator: std::vec::IntoIter<u8> => [::announce_std::iter::Iterator];
    fused_iterator: std::vec::IntoIter<u8> =>
        [::announce_std::iter::Iterator], [::announce_std::iter::FusedIterator];
    exact_size_iterator: std::vec::IntoIter<u8> =>
        [::announce_std::iter::Iterator], [::announce_std::iter::ExactSizeIterator];
    double_ended_iterator: std::vec::IntoIter<u8> =>
        [::announce_std::iter::Iterator], [::announce_std::iter::DoubleEndedIterator];
    to_socket_addrs: String => [::announce_std::net::ToSocketAddrs];
    deref: Box<usize> => [::announce_std::ops::Deref];
    deref_mut: Box<u8> => [::announce_std::ops::Deref], [::announce_std::ops::DerefMut];
    index: Vec<u8> => [for<Idx> ::announce_std::ops::Index<Idx>];
    index_mut: Vec<u8> =>
        [for<Idx> ::announce_std::ops::Index<Idx>], [for<Idx> ::announce_std::ops::IndexMut<Idx>];
    not: bool => [::announce_std::ops::Not];
    neg: isize => [::announce_std::ops::Neg];
    bit_and: usize => [for<Rhs> ::announce_std::ops::BitAnd<Rhs>];
    bit_or: usize => [for<Rhs> ::announce_std::ops::BitOr<Rhs>];
    bit_xor: usize => [for<Rhs> ::announce_std::ops::BitXor<Rhs>];
    shl: usize => [for<Rhs> ::announce_std::ops::Shl<Rhs>];
    shr: usize => [for<Rhs> ::announce_std::ops::Shr<Rhs>];
    bit_and_assign: usize => [for<Rhs> ::announce_std::ops::BitAndAssign<Rhs>];
    bit_or_assign: usize => [for<Rhs> ::announce_std::ops::BitOrAssign<Rhs>];
    bit_xor_assign: usize => [for<Rhs> ::announce_std::ops::BitXorAssign<Rhs>];
    shl_assign: usize => [for<Rhs> ::announce_std::ops::ShlAssign<Rhs>];
    shr_assign: usize => [for<Rhs> ::announce_std::ops::ShrAssign<Rhs>];
    add: usize => [for<Rhs> ::announce_std::ops::Add<Rhs>];
    sub: usize => [for<Rhs> ::announce_std::ops::Sub<Rhs>];
    mul: usize => [for<Rhs> ::announce_std::ops::Mul<Rhs>];
    div: usize => [for<Rhs> ::announce_std::ops::Div<Rhs>];
    rem: usize => [for<Rhs> ::announce_std::ops::Rem<Rhs>];
    add_assign: usize => [for<Rhs> ::announce_std::ops::AddAssign<Rhs>];
    sub_assign: usize => [for<Rhs> ::announce_std::ops::SubAssign<Rhs>];
    mul_assign: usize => [for<Rhs> ::announce_std::ops::MulAssign<Rhs>];
    div_assign: usize => [for<Rhs> ::announce_std::ops::DivAssign<Rhs>];
    rem_assign: usize => [for<Rhs> ::announce_std::ops::RemAssign<Rhs>];
    termination: std::process::ExitCode => [::announce_std::process::Termination];
    to_string: u8 => [::announce_std::string::ToString];
}
