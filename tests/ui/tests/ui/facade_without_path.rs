// This crate reaches newer-type only through the facade, so it has no
// `newer-type` dependency: `#[implement]` generates code with `::newer_type`
// by default, which does not resolve here, and needs `newer_type = ..`.
use newer_type_test_facade::newer_type::implement;

#[implement(newer_type_test_facade::UsesFoo)]
struct Wrap(u8);

fn main() {}
