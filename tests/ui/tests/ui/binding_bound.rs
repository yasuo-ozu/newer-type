// An associated type bound (`Item: Bound`) is not a binding.
use newer_type_test_facade::newer_type::implement;

#[implement(newer_type = ::newer_type_test_facade::newer_type, ::announce_std::iter::Iterator<Item: Copy>)]
struct Wrap(std::vec::IntoIter<u8>);

fn main() {}
