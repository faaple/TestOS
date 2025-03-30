#![no_std]
#![no_main]
#![allow(dead_code)]

mod lang_items;
mod console;
mod syscalls;
mod sbi;

core::arch::global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
fn rust_main() {
    println!("Hello, world!");
    crate::syscalls::sys_exit(9);
    crate::sbi::shutdown();
}