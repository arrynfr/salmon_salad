#![no_std]
#![no_main]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]

mod arch;
mod config;
#[macro_use]
mod print;
use core::panic::PanicInfo;
mod acpi;
mod efi;
mod user;

fn halt_system() -> ! {
    loop {
        arch::host::platform::wait_for_interrupt();
    }
}

#[panic_handler]
#[inline(never)]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    halt_system();
}

#[no_mangle]
pub extern fn kmain() -> ! {
    panic!("System halted!");
}
