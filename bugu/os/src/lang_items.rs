// use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    panic!("It should shutdown!");
}
