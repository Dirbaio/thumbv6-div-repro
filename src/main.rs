#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn div(left: u32, right: u32) -> u32 {
    left / right
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
