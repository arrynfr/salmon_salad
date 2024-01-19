use core::ptr::write_volatile;

pub unsafe fn serial_init() {

}

pub unsafe fn serial_putchar(c: char) {
    let serial_base = 0x10000000 as *mut u8;
    write_volatile(serial_base, c as u8);
}

pub fn serial_puts(string: &str) {
    for c in string.chars() {
        unsafe {
            serial_putchar(c);
        }
    }
}