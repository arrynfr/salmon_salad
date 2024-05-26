use core::f32::consts::E;
use core::ops::{Add, Sub};
use core::ptr::{self, addr_of_mut};
use core::arch::{asm, global_asm};
use crate::arch::aarch64::cpu;
use crate::driver::e1000::e1000;
use crate::driver::qemu::ramfb::*;
use crate::{driver, KernelStruct, KERNEL_STRUCT};
use super::driver::qemu::smp::*;
use super::platform::*;
use super::driver::serial::serial_init;
use crate::user::graphics::gfx::*;
use crate::arch::aarch64::driver::gicv3::{self, *};
use super::driver::apl::*;

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
    watchdog::Watchdog::disable();
    let mut k_struct = KernelStruct::default();
    let fb_addr = 0xbe20e4000 as *mut u8;
    let bpp = 4;
    let width = 2560;
    let height = 1600;
    let stride = width*bpp;
    let graphics_buffer =   GraphicsBuffer::new(fb_addr, (stride*height) as usize, 
    stride, width, height, PixelFormat::APL, bpp as usize);
    k_struct.framebuffer = Some(graphics_buffer);
    keyboard_backlight::AppleKeyboardBacklight::init();
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

    /*let f = fdt_parse::Fdt::new(0x4000_0000 as *const u8).unwrap();
    let mut x = 0;
    while x < f.dt_struct.len() {
        print!("{:x?}, ", f.dt_struct[x].to_be());
        x += 1;
    }
    println!("{:?}", f.get_string(0xba));*/

    unsafe {
        init_smp();
        let gic = GIC::new(gicv3::GICD_BASE, gicv3::GICR_BASE)
                        .expect("Error initializing GICv3");
        gic.init_gic();
        let timer_interrupt = 0x1e;
        gic.set_interrupt_trigger(timer_interrupt, false);
        gic.enable_interrupt(timer_interrupt);
        gic.set_interrupt_group(timer_interrupt, true);
        //for x in 32..1024 {
        //    gic.enable_interrupt(x);
        //    gic.set_interrupt_group(x, true);
        //    gic.set_interrupt_trigger(x, true);
        //}
    }

    k_struct     
}

fn hex_print(addr: *mut u8, lines: usize) {
    let num_vals = lines*16;
    unsafe {
        println!("{0:<12}: {1:2X?}", "Offset",[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);
        for x in (0..num_vals).step_by(16) {
            let mut values: [u8; 16] = [0; 16];
            for y in 0..16 {
                values[y] = addr.add(x+y).read_volatile();
            }
            println!("{0:<12p}: {1:2x?}", addr.add(x), values);
        }
    }
}

fn hex_print32(addr: *mut u8, lines: usize) {
    let num_vals = lines*16;
    unsafe {
        println!("{0:<12}: {1:2X?}", "Offset",[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);
        for x in (0..num_vals).step_by(16) {
            let mut values: [u8; 16] = [0; 16];
            for y in (0..16).step_by(4) {
                let val = (addr.add(x+y) as *mut u32).read_volatile().to_le() as u64;
                values[y+0] = ((val >> 0) & 0xFF) as u8;
                values[y+1] = ((val >> 8) & 0xFF) as u8;
                values[y+2] = ((val >> 16) & 0xFF) as u8;
                values[y+3] = ((val >> 32) & 0xFF) as u8;
            }
            println!("{0:<12p}: {1:2x?}", addr.add(x), values);
        }
        println!();
    }
}

#[no_mangle]
pub extern fn _start_rust(_argc: u64, _argv: *const *const u64) -> ! {
    let current_core = get_current_core_el1();
    if is_boot_core() {
        clear_bss();
        #[cfg(feature = "apl")]
        let ks = setup_apple();
        #[cfg(not(feature = "apl"))]
        let ks = setup_qemu();
        cpu::get_cpu_features();
        cpu::get_cpu_features2();
        //enable_all_interrupts();
        //enable_timer_interrupt(1000);
        let pci_s = 0x4010000000 as *mut u8;
        let mut pci;
        unsafe {
            pci = pci_s.add(0 << 20 | 1 << 15 | 0 << 12);
            hex_print(pci, 0x10);
            let dev = driver::pci::PCIHeader::new(pci);
            println!("{:#x?}", dev);
        }
        let mut pci_bus = driver::pci::PCIBus::new(pci_s);
        pci_bus.enumerate();
        //hex_print(0x3eff0000 as *mut u8, 0x10);
        let mut e1000 = e1000::new(core::ptr::null_mut(), 0x0, 0x0);
        for x in pci_bus.device_list {
            if x.is_some() {
                unsafe {
                    let dev = x.unwrap();
                    if (*dev).header.vendor_id.to_le() == 0x8086 && (*dev).header.device_id.to_le() == 0x100e {
                        e1000 = e1000::new(dev, 0x1000_0000, 0x0);
                    }
                }
            }
        }
        
        
        let io_space = 0x1000_0000 as *mut u32;
        //hex_print32(io_space as *mut u8, 0x10);
        e1000.init();
        unsafe {
            hex_print32(io_space.add(0) as *mut u8, 0xf);
        }
        crate::kmain(Some(ks));
    } else {
        unsafe {
            let gic = GIC::new(gicv3::GICD_BASE, gicv3::GICR_BASE)
                            .expect("Error getting device");
            gic.per_core_init();
            let timer_interrupt = 0x1e;
            gic.set_interrupt_trigger(timer_interrupt, false);
            gic.enable_interrupt(timer_interrupt);
            gic.set_interrupt_group(timer_interrupt, true);
        }
        dbg!("Booting on core: {current_core}\r\n");
        //enable_timer_interrupt(1000);
        //enable_all_interrupts();
        crate::kmain(None);
    }
}
