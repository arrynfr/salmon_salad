#![no_std]
#![no_main]
#[macro_use] mod print;
use core::panic::PanicInfo;

mod efi;
use crate::efi::EfiSystemTable;

#[panic_handler]
fn panic(_panic_message: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
extern fn efi_main(_handle: u64, table: *mut EfiSystemTable) {
    efi::register_efi_system_table(table);
    print!("Test\n");
    print!("We're booting in UEFI mode ayyy!\n");
    loop{};
}
