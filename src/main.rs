#![no_std]
#![no_main]

mod lang_items;
mod console;
mod syscalls;

#[unsafe(no_mangle)]
extern "C" fn _start() {
    println!("Hello, world!");
    crate::syscalls::sys_exit(9);
}