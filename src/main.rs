//! The main module and entrypoint
//! 
//! Various facilities of the kernels are implemented as submodules.

#![no_std]
#![no_main]
#![allow(dead_code)]

#[macro_use]
extern crate log;

use log::*;

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod logging;
mod batch;

pub mod syscall;
pub mod sync;
pub mod trap;

core::arch::global_asm!(include_str!("entry.asm"));
core::arch::global_asm!(include_str!("link_app.S"));

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
    unsafe extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack(); // stack lower bound
        fn boot_stack_top(); // stack top
    }
    clear_bss();
    logging::init();
    println!("[kernel] Hello, world!");
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize, etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top={:#x}, bottom={:#x}",
        boot_stack_top as usize, boot_stack as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    trap::init();
    batch::init();
    batch::run_next_app();
}
