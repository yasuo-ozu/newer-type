// A binding of an associated type that the trait does not have.
use newer_type_test_facade::newer_type::implement;

#[implement(newer_type = ::newer_type_test_facade::newer_type, ::announce_std::iter::Iterator<Itm = u8>)]
struct Wrap(std::vec::IntoIter<u8>);

#[implement(newer_type = ::newer_type_test_facade::newer_type, ::announce_std::fmt::Display<Item = u8>)]
struct NoAssoc(u8);

fn main() {}
