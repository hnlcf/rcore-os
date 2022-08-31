#![no_std]
#![no_main]

mod console;
mod lang_item;
mod sys;

use crate::console::*;
use crate::sys::*;

#[no_mangle]
extern "C" fn _start() {
    println!("hello world");
    sys_exit(9);
}
