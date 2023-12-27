//! Architecture dependent internal low level functions.  
//! These functions should not be used anywhere outside of 
//! the aarch64 module as they are strictly processor dependent 
//! and only make sense in the context of aarch64.

use core::arch::asm;

pub fn wait_for_interrupt() {
    unsafe {
        asm!("dsb sy");
        asm!("wfi");
    }
}