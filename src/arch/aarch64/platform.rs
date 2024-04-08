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

/// This is not very accurate since we're doing an integer division
pub fn get_current_poweron_time_in_s() -> u32 {
    (get_system_timer()/get_system_timer_frequency()) as u32
}

pub fn get_current_poweron_time_in_ms() -> u32 {
    (get_system_timer()/(get_system_timer_frequency()/1000)) as u32
}

pub fn get_current_poweron_time_in_us() -> u32 {
    (get_system_timer()/(get_system_timer_frequency()/1000000)) as u32
}

#[inline(always)]
pub fn get_system_timer_frequency() -> u64 {
    let frq:u64;
    unsafe { asm!("mrs x0, CNTFRQ_EL0", out("x0") frq); }
    frq
}

#[inline(always)]
pub fn get_system_timer() -> u64 {
    let current_cnt: u64;
    unsafe { asm!("mrs x0, CNTPCT_EL0", out("x0") current_cnt); }
    current_cnt
}

pub fn busy_sleep_s(delay: u32) {
    let start_time = get_current_poweron_time_in_s();
    while get_current_poweron_time_in_s() - start_time < delay {}
}