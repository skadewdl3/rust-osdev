#![no_std]
#![no_main]
pub mod serial;
pub mod tests;
pub mod vga_buffer;

use core::panic::PanicInfo;
use linkme::distributed_slice;
use tests::{exit_qemu, test_runner, Testable};

#[panic_handler]
#[cfg(not(testing))]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_main() {
    #[cfg(testing)]
    test_runner();
    println!("Hello World!");
}

#[distributed_slice(crate::tests::TESTS)]
fn test1() {
    serial_print!("Test 1");
    assert_eq!(1, 1);
}
