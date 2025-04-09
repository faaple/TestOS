//! Implement the public struct `TrapContext`
use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
/// Trap Context.
/// 
/// Save the physical resources when trap happens.
/// `#[repr(C)]` tells the compiler to lay out this struct like C would.
pub struct TrapContext {
    /// general regs[0..31]
    pub x: [usize; 32],
    /// CSR sstatus (U/S): `SPP` gives the privilege level of the CPU 
    /// right before the trap.
    pub sstatus: Sstatus,
    /// CSR sepc (Supervisor-mode Exception Program Counter): return address
    pub sepc: usize,
}

impl TrapContext {
    /// Set stack pointer to `x_2` reg (`sp` stack pointer)
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    /// Init user application context
    /// 
    /// - Set the previous privilege mode as "user mode" in `sstatus`'s `SPP`.
    /// - Set `sepc` as the entry point of the user application `0x80400000`.
    /// - Set the user stack pointer (at the stack base) in the `TrapContext`.
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read(); // CSR sstatus
        sstatus.set_spp(SPP::User); //previous privilege mode: user mode
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // entry point of app
        };
        cx.set_sp(sp); // app's user stack pointer
        cx // return initial Trap Context of app
    }
}
