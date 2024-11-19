use core::{ffi::CStr, fmt::{self, Display}};

pub struct RingBuffer {
    storage: [u8; 65565],
    write_ptr: usize,
    read_ptr: usize
}

impl RingBuffer {
    pub const fn new() -> Self {
        RingBuffer { storage: [0; 65565], write_ptr: 0, read_ptr: 0}
    }
}

