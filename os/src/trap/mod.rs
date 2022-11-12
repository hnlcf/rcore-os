pub mod context;

use core::arch::global_asm;

use riscv::register::scause::Exception;
use riscv::register::utvec::TrapMode;
use riscv::register::{scause, stval, stvec};

use crate::batch::run_next_app;
use crate::println;
use crate::syscall::syscall;

use self::context::TrapContext;

// `trap.S` saves all asm code about saving and recovering the
// context of trap.
//
// `__alltraps`: save trap context on kernel stack
// `__recover`: recover trap context from kernel stack
global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }

    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();

    match scause.cause() {
        scause::Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.reg[10] = syscall(cx.reg[17], [cx.reg[10], cx.reg[11], cx.reg[12]]) as usize;
        },
        scause::Trap::Exception(Exception::StoreFault)
        | scause::Trap::Exception(Exception::StoreMisaligned) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            run_next_app();
        },
        scause::Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] Illegalinstruction in application, kernel killed it.");
            run_next_app();
        },
        _ => panic!(
            "Unsupported trap {:?}, stval = {:#x}!",
            scause.cause(),
            stval
        ),
    }

    cx
}
