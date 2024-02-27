// TODO: Error handling with EFI_STATUS
use core::ptr;
use core::mem;
use core::sync::atomic::{AtomicPtr, Ordering};
use super::constants::*;

extern crate alloc;
use alloc::alloc::{GlobalAlloc, Layout};
use crate::efi::efi::alloc::string::*;
use core::ptr::{addr_of_mut, addr_of};

static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = AtomicPtr::new(ptr::null_mut());

const EFI_TABLE_SIGNATURE: u64 = 0x5453595320494249;

#[global_allocator]
static ALLOCATOR: EfiAllocator = EfiAllocator::new();

#[no_mangle]
extern "efiapi" fn efi_main(_handle: u64, table: *mut EfiSystemTable) {
    register_efi_system_table(table);
    clear_screen();
    output_string("We're booting in UEFI mode\r\n");
    unsafe {
        println!("{:#?}", *table);
    }
    _get_memory_map();
    output_string("looping...");
    {
        let mut x = String::from("Hello world from allocator!");
        println!("{x}");
        x.push_str("\r\nIt's growable too");
        println!("{x}");             
    }
    loop{}

    // From this point UEFI should not be used anywhere
    crate::kmain();
}

#[repr(C)]
#[derive(Debug)]
pub struct Guid(pub u32, pub u16, pub u16, pub [u8; 8]);

#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
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
    header: EfiHeader,

    // Task Priority Services
    pub raise_tpl: extern "C" fn(tpl: usize),
    pub restore_tpl: extern "C" fn(old_tpl: usize),

    // Memory Services
    pub allocate_pages: extern "C" fn(allocate_type: EfiAllocateType, memory_type: EfiMemoryType, num_pages: usize, buffer_ptr: *const u64) -> usize,
    pub free_pages: extern "C" fn(memory: u64, pages: usize),
    pub get_memory_map: extern "C" fn(
        MemoryMapSize: &usize,
        MemoryMap: *const EfiMemoryDescriptor,
        MapKey: &usize,
        DescriptorSize: &usize,
        DescriptorVersion: &u32,
    ) -> usize,
    pub allocate_pool: extern "C" fn(pool_type: EfiMemoryType, size:usize, buffer: *const *const u8) -> usize,
    pub free_pool: extern "C" fn(buffer: *const u8) -> usize,

    // Event & Timer Services
}

#[repr(C)]
#[derive(Debug,Copy,Clone)]
pub struct EfiMemoryDescriptor {
    pub memory_type: EfiMemoryType,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

// TODO: Replace EfiMemoryDescriptor with flat u8 buffer and parse into structs
pub fn _get_memory_map() {
    const BUFFER_SIZE: usize = 256;
    println!("Getting memory map...");
    let table = EFI_SYSTEM_TABLE.load(Ordering::Relaxed);
    let size = mem::size_of::<[EfiMemoryDescriptor; BUFFER_SIZE]>();
    let memory_map_descriptor_buf: [EfiMemoryDescriptor; BUFFER_SIZE] = [
        EfiMemoryDescriptor {
            memory_type: EfiMemoryType::EfiReservedMemoryType,
            physical_start: 0,
            virtual_start: 0,
            number_of_pages: 0,
            attribute: 0,
        }; BUFFER_SIZE
    ];
    let map_key = 0 as usize;
    let descriptor_size = 0 as usize;
    let descriptor_version = 0 as u32;
    unsafe {
        let boot_services = (*table).boot_services;
        let ret = ((*boot_services).get_memory_map)(
            &size,
            core::ptr::null(),
            &map_key,
            &descriptor_size,
            &descriptor_version,
        );
        println!("Return code: {:x?}\r\nNum of elements: {}", ret & (!(1 << 63)), size/mem::size_of::<EfiMemoryDescriptor>());
        println!("Descriptor size: {}\r\nDescriptor version: {}", descriptor_size, descriptor_version);
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

pub struct EfiAllocator {}

impl EfiAllocator {
    pub const fn new() -> Self {EfiAllocator{}}
}

unsafe impl GlobalAlloc for EfiAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut ptr = core::ptr::null();
        let table = EFI_SYSTEM_TABLE.load(Ordering::Relaxed);
        let boot_services = (*table).boot_services;
        let retval = ((*boot_services).allocate_pool)(EfiMemoryType::EfiLoaderData, layout.size(), addr_of!(ptr));
        match retval {
            0 => {/*println!("Memory allocated at: {:?}", ptr);*/},
            _ => {panic!("Memory allocation failed: {:x?}", retval);}
        }
        ptr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let table = EFI_SYSTEM_TABLE.load(Ordering::Relaxed);
        let boot_services = (*table).boot_services;
        let retval = ((*boot_services).free_pool)(ptr);
        match retval {
            0 => {/*println!("Memory deallocated at: {:?}",  ptr);*/},
            _ => {panic!("Memory deallocation failed: {:x?}", retval);}
        }
    }
}