use core::borrow::{Borrow, BorrowMut};
use core::f32::consts::E;
use core::fmt::{self, Write};
use core::ops::{Add, Sub};
use core::ptr::{self, addr_of_mut};
use core::arch::{asm, global_asm};
use core::slice;
use crate::arch::aarch64::cpu;
use crate::arch::aarch64::driver::mmu::{self, va_to_pa};
use crate::driver::e1000::E1000;
use crate::driver::qemu::ramfb::*;
use crate::user::graphics::console::GfxConsole;
use crate::{driver, KernelStruct, KERNEL_STRUCT};
use super::driver::qemu::smp::*;
use super::platform::{self, *};
use super::driver::serial::{serial_init, serial_putchar, serial_puts};
use crate::user::graphics::gfx::*;
use crate::arch::aarch64::driver::gicv3::{self, *};
use super::driver::apl::*;

global_asm!(include_str!("boot.s"));
global_asm!(include_str!("exception.s"));

extern "C" {
    static _bss_start: u8;
    static _bss_end: u8;
    static _stack_start: u8;
    static _stack_end: u8;
    static _base: u8;
    static _kernel_end: u8;
}

#[no_mangle]
pub extern fn clear_bss() {
    let bss_start = unsafe {&_bss_start} as *const u8 as usize;
    let bss_end = unsafe {&_bss_end} as *const u8 as usize;
    let bss_size = bss_end - bss_start;
    /*for x in 0..bss_size/core::mem::size_of::<u128>() {
        unsafe {
            (bss_start as *mut u128).add(x).write_volatile(0);
        }
    }*/
    unsafe {
        slice::from_raw_parts_mut(bss_start as *mut u8, bss_size).fill(0x0);
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
    
    let fb_addr = va_to_pa(unsafe {&_stack_end} as *const u8 as usize).unwrap() as *mut u8;
    let bpp = 3;
    let width = 1280;
    let height = 720;
    let stride = width*bpp;
    setup_ramfb(fb_addr, width, height);
    println!("Ramfb init succesful");
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
        gic.set_interrupt_group(timer_interrupt, true);
        gic.enable_interrupt(timer_interrupt);
        
        for x in 32..1024 {
            gic.set_interrupt_group(x, false);
            gic.set_interrupt_trigger(x, true);
            gic.enable_interrupt(x);
        }
    }

    enable_all_interrupts();
    enable_timer_interrupt(1000);
    let pci_s = 0x4010000000 as *mut u8;
    /*unsafe {
        let pci = pci_s.add(0 << 20 | 1 << 15 | 0 << 12);
        hex_print(pci, 0x10);
        let dev = driver::pci::PCIHeader::new(pci);
        println!("{:#x?}", dev);
    }*/
    let mut pci_bus = driver::pci::PCIBus::new(pci_s);
    pci_bus.enumerate();
    //hex_print(0x3eff0000 as *mut u8, 0x10);
    let mut e1000 = Option::None;
    for x in pci_bus.device_list {
        if x.is_some() {
            unsafe {
                let dev = x.unwrap();
                if (*dev).header.vendor_id.to_le() == 0x8086 && (*dev).header.device_id.to_le() == 0x100e {
                    e1000 = Some(E1000::new(dev, 0x1000_0000, 0x0));
                }
            }
        }
    }
    if let Some(e1000) = e1000.as_mut() {
        e1000.init();
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
        #[cfg(feature = "apl")]
        let ks = setup_apple();
        #[cfg(not(feature = "apl"))]
        let ks = setup_qemu();
        println!("{:p}", _start_rust as *const());
        /*unsafe {
            let x: u64;
            asm!("mrs {:x}, tpidr_el1", out(reg) x);
            println!("Core: {}", x);
        }*/ 
        
        crate::kmain(Some(ks));
    } else {
        //serial_puts("Test");

        unsafe {
            let gic = GIC::new(gicv3::GICD_BASE, gicv3::GICR_BASE)
                            .expect("Error getting device");
            gic.per_core_init();
            let timer_interrupt = 0x1e;
            gic.set_interrupt_trigger(timer_interrupt, false);
            gic.set_interrupt_group(timer_interrupt, true);
            gic.enable_interrupt(timer_interrupt);
        }
        enable_timer_interrupt(1000);
        enable_all_interrupts();
        
        crate::kmain(None);
    }
}
