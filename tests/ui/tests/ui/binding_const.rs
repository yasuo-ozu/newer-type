// An associated const cannot be bound.
use newer_type_test_facade::newer_type::implement;

#[implement(newer_type = ::newer_type_test_facade::newer_type, ::announce_std::iter::Iterator<N = 3>)]
struct Wrap(std::vec::IntoIter<u8>);

fn main() {}
