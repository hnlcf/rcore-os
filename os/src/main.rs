#![no_std]
#![no_main]

mod lang_item;
mod sys;

use crate::console::*;
use crate::sys::*;

#[no_mangle]
extern "C" fn _start() {
    sys_exit(9);
}
