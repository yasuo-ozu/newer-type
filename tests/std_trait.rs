// use newer_type::implement;
// use newer_type::traits::{
//     Borrow, BorrowMut, Eq, Extend, FusedIterator, IntoIter, Iterator, Ord,
// PartialEq, PartialOrd, };
//
// #[implement(
//     IntoIterator,
//     Iterator,
//     Extend,
//     FusedIterator,
//     Borrow,
//     BorrowMut,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord
// )]
// pub struct MyVec<T>(Vec<T>);
//
// impl<T> MyVec<T> {
//     pub fn new(inner: Vec<T>) -> Self {
//         Self(inner)
//     }
// }
//
// #[test]
// fn test_into_iterator() {
//     let my_vec = MyVec::new(vec![1, 2, 3]);
//     let collected: Vec<_> = my_vec.into_iter().collect();
//     assert_eq!(collected, vec![1, 2, 3]);
// }
//
// #[test]
// fn test_iterator() {
//     let my_vec = MyVec::new(vec![1, 2, 3]);
//     let mut iter = my_vec.into_iter();
//     assert_eq!(iter.next(), Some(1));
//     assert_eq!(iter.next(), Some(2));
//     assert_eq!(iter.next(), Some(3));
//     assert_eq!(iter.next(), None);
// }
//
// #[test]
// fn test_extend() {
//     let mut my_vec = MyVec::new(vec![1, 2]);
//     my_vec.extend(vec![3, 4]);
//     assert_eq!(my_vec.0, vec![1, 2, 3, 4]);
// }
//
// #[test]
// fn test_borrow() {
//     let my_vec = MyVec::new(vec![1, 2, 3]);
//     let borrowed: &Vec<i32> = my_vec.borrow();
//     assert_eq!(borrowed, &vec![1, 2, 3]);
// }
//
// #[test]
// fn test_borrow_mut() {
//     let mut my_vec = MyVec::new(vec![1, 2, 3]);
//     let borrowed_mut: &mut Vec<i32> = my_vec.borrow_mut();
//     borrowed_mut.push(4);
//     assert_eq!(borrowed_mut, &vec![1, 2, 3, 4]);
// }
//
// #[test]
// fn test_partial_eq() {
//     let a = MyVec::new(vec![1, 2, 3]);
//     let b = MyVec::new(vec![1, 2, 3]);
//     let c = MyVec::new(vec![1, 2, 4]);
//     assert_eq!(a, b);
//     assert_ne!(a, c);
// }
//
// #[test]
// fn test_partial_ord() {
//     let a = MyVec::new(vec![1, 2, 3]);
//     let b = MyVec::new(vec![1, 2, 4]);
//     assert!(a < b);
// }
