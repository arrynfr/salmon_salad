use core::arch::global_asm;

global_asm!(include_str!("boot.s"));
global_asm!(include_str!("trap.s"));

#[no_mangle]
pub extern fn _start_rust() -> ! {
    crate::kmain()
}
