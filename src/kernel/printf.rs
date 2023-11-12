use core::fmt::{self, Write};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn _print(args: fmt::Arguments) {
    let mut writer = UartWriter {};
    writer.write_fmt(args).expect("_print: error");
}


struct UartWriter;

impl Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            // @TODO send one character to the uart
        }
        Ok(())
    }
}