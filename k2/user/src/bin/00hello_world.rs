#![no_std]
#![no_main]
#![feature(llvm_asm)]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Hello, world!buhe app");
    unsafe {
        llvm_asm!("sret");
    }
    0
}