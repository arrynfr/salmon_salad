#![no_std]
#![no_main]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(panic_info_message)]

#[macro_use]
mod print;
#[cfg(feature = "uefi")]
mod efi; // This is the entry point on UEFI systems
mod acpi;
mod arch;
mod config;
mod driver;
use core::panic::PanicInfo;
mod user;

fn halt_system() -> ! {
    loop {
        arch::host::platform::wait_for_interrupt();
    }
}

#[panic_handler]
#[inline(never)]
fn panic(info: &PanicInfo) -> ! {
    println!("panicked at {}:", info.location().unwrap());
    println!("{}", info.message().unwrap());
    halt_system();
}

#[no_mangle]
pub extern fn kmain() -> ! {
    user::sh::sh_main();
    panic!("System halted!");
}
