use crate::emit_traits;
use newer_type::target;

emit_traits! {
    #[implement_of(newer_type_std::cmp::PartialEq)]
    #[slot(u8)]
    #[target(alternative = ::core::cmp::PartialEq)]
    pub trait PartialEq[Rhs = Self]
    where [Rhs: ?::core::marker::Sized,]
    {
        fn eq(&self, other: &Rhs) -> ::core::primitive::bool;
        fn ne(&self, other: &Rhs) -> ::core::primitive::bool;
    }

    #[implement_of(newer_type_std::cmp::PartialEq)]
    #[implement_of(newer_type_std::cmp::Eq)]
    #[slot(u8)]
    #[target(alternative = ::core::cmp::Eq)]
    pub trait Eq: [::core::cmp::PartialEq] {}


    #[implement_of(newer_type_std::cmp::PartialEq)]
    #[implement_of(newer_type_std::cmp::PartialOrd)]
    #[slot(u8)]
    #[target(alternative = ::core::cmp::PartialOrd)]
    pub trait PartialOrd[Rhs = Self]: [::core::cmp::PartialEq<Rhs>]
    where [Rhs: ?::core::marker::Sized,]
    {
        fn partial_cmp(&self, other: &Rhs)
            -> ::core::option::Option<::core::cmp::Ordering>;

        fn lt(&self, other: &Rhs) -> ::core::primitive::bool;
        fn le(&self, other: &Rhs) -> ::core::primitive::bool;
        fn gt(&self, other: &Rhs) -> ::core::primitive::bool;
        fn ge(&self, other: &Rhs) -> ::core::primitive::bool;
    }

    #[implement_of(newer_type_std::cmp::PartialEq)]
    #[implement_of(newer_type_std::cmp::Eq)]
    #[implement_of(newer_type_std::cmp::PartialOrd)]
    #[implement_of(newer_type_std::cmp::Ord)]
    #[slot(u8)]
    #[target(alternative = ::core::cmp::Ord)]
    pub trait Ord: [::core::cmp::Eq + ::core::cmp::PartialOrd] {
        fn cmp(&self, other: &Self) -> ::core::cmp::Ordering;
        // fn max(self, other: Self) -> Self
        // where
        //     Self: ::core::marker::Sized;
        // fn min(self, other: Self) -> Self
        // where
        //     Self: ::core::marker::Sized;
        // fn clamp(self, min: Self, max: Self) -> Self
        // where
        //     Self: ::core::marker::Sized;
    }
}
