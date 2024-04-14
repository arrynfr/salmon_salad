const CLOCK_GATE: *mut u32 = 0x23b7001e0 as *mut u32;
const PWM: *mut u32 = 0x235044000 as *mut u32;
const PWM_COMMIT: *mut u32 = 0x235044000 as *mut u32;
const STATUS: *mut u32 = 0x235044004 as *mut u32;
const ON_PERIOD: *mut u32 = 0x23504401c as *mut u32;
const OFF_PERIOD: *mut u32 = 0x235044018 as *mut u32;

const FREQUENCY: u32 = 24000000;

pub struct AppleKeyboardBacklight {
    pwm_commit: u32,
    unk1: u32,
    status: u32,
    unk2: u32,
    off_period: u32,
    on_period: u32
}

impl AppleKeyboardBacklight {
    pub fn init() {
        Self::enable_clock();
        Self::set_dutycycle(FREQUENCY, 0);
    }

    pub fn enable_clock() {
        unsafe {
            CLOCK_GATE.write_volatile(0xf);
        }
    }

    pub fn set_dutycycle(on_time: u32, off_time: u32) {
        unsafe {
            Self::enable_clock();
            ON_PERIOD.write_volatile(on_time);
            OFF_PERIOD.write_volatile(off_time);
            PWM_COMMIT.write_volatile(0x4239);
        }
    }
}

