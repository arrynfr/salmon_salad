use core::arch::asm;

use crate::print;

/// Put the core into a low power state
/// until an interrupt occours.
pub fn wait_for_interrupt() {
    unsafe {
        asm!(   "dsb sy",
                "wfi");
    }
}

/// Put the core into a low power state
/// until an event is fired.
pub fn wait_for_event() {
    unsafe {
        asm!(   "dsb sy",
                "wfe");
    }
}

/// Send a wake up event to another core.
pub fn send_event() {
    unsafe {
        asm!(   "dsb sy",
                "sev");
    }
}

pub fn get_current_core_el1() -> u64 {
    let current_core: u64;
    unsafe { asm!("mrs {}, MPIDR_EL1",
                out(reg) current_core,
                options(nostack, nomem));
            }
    current_core&0xFF
}

pub fn get_current_core() -> u64 {
    get_current_core_el1()
}

pub fn is_boot_core() -> bool {
    get_current_core_el1() == 0
}

pub fn get_current_el() -> u8 {
    let el: u8;
    unsafe {
        asm!("mrs {:x}, CurrentEL", out(reg) el, options(nostack, nomem));
    }
    el >> 2
}

/// Get elapsed time since poweron in seconds
#[inline(always)]
pub fn get_current_poweron_time_in_s() -> f64 {
    get_system_timer() as f64 / get_system_timer_frequency() as f64
}

/// Get elapsed time since poweron in milliseconds
pub fn get_current_poweron_time_in_ms() -> f64 {
    const MS_FACTOR: f64 = 1000_f64;
    (get_system_timer() as f64 * MS_FACTOR)/ get_system_timer_frequency() as f64
}

/// Get elapsed time since poweron in microseconds
pub fn get_current_poweron_time_in_us() -> f64 {
    const US_FACTOR: f64 = 1000000_f64;
    (get_system_timer() as f64 * US_FACTOR) / get_system_timer_frequency() as f64
}

/// Get timer frequency in ticks per second
#[inline(always)]
pub fn get_system_timer_frequency() -> u64 {
    let frq:u64;
    unsafe { asm!(  "mrs {}, CNTFRQ_EL0",
                    out(reg) frq,
                    options(nostack, nomem));
            }
    frq
}

/// Get elapsed time since poweron in ticks
#[inline(always)]
pub fn get_system_timer() -> u64 {
    let current_cnt: u64;
    unsafe { asm!(  "isb", 
                    "mrs {}, CNTPCT_EL0",
                    out(reg) current_cnt,
                    options(nostack, nomem));
            }
    current_cnt
}


/// Delay execution for not less than `ticks` ticks.
/// 
/// Ticks is an arbitraty amount of time dependent on
/// the timer frequency `CNTFRQ_EL0`.
/// The timer is at least 56 bits large
/// so we truncate to avoid waiting forever.
/// Not that this really matters since it's an absurd
/// amount of time to wait anyway.
/// It's important to note that the only guarantee that this function
/// makes is that the delay is not less than `ticks`,
/// but it could be above it.
pub fn delay_ticks(ticks: u64) {
    let ticks_capped = ticks & !(0xFF << 56);
    let start_time = get_system_timer();
    while get_system_timer() - start_time < ticks_capped {}
}

/// Delay execution for `microseconds` microseconds.
#[inline(always)]
pub fn delay_us(microseconds: u32) {
    const US_FACTOR: f64 = 0.000001;
    delay_ticks(((microseconds as u64 * get_system_timer_frequency()) as f64 * US_FACTOR) as u64);
}

/// Delay execution for `milliseconds` milliseconds.
#[inline(always)]
pub fn delay_ms(milliseconds: u32) {
    const MS_FACTOR: f64 = 0.001;
    delay_ticks(((milliseconds as u64 * get_system_timer_frequency()) as f64 * MS_FACTOR) as u64);
}

/// Delay execution for `seconds` seconds.
#[inline(always)]
pub fn delay_s(seconds: u32) {
    delay_ticks(seconds as u64 * get_system_timer_frequency());
}

#[inline(always)]
pub fn enable_all_interrupts() {
    // 0x7 to enable D, SError, IRQ, FIQ
    unsafe { asm!(  "msr DAIFClr, #0b1111",
                    "isb", 
                    options(nostack, nomem)) 
            }
}

#[inline(always)]
pub fn disable_all_interrupts() {
    // 0x7 to disable SError, IRQ, FIQ
    unsafe { asm!(  "msr DAIFSet, #0b1111",
                    "isb",
                    options(nostack, nomem))
            }
}

pub fn set_interrupt_mask(imask: u64) {
    assert!(imask <= 0b1111);
    unsafe { asm!(  "msr DAIF, {}",
                    "isb",
                    in(reg) imask,
                    options(nostack, nomem)
                )
}
} 

#[inline(always)]
#[no_mangle]
pub fn get_interrupt_mask() -> u64 {
    let status: u64;
    unsafe { asm!(  "mrs {:x}, DAIF",
                    "isb",
                    out(reg) status,
                    options(nostack, nomem))
            }
    status >> 6
}

pub fn get_sctlr_el1() -> u64 {
    let sctlr:u64;
    unsafe { asm!("mrs {}, SCTLR_EL1", out(reg) sctlr, options(nostack, nomem)); }
    sctlr
}

pub fn get_sctlr_el2() -> u64 {
    let sctlr:u64;
    unsafe { asm!("mrs {}, SCTLR_EL2", out(reg) sctlr, options(nostack, nomem)); }
    sctlr
}

pub fn get_mmu_state() -> bool {
    let sctlr:u64;
    unsafe { asm!("mrs {}, SCTLR_EL1", out(reg) sctlr, options(nostack, nomem)); }
    (sctlr & 1) == 1
}

pub fn set_timer_ticks(ticks: u32) {
    unsafe {
        asm!(
        "msr CNTP_TVAL_EL0, {1:x}",
        "msr CNTP_CTL_EL0, {0:x}",
        in(reg) 1, in(reg) ticks );
    }    
} 

pub fn enable_timer_interrupt(milliseconds: u32) {
    const MS_FACTOR: f64 = 0.001;
    set_timer_ticks(((milliseconds as u64 * get_system_timer_frequency()) as f64 * MS_FACTOR) as u32);
}

#[no_mangle]
pub fn drop_to_el0(code: *const u8, sp: *const u8) {
    unsafe {
        asm!(
            "msr spsr_el1, {:x}",
            "msr elr_el1, {}",
            "msr sp_el0, {}",
            "eret",
            in(reg) 0x3c0,
            in(reg) ((code as usize) & 0xFFFF_FFFF),
            in(reg) sp
        )
    }
}
