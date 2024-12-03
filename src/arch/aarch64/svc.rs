use crate::kernel::uname::*;

use super::exception::ExceptionFrame;

const SYSCALL_UNAME: u16 = 0x0;
const SYSCALL_PRINT: u16 = 0x1;

pub fn handle_svc(svc_number: u16, frame: &mut ExceptionFrame) {
    match svc_number {
        SYSCALL_UNAME => {
            unsafe {
                uname((frame.x0 as *mut Utsname).as_mut().unwrap());
            }
        }
        SYSCALL_PRINT => {
            unsafe {
                println!("{:?}", (frame.x0 as *const Utsname).as_ref().unwrap());
            }
        }
        0x1337 => { println!("EC_SVC from userspace: {:#x}!", svc_number);}
        _ => {println!("EC_SVC exception with number {:#x}!", svc_number);}
    }
}