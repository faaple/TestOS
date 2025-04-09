//! Trap handling functionality
//! 
//! `trap.S` has the assembly code for 
//! **context saving** (denoted as function by the symbol `__alltraps`) 
//! and **context recovery** (denoted as function by the symbol `__restore`).

mod context;

use crate::batch::run_next_app;
use crate::syscall::syscall;

use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap},
    stval, stvec,
};

core::arch::global_asm!(include_str!("trap.S"));

/// initialize CSR `stvec` as  trap handler entry point, i.e., `__alltraps`.
pub fn init() {
    unsafe extern "C" {
        unsafe fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[unsafe(no_mangle)]
/// handle an interrupt, exception, or system call from user space
/// 
/// Through `__alltraps` trap handler entry point, context is saved 
/// by the assembly code in `trap.S`.
/// Here, the trap is then dispatched and handled.
/// 
/// The function argument is a pointer (i.e., mutable borrow) to `TrapContext`.
/// By RISC-V calling convention, the function arguments are passed through registers `x10`-`x17` (`a0`-`a7`);
/// and the function return value is also passed through register `x10` (`a0`). 
/// 
/// Therefore, after handling the trap, `a0` register still point to the `TrapContext`, i.e., the stack top.
/// This function will return and continue to execute `__restore` in `trap.S`.
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read(); // get trap cause
    let stval = stval::read(); // get extra value
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_app();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}

pub use context::TrapContext;
