use newer_type::{implement, target};

pub trait Repeater<const TRAIT_ID: u64, const NTH: usize, T: ?Sized> {
    type Type;
}

mod m {
    type T = usize;
    #[super::target(repeater = crate::Repeater)]
    pub trait MyNewTrait {
        type MyType<'a>
        where
            Self: 'a;
        fn get<'a>(&'a self, a: T) -> Self::MyType<'a>;
    }
}

#[allow(unused)]
#[implement(m::MyNewTrait)]
struct MyWrapper(String);

impl m::MyNewTrait for String {
    type MyType<'a>
        = &'a str
    where
        Self: 'a;

    #[allow(unused)]
    fn get<'a>(&'a self, _a: usize) -> Self::MyType<'a> {
        todo!()
    }
}
