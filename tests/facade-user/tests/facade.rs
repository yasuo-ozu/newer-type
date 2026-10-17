//! A crate that reaches newer-type only through a facade,
//! and a cross-crate `crate::Foo` parameter.
use newer_type_test_facade as fac;

#[fac::newer_type::implement(newer_type = ::newer_type_test_facade::newer_type, fac::UsesFoo, fac::MakesLocal)]
struct Wrap(u8);

#[test]
fn facade_and_crate_path_param() {
    use fac::{MakesLocal, UsesFoo};
    assert_eq!(Wrap(1).get(fac::Foo(2)), 3);
    assert_eq!(fac::local_value(&Wrap(7).make()), 7);
}

// `newer_type = ..` may come in any position.
#[fac::newer_type::implement(fac::UsesFoo, newer_type = ::newer_type_test_facade::newer_type)]
struct WrapLast(u8);

#[test]
fn newer_type_argument_last() {
    use fac::UsesFoo;
    assert_eq!(WrapLast(4).get(fac::Foo(1)), 5);
    assert_eq!(fac::FacadeWrap(1).get(fac::Foo(1)), 2);
}

// A trait that is announced twice (the announcements merge).
#[fac::newer_type::implement(fac::Merged, newer_type = ::newer_type_test_facade::newer_type)]
struct WrapMerged(u8);

#[test]
fn merged_announcements() {
    use fac::Merged;
    assert_eq!(WrapMerged(5).label(), "u8:5");
}
