use core::fmt::{Result, Write};

use crate::arch;
pub struct StringWriter;

impl Write for StringWriter {
    fn write_str(&mut self, string: &str) -> Result {
        arch::host::serial::serial_puts(string);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        let _ = <$crate::print::StringWriter as core::fmt::Write>::write_fmt(
            &mut $crate::print::StringWriter,
            format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n\r")
    };
    ($($arg:tt)*) => {
        $crate::print!($($arg)*);
        $crate::print!("\n\r")
    }
}
