use crate::emit_traits;
use newer_type::target;

emit_traits! {
    #[implement_of(newer_type_std::hash::Hash)]
    #[slot(u8)]
    #[target(alternative = ::core::hash::Hash)]
    pub trait Hash {
        fn hash<H>(&self, state: &mut H)
            where H: ::core::hash::Hasher;
    }

    #[implement_of(newer_type_std::hash::Hasher)]
    #[slot(std::hash::DefaultHasher)]
    #[target(alternative = ::core::hash::Hasher)]
    pub trait Hasher {
        fn finish(&self) -> ::core::primitive::u64;
        fn write(&mut self, bytes: &[::core::primitive::u8]);
        fn write_u8(&mut self, i: ::core::primitive::u8);
        fn write_u16(&mut self, i: ::core::primitive::u16);
        fn write_u32(&mut self, i: ::core::primitive::u32);
        fn write_u64(&mut self, i: ::core::primitive::u64);
        fn write_u128(&mut self, i: ::core::primitive::u128);
        fn write_usize(&mut self, i: ::core::primitive::usize);
        fn write_i8(&mut self, i: ::core::primitive::i8);
        fn write_i16(&mut self, i: ::core::primitive::i16);
        fn write_i32(&mut self, i: ::core::primitive::i32);
        fn write_i64(&mut self, i: ::core::primitive::i64);
        fn write_i128(&mut self, i: ::core::primitive::i128);
        fn write_isize(&mut self, i: ::core::primitive::isize);
    }

    #[implement_of(newer_type_std::hash::BuildHasher)]
    #[slot(std::hash::RandomState)]
    #[target(alternative = ::core::hash::BuildHasher)]
    pub trait BuildHasher {
        type Hasher: ::core::hash::Hasher;
        fn build_hasher(&self) -> Self::Hasher;

        fn hash_one<T: ::core::hash::Hash>(&self, x: T) -> ::core::primitive::u64
        where
            Self: ::core::marker::Sized,
            Self::Hasher: ::core::hash::Hasher;
    }
}
