use newer_type::{implement, target};
use std::fmt::Debug;

pub trait Repeater<const TRAIT_ID: u64, const NTH: usize, T: ?Sized> {
    type Type;
}

// 1. 基本的なトレイトの拡張
#[target(repeater = Repeater)]
trait BasicTrait {
    fn get_number(&self) -> i32;
    fn double_number(&self) -> i32 {
        self.get_number() * 2
    }
}

struct BasicType(i32);

impl BasicTrait for BasicType {
    fn get_number(&self) -> i32 {
        self.0
    }
}

#[implement(BasicTrait)]
struct BasicNewType(BasicType);

#[test]
fn test_basic_trait() {
    let instance = BasicNewType(BasicType(10));
    assert_eq!(instance.get_number(), 10);
    assert_eq!(instance.double_number(), 20);
}

// 2. トレイトジェネリクスを持つケース
#[target(repeater = Repeater)]
trait GenericTrait<T> {
    fn process(&self, input: T) -> T;
}

impl GenericTrait<i32> for BasicType {
    fn process(&self, input: i32) -> i32 {
        self.0 + input
    }
}

#[implement(GenericTrait<i32>)]
struct GenericNewType(BasicType);

#[test]
fn test_generic_trait() {
    let instance = GenericNewType(BasicType(5));
    assert_eq!(instance.process(10), 15);
}

// 3. トレイトジェネリクスと関数ジェネリクスを持つケース
#[target(repeater = Repeater)]
trait AdvancedTrait<T> {
    fn compute<U>(&self, value: T, extra: U) -> (T, U);
}

impl AdvancedTrait<i32> for BasicType {
    fn compute<U>(&self, value: i32, extra: U) -> (i32, U) {
        (self.0 + value, extra)
    }
}

#[implement(AdvancedTrait<i32>)]
struct AdvancedNewType(BasicType);

#[test]
fn test_advanced_trait() {
    let instance = AdvancedNewType(BasicType(7));
    let result = instance.compute(3, "extra");
    assert_eq!(result, (10, "extra"));
}

// 4. `where` 節を持つトレイト
#[target(repeater = Repeater)]
trait ComplexTrait<T>
where
    T: ::core::clone::Clone + ::core::fmt::Debug,
{
    fn describe(&self, item: T) -> ::std::string::String;
}

impl ComplexTrait<String> for BasicType {
    fn describe(&self, item: String) -> String {
        format!("Value: {}, Extra: {}", self.0, item)
    }
}

#[implement(ComplexTrait<String>)]
struct ComplexNewType(BasicType);

#[test]
fn test_complex_trait() {
    let instance = ComplexNewType(BasicType(42));
    let description = instance.describe("test".to_string());
    assert_eq!(description, "Value: 42, Extra: test");
}

// 5. ジェネリックな `where` 節を持つケース
#[target(repeater = Repeater)]
trait UltimateTrait<T, U>
where
    T: ::core::fmt::Debug + ::core::clone::Clone,
    U: ::core::cmp::PartialEq,
{
    fn combine(&self, a: T, b: U) -> (T, bool);
}

impl UltimateTrait<String, i32> for BasicType {
    fn combine(&self, a: String, b: i32) -> (String, bool) {
        (format!("{}-{}", a, self.0), self.0 == b)
    }
}

#[implement(UltimateTrait<String, i32>)]
struct UltimateNewType(BasicType);

#[test]
fn test_ultimate_trait() {
    let instance = UltimateNewType(BasicType(99));
    let result = instance.combine("Hello".to_string(), 99);
    assert_eq!(result, ("Hello-99".to_string(), true));
}

