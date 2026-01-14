use core::sync::atomic::{AtomicUsize, Ordering};

const UARTDR: usize   = 0x000;
const UARTRSR: usize  = 0x004;
const UARTFR: usize   = 0x018;
const UARTLCR_H: usize = 0x02C;
const UARTCR: usize   = 0x030;

const UARTFR_TXFF: u32 = 1 << 5; // TX FIFO full
const UARTFR_RXFE: u32 = 1 << 4; // RX FIFO empty

const UARTCR_UARTEN: u32 = 1 << 0;
const UARTCR_TXE: u32 = 1 << 8;
const UARTCR_RXE: u32 = 1 << 9;

const UARTLCR_H_WLEN_8: u32 = 0b11 << 5;
const UARTLCR_H_FEN: u32 = 1 << 4;

/// Address of the serial port
static SERIAL_BASE:AtomicUsize = AtomicUsize::new(0);

/// Assign the address of the serial port
pub unsafe fn serial_init(serial_base: *mut u8) {
    assert!(!serial_base.is_null());
    SERIAL_BASE
        .compare_exchange(
            0,
            serial_base as usize,
            Ordering::Relaxed,
            Ordering::Relaxed,
        )
        .expect("Serial base already set");
    let base = serial_base as *mut u32;
    // Disable UART
    base.add(UARTCR / 4).write_volatile(0);
    // Clear errors
    base.add(UARTRSR / 4).write_volatile(0);
    // 8N1, FIFO enabled
    base.add(UARTLCR_H / 4)
        .write_volatile(UARTLCR_H_WLEN_8 | UARTLCR_H_FEN);
    // Enable RX, TX, UART
    base.add(UARTCR / 4)
        .write_volatile(UARTCR_UARTEN | UARTCR_TXE | UARTCR_RXE);
}


/// Get character from serial port
/// 
/// This is a noop if the serial port
/// address is null. 
pub fn serial_getchar() -> Option<u8> {
    let base = SERIAL_BASE.load(Ordering::Relaxed);
    if base == 0 {
        return None;
    }

    let base = base as *mut u32;

    unsafe {
        // RX FIFO empty?
        if base.add(UARTFR / 4).read_volatile() & UARTFR_RXFE != 0 {
            None
        } else {
            Some(base.add(UARTDR / 4).read_volatile() as u8)
        }
    }
}


/// Write character to serial port
/// 
/// This is a noop if the serial port
/// address is null. 
pub fn serial_putchar(c: u8) {
    let base = SERIAL_BASE.load(Ordering::Relaxed);
    if base == 0 {
        return;
    }

    let base = base as *mut u32;

    unsafe {
        // Wait until TX FIFO not full
        while base.add(UARTFR / 4).read_volatile() & UARTFR_TXFF != 0 {
            core::hint::spin_loop();
        }

        base.add(UARTDR / 4).write_volatile(c as u32);
    }
}


/// Write string to serial port
/// 
/// This is a noop if the serial port
/// address is null. 
pub fn serial_puts(string: &str) {
    for c in string.bytes() {
        if c == b'\n' {
            serial_putchar(b'\r');
        }
        unsafe {
            serial_putchar(c);
        }
    }
}