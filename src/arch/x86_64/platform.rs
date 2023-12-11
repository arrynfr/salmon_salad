//! Architecture dependent internal low level functions.  
//! These functions should not be used anywhere outside of 
//! the x86_64 module as they are strictly processor dependent 
//! and only make sense in the context of x86_64.

use core::arch::asm;

pub unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value);
}

pub unsafe fn inb(port: u16) -> u8 {
    let mut value: u8; 
    asm!("in al, dx", out("al") value, in("dx") port);
    value
}

pub fn wait_for_interrupt() {
    unsafe {
        asm!("hlt");
    }
}