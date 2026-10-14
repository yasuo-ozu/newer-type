// A trait method that takes `Self` in several parameters cannot be forwarded
// for an enum: the arguments can be of different variants. (A struct has one
// field for all of them: see tests/multi_self.rs.)
use newer_type_test_facade::newer_type::implement;

#[implement(newer_type = ::newer_type_test_facade::newer_type, ::announce_std::cmp::PartialEq)]
enum E {
    A(u8),
    B(u8),
}

fn main() {}
