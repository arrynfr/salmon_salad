pub extern crate alloc;
use alloc::alloc::{GlobalAlloc, Layout};

extern "C" {
    static _heap_size: u8;
    static _heap_start: u8;
    static _heap_end: u8;
}

pub fn get_heap_size() -> u64 {
    unsafe {
        &_heap_size as *const u8 as u64
    }
}
pub fn get_heap_start() -> *mut u8 {
    unsafe {
        &_heap_start as *const u8 as *mut u8
    }
}
pub fn get_heap_end() -> *mut u8 {
    unsafe {
        &_heap_end as *const u8 as *mut u8
    }
}

#[global_allocator]
static ALLOCATOR: KernelAllocator = KernelAllocator::new();

pub struct KernelAllocator {}

impl KernelAllocator {
    pub const fn new() -> Self {KernelAllocator{}}
}

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        println!("{:?}", layout);
        get_heap_start()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        panic!("Cannot deallocate memory yet!");
    }
}