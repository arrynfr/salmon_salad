use core::sync::atomic::{AtomicPtr, Ordering};
use core::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct Guid(pub u32, pub u16, pub u16, pub [u8; 8]);

/*#[repr(C)]
#[derive(Debug)]
pub struct Guid(pub u128);*/

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
pub struct EfiConfigurationTable {
	vendor_guid: Guid,
	vendor_table: *const usize
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
    con_out:            *const  EfiSimpleTextOutputProtocol,
    std_err_handle:	*const usize,
    std_err:		*const usize,
    runtime_services:	*const usize,
    loot_services:	*const usize,
    num_table_entries:	usize,
    efi_config_table:	*const usize,
}

static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = AtomicPtr::new(ptr::null_mut());

pub fn register_efi_system_table(table: *mut EfiSystemTable) {
    let _ = EFI_SYSTEM_TABLE.compare_exchange(core::ptr::null_mut(),
                                              table, Ordering::SeqCst,
                                              Ordering::SeqCst);
}

pub fn clear_screen() {
    let table = EFI_SYSTEM_TABLE.load(Ordering::Relaxed);
    unsafe {
         let console_out = (*table).con_out;
        ((*console_out).clear_screen)(console_out);
    }
}

pub fn output_string(string: &str) {
    let table = EFI_SYSTEM_TABLE.load(Ordering::Relaxed);
    //TODO: Convert string to UTF16 instead of writing char by char
    for c in string.chars() {
        let letter = [c as u16, 0 as u16];
        unsafe {
            let console_out = (*table).con_out;
            ((*console_out).output_string)(console_out, letter.as_ptr());
        }
    }
}

pub fn walk_config_table(guid: Guid) -> EfiConfigurationTable {
	EfiConfigurationTable {vendor_guid: guid, vendor_table: core::ptr::null()}
}
