use core::arch::global_asm;
use crate::arch::riscv64::serial::serial_init;

global_asm!(include_str!("boot.s"));
global_asm!(include_str!("trap.s"));

#[no_mangle]
pub extern fn _start_rust() -> ! {
    unsafe {
        serial_init(0x10000000);
    }
    crate::kmain()
}
