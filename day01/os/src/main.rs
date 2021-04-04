#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
use core::panic::PanicInfo;
use core::fmt::{self, Write};
// mod lang_items;
mod sbi;
use crate::sbi::sbi_call;
global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_SHUTDOWN: usize = 8;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("Panicked at {}:{} {}", location.file(), location.line(), info.message().unwrap());
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown()
}

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
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


// mod sbi;



// const SBI_SHUTDOWN: usize = 93;

// pub fn shutdown() -> ! {
//     sbi_call(SBI_SHUTDOWN, 0, 0, 0);
//     panic!("It should shutdown!");
// }

// #[no_mangle]
// extern "C" fn _start() {
//     shutdown();
// }
// fn syscall(id: usize, args: [usize; 3]) -> isize {
//     let mut ret: isize;
//     unsafe {
//         llvm_asm!("ecall"
//             : "={x10}" (ret)
//             : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]), "{x17}" (id)
//             : "memory"
//             : "volatile"
//         );
//     }
//     ret
// }

// pub fn sys_exit(xstate: i32) -> isize {
//     syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
// }

// const SYSCALL_WRITE: usize = 64;

// pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
//   syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
// }

// struct Stdout;

// impl Write for Stdout {
//     fn write_str(&mut self, s: &str) -> fmt::Result {
//         sys_write(1, s.as_bytes());
//         Ok(())
//     }
// }

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
        print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world! buhe");
    panic!("It should shutdown!");
}

// // fn main() {
//     // println!("Hello, world!");
// // }
// #[no_mangle]
// extern "C" fn _start() {
//     // loop{};
//     println!("Hello, world!");
//     sys_exit(9);
// }