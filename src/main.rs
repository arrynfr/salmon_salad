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
use core::{fmt::Write, panic::PanicInfo, ptr::{self, addr_of}, sync::atomic::AtomicPtr};

use user::graphics::gfx::GraphicsBuffer;
use crate::user::graphics::console::GfxConsole;
mod user;
mod kernel;

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

/// Stops execution of the kernel
fn halt_system() -> ! {
    loop {
        arch::host::platform::wait_for_interrupt();
    }
}

#[panic_handler]
#[inline(never)]
fn panic(info: &PanicInfo) -> ! {
    println!("panicked at {}: {}", 
    info.location().unwrap(),
    info.message().unwrap());
    halt_system();
}

#[no_mangle]
pub fn kmain(kernel_struct: Option<KernelStruct>) -> ! {
    if  arch::host::platform::is_boot_core() {
    if let Some(mut kernel_struct) = kernel_struct {
            let gfx_console;
            let fb = kernel_struct.framebuffer.as_ref().unwrap();
            let font_size = 2;
            gfx_console = GfxConsole::new(font_size, &fb);
            kernel_struct.console = Some(gfx_console);
            KERNEL_STRUCT.compare_exchange(ptr::null_mut(), addr_of!(kernel_struct) as *mut KernelStruct,
            core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::SeqCst)
            .expect("Couldn't initialize kernel struct!");
    }
        user::sh::sh_main();
    } else {
        //dbg!("Halting core: {}\r\n", arch::host::platform::get_current_core());
        halt_system();
    }
    panic!("End of kernel reached!\r\nSystem halted!");
}
