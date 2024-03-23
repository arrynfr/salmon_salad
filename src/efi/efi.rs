// TODO: Error handling with EFI_STATUS
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};
use super::constants::*;
use core::mem::size_of;
use core::ptr::addr_of;

extern crate alloc;
use alloc::alloc::{GlobalAlloc, Layout};
use self::alloc::string::*;
use self::alloc::vec::*;
use crate::user::graphics::gfx::*;

static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = AtomicPtr::new(ptr::null_mut());

#[global_allocator]
static ALLOCATOR: EfiAllocator = EfiAllocator::new();

#[no_mangle]
extern "efiapi" fn efi_main(_handle: u64, table: *mut EfiSystemTable) {
    let gop_guid = Guid(0x9042a9de,0x23dc,0x4a38,[0x96,0xfb,0x7a,0xde,0xd0,0x80,0x51,0x6a]);
    register_efi_system_table(table);
    clear_screen();
    println!("We're booting in UEFI mode\r\n");
    _get_memory_map();
    {
        let mut x = String::from("Hello world from allocator!");
        println!("{x}");
        x.push_str("\r\nIt's growable too");
        println!("{x}");             
    }
    unsafe {
        let table = EFI_SYSTEM_TABLE.load(Ordering::Relaxed);
        let boot_services = (*table).boot_services;
        let ptr: *const EfiGraphicsOutputProtocol = core::ptr::null();
        ((*boot_services).locate_protocol)(&gop_guid, core::ptr::null(), addr_of!(ptr));
        ((*ptr).set_mode)(ptr, 10);
        
        let fb = (*(*ptr).mode).frame_buffer_base;
        let fb_size = (*(*ptr).mode).frame_buffer_size;
        let ppl = (*(*(*ptr).mode).info).pixels_per_scanline;
        let fb_x = (*(*(*ptr).mode).info).horizontal_resolution;
        let fb_y = (*(*(*ptr).mode).info).vertical_resolution;
        let graphicsBuffer = GraphicsBuffer::new(fb, fb_size, ppl, fb_x, fb_y, PixelFormat::BGRX8, 4);
        graphicsBuffer.draw_rectangle(0, 0, graphicsBuffer.horizontal_resolution as isize, graphicsBuffer.vertical_resolution as isize, Color{r: 128, g: 128, b: 128});

        graphicsBuffer.draw_line(0, 0, graphicsBuffer.horizontal_resolution as isize, graphicsBuffer.vertical_resolution as isize, Color{r: 255, g: 255, b: 255});
        graphicsBuffer.draw_circle((500,500), 100, Color{r: 255, g: 0, b: 0});
        graphicsBuffer.draw_rectangle(60000, 60000, 100, 100, Color{r: 0, g: 0, b: 255});
    }

    // From this point UEFI should not be used anywhere
    crate::kmain();
}

#[repr(C)]
#[derive(Debug, PartialEq)]
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
#[derive(Debug)]
struct EfiSimpleTextOutputMode {
    max_mode: i32,
    mode: i32,
    attribute: i32,
    cursor_column: i32,
    cursor_row: i32,
    cursor_visible: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct EfiConfigurationTable {
    vendor_guid: Guid,
    vendor_table: *const usize,
}

#[repr(C)]
#[derive(Debug)]
enum EfiGraphicsPixelFormat {
    PixelRedGreenBlueReserved8BitPerColor,
    PixelBlueGreenRedReserved8BitPerColor,
    PixelBitMask,
    PixelBltOnly,
    PixelFormatMax
}

#[repr(C)]
#[derive(Debug)]
struct EfiGraphicsOutputBltPixel {
    blue: u8,
    green: u8,
    red: u8,
    reserved: u8
}

#[repr(C)]
#[derive(Debug)]
struct EfiPixelBitmask {
    red_mask: u32,
    green_mask: u32,
    blue_mask: u32,
    reserved_mask: u32
}

#[repr(C)]
enum EfiGraphicsOutputBltOperation {
    EfiBltVideoFill,
    EfiBltVideoToBltBuffer,
    EfiBltBufferToVideo,
    EfiBltVideoToVideo,
    EfiGraphicsOutputBltOperationMax
}

#[repr(C)]
#[derive(Debug)]
struct EfiGraphicsOutputModeInformation {
    version: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: EfiGraphicsPixelFormat,
    pixel_information: EfiPixelBitmask,
    pixels_per_scanline: u32    
}

#[repr(C)]
struct EfiGraphicsOutputProtocol {
    query_mode: extern "C" fn(   *const EfiGraphicsOutputProtocol, mode_number: u32, size_of_info: &usize,
                                info: *const *const EfiGraphicsOutputModeInformation),
    set_mode: extern "C" fn( *const EfiGraphicsOutputProtocol, mode_number: u32) -> usize,
    blt: extern "C" fn( *const EfiGraphicsOutputProtocol,
                        blt_buffer: *const EfiGraphicsOutputBltPixel,
                        EfiGraphicsOutputBltOperation,
                        source_x: usize,
                        source_y: usize,
                        dest_x: usize,
                        dest_y: usize,
                        width: usize,
                        heigth: usize,
                        delta: usize
                    ),
    mode: *const EfiGraphicsOutputProtocolMode
}

#[repr(C)]
#[derive(Debug)]
struct EfiGraphicsOutputProtocolMode {
    max_mode: u32,
    mode: u32,
    info: *const EfiGraphicsOutputModeInformation,
    size_of_info: usize,
    frame_buffer_base: *mut u8,
    frame_buffer_size: usize
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
    pub set_mode: extern "C" fn(*const EfiSimpleTextOutputProtocol, mode_number: usize) -> usize,
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
    efi_config_table: *const EfiConfigurationTable,
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
        MemoryMap: *const u8,
        MapKey: &usize,
        DescriptorSize: &usize,
        DescriptorVersion: &u32,
    ) -> usize,
    pub allocate_pool: extern "C" fn(pool_type: EfiMemoryType, size:usize, buffer: *const *const u8) -> usize,
    pub free_pool: extern "C" fn(buffer: *const u8) -> usize,

