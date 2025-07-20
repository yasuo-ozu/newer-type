use crate::emit_traits;
use newer_type::target;

#[rustversion::since(1.81)]
emit_traits! {
    #[implement_of(
        newer_type_std::fmt::Display,
        newer_type_std::fmt::Debug,
        newer_type_std::error::Error
    )]
    #[slot(std::io::Error)]
    #[target(alternative = ::core::error::Error)]
    pub trait Error: [::core::fmt::Debug + ::core::fmt::Display] {
        fn source(&self) -> ::core::option::Option<&(dyn ::core::error::Error + 'static)>;
    }
}

#[rustversion::before(1.81)]
emit_traits! {
    #[implement_of(
        newer_type_std::fmt::Display,
        newer_type_std::fmt::Debug,
        newer_type_std::error::Error
    )]
    #[slot(std::io::Error)]
    #[target(alternative = ::std::error::Error)]
    pub trait Error: [::core::fmt::Debug + ::core::fmt::Display] {
        fn source(&self) -> ::core::option::Option<&(dyn ::std::error::Error + 'static)>;
    }
}
