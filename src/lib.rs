#![no_std]
#![no_main]
mod tests;
mod vga_buffer;

use core::panic::PanicInfo;
use linkme::distributed_slice;
use tests::test_runner;

#[panic_handler]
#[no_mangle]
fn panic_fmt(_info: &PanicInfo) -> ! {
    println!("Panic!");
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_main() {
    #[cfg(testing)]
    test_runner();
    println!("Hello World!");
}

#[distributed_slice(tests::TESTS)]
fn test1() {
    println!("trivial_assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}
