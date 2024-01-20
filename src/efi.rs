// TODO: Error handling with EFI_STATUS
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = AtomicPtr::new(ptr::null_mut());

#[repr(C)]
#[derive(Debug)]
pub struct Guid(pub u32, pub u16, pub u16, pub [u8; 8]);

#[repr(C)]
struct EfiHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

#[repr(C)]
struct EfiSimpleTextOutputMode {
    max_mode: i32,
    mode: i32,
    attribute: i32,
    cursor_column: i32,
    cursor_row: i32,
    cursor_visible: bool,
}

#[repr(C)]
pub struct _EfiConfigurationTable {
    vendor_guid: Guid,
    vendor_table: *const usize,
}

#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    pub reset: extern "C" fn(*const EfiSimpleTextOutputProtocol, ExtendedVerification: bool),
    pub output_string: extern "C" fn(*const EfiSimpleTextOutputProtocol, string: *const u16),
    pub test_string: extern "C" fn(*const EfiSimpleTextOutputProtocol, string: *const u16),
    pub query_mode: extern "C" fn(
        *const EfiSimpleTextOutputProtocol,
        mode_number: usize,
        columns: *const usize,
        rows: *const usize,
    ),
    pub set_mode: extern "C" fn(*const EfiSimpleTextOutputProtocol, mode_number: usize),
    pub set_attribute: extern "C" fn(*const EfiSimpleTextOutputProtocol, attribute: usize),
    pub clear_screen: extern "C" fn(*const EfiSimpleTextOutputProtocol),
    pub set_cursor_position:
        extern "C" fn(*const EfiSimpleTextOutputProtocol, column: usize, row: usize),
    pub enable_cursor: extern "C" fn(*const EfiSimpleTextOutputProtocol, visible: bool),
    mode: *const EfiSimpleTextOutputMode,
}

#[repr(C)]
pub struct EfiSystemTable {
    header: EfiHeader,
    firmware_vendor: *const u16,
    firmware_revision: u32,
    con_in_handle: *const usize,
    con_in: *const usize,
    con_out_handle: *const usize,
    con_out: *const EfiSimpleTextOutputProtocol,
    std_err_handle: *const usize,
    std_err: *const usize,
    runtime_services: *const usize,
    boot_services: *const EfiBootServices,
    num_table_entries: usize,
    efi_config_table: *const usize,
}

#[repr(C)]
pub struct EfiBootServices {
    pub raise_tpl: extern "C" fn(tpl: usize),
    pub restore_tpl: extern "C" fn(old_tpl: usize),
    pub allocate_pages: extern "C" fn(*const EfiBootServices, string: *const u16),
    pub free_pages: extern "C" fn(*const EfiSimpleTextOutputProtocol),
    pub get_memory_map: extern "C" fn(
        MemoryMapSize: &usize,
        MemoryMap: *const EfiMemoryDescriptor,
        MapKey: &usize,
        DescriptorSize: &usize,
        DescriptorVersion: &u32,
    ) -> usize
}

#[repr(C)]
#[derive(Debug)]
pub struct EfiMemoryDescriptor {
    pub memory_type: u32,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

impl Default for EfiMemoryDescriptor {
    fn default() -> Self {
        EfiMemoryDescriptor {
            memory_type: 0,
            physical_start: 0,
            virtual_start: 0,
            number_of_pages: 0,
            attribute: 0,
        }
    }
}

pub fn _get_memory_map() {
    let table = EFI_SYSTEM_TABLE.load(Ordering::Relaxed);
    let size = 100 as usize;
    let memory_map_descriptor_buf: [EfiMemoryDescriptor; 32] = Default::default();
    let map_key = 0 as usize;
    let descriptor_size = 0 as usize;
    let descriptor_version = 0 as u32;
    unsafe {
        let boot_services = (*table).boot_services;
        let ret = ((*boot_services).get_memory_map)(
            &size,
            memory_map_descriptor_buf.as_ptr(),
            &map_key,
            &descriptor_size,
            &descriptor_version,
        );
        println!("Return code: {}", ret);
    }
    if size > 0 {
        println!("{:?}\r\n", memory_map_descriptor_buf);
    } else {
        println!("Size was {}", size);
    }
}

pub fn register_efi_system_table(table: *mut EfiSystemTable) {
    let _ = EFI_SYSTEM_TABLE.compare_exchange(
        core::ptr::null_mut(),
        table,
        Ordering::SeqCst,
        Ordering::SeqCst,
    );
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
    //TODO: Convert string to UTF16 instead of writing char by char?
    for c in string.encode_utf16() {
        unsafe {
            let console_out = (*table).con_out;
            ((*console_out).output_string)(console_out, [c, 0 as u16].as_ptr());
        }
    }
}

pub fn _walk_config_table(guid: Guid) -> _EfiConfigurationTable {
    _EfiConfigurationTable {
        vendor_guid: guid,
        vendor_table: core::ptr::null(),
    }
}
