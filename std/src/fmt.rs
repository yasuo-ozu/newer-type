use crate::emit_traits;
use newer_type::target;

emit_traits! {
    #[implement_of(newer_type_std::fmt::Display)]
    #[slot(u8)]
    #[target(alternative = ::core::fmt::Display)]
    pub trait Display {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result;
    }

    #[implement_of(newer_type_std::fmt::Debug)]
    #[slot(u8)]
    #[target(alternative = ::core::fmt::Debug)]
    pub trait Debug {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result;
    }


    #[implement_of(newer_type_std::fmt::Binary)]
    #[slot(u8)]
    #[target(alternative = ::core::fmt::Binary)]
    pub trait Binary {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result;
    }


    #[implement_of(newer_type_std::fmt::Octal)]
    #[slot(u8)]
    #[target(alternative = ::core::fmt::Octal)]
    pub trait Octal {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result;
    }


    #[implement_of(newer_type_std::fmt::LowerHex)]
    #[slot(u8)]
    #[target(alternative = ::core::fmt::LowerHex)]
    pub trait LowerHex {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result;
    }


    #[implement_of(newer_type_std::fmt::UpperHex)]
    #[slot(u8)]
    #[target(alternative = ::core::fmt::UpperHex)]
    pub trait UpperHex {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result;
    }


    #[implement_of(newer_type_std::fmt::LowerExp)]
    #[slot(u8)]
    #[target(alternative = ::core::fmt::LowerExp)]
    pub trait LowerExp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result;
    }


    #[implement_of(newer_type_std::fmt::UpperExp)]
    #[slot(u8)]
    #[target(alternative = ::core::fmt::UpperExp)]
    pub trait UpperExp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result;
    }


    #[implement_of(newer_type_std::fmt::Pointer)]
    #[slot(Box<u8>)]
    #[target(alternative = ::core::fmt::Pointer)]
    pub trait Pointer {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result;
    }


    #[implement_of(newer_type_std::fmt::Write)]
    #[slot(String)]
    #[target(alternative = ::core::fmt::Write)]
    pub trait Write {
        fn write_str(&mut self, s: &::core::primitive::str) -> ::core::fmt::Result;
        fn write_char(&mut self, c: ::core::primitive::char) -> ::core::fmt::Result;
        fn write_fmt(&mut self, args: ::core::fmt::Arguments<'_>) -> ::core::fmt::Result;
    }
}
