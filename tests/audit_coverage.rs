//! Audit coverage tests: behaviours that were untested (so a future regression
//! would go unnoticed), even though the current implementation handles them.
//!
//! * GATs: `tests/test5.rs` only *compiles* a generic associated type behind a
//!   `todo!()` body — it never exercises the forwarded value at runtime.
//! * `&mut self` / by-value `self` forwarding through an *enum* (existing
//!   mutation/consume tests only cover structs).
//! * Wrapper with a lifetime parameter.
//! * `#[target]` with no `repeater = ...` argument (the synthesized-repeater path
//!   added in commit "Make repeater argument optional").

use newer_type::{implement, target};

pub trait Repeater<const TRAIT_ID: u64, const NTH: usize, T: ?Sized> {
    type Type;
}

// ---- GAT forwarding actually returns the right value -----------------------

#[target(repeater = Repeater)]
trait Lender {
    type Out<'a>
    where
        Self: 'a;
    fn lend<'a>(&'a self) -> Self::Out<'a>;
}

impl Lender for String {
    type Out<'a> = &'a str;
    fn lend(&self) -> &str {
        self.as_str()
    }
}

#[implement(Lender)]
struct Lent(String);

#[test]
fn gat_forwarding_runtime() {
    let w = Lent(String::from("hello"));
    let borrowed: &str = w.lend();
    assert_eq!(borrowed, "hello");
}

// ---- enum `&mut self` and by-value `self` ----------------------------------

#[target(repeater = Repeater)]
trait Counter {
    fn bump(&mut self);
    fn into_value(self) -> i32;
}

#[derive(Clone)]
struct Cell(i32);

impl Counter for Cell {
    fn bump(&mut self) {
        self.0 += 1;
    }
    fn into_value(self) -> i32 {
        self.0
    }
}

#[implement(Counter)]
#[allow(dead_code)]
enum Counters {
    A(Cell),
    B(Cell),
}

#[test]
fn enum_mut_and_consume() {
    let mut e = Counters::A(Cell(10));
    e.bump();
    e.bump();
    assert_eq!(e.into_value(), 12);
}

// ---- lifetime-parameterized wrapper ----------------------------------------

#[target(repeater = Repeater)]
trait Speak {
    fn speak(&self) -> String;
}

impl Speak for &str {
    fn speak(&self) -> String {
        self.to_string()
    }
}

#[implement(Speak)]
struct Borrowed<'a>(&'a str);

#[test]
fn lifetime_wrapper() {
    let s = String::from("hi");
    assert_eq!(Borrowed(s.as_str()).speak(), "hi");
}

// ---- `#[target]` with no explicit repeater ---------------------------------

#[target]
trait Greet {
    fn greet(&self) -> String;
}

impl Greet for String {
    fn greet(&self) -> String {
        format!("hi {self}")
    }
}

#[implement(Greet)]
struct Named(String);

#[test]
fn target_without_repeater_argument() {
    assert_eq!(Named(String::from("bob")).greet(), "hi bob");
}
