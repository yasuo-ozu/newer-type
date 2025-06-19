use newer_type::implement;

#[implement(for<Rhs> newer_type_std::ops::Add<Rhs>)]
#[implement(for<Rhs> newer_type_std::ops::Sub<Rhs>)]
#[implement(for<Rhs> newer_type_std::ops::Mul<Rhs>)]
#[implement(for<Rhs> newer_type_std::ops::Div<Rhs>)]
#[implement(for<Rhs> newer_type_std::ops::Rem<Rhs>)]
#[implement(newer_type_std::ops::Neg)]
#[derive(Copy, Clone)]
pub struct MyOpsStruct {
    inner: i32,
}

#[test]
fn test_ops_struct() {
    let a = MyOpsStruct { inner: 10 };
    assert_eq!((a + 3), 13);
    assert_eq!((a - 3), 7);
    assert_eq!((a * 3), 30);
    assert_eq!((a / 3), 3);
    assert_eq!((a % 3), 1);
    assert_eq!(-a, -10);
}
