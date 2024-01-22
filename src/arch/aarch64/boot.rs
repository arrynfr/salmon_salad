use core::arch::global_asm;
use crate::arch::aarch64::serial::serial_init;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern fn _start_rust() -> ! {
    unsafe {
        serial_init(0x09000000);
    }
    crate::kmain()
}