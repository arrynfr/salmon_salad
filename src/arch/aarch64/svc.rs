pub fn handle_svc(svc_number: u16) {
    match svc_number {
        0x1337 => { println!("EC_SVC from userspace: {:#x}!", svc_number);}
        _ => {println!("EC_SVC exception with number {:#x}!", svc_number);}
    }
}