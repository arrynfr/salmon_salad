use core::ptr::write_volatile;

pub unsafe fn serial_init(base_addr: usize) {
    let _ptr = base_addr as *mut u8;
}


pub fn serial_getchar() -> Option<u8> {
    let ptr = 0x10000000 as *mut u8;
    unsafe {
        if ptr.add(5).read_volatile() & 1 == 0 {
            None
        }
        else {
            Some(ptr.add(0).read_volatile())
        }
    }
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