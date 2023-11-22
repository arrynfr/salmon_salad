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
    efi::clear_screen();
    if config::is_debug() {print!("You are running a debug build!\n\r");}
    if !config::is_debug() {print!("You are running a release build!\n\r");}
    print!("We're booting in UEFI mode ayyy!\n\r");
    println!();
    println!("Test2");
    loop{};
}
