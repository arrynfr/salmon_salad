#[repr(C)]
struct EfiHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    reset: *const u64,
    pub output_string: extern fn(*const EfiSimpleTextOutputProtocol, *const u16),
}

#[repr(C)]
pub struct EfiSystemTable {
    header: EfiHeader,
    firmware_vendor: *const u64,
    pub firmware_revision: u32,
    con_in_handle:  *const u64,
    con_in:         *const u64,
    con_out_handle: *const u64,
    pub con_out:    *const  EfiSimpleTextOutputProtocol,
}
