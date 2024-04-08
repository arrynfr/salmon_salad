use core::sync::atomic::AtomicUsize;

const _UARTLCR_H: usize = 0x02C;
const _UARTCR: usize = 0x30;
const _UARTRSR: usize = 0x004;
const UARTFR: usize = 0x018;

const _UARTCR_UART_ENABLE: u8 = 1 << 0;

static SERIAL_BASE:AtomicUsize = AtomicUsize::new(0);

pub fn serial_init(serial_base: *mut u8) {
    SERIAL_BASE.compare_exchange(0, serial_base as usize,
            core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::SeqCst)
            .expect("Serial base already set!");
}

pub fn serial_getchar() -> Option<u8> {
    let serial_base = SERIAL_BASE.load(core::sync::atomic::Ordering::Relaxed) as *mut u8;
    if serial_base != 0 as *mut u8 {
        unsafe {
            if serial_base.add(UARTFR).read_volatile() & 1 << 4 == 0 {
                Some(serial_base.add(0).read_volatile())
            }
            else {
                None
            }
        }
    } else {
        panic!("Serial port not initialized");
    }
}
    
pub unsafe fn serial_putchar(c: char) {
    let serial_base = SERIAL_BASE.load(core::sync::atomic::Ordering::Relaxed) as *mut u8;
    serial_base.write_volatile(c as u8);
}
    
pub fn serial_puts(string: &str) {
    for c in string.chars() {
        unsafe {
            serial_putchar(c);
        }
    }
}