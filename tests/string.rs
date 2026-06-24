use newer_type::{implement, target};

pub trait Repeater<T: ?Sized, const TRAIT_ID: u64, const NTH: usize> {
    type Type;
}

#[target(alternative = ::std::string::ToString, newer_type = ::newer_type, repeater = Repeater)]
pub trait ToString {
    fn to_string(&self) -> String;
}

#[allow(unused)]
#[implement(ToString)]
struct MyStruct {
    slot: u8,
}
