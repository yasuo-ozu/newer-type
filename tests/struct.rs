use newer_type::{implement, target};

#[target]
trait MyTrait {
    fn my_func(&self);
}

struct MyStruct1;

impl MyTrait for MyStruct1 {
    fn my_func(&self) {}
}

#[implement(MyTrait)]
struct MyStruct {
    slot: MyStruct1,
}

#[implement(MyTrait)]
enum MyEnum {
    V1(MyStruct1),
    V2(MyStruct1),
}
