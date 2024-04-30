use core::{fmt::{Result, Write}, ptr, sync::atomic::{AtomicBool, Ordering}};
use crate::{user::graphics::console::GfxConsole, KERNEL_STRUCT};
use crate::arch;

#[cfg(feature = "uefi")]
use crate::efi;


static PRINT_LOCK: AtomicBool = AtomicBool::new(false);
static DBG_LOCK: AtomicBool = AtomicBool::new(false);
pub struct StringWriter;
pub struct DbgWriter;

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

        let ks = KERNEL_STRUCT.load(Ordering::SeqCst);
        if ks != ptr::null_mut() {
            GfxConsole::_aquire();
            unsafe {
                match &mut (*ks).console {
                    Some(c) => {c.write(string)}
                    None => {}
                }
            }
            GfxConsole::_release();
        }
        Ok(())
    }
}

impl Write for DbgWriter {
    fn write_str(&mut self, string: &str) -> Result {
        arch::host::driver::serial::serial_puts(string);
        Ok(())
    }
}

impl DbgWriter {
    pub fn aquire_lock() {
        while DBG_LOCK.compare_exchange(
            false,
            true,
            Ordering::SeqCst,
            Ordering::SeqCst,
        ).is_err() {}
    }

    pub fn release_lock() {
        while DBG_LOCK.compare_exchange(
            true,
            false,
            Ordering::SeqCst,
            Ordering::SeqCst,
        ).is_err() {}
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        let irbits = crate::arch::host::platform::get_interrupt_mask();
        crate::arch::host::platform::disable_all_interrupts();
        <$crate::print::StringWriter>::aquire_lock();
        let _ = <$crate::print::StringWriter as core::fmt::Write>::write_fmt(
            &mut $crate::print::StringWriter,
            format_args!($($arg)*));
        <$crate::print::StringWriter>::release_lock();
        $crate::dbg!("{}", format_args!($($arg)*));
        crate::arch::host::platform::set_interrupt_mask(irbits);
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

/// For printing to serial console only.
/// This might later be replaced with a kernel
/// ring buffer akin to dmesg, when dynamic
/// allocations are working.
#[macro_export]
macro_rules! dbg {
    ($($arg:tt)*) => {
        <$crate::print::DbgWriter>::aquire_lock();
        let _ = <$crate::print::DbgWriter as core::fmt::Write>::write_fmt(
            &mut $crate::print::DbgWriter,
            format_args!($($arg)*));
        <$crate::print::DbgWriter>::release_lock();
    }
}
