use core::arch::global_asm;
use crate::arch::riscv64::driver::serial::serial_init;
use crate::driver::qemu::ramfb::setup_ramfb;
use crate::user::graphics::gfx::*;
use crate::{KernelStruct, KERNEL_STRUCT};
use super::platform::*;
use core::ptr;

global_asm!(include_str!("boot.s"));
global_asm!(include_str!("trap.s"));

extern "C" {
    static _stack_end: u8;
}

pub fn setup_qemu() -> KernelStruct<'static> {
    let mut k_struct = KernelStruct::default();
    k_struct.serial_addr = Some(0x1000_0000 as *mut u8);
    serial_init(k_struct.serial_addr.unwrap());
    println!("Serial init succesful");

    let bpp = 3;
    let width = 1280;
    let height = 720;
    let fb_addr = unsafe { &_stack_end as *const u8 as *mut u8 };
    let stride = width*bpp;
    setup_ramfb(fb_addr, width, height);

    let graphics_buffer =   GraphicsBuffer::new(fb_addr, (stride*height) as usize, 
    stride, width, height, PixelFormat::BGR8, bpp as usize);
    k_struct.framebuffer = Some(graphics_buffer);

    k_struct
}

#[no_mangle]
pub extern fn _start_rust() -> ! {
    if is_boot_core() {
        let ks = setup_qemu();
        crate::kmain(Some(ks));
    } else {
        while KERNEL_STRUCT.load(core::sync::atomic::Ordering::SeqCst) == ptr::null_mut() {};
        //dbg!("Booting on core: {current_core}\r\n");
        crate::kmain(None);
    }
}
