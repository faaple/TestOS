#![no_std]
#![no_main]
#![allow(dead_code)]

mod lang_items;
mod console;
mod syscalls;
mod sbi;

#[unsafe(no_mangle)]
extern "C" fn _start() {
    println!("Hello, world!");
    crate::syscalls::sys_exit(9);
    crate::sbi::shutdown();
}