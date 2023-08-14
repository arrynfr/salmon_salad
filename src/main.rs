#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_moose: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
extern fn efi_main() {
    let point = 0x4141414141414141 as *mut u64;
    unsafe {
        core::ptr::write_volatile(point, 0);}    
    loop{}
}
