#![no_std]
#![no_main]
#![feature(allow_internal_unstable)]

mod config;
#[macro_use] mod print;
use core::panic::PanicInfo;
mod efi;
mod acpi;
mod arch;

fn halt_system() -> ! {
    loop{
        arch::host::platform::wait_for_interrupt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}", info);
    halt_system();
}

#[no_mangle]
extern fn efi_main(_handle: u64, table: *mut efi::EfiSystemTable) {
    efi::register_efi_system_table(table);
    efi::clear_screen();
    unsafe {
        arch::host::serial::serial_init();
    }
    if config::IS_DEBUG {
        print!("You are running a debug build!\n\r");
    }
    println!("We're booting in UEFI mode↑↑");
    halt_system();
}
