use crate::emit_traits;
use newer_type::target;

emit_traits! {
    #[implement_of(for<T> newer_type_std::convert::AsRef<T>)]
    #[slot(u8)]
    #[target(alternative = ::core::convert::AsRef)]
    pub trait AsRef[T: ?::core::marker::Sized] {
        fn as_ref(&self) -> &T;
    }

    #[implement_of(for<T> newer_type_std::convert::AsMut<T>)]
    #[slot(u8)]
    #[target(alternative = ::core::convert::AsMut)]
    pub trait AsMut[T: ?::core::marker::Sized] {
        fn as_mut(&mut self) -> &mut T;
    }

    #[implement_of(newer_type_std::convert::Into<u32>)]
    #[slot(char)]
    #[target(alternative = ::core::convert::Into)]
    pub trait Into[T]: [::core::marker::Sized] {
        fn into(self) -> T;
    }

    #[implement_of(newer_type_std::convert::TryInto<u8>)]
    #[slot(i32)]
    #[target(alternative = ::core::convert::TryInto)]
    pub trait TryInto[T]: [::core::marker::Sized] {
        type Error;
        fn try_into(self) -> ::core::result::Result<T, Self::Error>;
    }
}
