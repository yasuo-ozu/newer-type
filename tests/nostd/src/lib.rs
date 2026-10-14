//! A `#![no_std]` crate that announces a trait (with a `repeater`) and uses
//! `#[implement]`, through newer-type alone (no direct dependency on
//! `announce`). It must build for a target without std:
//! `cargo build -p newer-type-test-nostd --target x86_64-unknown-none`.
#![no_std]

use newer_type::announce::{announce, define_repeater};
use newer_type::implement;

define_repeater!(pub Repeater);

pub struct Meters(pub u32);

/// A plain trait, and one with a relative type (`Meters`) that needs the
/// repeater.
#[announce(crate = ::newer_type::announce, repeater = crate::Repeater)]
pub trait Measure {
    fn measure(&self) -> Meters;
    fn scaled(&self, by: u32) -> u32;
}

impl Measure for u32 {
    fn measure(&self) -> Meters {
        Meters(*self)
    }

    fn scaled(&self, by: u32) -> u32 {
        *self * by
    }
}

#[implement(Measure)]
pub struct Width(pub u32);

#[implement(Measure)]
pub enum Either {
    Left(u32),
    Right(u32),
}
