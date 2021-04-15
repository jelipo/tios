use core::fmt;
use core::fmt::Write;

use crate::sbi::sbi_call;

const SBI_CONSOLE_PUTCHAR: usize = 1;

pub fn console_putchar(c: usize) {
    syscall(SBI_CONSOLE_PUTCHAR, [c, 0, 0]);
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        //sys_write(STDOUT, s.as_bytes());
        for c in s.chars() {
            console_putchar(c as usize);
        }
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

#[macro_export]
macro_rules! sys_info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(concat!("\x1b[94m[INFO] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?)
        );
    }
}

#[macro_export]
macro_rules! sys_error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(concat!("\x1b[91m[ERROR] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?)
        );
    }
}

#[macro_export]
macro_rules! sys_warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(concat!("\x1b[93m[WARN] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?)
        );
    }
}


fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]), "{x17}" (id)
            : "memory"
            : "volatile"
        );
    }
    ret
}