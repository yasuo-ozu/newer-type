use newer_type::{implement, target};
use std::fmt::Debug;

#[allow(unused)]
struct Implementor<T>(core::marker::PhantomData<T>, core::convert::Infallible);

// 1. トレイト関数のシグネチャとして複数の `Self` を持つが、戻り値に `Self`
//    を含まない
#[target(implementor = Implementor)]
trait MultiSelfArgTrait {
    fn process(self, other: Self, reference: &Self, mutable: &mut Self) -> i32;
    fn process_no_receiver(other: Self, reference: &Self) -> bool;
    fn process_with_ref(&self, other: &Self) -> String;
    fn process_with_mut(&mut self, other: &mut Self);
}

#[derive(Debug, Clone, PartialEq)]
struct TestType(i32);

impl MultiSelfArgTrait for TestType {
    fn process(self, other: Self, reference: &Self, mutable: &mut Self) -> i32 {
        mutable.0 += self.0 + other.0 + reference.0;
        mutable.0
    }

    fn process_no_receiver(other: Self, reference: &Self) -> bool {
        other.0 == reference.0
    }

    fn process_with_ref(&self, other: &Self) -> String {
        format!("{} + {}", self.0, other.0)
    }

    fn process_with_mut(&mut self, other: &mut Self) {
        self.0 += other.0;
        other.0 = 0;
    }
}

#[implement(MultiSelfArgTrait)]
struct MultiSelfArgNewType(TestType);

#[test]
fn test_process() {
    let instance = MultiSelfArgNewType(TestType(10));
    let other = MultiSelfArgNewType(TestType(20));
    let reference = MultiSelfArgNewType(TestType(5));
    let mut mutable = MultiSelfArgNewType(TestType(0));

    let result = instance.process(other, &reference, &mut mutable);
    assert_eq!(result, 35);
}

#[test]
fn test_process_no_receiver() {
    let other = MultiSelfArgNewType(TestType(15));
    let reference = MultiSelfArgNewType(TestType(15));

    assert!(MultiSelfArgNewType::process_no_receiver(other, &reference));
}

#[test]
fn test_process_with_ref() {
    let instance = MultiSelfArgNewType(TestType(7));
    let other = MultiSelfArgNewType(TestType(3));

    let result = instance.process_with_ref(&other);
    assert_eq!(result, "7 + 3");
}

#[test]
fn test_process_with_mut() {
    let mut instance = MultiSelfArgNewType(TestType(30));
    let mut other = MultiSelfArgNewType(TestType(10));

    instance.process_with_mut(&mut other);
    assert_eq!(instance.0 .0, 40);
    assert_eq!(other.0 .0, 0);
}

// 2. 名前付きフィールドを持つ構造体で `#[implement]`
#[derive(Debug, Clone, PartialEq)]
#[implement]
struct NamedStruct {
    field1: i32,
    #[implement(MultiSelfArgTrait)]
    field2: TestType,
    field3: bool,
}

#[implement]
struct NamedStructWrapper {
    #[implement(MultiSelfArgTrait)]
    inner: NamedStruct,
    _additional: String,
}

#[test]
fn test_named_struct_wrapper() {
    let instance = NamedStructWrapper {
        inner: NamedStruct {
            field1: 5,
            field2: TestType(10),
            field3: false,
        },
        _additional: "extra".to_string(),
    };
    let other = TestType(20);
    let reference = TestType(5);
    let mut mutable = TestType(0);

    let result = instance
        .inner
        .field2
        .process(other, &reference, &mut mutable);
    assert_eq!(result, 35);
}

// 3. タプル構造体で `#[implement]`
#[derive(Debug, Clone, PartialEq)]
#[implement]
struct TupleStruct(i32, #[implement(MultiSelfArgTrait)] TestType, bool);

#[implement]
#[allow(dead_code)]
struct TupleStructWrapper(#[implement(MultiSelfArgTrait)] TupleStruct, String);

#[test]
fn test_tuple_struct_wrapper() {
    let instance = TupleStructWrapper(TupleStruct(5, TestType(10), false), "extra".to_string());
    let other = TestType(20);
    let reference = TestType(5);
    let mut mutable = TestType(0);

    let result = instance.0 .1.process(other, &reference, &mut mutable);
    assert_eq!(result, 35);
}

// 4. ネストした `#[implement]`
#[derive(Debug, Clone, PartialEq)]
#[implement]
struct NestedStruct {
    level1: NamedStruct,
    #[implement(MultiSelfArgTrait)]
    level2: TestType,
}

#[implement]
struct DeeplyNestedStruct {
    #[implement(MultiSelfArgTrait)]
    deep: NestedStruct,
    _extra_data: i64,
}

#[test]
fn test_deeply_nested_struct() {
    let instance = DeeplyNestedStruct {
        deep: NestedStruct {
            level1: NamedStruct {
                field1: 3,
                field2: TestType(7),
                field3: true,
            },
            level2: TestType(12),
        },
        _extra_data: 99,
    };
    let other = TestType(8);
    let reference = TestType(4);
    let mut mutable = TestType(0);

    let result = instance
        .deep
        .level2
        .process(other, &reference, &mut mutable);
    assert_eq!(result, 24);
}
