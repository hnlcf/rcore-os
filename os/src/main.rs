#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
extern crate lazy_static;

mod batch;
mod console;
mod lang_item;
mod sbi;
mod sync;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, World!");
    panic!("Shutdown machine!");
}

/// Clear bss section
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
