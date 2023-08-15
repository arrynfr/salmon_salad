#![no_std]
#![no_main]
use core::panic::PanicInfo;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

mod efi;
use crate::efi::EfiSystemTable;

static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = AtomicPtr::new(ptr::null_mut());

#[panic_handler]
fn panic(_panic_message: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
extern fn efi_main(_handle: u64, table: *mut EfiSystemTable) {

    let _ = EFI_SYSTEM_TABLE.compare_exchange(core::ptr::null_mut(), 
                                              table, Ordering::SeqCst,
                                              Ordering::SeqCst);
    let s1 = ['H' as u16, 'i' as u16, ' ' as u16, 'W' as u16, 'o' as u16, 'r' as u16, 'l' as u16, 'd' as u16, '!' as u16, '\n' as u16, 0 as u16];
    unsafe {
        let console_out = (*table).con_out;
        ((*console_out).output_string)(console_out, s1.as_ptr());
    }
    loop{};
}