    // Event & Timer Services
    pub create_event: extern "C" fn(),
    pub set_timer: extern "C" fn(),
    pub wait_for_event: extern "C" fn(),
    pub signal_event: extern "C" fn(),
    pub close_event: extern "C" fn(),
    pub check_event: extern "C" fn(),

    // Protocol Handler Services
    pub install_protocol_interface: extern "C" fn(),
    pub reinstall_protocol_interface: extern "C" fn(),
    pub uninstall_protocol_interface: extern "C" fn(),
    pub handle_protocol: extern "C" fn(),
    Reserved: usize,
    pub register_protocol_notify: extern "C" fn(),
    pub locate_handle: extern "C" fn(),
    pub locate_device_path: extern "C" fn(),
    pub install_configuration_table: extern "C" fn(),

    // Image Services
    pub load_image: extern "C" fn(),
    pub start_image: extern "C" fn(),
    pub exit: extern "C" fn(),
    pub unload_image: extern "C" fn(),
    pub exit_boot_services: extern "C" fn(),

    // Misc Services
    pub get_next_monotonic_count: extern "C" fn(),
    pub stall: extern "C" fn(),
    pub set_watchdog_timer: extern "C" fn(),

    // DriverSupport Services
    pub connect_controller: extern "C" fn(),
    pub disconnect_controller: extern "C" fn(),

    // Open and Close Protocol Services
    pub open_protocol: extern "C" fn(),
    pub close_protocol: extern "C" fn(),
    pub open_protocol_information: extern "C" fn(),

    // Library Services
    pub protocols_per_handle: extern "C" fn(),
    pub locate_handle_buffer: extern "C" fn(),
    locate_protocol: extern "C" fn(protocol: *const Guid, registration: *const u8, interface: *const *const EfiGraphicsOutputProtocol) -> usize,
    pub install_multiple_protocol_interfaces: extern "C" fn(),
    pub uninstall_multiple_protocol_interfaces: extern "C" fn(),

    //32-bit CRC Services
    pub calculate_crc32: extern "C" fn(),

    // Misc Services Cont.
    pub copy_mem: extern "C" fn(),
    pub set_mem: extern "C" fn(),
    pub create_event_ex: extern "C" fn(),
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
    println!("Getting memory map...");
    let table = EFI_SYSTEM_TABLE.load(Ordering::Relaxed);
    let map_key = 0 as usize;
    let descriptor_size = 0 as usize;
    let descriptor_version = 0 as u32;
    let size = 0;
    unsafe {
        let boot_services = (*table).boot_services;
        let ret = ((*boot_services).get_memory_map)(
            &size,
            core::ptr::null(),
            &map_key,
            &descriptor_size,
            &descriptor_version,
        );
        println!("Return code: {:x?}\r\nSize: {}", ret & (!(1 << size_of::<u64>()*8-1)), size);

        let my_vec: Vec<u8> = Vec::with_capacity(size);
        let ret = ((*boot_services).get_memory_map)(
            &size,
            my_vec.as_ptr(),
            &map_key,
            &descriptor_size,
            &descriptor_version,
        );
        println!("Return code: {:x?}\r\nSize: {}", ret & (!(1 << 63)), size);
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

pub fn _walk_config_table(guid: Guid) -> *const EfiConfigurationTable {
    let table = EFI_SYSTEM_TABLE.load(Ordering::Relaxed);
    unsafe {
        let mut num_entries = (*table).num_table_entries;
        let mut table_ptr = (*table).efi_config_table;
        while num_entries != 0 {
            println!("{:x?}", *table_ptr);
            if guid == (*table_ptr).vendor_guid {
                break;
            }
            table_ptr = table_ptr.wrapping_add(1);
            num_entries = num_entries-1;
        }
    }
    todo!();
}

pub struct EfiAllocator {}

impl EfiAllocator {
    pub const fn new() -> Self {EfiAllocator{}}
}

unsafe impl GlobalAlloc for EfiAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = core::ptr::null();
        let table = EFI_SYSTEM_TABLE.load(Ordering::Relaxed);
        let boot_services = (*table).boot_services;
        let retval = ((*boot_services).allocate_pool)(EfiMemoryType::EfiLoaderData, layout.size(), addr_of!(ptr));
        match retval {
            0 => {/*println!("Memory allocated at: {:?}", ptr);*/},
            _ => {panic!("Memory allocation failed: {:x?}", retval);}
        }
        ptr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let table = EFI_SYSTEM_TABLE.load(Ordering::Relaxed);
        let boot_services = (*table).boot_services;
        let retval = ((*boot_services).free_pool)(ptr);
        match retval {
            0 => {/*println!("Memory deallocated at: {:?}",  ptr);*/},
            _ => {panic!("Memory deallocation failed: {:x?}", retval);}
        }
    }
}