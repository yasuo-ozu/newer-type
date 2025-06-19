use crate::emit_traits;
use newer_type::target;

emit_traits! {
    #[implement_of(newer_type_std::string::ToString)]
    #[slot(u8)]
    #[target(alternative = ::std::string::ToString)]
    pub trait ToString {
        fn to_string(&self) -> ::std::string::String;
    }
}
