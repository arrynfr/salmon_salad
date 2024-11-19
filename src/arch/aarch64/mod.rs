//! Architecture dependent internal low level functions.
//! 
//! These functions should not be used anywhere outside of 
//! the aarch64 module as they are strictly processor dependent 
//! and only make sense in the context of aarch64.
//! Documentation for the aarch64 architecture can be found under:
//! <https://developer.arm.com/documentation/ddi0487/latest/>
pub mod boot;
pub mod platform;
pub mod driver;
pub mod cpu;
pub mod exception;
pub mod svc;