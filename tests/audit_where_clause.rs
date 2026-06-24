//! Audit regression test: the wrapper type's own `where` clause was dropped.
//!
//! Bug #2 (`EmitImpl::emit_impl` in `macro/implement_internal.rs`): the generated
//! `impl` used only `adt_generics.split_for_impl().1` (the type generics) and
//! never re-emitted `adt_generics.where_clause`. For a wrapper whose existence is
//! gated on a bound (e.g. `struct Wrap<T>(T) where T: Clone;`), the generated
//! `impl<T> Trait for Wrap<T>` is not well-formed, so before the fix this file
//! fails to compile with:
//!     error[E0277]: the trait bound `...: Clone` is not satisfied
//!     note: required by a bound in `Wrap`

use newer_type::{implement, target};

pub trait Repeater<T: ?Sized, const TRAIT_ID: u64, const NTH: usize> {
    type Type;
}

#[target(repeater = Repeater)]
trait Speak {
    fn speak(&self) -> String;
}

impl Speak for String {
    fn speak(&self) -> String {
        self.clone()
    }
}

// (1) Struct wrapper carrying its own `where` clause.
#[implement(Speak)]
struct Wrap<T>(T)
where
    T: Clone;

#[test]
fn struct_where_clause_is_preserved() {
    let w = Wrap(String::from("hi"));
    assert_eq!(w.speak(), "hi");
}

// (2) Same situation for an enum wrapper.
#[implement(Speak)]
#[allow(dead_code)]
enum MaybeWrap<T>
where
    T: Clone,
{
    Here(T),
    There(T),
}

#[test]
fn enum_where_clause_is_preserved() {
    let e = MaybeWrap::Here(String::from("yo"));
    match e {
        MaybeWrap::Here(v) | MaybeWrap::There(v) => assert_eq!(v.speak(), "yo"),
    }
}
