use crate::emit_traits;
use newer_type::target;

emit_traits! {
    #[implement_of(newer_type_std::io::Read)]
    #[slot(std::io::Cursor<Vec<u8>>)]
    #[target(alternative = ::std::io::Read)]
    pub trait Read {
        fn read(&mut self, buf: &mut [::core::primitive::u8]) -> ::std::io::Result<::core::primitive::usize>;
    }

    #[implement_of(newer_type_std::io::Read)]
    #[slot(std::io::Cursor<Vec<u8>>)]
    #[target(alternative = ::std::io::Write)]
    pub trait Write {
        fn write(&mut self, buf: &[::core::primitive::u8]) -> ::std::io::Result<::core::primitive::usize>;
        fn flush(&mut self) -> ::std::io::Result<()>;
        fn write_all(&mut self, mut buf: &[::core::primitive::u8]) -> ::std::io::Result<()>;
    }

    #[implement_of(
        newer_type_std::io::Read,
        newer_type_std::io::BufRead,
    )]
    #[slot(std::io::Cursor<Vec<u8>>)]
    #[target(alternative = ::std::io::BufRead)]
    pub trait BufRead: [::std::io::Read] {
        fn fill_buf(&mut self) -> ::std::io::Result<&[::core::primitive::u8]>;
        fn consume(&mut self, amt: ::core::primitive::usize);
    }

    #[implement_of(newer_type_std::io::Seek)]
    #[slot(std::fs::File)]
    #[target(alternative = ::std::io::Seek)]
    pub trait Seek {
        fn seek(&mut self, pos: ::std::io::SeekFrom) -> ::std::io::Result<::core::primitive::u64>;
        fn rewind(&mut self) -> ::std::io::Result<()>;
        fn stream_position(&mut self) -> ::std::io::Result<::core::primitive::u64>;
        fn seek_relative(&mut self, offset: ::core::primitive::i64) -> ::std::io::Result<()>;
    }
}
