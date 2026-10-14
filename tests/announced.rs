//! Traits announced with `#[announce]` (there is no `#[target]`): the options
//! that `#[implement]` reads, and the relative types carried by a `repeater`.

use newer_type::announce::{announce, define_repeater};
use newer_type::implement;
use std::marker::PhantomData;

define_repeater!(pub Repeater);

// ---- a trait with absolute paths needs no option
// --------------------------------

#[announce]
trait Plain {
    fn plain(&self) -> ::std::string::String;
}

impl Plain for u8 {
    fn plain(&self) -> ::std::string::String {
        format!("plain {}", self)
    }
}

#[implement(Plain)]
struct PlainWrap(u8);

#[test]
fn plain_announce() {
    assert_eq!(PlainWrap(3).plain(), "plain 3");
}

// ---- `crate = ..` names the announce runtime through newer-type
// ------------------

#[newer_type::announce::announce(crate = ::newer_type::announce, allow_std_prelude = true)]
trait ViaReexport {
    fn via(&self) -> Option<u8>;
}

impl ViaReexport for u8 {
    fn via(&self) -> Option<u8> {
        Some(*self)
    }
}

#[implement(ViaReexport)]
struct ViaWrap(u8);

#[test]
fn announce_through_reexport() {
    assert_eq!(ViaWrap(5).via(), Some(5));
}

// ---- `mock = ..` implements another trait
// --------------------------------------

mod mirror {
    // The mirror of `std::ops::Neg`; `#[implement(Neg)]` implements the real
    // one.
    #[newer_type::announce::announce(mock = ::core::ops::Neg)]
    pub trait Neg {
        type Output;
        fn neg(self) -> Self::Output;
    }
}

#[derive(Debug, PartialEq)]
#[implement(mirror::Neg)]
struct Negative(i8);

#[test]
fn mock_implements_the_real_trait() {
    assert_eq!(-Negative(4), -4);
}

// ---- relative types through a `repeater`
// ---------------------------------------

#[derive(Debug, PartialEq)]
pub struct Tagged<T>(pub T);

#[derive(Debug, PartialEq)]
pub struct Plainly(pub u8);

#[announce(repeater = crate::Repeater)]
trait Tag<T> {
    fn tag(&self, t: T) -> Tagged<T>;
    fn plainly(&self) -> Plainly;
}

impl<T> Tag<T> for u8 {
    fn tag(&self, t: T) -> Tagged<T> {
        Tagged(t)
    }

    fn plainly(&self) -> Plainly {
        Plainly(*self)
    }
}

// The argument of the trait is a concrete type, ..
#[implement(Tag<u16>)]
struct Concrete(u8);

// .. a parameter of the implementor, ..
#[implement(for<U> Tag<U>)]
struct Free(u8);

// .. or a parameter of the wrapped type.
#[implement(Tag<V>)]
struct Own<V>(V);

#[implement(Tag<u32>)]
enum Either {
    A(u8),
    B(u8),
}

#[test]
fn repeater_with_trait_arguments() {
    assert_eq!(Concrete(1).tag(7u16), Tagged(7u16));
    assert_eq!(Concrete(1).plainly(), Plainly(1));
    assert_eq!(Free(2).tag("x"), Tagged("x"));
    assert_eq!(Own(3u8).tag(9u8), Tagged(9u8));
    assert_eq!(Either::A(4).tag(5u32), Tagged(5u32));
    assert_eq!(Either::B(6).plainly(), Plainly(6));
}

// A defaulted parameter, omitted by `#[implement]`.
#[announce(repeater = crate::Repeater)]
trait Defaulted<T = u8> {
    fn defaulted(&self, t: T) -> Tagged<T>;
}

impl<T> Defaulted<T> for u8 {
    fn defaulted(&self, t: T) -> Tagged<T> {
        Tagged(t)
    }
}

#[implement(Defaulted)]
struct OmitsDefault(u8);

#[implement(Defaulted<i64>)]
struct GivesArgument(u8);

#[test]
fn repeater_with_defaulted_parameter() {
    assert_eq!(OmitsDefault(0).defaulted(1u8), Tagged(1u8));
    assert_eq!(GivesArgument(0).defaulted(-1i64), Tagged(-1i64));
}

// A relative type that names a generic of the method.
#[announce(repeater = crate::Repeater)]
trait Generic {
    fn generic<U>(&self, u: U) -> Tagged<U>;
    // A provided method: its body is not forwarded, and is not analysed.
    fn twice<U: ::core::clone::Clone>(&self, u: U) -> Tagged<U> {
        let _local: Tagged<U> = Tagged(u.clone());
        Tagged(u)
    }
}

impl Generic for u8 {
    fn generic<U>(&self, u: U) -> Tagged<U> {
        Tagged(u)
    }
}

#[implement(Generic)]
struct GenericWrap(u8);

#[test]
fn repeater_with_method_generics() {
    assert_eq!(GenericWrap(0).generic("g"), Tagged("g"));
    assert_eq!(GenericWrap(0).twice("t"), Tagged("t"));
}

// A relative type in an associated type and with a lifetime.
#[announce(repeater = crate::Repeater)]
trait Borrow2 {
    type Out<'a>
    where
        Self: 'a;
    fn borrow2<'a>(&'a self, p: PhantomData<&'a ()>) -> Self::Out<'a>;
}

impl Borrow2 for String {
    type Out<'a> = &'a str;

    fn borrow2<'a>(&'a self, _: PhantomData<&'a ()>) -> &'a str {
        self
    }
}

#[implement(Borrow2)]
struct BorrowWrap(String);

#[test]
fn repeater_with_lifetime() {
    assert_eq!(BorrowWrap("b".into()).borrow2(PhantomData), "b");
}

// ---- the paths of an inline-module copy resolve where `#[implement]` is used
// ------

mod copy {
    pub trait Where {
        fn here(&self) -> super::Here;
    }
    // Only the trampoline is emitted; `Here` is looked up at the `#[implement]`
    // site.
    #[newer_type::announce::announce(emit_item = false)]
    #[allow(non_snake_case)]
    pub mod Where {
        pub trait Where {
            fn here(&self) -> Here;
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Here(pub u8);

impl copy::Where for u8 {
    fn here(&self) -> Here {
        Here(*self)
    }
}

#[implement(copy::Where)]
struct CopyWrap(u8);

#[test]
fn module_copy_resolves_at_the_use_site() {
    use copy::Where;
    assert_eq!(CopyWrap(8).here(), Here(8));
}
