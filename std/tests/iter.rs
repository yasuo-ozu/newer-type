use newer_type::implement;
use std::collections::VecDeque;
use std::iter::{ExactSizeIterator, Extend, Iterator};

#[implement(newer_type_std::iter::IntoIterator)]
#[implement(for<A> newer_type_std::iter::Extend<A>)]
pub struct MyStruct1<TTT> {
    inner: Vec<TTT>,
}

#[implement(
    newer_type_std::iter::Iterator,
    newer_type_std::iter::FusedIterator,
    newer_type_std::iter::ExactSizeIterator,
    newer_type_std::iter::DoubleEndedIterator
)]
pub struct MyStruct2 {
    inner: std::vec::IntoIter<usize>,
}

#[implement(
    newer_type_std::iter::Iterator,
    newer_type_std::iter::FusedIterator,
    newer_type_std::iter::ExactSizeIterator,
    newer_type_std::iter::DoubleEndedIterator
)]
pub enum MyEnum<T> {
    VariantA(std::vec::IntoIter<T>),
    VariantB(std::collections::vec_deque::IntoIter<T>),
}

impl<T> MyEnum<T> {
    fn from_a(data: Vec<T>) -> Self {
        MyEnum::VariantA(data.into_iter())
    }

    fn from_b(data: VecDeque<T>) -> Self {
        MyEnum::VariantB(data.into_iter())
    }
}

#[test]
fn test_into_iterator() {
    let wrapper = MyStruct1 {
        inner: vec![1, 2, 3],
    };
    let collected: Vec<_> = wrapper.into_iter().collect();
    assert_eq!(collected, vec![1, 2, 3]);
}

#[test]
fn test_extend_trait() {
    let mut wrapper = MyStruct1 { inner: vec![1, 2] };
    wrapper.extend(vec![3, 4]);
    assert_eq!(wrapper.inner, vec![1, 2, 3, 4]);
}

#[test]
fn test_iterator_trait() {
    let wrapper = MyStruct2 {
        inner: vec![4, 5, 6].into_iter(),
    };
    let sum: usize = wrapper.into_iter().sum();
    assert_eq!(sum, 15);
}

#[test]
fn test_fused_exact_double_ended_iterator() {
    let mut wrapper = MyStruct2 {
        inner: vec![10, 20, 30].into_iter(),
    };
    assert_eq!(wrapper.len(), 3);
    assert_eq!(wrapper.size_hint(), (3, Some(3)));
    assert_eq!(wrapper.next_back(), Some(30));
    assert_eq!(wrapper.next(), Some(10));
    assert_eq!(wrapper.next(), Some(20));
    assert_eq!(wrapper.next(), None);
    assert_eq!(wrapper.next(), None); // FusedIterator should return None
                                      // repeatedly
}

#[test]
fn test_enum_variant_a_iterator_behavior() {
    let wrapper = MyEnum::from_a(vec![100, 200, 255]);
    assert_eq!(wrapper.size_hint(), (3, Some(3)));
    let rev: Vec<_> = wrapper.rev().collect();
    assert_eq!(rev, vec![255, 200, 100]);
}

#[test]
fn test_enum_variant_b_iterator_behavior() {
    let deque = VecDeque::from(vec!['a', 'b', 'c']);
    let mut wrapper = MyEnum::from_b(deque);
    assert_eq!(wrapper.size_hint(), (3, Some(3)));
    assert_eq!(wrapper.next(), Some('a'));
    assert_eq!(wrapper.next_back(), Some('c'));
    assert_eq!(wrapper.len(), 1);
    assert_eq!(wrapper.next(), Some('b'));
    assert_eq!(wrapper.next(), None);
}
