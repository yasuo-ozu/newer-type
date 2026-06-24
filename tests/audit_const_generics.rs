//! Audit regression tests: const-generic wrapper types.
//!
//! These reproduce two defects in `macro/implement_internal.rs`:
//!
//! Bug #1 (`ModifyGenerics::append_const`): the const-parameter map was built
//! with `parse_quote!(nident)` instead of `parse_quote!(#nident)`, so a const
//! generic in a wrapper was rewritten to the *literal* identifier `nident`.
//! Before the fix this file fails to compile with:
//! `error[E0425]: cannot find value 'nident' in this scope`.
//!
//! Bug #4: even once the const param is substituted correctly, the synthesized
//! name (`NewerTypeTypeParam{N}Of{nonce}`) was not UPPER_CASE, so it tripped
//! `non_upper_case_globals`. The crate-level `#![deny(...)]` below turns that
//! warning into a hard error, so this file fails to compile without the fix:
//! `error: const parameter '...' should have an upper case name`.
#![deny(non_upper_case_globals)]

use newer_type::{implement, target};

pub trait Repeater<const TRAIT_ID: u64, const NTH: usize, T: ?Sized> {
    type Type;
}

#[target(repeater = Repeater)]
trait Len {
    fn length(&self) -> usize;
}

impl<const N: usize> Len for [u8; N] {
    fn length(&self) -> usize {
        N
    }
}

// The const generic `N` is used inside the inner field type `[u8; N]`, which is
// exactly the position that triggered the `nident` substitution bug.
#[implement(Len)]
struct Arr<const N: usize>([u8; N]);

#[test]
fn const_generic_wrapper_compiles_and_forwards() {
    let a = Arr([1u8, 2, 3, 4]);
    assert_eq!(a.length(), 4);

    let b = Arr([]);
    assert_eq!(b.length(), 0);
}
