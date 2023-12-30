//! Serial output related code

use super::platform::{outb, inb};

const SERIAL_PORT1: u16 = 0x3F8;
const _SERIAL_PORT2: u16 = 0x2F8;
const _SERIAL_PORT3: u16 = 0x3E8;
const _SERIAL_PORT4: u16 = 0x2E8;
const _SERIAL_PORT5: u16 = 0x5F8;
const _SERIAL_PORT6: u16 = 0x4F8;
const _SERIAL_PORT7: u16 = 0x5E8;
const _SERIAL_PORT8: u16 = 0x4E8;

const DATA_REGISTER: u16 = 0;
const BAUD_DIVISOR_LO: u16 = 0; // With DLAB = 1
const INTERRUPT_ENABLE: u16 = 1;
const BAUD_DIVISOR_HI: u16 = 1; // With DLAB = 1
const IDFK_WHAT_THIS_DOES: u16 = 2;
const LINE_CONTROL_REGISTER: u16 = 3;
const MODEM_CONTROL_REGISTER: u16 = 4;
const _LINE_STATUS_REGISTER: u16 = 5;
const _MODEM_STATUS_REGISTER: u16 = 6;
const _SCRATCH_REGISTER: u16 = 7;

pub unsafe fn serial_init() {
    let port: u16 = SERIAL_PORT1;
    let looptest: u8;
    outb(port+INTERRUPT_ENABLE, 0x00);
    outb(port+LINE_CONTROL_REGISTER, 0x80);
    outb(port+BAUD_DIVISOR_LO, 0x0C);
    outb(port+BAUD_DIVISOR_HI, 0x00);
    outb(port+LINE_CONTROL_REGISTER, 0x03);
    outb(port+IDFK_WHAT_THIS_DOES, 0xC7);
    outb(port+MODEM_CONTROL_REGISTER, 0x0B);
    outb(port+MODEM_CONTROL_REGISTER, 0x1E);
    outb(port+DATA_REGISTER, 0xDE as u8);
    looptest = inb(port+DATA_REGISTER);
    if looptest == 0xDE {
        outb(port+MODEM_CONTROL_REGISTER, 0x0F);
    } else { panic!("No serial init!");}
}

pub unsafe fn serial_putchar(c: char) {
    let port: u16 = SERIAL_PORT1;
    outb(port+DATA_REGISTER, c as u8);
}

pub unsafe fn serial_puts(string: &str) {
    for c in string.chars() {
        unsafe {
            serial_putchar(c);
        }
    }
}