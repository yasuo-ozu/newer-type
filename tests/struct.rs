use newer_type::{implement, target};

#[target]
trait MyTrait {
    fn my_func(&self);
    fn my_func_2(self);
    fn my_func_3(_a: usize, this: Self, _c: usize);
}

struct MyStruct1;

impl MyTrait for MyStruct1 {
    fn my_func(&self) {}

    fn my_func_2(self) {}

    fn my_func_3(_a: usize, this: Self, _c: usize) {}
}

#[implement(MyTrait)]
struct MyStruct {
    slot: MyStruct1,
}

#[implement()]
struct MyStruct2 {
    item1: usize,
    #[implement(MyTrait)]
    item2: MyStruct1,
}

#[implement(MyTrait)]
enum MyEnum {
    V1(MyStruct1),
    V2(MyStruct1),
}
