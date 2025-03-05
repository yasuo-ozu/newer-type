use newer_type::{implement, target};
use std::fmt::Debug;

// 1. Enum に対して `#[implement]` を適用する基本例
#[target]
trait BasicEnumTrait {
    fn value(&self) -> i32;
}

#[derive(Debug, Clone, PartialEq)]
#[implement]
enum BasicEnum {
    VariantA(#[implement(BasicEnumTrait)] i32),
    VariantB(#[implement(BasicEnumTrait)] i32),
}

impl BasicEnumTrait for i32 {
    fn value(&self) -> i32 {
        *self
    }
}

#[test]
fn test_basic_enum_trait() {
    let a = BasicEnum::VariantA(10);
    let b = BasicEnum::VariantB(20);

    if let BasicEnum::VariantA(val) = a {
        assert_eq!(val.value(), 10);
    }

    if let BasicEnum::VariantB(val) = b {
        assert_eq!(val.value(), 20);
    }
}

// 2. 名前付きフィールドを持つ Enum のトレイト実装
#[target]
trait NamedEnumTrait {
    fn sum(&self) -> i32;
}

#[derive(Debug, Clone, PartialEq)]
#[implement]
enum NamedEnum {
    Named {
        a: i32,
        #[implement(NamedEnumTrait)]
        b: i32,
    },
    Tuple(#[implement(NamedEnumTrait)] i32, i32),
}

impl NamedEnumTrait for i32 {
    fn sum(&self) -> i32 {
        *self
    }
}

#[test]
fn test_named_enum_trait() {
    let named = NamedEnum::Named { a: 10, b: 20 };
    let tuple = NamedEnum::Tuple(15, 25);

    #[allow(unused)]
    if let NamedEnum::Named { a, b } = named {
        assert_eq!(b.sum(), 20);
    }

    #[allow(unused)]
    if let NamedEnum::Tuple(x, y) = tuple {
        assert_eq!(x.sum(), 15);
    }
}

// 3. ジェネリクスを含む Enum のトレイト実装
#[target]
trait GenericEnumTrait<T> {
    fn describe(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
#[implement]
enum GenericEnum<T: Clone + Debug> {
    First(#[implement(GenericEnumTrait<T>)] T),
    Second(#[implement(GenericEnumTrait<T>)] T),
}

impl<T: Clone + Debug> GenericEnumTrait<T> for T {
    fn describe(&self) -> String {
        format!("Value: {:?}", self)
    }
}

#[test]
fn test_generic_enum_trait() {
    let first = GenericEnum::First(42);
    let second = GenericEnum::Second("Hello".to_string());

    if let GenericEnum::First(val) = first {
        assert_eq!(val.describe(), "Value: 42");
    }

    if let GenericEnum::Second(val) = second {
        assert_eq!(val.describe(), "Value: \"Hello\"");
    }
}

// 4. 複雑なフィールドを持つ Enum のトレイト実装
#[target]
trait ComplexEnumTrait {
    fn compute(&self) -> i32;
}

#[derive(Debug, Clone, PartialEq)]
#[implement]
enum ComplexEnum {
    Named {
        id: u32,
        #[implement(ComplexEnumTrait)]
        data: (i32, i32),
    },
    Tuple(u32, #[implement(ComplexEnumTrait)] (i32, i32)),
}

impl ComplexEnumTrait for (i32, i32) {
    fn compute(&self) -> i32 {
        self.0 * self.1
    }
}

#[test]
fn test_complex_enum_trait() {
    let named = ComplexEnum::Named {
        id: 1,
        data: (3, 4),
    };
    let tuple = ComplexEnum::Tuple(2, (5, 6));

    #[allow(unused)]
    if let ComplexEnum::Named { id, data } = named {
        assert_eq!(data.compute(), 12);
    }

    #[allow(unused)]
    if let ComplexEnum::Tuple(id, data) = tuple {
        assert_eq!(data.compute(), 30);
    }
}
// 5. ネストした型を持つ Enum のトレイト実装
#[target]
trait NestedEnumTrait {
    fn nested_value(&self) -> i32;
}

#[derive(Debug, Clone, PartialEq)]
#[implement]
enum NestedEnum {
    Variant(#[implement(NestedEnumTrait)] Box<i32>),
}

impl NestedEnumTrait for Box<i32> {
    fn nested_value(&self) -> i32 {
        **self
    }
}

#[test]
fn test_nested_enum_trait() {
    let variant = NestedEnum::Variant(Box::new(99));

    let NestedEnum::Variant(val) = variant;
    assert_eq!(val.nested_value(), 99);
}

// 6. 複数の `#[implement]` を持つ Enum のトレイト実装
#[target]
trait MultiImplementTrait {
    fn double(&self) -> i32;
}

#[derive(Debug, Clone, PartialEq)]
#[implement]
enum MultiImplementEnum {
    VariantOne(#[implement(MultiImplementTrait)] i32),
    VariantTwo(#[implement(MultiImplementTrait)] i32, i32),
}

impl MultiImplementTrait for i32 {
    fn double(&self) -> i32 {
        *self * 2
    }
}

#[test]
fn test_multi_implement_enum_trait() {
    let variant1 = MultiImplementEnum::VariantOne(5);
    let variant2 = MultiImplementEnum::VariantTwo(3, 7);

    if let MultiImplementEnum::VariantOne(val) = variant1 {
        assert_eq!(val.double(), 10);
    }

    if let MultiImplementEnum::VariantTwo(val1, val2) = variant2 {
        assert_eq!(val1.double(), 6);
        assert_eq!(val2.double(), 14);
    }
}
