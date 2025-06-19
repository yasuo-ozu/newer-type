use newer_type::implement;
use std::alloc::{GlobalAlloc, Layout, System};
use std::ptr;

#[implement(newer_type_std::alloc::GlobalAlloc)]
pub struct MyAllocStruct {
    inner: System,
}

unsafe impl Send for MyAllocStruct {}
unsafe impl Sync for MyAllocStruct {}

#[test]
fn test_global_alloc_alloc_dealloc() {
    let alloc = MyAllocStruct { inner: System };
    unsafe {
        let layout = Layout::from_size_align(16, 8).unwrap();
        let ptr = alloc.alloc(layout);
        assert!(!ptr.is_null());
        ptr::write_bytes(ptr, 0xAB, 16);
        alloc.dealloc(ptr, layout);
    }
}

#[test]
fn test_global_alloc_zeroed() {
    let alloc = MyAllocStruct { inner: System };
    unsafe {
        let layout = Layout::from_size_align(8, 8).unwrap();
        let ptr = alloc.alloc_zeroed(layout);
        assert!(!ptr.is_null());
        for i in 0..8 {
            assert_eq!(*ptr.add(i), 0);
        }
        alloc.dealloc(ptr, layout);
    }
}
