use core::{fmt::{Result, Write}, sync::atomic::{AtomicBool, Ordering}};

#[cfg(not(feature = "uefi"))]
use crate::arch;

#[cfg(feature = "uefi")]
use crate::efi;

static PRINT_LOCK: AtomicBool = AtomicBool::new(false);
pub struct StringWriter;

impl StringWriter {
    pub fn aquire_lock() {
        while PRINT_LOCK.compare_exchange(
            false,
            true,
            Ordering::SeqCst,
            Ordering::SeqCst,
        ).is_err() {}
    }

    pub fn release_lock() {
        while PRINT_LOCK.compare_exchange(
            true,
            false,
            Ordering::SeqCst,
            Ordering::SeqCst,
        ).is_err() {}
    }
}

impl Write for StringWriter {
    fn write_str(&mut self, string: &str) -> Result {
        #[cfg(feature = "uefi")]
        efi::efi::output_string(string);
        #[cfg(not(feature = "uefi"))]
        arch::host::serial::serial_puts(string);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        <$crate::print::StringWriter>::aquire_lock();
        let _ = <$crate::print::StringWriter as core::fmt::Write>::write_fmt(
            &mut $crate::print::StringWriter,
            format_args!($($arg)*));
        <$crate::print::StringWriter>::release_lock();
    }
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n\r")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}{}", format_args!($($arg)*), "\r\n");
    }
}
