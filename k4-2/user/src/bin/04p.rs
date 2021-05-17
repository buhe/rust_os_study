#![no_std]
#![no_main]
#![feature(llvm_asm)]

#[macro_use]
extern crate user_lib;

#[no_mangle]
unsafe fn main() -> i32 {
    let s = (0x8002b032 as *mut u8).read_volatile();
    println!("s = {}", s);
    0
}
