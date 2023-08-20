#![no_std]
#![no_main]

mod config;
#[macro_use] mod print;
use core::panic::PanicInfo;

mod efi;
use crate::efi::EfiSystemTable;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}", info);
    loop{}
}

#[no_mangle]
extern fn efi_main(_handle: u64, table: *mut EfiSystemTable) {
    efi::register_efi_system_table(table);
    if config::is_debug() {print!("You are running a debug build!\n");}
    print!("We're booting in UEFI mode ayyy!\n");
    loop{};
}
