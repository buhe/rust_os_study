#![no_std]
#![no_main]
#![feature(llvm_asm)]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("[1]Into Test store_fault, we will insert an invalid store operation...");
    println!("[1]Kernel should kill this application!");
    unsafe { (0x0 as *mut u8).write_volatile(0); }
    0
}