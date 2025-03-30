//! The main module and entrypoint
//! 
//! Various facilities of the kernels are implemented as submodules.

#![no_std]
#![no_main]
#![allow(dead_code)]

mod lang_items;
mod console;
mod syscalls;
mod sbi;

core::arch::global_asm!(include_str!("entry.asm"));

/// Clear BSS segment
/// 
/// It first declares external symbols. `extern "C"` block tells Rust that these symbols
/// come from outside Rust (in this case, the linker script `linker.ld`).
/// `sbss` are `ebss` are both function pointers, but we cast them into `usize`.
/// 
/// `a as *mut u8` converts the address to a pointer to a mutable `u8` data.
/// Note that each address store a 8-bit (byte) data.
/// 
/// `.write_volatile(0)` writes `0` to the memory location
/// and ensure the compiler does not optimize it away.
fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8). write_volatile(0)}
    });
}

#[unsafe(no_mangle)]
/// The entry point
fn rust_main() {
    clear_bss();
    println!("Hello, world!");
    crate::sbi::shutdown();
}
