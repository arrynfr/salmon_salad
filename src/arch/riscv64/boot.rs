use core::arch::global_asm;
use crate::arch::riscv64::serial::serial_init;
use crate::driver::qemu::ramfb::setup_ramfb;
use crate::user::graphics::gfx::*;

global_asm!(include_str!("boot.s"));
global_asm!(include_str!("trap.s"));

extern "C" {
    static _stack_end: u8;
}

#[no_mangle]
pub extern fn _start_rust() -> ! {
    unsafe {
        serial_init(0x10000000);
    }

    let bpp = 3;
    let width = 1280;
    let height = 720;
    let fb_addr;
    unsafe {
        fb_addr = &_stack_end as *const u8 as *mut u8;
    }
    let stride = width*bpp;
    setup_ramfb(fb_addr, width, height);
    let graphics_buffer =   GraphicsBuffer::new(fb_addr, (stride*height) as usize, 
                                            stride as u32, width, height, PixelFormat::BGR8, bpp as usize);

    unsafe {
        //graphics_buffer.draw_circle((100,100), 100, Color { b: 255, g: 0, r: 0 });
        //graphics_buffer.draw_rectangle(0, 0, graphics_buffer.horizontal_resolution as isize, graphics_buffer.vertical_resolution as isize, Color{r: 128, g: 128, b: 128});

        //graphics_buffer.draw_line(0, 0, graphics_buffer.horizontal_resolution as isize, graphics_buffer.vertical_resolution as isize, Color{r: 255, g: 255, b: 255});
        //graphics_buffer.draw_circle((500,500), 100, Color{r: 255, g: 0, b: 0});
        //graphics_buffer.draw_rectangle(600, 600, 100, 100, Color{r: 0, g: 0, b: 255});
        graphics_buffer.draw_string(10, 10, "Ubuntu 18.04 ubuntu tty1", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*2), "ubuntu login: Ubuntu", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*3), "Password:", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*4), "Welcome to Ubuntu 18.04 (GNU/Linux 4.15.0-23-generic)", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*6), " * Documentation:  https://help.ubuntu.com/", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*8), "278 packages can be updated.", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*9), "71 updates are security updates.", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*11), "The programs included with the Ubuntu system are free software;", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*12), "the exact distribution terms for each program are described in the", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*13), "individual files in /usr/share/doc/*/copyright.", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*15), "Ubuntu comes with ABSOLUTELY NO WARRANTY, to the extent permitted by applicable law.", Color { b: 0x99, g: 0x99, r: 0x99 });
        graphics_buffer.draw_string(10, 10+(8*17), "Ubuntu@ubuntu:~$", Color { b: 0x99, g: 0x99, r: 0x99 });
    }

    crate::kmain()
}
