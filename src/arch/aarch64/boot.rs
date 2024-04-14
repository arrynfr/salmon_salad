use core::arch::global_asm;
use core::borrow::BorrowMut;
use core::ffi::CStr;
use core::fmt::Write;
use core::ptr;

use fdt_parse::Fdt;

use crate::driver::qemu::ramfb::*;
use crate::user::graphics::console::GfxConsole;
use crate::{KernelStruct, KERNEL_STRUCT};
use super::driver::qemu::smp::*;
use super::platform::*;
use super::serial::serial_init;
use crate::user::graphics::gfx::{self, *};
use crate::arch::aarch64::gicv3::init_gic;
use super::gicv3::init_gicr;
use super::driver::apl::watchdog::*;
use super::driver::apl::keyboard_backlight::*;

global_asm!(include_str!("boot.s"));
global_asm!(include_str!("exception.s"));

/*
const PAGE_SIZE: usize = 16*1024; // 16 KB page size

// Page table entry flags
const TABLE_DESC: usize = 0b11; // Table descriptor
const PAGE_DESC: usize = 0b11;  // Page descriptor
const RW_EL1: usize = 0b01;    // Read/Write permissions for EL1

// Function to initialize the MMU
fn init_mmu() {
    unsafe {
        // Disable MMU and caches
        asm!("
            mrs x0, SCTLR_EL1
            bic x0, x0, #1
            msr SCTLR_EL1, x0
            isb
        ");

        // Set up page tables
        let translation_table_base = 0x4008_0000; // Adjust this based on your memory layout
        init_page_tables(translation_table_base);

        // Enable MMU and caches
        asm!("
            mrs x0, SCTLR_EL1
            orr x0, x0, #1
            msr SCTLR_EL1, x0
            isb
        ");
    }
}

// Function to initialize page tables
fn init_page_tables(translation_table_base: usize) {
    // Create the first-level table
    let mut first_level_table: [usize; 512] = [0; 512];

    // Create a second-level table for a specific region (adjust as needed)
    let mut second_level_table: [usize; 512] = [0; 512];

    // Calculate the physical address of the second-level table
    let second_level_table_phys = &second_level_table as *const _ as usize;

    // Set up entries in the second-level table
    for i in 0..512 {
        let virtual_addr = i * PAGE_SIZE;
        let physical_addr = virtual_addr; // For simplicity, use 1:1 mapping
        let entry = physical_addr | PAGE_DESC | RW_EL1;
        second_level_table[i] = entry;
    }

    // Calculate the physical address of the first-level table
    let first_level_table_phys = &first_level_table as *const _ as usize;

    // Set up entries in the first-level table
    let entry = second_level_table_phys | TABLE_DESC | RW_EL1;
    first_level_table[0] = entry;

    unsafe {
        // Load the base address of the page tables into the Translation Lookaside Buffer (TLB)
        asm!(
            "msr TTBR0_EL1, {}",
            "isb",
            in(reg) first_level_table_phys
        );


        // Invalidate TLB entries to ensure the changes take effect
        asm!("
            tlbi alle1
            isb
        ");
    }
}*/

extern "C" {
    static _bss_start: u8;
    static _bss_end: u8;
    static _stack_start: u8;
    static _stack_end: u8;
    static _base: u8;
}

fn clear_bss() {
    let bss_start = unsafe {&_bss_start} as *const u8 as usize;
    let bss_end = unsafe {&_bss_end} as *const u8 as usize;
    let bss_size = bss_end - bss_start;

    for x in 0..bss_size {
        unsafe {
            (bss_start as *mut u8).add(x).write_volatile(0);
        }
    }
}

fn setup_apple() -> KernelStruct<'static> {
    Watchdog::disable();
    AppleKeyboardBacklight::init();
    let mut k_struct = KernelStruct::default();
    let fb_addr = 0xbe20e4000 as *mut u8;
    let bpp = 4;
    let width = 2560;
    let height = 1600;
    let stride = width*bpp;
    let graphics_buffer =   GraphicsBuffer::new(fb_addr, (stride*height) as usize, 
    stride, width, height, PixelFormat::APL, bpp as usize);
    k_struct.framebuffer = Some(graphics_buffer);

    k_struct
}

fn setup_qemu() -> KernelStruct<'static> {
    let mut k_struct = KernelStruct::default();
    k_struct.serial_addr = Some(0x0900_0000 as *mut u8);
    serial_init(k_struct.serial_addr.unwrap());
    println!("Serial init succesful");
    
    let fb_addr = unsafe {&_stack_end} as *const u8 as *mut u8;
    let bpp = 3;
    let width = 1280;
    let height = 720;
    let stride = width*bpp;
    setup_ramfb(fb_addr, width, height);
    let graphics_buffer =   GraphicsBuffer::new(fb_addr, (stride*height) as usize, 
    stride, width, height, PixelFormat::BGR8, bpp as usize);
    k_struct.framebuffer = Some(graphics_buffer);
    
    unsafe {
        init_smp();
        //init_gic();
        //init_gicr();
    }

    k_struct     
}

#[no_mangle]
pub extern fn _start_rust(argc: u64, argv: *const *const u64) -> ! {
    let current_core = get_current_core_el1();
    if is_boot_core() {
        clear_bss();
        //let ks = setup_apple();
        let ks = setup_qemu();
        crate::kmain(Some(ks));
    } else {
        while KERNEL_STRUCT.load(core::sync::atomic::Ordering::SeqCst) == ptr::null_mut() {};
        dbg!("Booting on core: {current_core}\r\n");
        crate::kmain(None);
    }
}