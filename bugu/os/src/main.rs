#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod sbi;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
        fn bugu();
    }
    clear_bss();
    println!("Hello, world!223");
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    error!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    debug!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    trace!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    warn!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    info!("bugu is here {:#x}", bugu as usize);
    panic!("Shutdown machine!");
}
