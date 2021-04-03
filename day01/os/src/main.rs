#![no_std]
#![no_main]

mod lang_items;

// fn main() {
    // println!("Hello, world!");
// }
#[no_mangle]
extern "C" fn _start() {
    loop{};
}