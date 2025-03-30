//! SBI (Supervisor Binary Interface) calls wrappers
//! 
//! SBI is the interface between an **operating system** (running is **supervisor mode**, S-mode)
//! and the **firmware/hypervisor** (running in **machine mode**, M-mode).
//! It allows the OS to request privileged operations.

const SBI_SHUTDOWN: usize = 8;

/// General sbi call
/// 
/// Note that `x16` (a6) must be `0` for SBI calls.
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x16") 0,
            in("x17") which,
        );
    }
    ret
}

/// Use sbi call to shutdown the kernel
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shotdown!");
}