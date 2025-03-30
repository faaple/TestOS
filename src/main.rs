#![no_std]
#![no_main]
#![allow(dead_code)]

mod lang_items;
mod console;
mod syscalls;
mod sbi;

core::arch::global_asm!(include_str!("entry.asm"));

// TODO: understand the syntax
// clear BSS segment
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
fn rust_main() {
    println!("Hello, world!");
    crate::syscalls::sys_exit(9);
    clear_bss();
    crate::sbi::shutdown();
}