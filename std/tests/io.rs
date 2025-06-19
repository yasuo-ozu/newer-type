use newer_type::implement;
use std::io::{BufRead, Cursor, Read, Seek, SeekFrom, Write};

#[implement(
    newer_type_std::io::Read,
    newer_type_std::io::Write,
    newer_type_std::io::Seek,
    newer_type_std::io::BufRead
)]
pub struct MyIoStruct {
    inner: Cursor<Vec<u8>>,
}

#[test]
fn test_io_read() {
    let data = b"abcde12345".to_vec();
    let mut wrapper = MyIoStruct {
        inner: Cursor::new(data),
    };
    let mut buf = [0u8; 5];
    wrapper.read_exact(&mut buf).unwrap();
    assert_eq!(&buf, b"abcde");
}

#[test]
fn test_io_seek_and_bufread() {
    let data = b"line1\nline2\n".to_vec();
    let mut wrapper = MyIoStruct {
        inner: Cursor::new(data),
    };
    wrapper.seek(SeekFrom::Start(6)).unwrap();
    let mut line = String::new();
    wrapper.read_line(&mut line).unwrap();
    assert_eq!(line.trim_end(), "line2");
}

#[test]
fn test_io_write() {
    let mut writer = MyIoStruct {
        inner: Cursor::new(Vec::new()),
    };
    writer.write_all(b"hello world").unwrap();
    assert_eq!(writer.inner.get_ref(), b"hello world");
}

#[implement(
    newer_type_std::io::Read,
    newer_type_std::io::Write,
    newer_type_std::io::BufRead,
    newer_type_std::io::Seek
)]
pub enum MyIoEnum {
    A(Cursor<Vec<u8>>),
    B(Cursor<Vec<u8>>),
}

#[test]
fn test_io_enum_read_write_seek() {
    let mut wrapper = MyIoEnum::A(Cursor::new(b"abcdef".to_vec()));
    let mut buf = [0u8; 3];
    wrapper.read_exact(&mut buf).unwrap();
    assert_eq!(&buf, b"abc");

    let s = vec![0x78_u8, 0x79, 0x7a];
    let mut wrapper = MyIoEnum::B(Cursor::new(s));
    let mut buf2 = Vec::new();
    wrapper.read_to_end(&mut buf2).unwrap();
    assert_eq!(&buf2, b"xyz");
}
