use core::ptr::write_volatile;

const _UARTLCR_H: usize = 0x02C;
const _UARTCR: usize = 0x30;
const _UARTRSR: usize = 0x004;
const UARTFR: usize = 0x018;

const _UARTCR_UART_ENABLE: u8 = 1 << 0;

pub unsafe fn serial_init(base_addr: usize) {
    let _ptr = base_addr as *mut u8;
    //let uartcr = ptr.add(UARTLCR_H).read_volatile();
    //println!("Uart register is: {uartcr:x?}");
    //ptr.add(UARTCR).write_volatile(uartcr & !(UARTCR_UART_ENABLE));
}


pub fn serial_getchar() -> Option<u8> {
    let ptr = 0x09000000 as *mut u8;
    unsafe {
        if ptr.add(UARTFR).read_volatile() & 1 << 4 == 0 {
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