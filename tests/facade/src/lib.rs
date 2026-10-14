//! A facade crate: re-exports newer-type and announces traits for
//! `#[implement]` with it, without a dependency on `announce` of its own.
pub use newer_type;

newer_type::announce::define_repeater!(pub Repeater);

pub struct Foo(pub u8);

/// A parameter typed `crate::Foo` (it must read `$crate::Foo` in the
/// trampoline).
#[newer_type::announce::announce(crate = ::newer_type::announce)]
pub trait UsesFoo {
    fn get(&self, b: crate::Foo) -> u8;
}

impl UsesFoo for u8 {
    fn get(&self, b: crate::Foo) -> u8 {
        *self + b.0
    }
}

mod inner {
    pub struct Local(pub u8);
}
use inner::Local;

/// A relative (leaked) type, resolved through `Repeater`.
#[newer_type::announce::announce(crate = ::newer_type::announce, repeater = crate::Repeater)]
pub trait MakesLocal {
    fn make(&self) -> Local;
}

impl MakesLocal for u8 {
    fn make(&self) -> Local {
        Local(*self)
    }
}

pub fn local_value(l: &inner::Local) -> u8 {
    l.0
}

/// The facade itself depends on newer-type directly: `#[implement]` needs no
/// `newer_type = ..`.
#[newer_type::implement(UsesFoo)]
pub struct FacadeWrap(pub u8);
