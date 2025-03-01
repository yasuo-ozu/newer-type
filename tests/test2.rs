use newer_type::{implement, target};

// 1. シンプルな newtype のテスト
#[target]
trait MyTrait {
    fn value(&self) -> i32;
}

#[derive(Copy, Clone)]
struct MyExistingType(i32);

impl MyTrait for MyExistingType {
    fn value(&self) -> i32 {
        self.0
    }
}

#[implement(MyTrait)]
struct MyNewType(MyExistingType);

#[test]
fn test_simple_newtype() {
    let instance = MyNewType(MyExistingType(42));
    assert_eq!(instance.value(), 42);
}

// 2. フィールドの一部だけがトレイトを実装するパターン
#[implement]
struct ComplexNewType {
    field1: usize,
    #[implement(MyTrait)]
    field2: MyExistingType,
    field3: (),
}

#[test]
fn test_partial_implement() {
    let instance = ComplexNewType {
        field1: 10,
        field2: MyExistingType(100),
        field3: (),
    };
    assert_eq!(instance.field2.value(), 100);
}

// 3. 異なるトレイトの適用
#[target]
trait AnotherTrait {
    fn double_value(&self) -> i32;
}

impl AnotherTrait for MyExistingType {
    fn double_value(&self) -> i32 {
        self.0 * 2
    }
}

#[implement(MyTrait, AnotherTrait)]
struct DualTraitNewType(MyExistingType);

#[test]
fn test_dual_traits() {
    let instance = DualTraitNewType(MyExistingType(10));
    assert_eq!(instance.value(), 10);
    assert_eq!(instance.double_value(), 20);
}

// 4. タプル構造体のサポート
#[implement(MyTrait)]
struct TupleNewType(MyExistingType);

#[test]
fn test_tuple_newtype() {
    let instance = TupleNewType(MyExistingType(55));
    assert_eq!(instance.value(), 55);
}

// 5. ジェネリック型のサポート
#[target]
trait GenericTrait<T> {
    fn get_value(&self) -> &T;
}

impl<T> GenericTrait<T> for Option<T> {
    fn get_value(&self) -> &T {
        self.as_ref().unwrap()
    }
}

#[implement(GenericTrait<T>)]
struct GenericNewType<T>(Option<T>);

#[test]
fn test_generic_newtype() {
    let instance = GenericNewType(Some(99));
    assert_eq!(*instance.get_value(), 99);
}

// 6. ネストした `#[implement]` の動作
#[implement]
struct NestedNewType {
    #[implement(MyTrait)]
    inner: MyNewType,
}

#[test]
fn test_nested_newtype() {
    let instance = NestedNewType {
        inner: MyNewType(MyExistingType(77)),
    };
    assert_eq!(instance.inner.value(), 77);
}

// 7. Enum に対する `#[implement]`（すべてのバリアントが MyTrait を実装）
#[implement(MyTrait)]
enum MyEnum {
    Variant1(MyExistingType),
    Variant2(MyExistingType),
}

#[test]
fn test_enum_newtype() {
    let instance = MyEnum::Variant1(MyExistingType(88));
    match instance {
        MyEnum::Variant1(inner) | MyEnum::Variant2(inner) => assert_eq!(inner.value(), 88),
    }
}

// 8. Copy/Clone を持つ型
#[derive(Copy, Clone)]
#[implement(MyTrait)]
struct CopyNewType(MyExistingType);

#[test]
fn test_copy_clone_newtype() {
    let instance = CopyNewType(MyExistingType(33));
    let copy = instance;
    assert_eq!(copy.value(), 33);
}

// 9. Vec<T> の newtype
#[implement]
struct VecNewType<T>(Vec<T>);

#[test]
fn test_vec_newtype() {
    let instance = VecNewType(vec![1, 2, 3]);
    assert_eq!(instance.0.len(), 3);
}

// 10. トレイトのデフォルト実装
#[target]
trait DefaultTrait {
    fn default_value(&self) -> i32 {
        999
    }
}

impl DefaultTrait for MyExistingType {}

#[implement(DefaultTrait)]
struct DefaultNewType(MyExistingType);

#[test]
fn test_default_trait() {
    let instance = DefaultNewType(MyExistingType(0));
    assert_eq!(instance.default_value(), 999);
}
