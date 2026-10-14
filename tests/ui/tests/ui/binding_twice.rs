// An associated type is bound once.
use newer_type_test_facade::newer_type::implement;

#[implement(newer_type = ::newer_type_test_facade::newer_type, ::announce_std::iter::Iterator<Item = u8, Item = u8>)]
struct Wrap(std::vec::IntoIter<u8>);

fn main() {}
