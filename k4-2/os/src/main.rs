#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
// TODO
#![feature(const_in_array_repeat_expressions)]

#![feature(alloc_error_handler)]

#[macro_use]
extern crate bitflags;

extern crate alloc;
#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
mod sbi;
mod syscall;
mod task;
mod timer;
mod trap;
mod mm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

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
    }
    clear_bss();

    println!("Hello, world!");
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    error!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    debug!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    trace!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize,
        boot_stack_top as usize
    );
    warn!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
     mm::init();
    println!("[kernel] back to world!");
    mm::heap_test();
    mm::remap_test();
    trap::init();
    //trap::enable_interrupt();   
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();

    panic!("Unreachable in rust_main!");
}
