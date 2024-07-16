#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
fn panic_fmt(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_main() {}
