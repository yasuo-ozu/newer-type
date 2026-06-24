//! Audit regression test: synthesized associated-type parameters were not
//! valid camel-case type names.
//!
//! Bug #5 (`EmitImpl::emit_impl` in `macro/implement_internal.rs`): when an enum
//! implements a trait that has an associated type, the macro introduces a fresh
//! generic *type* parameter to unify that associated type across the variants.
//! It was named `ASSOC_{Name}_{nonce}` (and, on the supertrait path,
//! `IMPL_ASSOC_{Name}_{nonce}`) — SCREAMING_SNAKE_CASE used in type position.
//! The crate-level `#![deny(...)]` turns the resulting lint into a hard error,
//! so before the fix this file fails to compile with:
//!     error: type parameter `ASSOC_Item_...` should have an upper camel case name
#![deny(non_camel_case_types)]

use newer_type::{implement, target};

pub trait Repeater<const TRAIT_ID: u64, const NTH: usize, T: ?Sized> {
    type Type;
}

#[target(repeater = Repeater)]
trait Producer {
    type Item;
    fn produce(&self) -> Self::Item;
}

impl Producer for i32 {
    type Item = i32;
    fn produce(&self) -> i32 {
        *self
    }
}

// An enum over `i32` predicates: all variants share the associated `Item`, which
// the macro models with a synthesized type parameter.
#[implement(Producer)]
#[allow(dead_code)]
enum E {
    A(i32),
    B(i32),
}

#[test]
fn enum_assoc_type_param_is_camel_case() {
    assert_eq!(E::A(7).produce(), 7);
    assert_eq!(E::B(9).produce(), 9);
}
