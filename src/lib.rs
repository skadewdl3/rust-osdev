#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]

pub mod interrupts;
pub mod serial;
pub mod tests;
pub mod vga_buffer;

use core::panic::PanicInfo;
use linkme::distributed_slice;

#[panic_handler]
#[cfg(not(testing))]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_main(multiboot_info_ptr: usize) {
    println!("Hello World!");

    interrupts::init();
    unsafe { core::arch::asm!("mov dx, 0", "div dx") }
    #[cfg(testing)]
    test_runner();

    let x = (1u64, 2u64, 3u64);
    let y = Some(x);
    for i in (0..100).map(|z| (z, z - 1)) {}

    println!("It did not crash");
    loop {}
}

#[distributed_slice(crate::tests::TESTS)]
fn test1() {
    serial_print!("Test 1");
    assert_eq!(1, 1);
}
