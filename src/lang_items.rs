//! The panic handler

use core::panic::PanicInfo;

#[panic_handler]
/// panic handler
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
