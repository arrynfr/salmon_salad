use core::arch::global_asm;
use core::arch::asm;
use crate::driver::qemu::ramfb::*;
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
        let translation_table_base = 0x1000_0000; // Adjust this based on your memory layout
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
            tlbi vmalle1
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
    let bpp = 3;
    let width = 1280;
    let height = 720;
    let fb_addr;
    unsafe {
        fb_addr = &_stack_end as *const u8 as *mut u8;
    }
    let stride = width*bpp;
    setup_ramfb(fb_addr, width, height);
    let graphics_buffer =   GraphicsBuffer::new(fb_addr, (stride*height) as usize, 
                                            stride as u32, width, height, PixelFormat::BGR8, bpp as usize);

    unsafe {
        //graphics_buffer.draw_circle((100,100), 100, Color { b: 255, g: 0, r: 0 });
        //graphics_buffer.draw_rectangle(0, 0, graphics_buffer.horizontal_resolution as isize, graphics_buffer.vertical_resolution as isize, Color{r: 128, g: 128, b: 128});

        //graphics_buffer.draw_line(0, 0, graphics_buffer.horizontal_resolution as isize, graphics_buffer.vertical_resolution as isize, Color{r: 255, g: 255, b: 255});
        //graphics_buffer.draw_circle((500,500), 100, Color{r: 255, g: 0, b: 0});
        //graphics_buffer.draw_rectangle(600, 600, 100, 100, Color{r: 0, g: 0, b: 255});
        graphics_buffer.draw_string(10, 10, "Ubuntu 18.04 ubuntu tty1", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*2), "ubuntu login: Ubuntu", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*3), "Password:", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*4), "Welcome to Ubuntu 18.04 (GNU/Linux 4.15.0-23-generic)", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*6), " * Documentation:  https://help.ubuntu.com/", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*8), "278 packages can be updated.", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*9), "71 updates are security updates.", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*11), "The programs included with the Ubuntu system are free software;", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*12), "the exact distribution terms for each program are described in the", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*13), "individual files in /usr/share/doc/*/copyright.", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*15), "Ubuntu comes with ABSOLUTELY NO WARRANTY, to the extent permitted by applicable law.", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*17), "Ubuntu@ubuntu:~$", Color { b: 0x99, g: 0x99, r: 0x99 });
    }

    crate::kmain()
}