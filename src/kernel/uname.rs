use core::ascii;

const SYSTEM_NAME: &str = "salmon_salad";
const NODE_NAME: &str = "unnamed";
const RELEASE: &str = "0.1";
const VERSION: &str = "0.1";
const MACHINE: &str = "aarch64";

#[derive(Debug)]
pub struct Utsname {
    pub sysname:[ascii::Char; 128],
    pub nodename:[ascii::Char; 128],
    pub release:[ascii::Char; 128],
    pub version:[ascii::Char; 128],
    pub machine:[ascii::Char; 128]
}

impl Default for Utsname {
    fn default() -> Utsname {
        Utsname {
            sysname: [ascii::Char::default(); 128],
            nodename:[ascii::Char::default(); 128],
            release:[ascii::Char::default(); 128],
            version:[ascii::Char::default(); 128],
            machine:[ascii::Char::default(); 128]
        }
    }
}

pub fn uname(utsname: &mut Utsname) {
    for (idx,ch) in SYSTEM_NAME.chars().enumerate() {
        utsname.sysname[idx] = ch.as_ascii().unwrap();
    }
    for (idx,ch) in NODE_NAME.chars().enumerate() {
        utsname.nodename[idx] = ch.as_ascii().unwrap();
    }
    for (idx,ch) in RELEASE.chars().enumerate() {
        utsname.release[idx] = ch.as_ascii().unwrap();
    }
    for (idx,ch) in VERSION.chars().enumerate() {
        utsname.version[idx] = ch.as_ascii().unwrap();
    }
    for (idx,ch) in MACHINE.chars().enumerate() {
        utsname.machine[idx] = ch.as_ascii().unwrap();
    }
}