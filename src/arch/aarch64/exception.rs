use core::arch::asm;

use super::{driver::gicv3, platform::*};

const EC_UNK:    u8 = 0b00_00_00;
const _EC_WF:    u8 = 0b00_00_01;
const _EC_FNCT:  u8 = 0b00_01_11;
const _EC_LS64:  u8 = 0b00_10_10;
const _EC_BTI:   u8 = 0b00_11_01;
const _EC_ILLE:  u8 = 0b00_11_10;
const _EC_S128:  u8 = 0b01_01_00;
const EC_SVC:    u8 = 0b01_01_01;
const _EC_HVC:   u8 = 0b01_01_10;
const _EC_MSRT:  u8 = 0b01_10_00;
const _EC_SVE:   u8 = 0b01_10_01;
const _EC_TME:   u8 = 0b01_10_11;
const _EC_PAC:   u8 = 0b01_11_00;
const _EC_SME:   u8 = 0b01_11_01;
const EC_IABTL:  u8 = 0b10_00_00; // Used for MMU faults for instruction access
const EC_IABT:   u8 = 0b10_00_01; // Used for MMU faults for instruction access
const EC_PCAL:  u8 = 0b10_00_10;
const EC_DABTL:  u8 = 0b10_01_00; // Used for MMU faults for data access
const EC_DABT:   u8 = 0b10_01_01; // Used for MMU faults for data access
const _EC_SPAL:  u8 = 0b10_01_10;
const _EC_MOPS:  u8 = 0b10_01_11;
const _EC_FPE:   u8 = 0b10_11_00;
const _EC_GCS:   u8 = 0b10_11_01;
const _EC_SERR:  u8 = 0b10_11_11;
const _EC_BKPTL: u8 = 0b11_00_00;
const _EC_BKPT:  u8 = 0b11_00_01;
const _EC_SSTPL: u8 = 0b11_00_10;
const _EC_SSTP:  u8 = 0b11_00_11;
const _EC_WTPTL: u8 = 0b11_01_00;
const _EC_WTPT:  u8 = 0b11_01_01;
const EC_BRK:    u8 = 0b11_11_00;
const _EC_EBEP:  u8 = 0b11_11_01;

/*enum ExceptionCause {
 EC_UNK = 0b00_00_00,
 _EC_WF = 0b00_00_01,
 _EC_FNCT = 0b00_01_11,
 _EC_LS64 = 0b00_10_10,
 _EC_BTI = 0b00_11_01,
 _EC_ILLE = 0b00_11_10,
 _EC_S128 = 0b01_01_00,
 EC_SVC = 0b01_01_01,
 _EC_MSRT = 0b01_10_00,
 _EC_SVE = 0b01_10_01,
 _EC_TME = 0b01_10_11,
 _EC_PAC = 0b01_11_00,
 _EC_SME = 0b01_11_01,
 EC_IABTL = 0b10_00_00,
 EC_IABT = 0b10_00_01,
 _EC_PCAL = 0b10_00_10,
 EC_DABTL = 0b10_01_00,
 EC_DABT = 0b10_01_01,
 _EC_SPAL = 0b10_01_10,
 _EC_MOPS = 0b10_01_11,
 _EC_FPE = 0b10_11_00,
 _EC_GCS = 0b10_11_01,
 _EC_SERR = 0b10_11_11,
 _EC_BKPTL = 0b11_00_00,
 _EC_BKPT = 0b11_00_01,
 _EC_SSTPL = 0b11_00_10,
 _EC_SSTP = 0b11_00_11,
 _EC_WTPTL = 0b11_01_00,
 _EC_WTPT = 0b11_01_01,
 _EC_BRK = 0b11_11_00,
 _EC_EBEP = 0b11_11_01
}*/

// Credits to: https://krinkinmu.github.io/2021/01/10/aarch64-interrupt-handling.html
#[derive(Debug)]
#[repr(C)]
pub struct ExceptionFrame {
    x0: u64,
    x1: u64,
    x2: u64,
    x3: u64,
    x4: u64,
    x5: u64,
    x6: u64,
    x7: u64,
    x8: u64,
    x9: u64,
    x10: u64,
    x11: u64,
    x12: u64,
    x13: u64,
    x14: u64,
    x15: u64,
    x16: u64,
    x17: u64,
    x18: u64,
    fp: u64,
    lr: u64,
    elr: u64,
    esr: u64,
    far: u64
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn exception_handler(frame: *mut ExceptionFrame) {
    unsafe { exception(&mut *frame) }
}

fn exception(frame: &mut ExceptionFrame) {
    let ec: u8 = (frame.esr >> 26 & 0b111111) as u8;
    //let instruction_length: u8 = (frame.esr >> 25 & 0b1) as u8; // 0x0:16bit / 0x1:32bit
    match ec {
        EC_UNK => {
            panic!("Unknown exception!\r\n{:#x?}", frame);
        }
        EC_DABT | EC_DABTL => {
            println!("Data abort with ISS: {}", frame.esr & 0xFFFF);
            panic!("EC_DABT exception!\r\n{:#x?}", frame);
        }
        EC_IABT | EC_IABTL => {
            panic!("EC_IABT exception!\r\n{:#x?}", frame);
        }
        EC_BRK => {
            println!("Software break point");
            loop {}
        }
        EC_SVC => {
            let svc_number = (frame.esr & 0xFFFF) as u16;
            if svc_number == 0x0db9 {
                println!("{:#x?}", frame);
            } else {
                super::svc::handle_svc(svc_number);
            }
        }
        EC_PCAL => {
            panic!("PC unalinged");
        }
        _ => { panic!("Unknown EC: {:b}", ec); }
    }
}

#[no_mangle]
pub extern "C" fn irq_handler(frame: *mut ExceptionFrame) {
    unsafe { irq(&mut *frame) }
}

fn irq(frame: &mut ExceptionFrame) {
    let intid: u64;
    unsafe { asm!("mrs {:x}, ICC_IAR1_EL1", out(reg) intid) };
    match intid {
        0x1e => { handle_timer_irq() },
        0x24 => { handle_pci_intA() },
        _ => { println!("Got unknown interrupt 0x{:x?}", intid); }
    }
    gicv3::GIC::acknowledge_interrupt(intid);
}

fn handle_timer_irq() {
    enable_timer_interrupt(500);
}

fn handle_pci_intA() {
    
}

#[no_mangle]
fn unhandled_exception_vector(frame: *mut ExceptionFrame) -> ! {
    let intid: u64;
    unsafe { asm!("mrs {:x}, ICC_IAR1_EL1", out(reg) intid) };
    gicv3::GIC::acknowledge_interrupt(intid);
    disable_all_interrupts();
    
    println!("Interrupt: {intid:x?}");
    unsafe {
        let ec: u8 = ((*frame).esr >> 26 & 0b111111) as u8;
        panic!("Jump to unhandled exception vector!\r\n{:#x?}{:#b}", *frame, ec);
    }
}