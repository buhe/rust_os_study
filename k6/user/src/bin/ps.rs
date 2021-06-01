#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{list};


#[no_mangle]
pub fn main() -> i32 {
    let mut buffer = [0u8; 32];
    let len_read = list(&mut buffer) as usize;
    let apps = core::str::from_utf8(&buffer[..len_read]).unwrap();
    println!("apps is {}", apps);
   0
}

