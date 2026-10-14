//! Bindings of associated types in the implementor:
//! `#[implement(Trait<Arg, Assoc = Type>)]`. The binding is not an argument of
//! the trait in the impl header; it constrains the wrapped types.

use newer_type::{announce::announce, implement};

#[announce]
trait Source {
    type Out;
    fn out(&self) -> Self::Out;
}

impl Source for u8 {
    type Out = ::std::string::String;

    fn out(&self) -> ::std::string::String {
        format!("u8:{}", self)
    }
}

impl Source for u16 {
    type Out = ::std::string::String;

    fn out(&self) -> ::std::string::String {
        format!("u16:{}", self)
    }
}

impl Source for char {
    type Out = u32;

    fn out(&self) -> u32 {
        *self as u32
    }
}

#[implement(Source<Out = ::std::string::String>)]
struct Plain(u8);

#[implement(Source<Out = ::std::string::String>)]
struct Named {
    _unused: (),
    inner: u16,
}

// The binding is a constraint: `Wrapped<I>` has `Source` if `I` has it with
// that type.
#[implement(Source<Out = ::std::string::String>)]
struct Wrapped<I>(I);

#[test]
fn struct_with_binding() {
    assert_eq!(Plain(1).out(), "u8:1");
    assert_eq!(Wrapped(2u16).out(), "u16:2");
    assert_eq!(Wrapped(Plain(3)).out(), "u8:3");
    assert_eq!(Wrapped(Wrapped(4u8)).out(), "u8:4");
    assert_eq!(
        Named {
            _unused: (),
            inner: 5,
        }
        .out(),
        "u16:5"
    );
}

// The associated type of the impl is the bound one.
fn out_of<S: Source<Out = ::std::string::String>>(s: &S) -> ::std::string::String {
    s.out()
}

#[test]
fn bound_type_is_the_associated_type() {
    assert_eq!(out_of(&Plain(6)), "u8:6");
    assert_eq!(out_of(&Wrapped(7u16)), "u16:7");
}

// ---- an enum: the bound type is used instead of a synthesized parameter ----

#[implement(Source<Out = ::std::string::String>)]
enum Either {
    Small(u8),
    Large(u16),
}

