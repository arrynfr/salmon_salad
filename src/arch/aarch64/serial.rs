use core::ptr::write_volatile;

pub unsafe fn serial_init(base_addr: usize) {
    let _ptr = base_addr as *mut u8;
}


pub fn serial_getchar() -> Option<u8> {
    let ptr = 0x09000000 as *mut u8;
    unsafe {
        if ptr.add(0x18).read_volatile() & 1 << 4 == 0 {
            Some(ptr.add(0).read_volatile())
        }
        else {
            None
        }
    }
}

pub unsafe fn serial_putchar(c: char) {
    let serial_base = 0x09000000 as *mut u8;
    write_volatile(serial_base, c as u8);
}

pub fn serial_puts(string: &str) {
    for c in string.chars() {
        unsafe {
            serial_putchar(c);
        }
    }
}