#![no_std]
#![no_main]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(panic_info_message)]

#[macro_use]
mod print;
#[cfg(feature = "uefi")]
mod efi; // efi_main() is the entry point on UEFI systems
mod acpi;
mod arch;
mod driver;
mod util;
mod mm;
use core::{alloc, panic::PanicInfo, ptr::{self, addr_of}, sync::atomic::AtomicPtr};

use driver::pci::PCIBus;
use mm::{get_heap_end, get_heap_size, get_heap_start};
use user::graphics::{console::GfxConsole, gfx::GraphicsBuffer};
use mm::alloc::string::*;
use mm::alloc::vec::*;
use mm::alloc::format;
mod user;
mod kernel;

#[cfg(feature = "apl")]
use crate::arch::aarch64::driver::apl::keyboard_backlight::AppleKeyboardBacklight;

static KERNEL_STRUCT: AtomicPtr<KernelStruct> = AtomicPtr::new(ptr::null_mut());
static KBUF: util::ringbuffer::RingBuffer = util::ringbuffer::RingBuffer::new();

#[derive(Debug)]
#[repr(align(64))]
pub struct KernelStruct<'a> {
    framebuffer: Option<GraphicsBuffer>,
    console: Option<GfxConsole<'a>>,
    serial_addr: Option<*mut u8>,
    pci:    Option<PCIBus>
}

impl Default for KernelStruct<'static> {
    fn default() -> Self {
        KernelStruct {
            framebuffer: None,
            console: None,
            serial_addr: None,
            pci: None
        }
    }
}

#[derive(Debug)]
pub enum KernelError {
    KernelStructNotInitialized,
    FramebufferNotInitialized,
}

/// Stops execution of the kernel
fn halt_system() -> ! {
    loop {
        arch::host::platform::disable_all_interrupts();
        arch::host::platform::wait_for_interrupt();
        arch::host::platform::wait_for_event();
    }
}

#[panic_handler]
#[inline(never)]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(feature = "apl")]
    AppleKeyboardBacklight::set_dutycycle(1200000,1200000);
    /*let kernel_struct = unsafe { KERNEL_STRUCT.load(core::sync::atomic::Ordering::SeqCst).as_mut() };
    if let Some(kernel_struct) = kernel_struct {
        if let Some(console) = kernel_struct.console.as_mut() {
            console.clear();
        }
    }*/
    println!("Core {} panicked at {}:\r\n{}",
    arch::host::platform::get_current_core(),
    info.location().unwrap(),
    info.message());

    loop {
        halt_system();  
    }
}

pub fn enable_text_mode() -> Result<(),KernelError> {
    let kernel_struct = unsafe { KERNEL_STRUCT.load(core::sync::atomic::Ordering::SeqCst).as_mut() };
    if let Some(kernel_struct) = kernel_struct {
        let gfx_console;
        if let Some(fb) = kernel_struct.framebuffer.as_ref() {
            let font_size = 2;
            gfx_console = GfxConsole::new(font_size, &fb);
            kernel_struct.console = Some(gfx_console);
            return Ok(())
        } else {
            return Err(KernelError::FramebufferNotInitialized)
        }
    }
    Err(KernelError::KernelStructNotInitialized)
}

pub fn disable_text_mode() {
    let kernel_struct;
    unsafe {
        kernel_struct = KERNEL_STRUCT.load(core::sync::atomic::Ordering::SeqCst).as_mut();
    }
    if let Some(kernel_struct) = kernel_struct {
        kernel_struct.console = None;
    }
}


/// The main function of the actual kernel from
/// here platform specific implementations of
/// low level functions are only to be used
/// with the arch::host:: re-export and must be
/// implemented by every target platform.
/// This is to keep it generic across all targets
/// the kernel can be compiled for.
#[no_mangle]
pub fn kmain(kernel_struct: Option<KernelStruct>) -> ! {
    if  arch::host::platform::is_boot_core() {
        if let Some(kernel_struct) = kernel_struct {
                
                let _ = KERNEL_STRUCT.compare_exchange(ptr::null_mut(), addr_of!(kernel_struct) as *mut KernelStruct,
                core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::SeqCst);
        }
        //let kernel_struct = unsafe { KERNEL_STRUCT.load(core::sync::atomic::Ordering::SeqCst).as_mut() };
        match enable_text_mode() {
            Ok(_) => { dbg!("Textmode started!\r\n"); },
            Err(e) => { dbg!("Couldn't start textmode: {:?}\r\n", e); }
        }
        println!("====Salmon salad operating system====");
        println!("Starting kernel at: {:p}", kmain as *mut());
        println!("Heap is {:#x} bytes at: {:p} to {:p}", get_heap_size(), get_heap_start(), get_heap_end());
        let x = String::from("Test");
        user::sh::sh_main();
    } else {
        while KERNEL_STRUCT.load(core::sync::atomic::Ordering::SeqCst) == ptr::null_mut() {};
        dbg!("Core: {} waiting for interrupt loop reached!\r\n", arch::host::platform::get_current_core());
        loop { 
            arch::host::platform::wait_for_interrupt(); 
        }
    }
    panic!("End of kernel reached!\r\nSystem halted!");
}
