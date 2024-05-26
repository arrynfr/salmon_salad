#[cfg(not(feature = "uefi"))]
pub mod boot;
pub mod platform;
pub mod driver;