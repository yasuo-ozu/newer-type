use newer_type::{implement, target};
use std::fmt::Debug;

struct Implementor<T>(core::marker::PhantomData<T>, core::convert::Infallible);

// 1. Associated Consts & Associated Types を持つトレイト
#[target(implementor = Implementor)]
trait ComplexTrait {
    const SCALE: i32;
    type Output;

    fn compute(&self, input: i32) -> Self::Output;
}

struct AdvancedType(i32);

impl ComplexTrait for AdvancedType {
    type Output = i32;

    const SCALE: i32 = 10;

    fn compute(&self, input: i32) -> Self::Output {
        self.0 + input * Self::SCALE
    }
}

#[implement(ComplexTrait)]
struct ComplexNewType(AdvancedType);

#[test]
fn test_complex_trait() {
    let instance = ComplexNewType(AdvancedType(5));
    assert_eq!(instance.compute(3), 35);
    assert_eq!(ComplexNewType::SCALE, 10);
}

// 2. Associated Consts, Types, and Generics
#[target(implementor = Implementor)]
trait MultiAssocTrait<T> {
    const FACTOR: T;
    type Result;

    fn transform(&self, input: T) -> Self::Result;
}

impl MultiAssocTrait<i32> for AdvancedType {
    type Result = i32;

    const FACTOR: i32 = 2;

    fn transform(&self, input: i32) -> Self::Result {
        self.0 * input * Self::FACTOR
    }
}

#[implement(MultiAssocTrait<i32>)]
struct MultiAssocNewType(AdvancedType);

#[test]
fn test_multi_assoc_trait() {
    let instance = MultiAssocNewType(AdvancedType(4));
    assert_eq!(instance.transform(5), 40);
    assert_eq!(MultiAssocNewType::FACTOR, 2);
}

// 3. `where` 節を含むトレイト
#[target(implementor = Implementor)]
trait ConstrainedTrait<T>
where
    T: ::core::fmt::Debug + ::core::clone::Clone + ::core::default::Default,
{
    const LIMIT: usize;
    type Item;
    fn process(&self, input: T) -> Self::Item;
}

impl ConstrainedTrait<String> for AdvancedType {
    type Item = String;

    const LIMIT: usize = 5;

    fn process(&self, input: String) -> Self::Item {
        format!("{}-processed", input)
    }
}

#[implement(ConstrainedTrait<String>)]
struct ConstrainedNewType(AdvancedType);

#[test]
fn test_constrained_trait() {
    let instance = ConstrainedNewType(AdvancedType(0));
    assert_eq!(instance.process("Hello".to_string()), "Hello-processed");
    assert_eq!(ConstrainedNewType::LIMIT, 5);
}

// 4. 自由パラメータを含む高度なトレイト
#[target(implementor = Implementor)]
trait FreeParamComplex<'a, A, B>
where
    A: ::core::fmt::Debug + ::core::clone::Clone,
    B: ::core::default::Default,
{
    const MULTIPLIER: i32;
    type Output;
    fn perform(&self, input: &'a A) -> (Self::Output, B);
}

impl<'a, A> FreeParamComplex<'a, A, String> for AdvancedType
where
    A: Debug + Clone,
{
    type Output = i32;

    const MULTIPLIER: i32 = 3;

    fn perform(&self, input: &'a A) -> (Self::Output, String) {
        (self.0 * 3, format!("{:?}", input))
    }
}

#[implement(for<'a, A> FreeParamComplex<'a, A, String> where A: Debug + Clone)]
struct FreeParamComplexNewType(AdvancedType);

#[test]
fn test_free_param_complex() {
    let instance = FreeParamComplexNewType(AdvancedType(7));
    let input = "test".to_string();
    let result = instance.perform(&input);
    assert_eq!(result, (21, "\"test\"".to_string()));
}
