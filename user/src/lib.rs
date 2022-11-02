#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

pub mod console;
mod lang_term;
mod syscall;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("Unreachable after sys_exit!");
}

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|byte| unsafe {
        (byte as *mut u8).write_volatile(0);
    });
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main");
}

pub fn exit(xstate: i32) -> isize {
    syscall::sys_exit(xstate)
}

pub fn write(fd: usize, buffer: &[u8]) -> isize {
    syscall::sys_write(fd, buffer)
}
