#![no_std]
#![no_main]
#![feature(llvm_asm)]

#[macro_use]
extern crate bugu_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Hello, world!buhe app");
    0
}