// The variants bind in the attribute of their fields, too.
#[implement]
enum FieldAttr {
    Small(#[implement(Source<Out = ::std::string::String>)] u8),
    Large(#[implement(Source<Out = ::std::string::String>)] u16),
}

// Without the binding, the type is a parameter that the variants share.
#[implement(Source)]
enum Free {
    A(u8),
    B(u16),
}

#[test]
fn enum_with_binding() {
    assert_eq!(Either::Small(1).out(), "u8:1");
    assert_eq!(Either::Large(2).out(), "u16:2");
    assert_eq!(out_of(&Either::Small(3)), "u8:3");
    assert_eq!(FieldAttr::Small(4).out(), "u8:4");
    assert_eq!(FieldAttr::Large(5).out(), "u16:5");
    assert_eq!(out_of(&FieldAttr::Large(6)), "u16:6");
    assert_eq!(Free::A(7).out(), "u8:7");
    assert_eq!(Free::B(8).out(), "u16:8");
}

// A parameter of the wrapper in the binding.
#[announce]
trait Get {
    type Item;
    fn get(&self) -> Self::Item;
}

#[derive(Clone)]
struct Cell<T>(T);

#[derive(Clone)]
struct Pair<T>(T, T);

impl<T: Clone> Get for Cell<T> {
    type Item = T;

    fn get(&self) -> T {
        self.0.clone()
    }
}

impl<T: Clone> Get for Pair<T> {
    type Item = T;

    fn get(&self) -> T {
        self.1.clone()
    }
}

#[implement(Get<Item = T>)]
enum Shape<T: Clone> {
    One(Cell<T>),
    Two(Pair<T>),
}

#[implement(Get<Item = T>)]
struct Boxed<T: Clone>(Cell<T>);

#[test]
fn binding_with_a_parameter() {
    assert_eq!(Shape::One(Cell(1)).get(), 1);
    assert_eq!(Shape::Two(Pair(1, 2)).get(), 2);
    assert_eq!(Boxed(Cell("b")).get(), "b");
}

// ---- positional arguments and bindings ----

#[announce]
trait Conv<T> {
    type Out;
    fn conv(&self, t: T) -> Self::Out;
}

struct Id;

impl<T> Conv<T> for Id {
    type Out = T;

    fn conv(&self, t: T) -> T {
        t
    }
}

struct Show;

impl Conv<u8> for Show {
    type Out = ::std::string::String;

    fn conv(&self, t: u8) -> ::std::string::String {
        format!("<{}>", t)
    }
}

impl Conv<u16> for Show {
    type Out = ::std::string::String;

    fn conv(&self, t: u16) -> ::std::string::String {
        format!("<{}>", t)
    }
}

#[implement(Conv<u8, Out = ::std::string::String>)]
struct ShowU8(Show);

// The positional arguments stay in the header; `for<T>` is bound in the
// binding as well.
#[implement(for<T> Conv<T, Out = T>)]
struct Identity(Id);

#[implement(for<T: ::core::fmt::Debug> Conv<T, Out = T>)]
struct DebugIdentity(Id);

#[implement(Conv<T, Out = T>)]
struct Own<T>(PhantomIdentity<T>);

struct PhantomIdentity<T>(::core::marker::PhantomData<T>);

impl<T> Conv<T> for PhantomIdentity<T> {
    type Out = T;

    fn conv(&self, t: T) -> T {
        t
    }
}

#[implement(Conv<u8, Out = ::std::string::String>)]
enum Shows {
    A(Show),
    B(Show),
}

#[implement(for<T> Conv<T, Out = T>)]
enum Ids {
    A(Id),
    B(Id),
}

#[test]
fn positional_arguments_and_bindings() {
    assert_eq!(ShowU8(Show).conv(3u8), "<3>");
    assert_eq!(Identity(Id).conv("s"), "s");
    assert_eq!(Identity(Id).conv(4u8), 4u8);
    assert_eq!(DebugIdentity(Id).conv(Some(1)), Some(1));
    assert_eq!(
        Own(PhantomIdentity(::core::marker::PhantomData)).conv(5i8),
        5
    );
    assert_eq!(Shows::A(Show).conv(5u8), "<5>");
    assert_eq!(Shows::B(Show).conv(6u8), "<6>");
    assert_eq!(Ids::A(Id).conv('c'), 'c');
    assert_eq!(Ids::B(Id).conv(1.5), 1.5);
}

// ---- several associated types: the binding of one, the other is synthesized
// ----

#[announce]
trait Two {
    type A;
    type B;
    fn a(&self) -> Self::A;
    fn b(&self) -> Self::B;
}

struct Pairing(u8, char);

impl Two for Pairing {
    type A = u8;
    type B = char;

    fn a(&self) -> u8 {
        self.0
    }

    fn b(&self) -> char {
        self.1
    }
}

#[implement(Two<B = char>)]
enum Pairings {
    X(Pairing),
    Y(Pairing),
}

#[implement(Two<B = char, A = u8>)]
enum PairingsAll {
    X(Pairing),
    Y(Pairing),
}

#[implement(Two<B = char>)]
struct PairingStruct(Pairing);

#[test]
fn partial_bindings() {
    assert_eq!(Pairings::Y(Pairing(1, 'y')).a(), 1);
    assert_eq!(Pairings::X(Pairing(1, 'x')).b(), 'x');
    assert_eq!(PairingsAll::X(Pairing(2, 'x')).b(), 'x');
    assert_eq!(PairingsAll::Y(Pairing(2, 'y')).a(), 2);
    assert_eq!(PairingStruct(Pairing(3, 'z')).a(), 3);
}

// ---- the traits of std ----

#[implement(::announce_std::iter::Iterator<Item = usize>)]
struct Counter(std::ops::Range<usize>);

#[implement(::announce_std::iter::Iterator<Item = T>)]
struct Items<T>(std::vec::IntoIter<T>);

#[implement(::announce_std::iter::Iterator<Item = u8>)]
struct Bytes<I>(I);

#[implement(::announce_std::iter::Iterator<Item = T>)]
enum Either2<T> {
    Vec(std::vec::IntoIter<T>),
    Deque(std::collections::vec_deque::IntoIter<T>),
}

#[implement(::announce_std::iter::IntoIterator<Item = u8, IntoIter = std::vec::IntoIter<u8>>)]
struct IntoBytes(Vec<u8>);

#[test]
fn std_iterator_with_binding() {
    assert_eq!(Counter(0..3).collect::<Vec<_>>(), vec![0, 1, 2]);
    assert_eq!(Items(vec!["a", "b"].into_iter()).count(), 2);
    assert_eq!(Bytes(vec![1u8, 2].into_iter()).sum::<u8>(), 3);
    assert_eq!(Bytes(1u8..4).max(), Some(3));
    assert_eq!(
        Either2::Vec(vec![1, 2].into_iter()).collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(
        Either2::Deque(std::collections::VecDeque::from(vec![3]).into_iter()).next(),
        Some(3)
    );
    assert_eq!(
        IntoBytes(vec![9, 8]).into_iter().collect::<Vec<_>>(),
        [9, 8]
    );
}

// ---- the associated type of a supertrait ----

#[announce]
trait Peek: ::core::iter::Iterator {
    fn peek_item(&mut self) -> ::core::option::Option<Self::Item>;
}

impl Peek for std::vec::IntoIter<u8> {
    fn peek_item(&mut self) -> ::core::option::Option<u8> {
        self.next()
    }
}

#[implement(::announce_std::iter::Iterator<Item = u8>, Peek<Item = u8>)]
struct Peeker(std::vec::IntoIter<u8>);

#[implement(::announce_std::iter::Iterator<Item = u8>, Peek<Item = u8>)]
enum Peekers {
    A(std::vec::IntoIter<u8>),
    B(std::vec::IntoIter<u8>),
}

#[test]
fn binding_of_a_supertrait_type() {
    assert_eq!(Peeker(vec![1].into_iter()).peek_item(), Some(1));
    assert_eq!(Peekers::A(vec![2].into_iter()).peek_item(), Some(2));
    assert_eq!(Peekers::B(vec![3].into_iter()).peek_item(), Some(3));
}

// ---- lifetimes ----

#[announce]
trait Lend<'a> {
    type Out;
    fn lend(&'a self) -> Self::Out;
}

impl<'a> Lend<'a> for u8 {
    type Out = &'a u8;

    fn lend(&'a self) -> &'a u8 {
        self
    }
}

#[implement(for<'a> Lend<'a, Out = &'a u8>)]
struct Lender(u8);

#[implement(Lend<'static, Out = &'static u8>)]
struct StaticLender(u8);

#[test]
fn lifetimes_in_bindings() {
    assert_eq!(Lender(1).lend(), &1);
    assert_eq!(StaticLender(2).lend(), &2);
}
