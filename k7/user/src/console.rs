use core::fmt::{self, Write};

const STDIN: usize = 0;
const STDOUT: usize = 1;
const STDERROUT: usize = 2;

use super::{read, write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

struct Stderrout;

impl Write for Stderrout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDERROUT, s.as_bytes());
        Ok(())
    }
}

pub fn error(args: fmt::Arguments) {
    Stderrout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::error(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! errorln {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::error(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
pub fn getchar() -> u8 {
    let mut c = [0u8; 1];
    read(STDIN, &mut c);
    c[0]
}
