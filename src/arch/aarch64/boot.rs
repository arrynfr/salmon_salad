use core::arch::global_asm;
use core::arch::asm;
use crate::driver::qemu::ramfb::*;
use crate::driver::qemu::smp::*;
use crate::user::graphics::gfx::*;
use crate::arch::aarch64::serial::serial_init;

global_asm!(include_str!("boot.s"));

const PAGE_SIZE: usize = 4096; // 4 KB page size

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
}

extern "C" {
    static _stack_end: u8;
}

#[no_mangle]
pub extern fn _start_rust() -> ! {
    /*unsafe {
        serial_init(0x09000000);
    }*/
    //init_mmu();
    unsafe {
        init_smp();
    }
    
    let bpp = 3;
    let width = 1024;
    let height = 768;
    let fb_addr;
    unsafe {
        fb_addr = &_stack_end as *const u8 as *mut u8;
    }
    let stride = width*bpp;
    setup_ramfb(fb_addr, width, height);
    let graphics_buffer =   GraphicsBuffer::new(fb_addr, (stride*height) as usize, 
                                            stride as u32, width, height, PixelFormat::BGR8, bpp as usize);

    unsafe {
        graphics_buffer.clear_screen();
        graphics_buffer.draw_string(10, 10, "Hello world from salmon_salad", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*1), "Salmon is my passion!", Color { b: 0x99, g: 0x99, r: 0x99 });
    }

    for y in (0xa000000..0xa003e00).step_by(0x200) {
        let some_virtio = (y as usize) as *mut u8;
        let mut some_data: [u8; 0x200] = [0; 0x200];
        for x in 0..some_data.len() {
            unsafe {
                some_data[x] = some_virtio.add(x).read_volatile();
            }
        }
        //println!("Addr 0x{y:x?}:\r\n{some_data:02x?}");
    }

    
    crate::kmain()
}