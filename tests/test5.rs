use newer_type::{implement, target};
use std::fmt::Debug;

mod m {
    type T = usize;
    #[super::target]
    pub trait MyNewTrait {
        type MyType<'a>
        where
            Self: 'a;
        fn get<'a>(&'a self, a: T) -> Self::MyType<'a>;
    }
}

#[implement(m::MyNewTrait)]
struct MyWrapper(String);

impl m::MyNewTrait for String {
    type MyType<'a>
        = &'a str
    where
        Self: 'a;

    fn get<'a>(&'a self, a: usize) -> Self::MyType<'a> {
        todo!()
    }
}
