// The point of the test is to implement `ToString` directly.
#![allow(clippy::to_string_trait_impl)]

use newer_type::{announce::announce, implement};

#[announce(mock = ::std::string::ToString, allow_std_prelude = true)]
pub trait ToString {
    fn to_string(&self) -> String;
}

#[allow(unused)]
#[implement(ToString)]
struct MyStruct {
    slot: u8,
}

#[test]
fn mock_implements_std_to_string() {
    // `MyStruct` is not `Display`: `to_string` is the one that `#[implement]`
    // forwarded.
    assert_eq!(MyStruct { slot: 7 }.to_string(), "7");
}
