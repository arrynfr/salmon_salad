#![no_std]
#![no_main]

mod arch;
mod config;
#[macro_use]
mod print;
use core::panic::PanicInfo;
mod acpi;
mod efi;

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
    println!("Hello world!");
    panic!("System halted!");
}

#[no_mangle]
extern "efiapi" fn efi_main(_handle: u64, table: *mut efi::EfiSystemTable) {
    efi::register_efi_system_table(table);
    efi::clear_screen();
    println!("We're booting in UEFI mode↑↑");
    kmain();
}
