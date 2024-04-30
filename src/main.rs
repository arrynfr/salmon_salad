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
use core::{panic::PanicInfo, ptr::{self, addr_of}, sync::atomic::AtomicPtr};

use user::graphics::{console::GfxConsole, gfx::GraphicsBuffer};
mod user;
mod kernel;

#[cfg(feature = "apl")]
use crate::arch::aarch64::driver::apl::keyboard_backlight::AppleKeyboardBacklight;

static KERNEL_STRUCT: AtomicPtr<KernelStruct> = AtomicPtr::new(ptr::null_mut());

#[derive(Debug)]
#[repr(align(64))]
pub struct KernelStruct<'a> {
    framebuffer: Option<GraphicsBuffer>,
    console: Option<GfxConsole<'a>>,
    serial_addr: Option<*mut u8>,
}

impl Default for KernelStruct<'static> {
    fn default() -> Self {
        KernelStruct {
            framebuffer: None,
            console: None,
            serial_addr: None
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
    }
}

#[panic_handler]
#[inline(never)]
fn panic(info: &PanicInfo) -> ! {
    /*let kernel_struct = unsafe { KERNEL_STRUCT.load(core::sync::atomic::Ordering::SeqCst).as_mut() };
    if let Some(kernel_struct) = kernel_struct {
        if let Some(console) = kernel_struct.console.as_mut() {
            console.clear();
        }
    }*/
    println!("panicked at {}:\r\n{}", 
    info.location().unwrap(),
    info.message().unwrap());
    loop {
        #[cfg(feature = "apl")]
        AppleKeyboardBacklight::set_dutycycle(1200000,1200000);
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
                KERNEL_STRUCT.compare_exchange(ptr::null_mut(), addr_of!(kernel_struct) as *mut KernelStruct,
                core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::SeqCst)
                .expect("Couldn't initialize kernel struct!");
        }
        let kernel_struct = unsafe { KERNEL_STRUCT.load(core::sync::atomic::Ordering::SeqCst).as_mut() };
        match enable_text_mode() {
            Ok(_) => { println!("Text mode started!"); },
            Err(e) => { println!("Couldn't start textmode: {:?}", e); }
        }
        println!("Booting in EL: {}", arch::host::platform::get_current_el());
        user::sh::sh_main();
    } else {
        while KERNEL_STRUCT.load(core::sync::atomic::Ordering::SeqCst) == ptr::null_mut() {};
        arch::host::gicv3::send_sgi(13);
        dbg!("Halting core: {}\r\n", arch::host::platform::get_current_core());
        loop { halt_system() }
    }
    panic!("End of kernel reached!\r\nSystem halted!");
}
