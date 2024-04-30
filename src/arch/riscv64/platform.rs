//! Architecture dependent internal low level functions.  
//! These functions should not be used anywhere outside of 
//! the aarch64 module as they are strictly processor dependent 
//! and only make sense in the context of aarch64.

use core::arch::asm;

pub fn wait_for_interrupt() {
    unsafe {
        asm!("wfi");
    }
}

pub fn get_current_core() -> usize {
    let hartid: usize;
    unsafe {
        asm!("csrr a0, mhartid", out("a0") hartid);
    }
    hartid
}

pub fn is_boot_core() -> bool {
    get_current_core() == 0
}

pub fn delay_s(seconds: u32) {
    unimplemented!();
}

pub fn get_interrupt_mask() -> u64 {
    unimplemented!();
}

pub fn disable_all_interrupts() {
    unimplemented!();
}

pub fn set_interrupt_mask(imask: u64) {
    unimplemented!();
}