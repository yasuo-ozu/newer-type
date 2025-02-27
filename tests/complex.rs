use newer_type::{implement, target};

#[target]
trait MyTrait<'a, T, V> {
    fn my_func<'r>(&'r mut self, t: &'a T) -> &'a T;
}

struct MyStruct1<'a, T, U> {
    _a: &'a T,
    _b: U,
}

impl<'a, T, U, V> MyTrait<'a, T, V> for MyStruct1<'a, T, U> {
    fn my_func<'r>(&'r mut self, t: &'a T) -> &'a T {
        t
    }
}

#[implement(for<V> MyTrait<'a, T, V> where V: Clone)]
struct MyStruct<'a, T> {
    slot: MyStruct1<'a, T, T>,
}

#[implement(for<V> MyTrait<'a, T, V> where V: Clone)]
enum MyEnum<'a, T> {
    V1(MyStruct1<'a, T, usize>),
    V2(MyStruct1<'a, T, u64>),
}
