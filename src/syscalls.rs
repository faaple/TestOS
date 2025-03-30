//! System calls wrappers

/// System call number for exit
const SYSCALL_EXIT: usize = 93;

/// System call number for write
const SYSCALL_WRITE: usize = 64;

/// Perform a system call in RISC-V using *inline assembly*.
/// 
/// `isize` -> the pointer-sized signed integer type
/// 
/// ## What actually happens at Runtime
/// 1. Arguments moved into registers `x10` (a0), `x11` (a1), and `x12` (a2).
/// 2. Syscall ID moved into register `x17` (a7).
/// 3. Execute syscall with the `ecall` instruction.
/// 4. The return value of `ecall` will be placed in `x10` (a0),
/// so we copy it as the return value of our `syscall` function.
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
        );
    }
    ret
}

/// Wrapper for exist system call
pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

/// Wrapper for write system call
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}
