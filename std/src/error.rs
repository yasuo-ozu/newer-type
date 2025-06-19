use crate::emit_traits;
use newer_type::target;

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