// 6. 自由パラメータを持つトレイトの適用
#[target(repeater = Repeater)]
trait FreeParamTrait<'a, A, B>
where
    A: ::core::clone::Clone,
{
    fn complex_method(&self, input: &'a A) -> B;
}

impl<'a, A> FreeParamTrait<'a, A, u32> for BasicType
where
    A: Clone,
{
    fn complex_method(&self, _input: &'a A) -> u32 {
        self.0 as u32 + 1
    }
}

#[implement(for<'a, A> FreeParamTrait<'a, A, u32> where A: Clone)]
struct FreeParamNewType(BasicType);

#[test]
fn test_free_param_trait() {
    let instance = FreeParamNewType(BasicType(50));
    let input = "test".to_string();
    assert_eq!(instance.complex_method(&input), 51);
}

// 7. 高度な自由パラメータ + `where` 節
#[target(repeater = Repeater)]
trait AdvancedFreeParam<'a, A, B, C>
where
    A: ::core::clone::Clone + ::core::fmt::Debug,
    B: ::core::cmp::PartialEq<i32>,
    C: ::core::default::Default,
{
    fn advanced_method(&self, input: &'a A, flag: B) -> C;
}

impl<'a, A, B> AdvancedFreeParam<'a, A, B, String> for BasicType
where
    A: Clone + Debug,
    B: PartialEq<i32>,
{
    fn advanced_method(&self, input: &'a A, flag: B) -> String {
        if flag == self.0 {
            format!("Matched: {:?}", input)
        } else {
            "No Match".to_string()
        }
    }
}

#[implement(for<'a, A, B> AdvancedFreeParam<'a, A, B, String> where A: Clone + Debug, B: PartialEq<i32>)]
struct AdvancedFreeParamNewType(BasicType);

#[test]
fn test_advanced_free_param_trait() {
    let instance = AdvancedFreeParamNewType(BasicType(42));
    let input = "complex".to_string();
    assert_eq!(instance.advanced_method(&input, 42), "Matched: \"complex\"");
    assert_eq!(instance.advanced_method(&input, 100), "No Match");
}

// 2. 関数ポインタを扱うトレイト
#[target(repeater = Repeater)]
trait FunctionPointerTrait {
    fn apply_fn(&self, f: fn(i32) -> i32) -> i32;
}

impl FunctionPointerTrait for BasicType {
    fn apply_fn(&self, f: fn(i32) -> i32) -> i32 {
        f(self.0)
    }
}

#[implement(FunctionPointerTrait)]
struct FunctionPointerNewType(BasicType);

#[test]
fn test_function_pointer_trait() {
    let instance = FunctionPointerNewType(BasicType(5));
    assert_eq!(instance.apply_fn(|x| x * 2), 10);
}

// 3. 関連型を持つトレイト
#[target(repeater = Repeater)]
trait AssociatedTypeTrait {
    type Output;
    fn compute(&self) -> Self::Output;
}

impl AssociatedTypeTrait for BasicType {
    type Output = i32;

    fn compute(&self) -> Self::Output {
        self.0 * 2
    }
}

#[implement(AssociatedTypeTrait)]
struct AssociatedTypeNewType(BasicType);

#[test]
fn test_associated_type_trait() {
    let instance = AssociatedTypeNewType(BasicType(6));
    assert_eq!(instance.compute(), 12);
}

// 5. `&mut self` を扱うトレイト
#[target(repeater = Repeater)]
trait MutatingTrait {
    fn increment(&mut self);
}

impl MutatingTrait for BasicType {
    fn increment(&mut self) {
        self.0 += 1;
    }
}

#[implement(MutatingTrait)]
struct MutatingNewType(BasicType);

#[test]
fn test_mutating_trait() {
    let mut instance = MutatingNewType(BasicType(10));
    instance.increment();
    assert_eq!(instance.0 .0, 11);
}

// 7. 複数の型制約を持つトレイト
#[target(repeater = Repeater)]
trait ComplexConstraintTrait<T>
where
    T: ::core::fmt::Debug
        + ::core::clone::Clone
        + ::core::cmp::PartialEq
        + ::core::default::Default,
{
    fn process_item(&self, item: T) -> T;
}

impl ComplexConstraintTrait<String> for BasicType {
    fn process_item(&self, item: String) -> String {
        format!("{}-processed", item)
    }
}

#[implement(ComplexConstraintTrait<String>)]
struct ComplexConstraintNewType(BasicType);

#[test]
fn test_complex_constraint_trait() {
    let instance = ComplexConstraintNewType(BasicType(0));
    let result = instance.process_item("Test".to_string());
    assert_eq!(result, "Test-processed");
}

// 6. Associated Consts を持つトレイト
#[target(repeater = Repeater)]
trait AssociatedConstTrait {
    const VALUE: i32;
    fn get_const_value(&self) -> i32 {
        Self::VALUE
    }
}

impl AssociatedConstTrait for BasicType {
    const VALUE: i32 = 100;
}

#[implement(AssociatedConstTrait)]
struct AssociatedConstNewType(BasicType);

#[test]
fn test_associated_const_trait() {
    let instance = AssociatedConstNewType(BasicType(0));
    assert_eq!(instance.get_const_value(), 100);
}
