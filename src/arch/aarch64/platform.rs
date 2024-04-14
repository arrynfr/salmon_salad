use core::arch::asm;

/// Put the core into a low power state
/// until an interrupt occours.
pub fn wait_for_interrupt() {
    unsafe {
        asm!("dsb sy");
        asm!("wfi");
    }
}

/// Put the core into a low power state
/// until an event is fired.
pub fn wait_for_event() {
    unsafe {
        asm!("dsb sy");
        asm!("wfe");
    }
}

/// Send a wake up event to another core.
pub fn send_event() {
    unsafe {
        asm!("dsb sy");
        asm!("sev");
    }
}

pub fn get_current_core_el1() -> u64 {
    let current_core: u64;
    unsafe { asm!("mrs x0, MPIDR_EL1", out("x0") current_core); }
    current_core&0xFF
}

pub fn get_current_core() -> u64 {
    get_current_core_el1()
}

pub fn is_boot_core() -> bool {
    get_current_core_el1() == 0
}

/// Get elapsed time since poweron in seconds
pub fn get_current_poweron_time_in_s() -> u32 {
    (get_system_timer()/get_system_timer_frequency()) as u32
}

/// Get elapsed time since poweron in milliseconds
pub fn get_current_poweron_time_in_ms() -> u32 {
    (get_system_timer()/(get_system_timer_frequency()/1000)) as u32
}

/// Get elapsed time since poweron in microseconds
pub fn get_current_poweron_time_in_us() -> u32 {
    (get_system_timer()/(get_system_timer_frequency()/1000000)) as u32
}

/// Get timer frequency in ticks per second
#[inline(always)]
pub fn get_system_timer_frequency() -> u64 {
    let frq:u64;
    unsafe { asm!("mrs x0, CNTFRQ_EL0", out("x0") frq); }
    frq
}

/// Get elapsed time since poweron in ticks
#[inline(always)]
pub fn get_system_timer() -> u64 {
    let current_cnt: u64;
    unsafe { asm!("mrs x0, CNTPCT_EL0", out("x0") current_cnt); }
    current_cnt
}


/// Delay execution for `ticks` ticks.
/// 
/// Ticks is an arbitraty amount of time dependent on
/// the timer frequency `CNTFRQ_EL0`.
/// The timer is at least 56 bits large
/// so we truncate to avoid waiting forever.
/// Not that this really matters since it's an absurd
/// amount of time to wait anyway.
pub fn delay_ticks(ticks: u64) {
    let ticks_capped = ticks & !(0xFF << 56);
    let start_time = get_system_timer();
    while get_system_timer() - start_time < ticks_capped {}
}

/// Delay execution for `microseconds` microseconds.
pub fn delay_us(microseconds: u32) {
    delay_ticks((microseconds as u64 * get_system_timer_frequency())/1000000);
}

/// Delay execution for `milliseconds` milliseconds.
pub fn delay_ms(milliseconds: u32) {
    delay_ticks(milliseconds as u64 * (get_system_timer_frequency()/1000));
}

/// Delay execution for `seconds` seconds.
pub fn delay_s(seconds: u32) {
    delay_ticks(seconds as u64 * get_system_timer_frequency());
}

pub fn msr_daifclr(value: u64) {
    // 0x7 to enable SError, IRQ, FIQ
    unsafe { asm!("mrs DAIFClr, {}", in(reg) value); }
}
