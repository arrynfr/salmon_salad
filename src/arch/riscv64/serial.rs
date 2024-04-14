use core::sync::atomic::AtomicUsize;

/// Address of the serial port
static SERIAL_BASE:AtomicUsize = AtomicUsize::new(0);


/// Assign the address of the serial port
pub fn serial_init(serial_base: *mut u8) {
    SERIAL_BASE.compare_exchange(0, serial_base as usize,
            core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::SeqCst)
            .expect("Serial base already set!");
}

/// Get character from serial port
/// 
/// This is a noop if the serial port
/// address is null. 
pub fn serial_getchar() -> Option<u8> {
    let serial_base = SERIAL_BASE.load(core::sync::atomic::Ordering::Relaxed) as *mut u8;
    if serial_base != 0 as *mut u8 {
        unsafe {
            if serial_base.add(5).read_volatile() & 1 == 0 {
                return None
            } else { return Some(serial_base.add(0).read_volatile()) }
        }
    }
    None
}

/// Write character to serial port
/// 
/// This is a noop if the serial port
/// address is null. 
pub unsafe fn serial_putchar(c: char) {
    let serial_base = SERIAL_BASE.load(core::sync::atomic::Ordering::Relaxed) as *mut u8;
    if serial_base != 0 as *mut u8 {
        serial_base.write_volatile(c as u8);
    }
}

/// Write string to serial port
/// 
/// This is a noop if the serial port
/// address is null. 
pub fn serial_puts(string: &str) {
    for c in string.chars() {
        unsafe {
            serial_putchar(c);
        }
    }
}