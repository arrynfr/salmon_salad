#[repr(C)]
struct EfiHeader {
    signature:      u64,
    revision:       u32,
    header_size:    u32,
    crc32:          u32,
    reserved:       u32,
}

#[repr(C)]
struct EfiSimpleTextOutputMode {
    max_mode:       i32,
    mode:           i32,
    attribute:      i32,
    cursor_column:  i32,
    cursor_row:     i32,
    cursor_visible: bool,
}

#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    pub reset:                  extern fn(*const EfiSimpleTextOutputProtocol,
                                         ExtendedVerification: bool),
    pub output_string:          extern fn(*const EfiSimpleTextOutputProtocol,
                                         string: *const u16),
    pub test_string:            extern fn(*const EfiSimpleTextOutputProtocol,
                                         string: *const u16),
    pub query_mode:             extern fn(*const EfiSimpleTextOutputProtocol,
                                         mode_number: usize,
                                         columns: *const usize,
                                         rows: *const usize),
    pub set_mode:               extern fn(*const EfiSimpleTextOutputProtocol,
                                         mode_number: usize),
    pub set_attribute:          extern fn(*const EfiSimpleTextOutputProtocol,
                                         attribute: usize),
    pub clear_screen:           extern fn(*const EfiSimpleTextOutputProtocol),
    pub set_cursor_position:    extern fn(*const EfiSimpleTextOutputProtocol,
                                         column: usize,
                                         row: usize),
    pub enable_cursor:          extern fn(*const EfiSimpleTextOutputProtocol,
                                         visible: bool),
    mode:                       *const EfiSimpleTextOutputMode,
}

#[repr(C)]
pub struct EfiSystemTable {
    header:             EfiHeader,
    firmware_vendor:    *const u16,
    firmware_revision:  u32,
    con_in_handle:      *const usize,
    con_in:             *const usize,
    con_out_handle:     *const usize,
    pub con_out:        *const  EfiSimpleTextOutputProtocol,
}
