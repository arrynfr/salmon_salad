const WATCHDOG_BASE: *mut Watchdog = 0x23d2b0000 as *mut Watchdog;
const WATCHDOG_FREQ: u32 = 24*1000*1000;
const WATCHDOG_ENABLE: u32 = 0b100;

#[repr(C)]
pub struct Watchdog {
    unk0: u32,
    unk1: u32,
    unk2: u32,
    unk3: u32,

    timer1: u32,
    cmp_val: u32,
    unk4: u32,
    ctrl_reg: u32
}

impl Watchdog {
    pub fn disable() {
        unsafe { (*WATCHDOG_BASE).ctrl_reg = 0; }
    }

    pub fn enable() {
        unsafe { (*WATCHDOG_BASE).ctrl_reg |= WATCHDOG_ENABLE; }
    }

    pub fn read_cmp_val() -> u32 {
        unsafe { (*WATCHDOG_BASE).cmp_val }
    }

    pub fn read_timer() -> u32 {
        unsafe { (*WATCHDOG_BASE).timer1 }
    }

    pub fn read_freq() -> u32 {
        WATCHDOG_FREQ
    }
}