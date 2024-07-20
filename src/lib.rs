#![no_std]
#![no_main]
#![feature(abi_x86_interrupt, naked_functions, lang_items)]
#![allow(internal_features)]

pub mod interrupts;
pub mod memory;
pub mod panic;
pub mod serial;
pub mod tests;
pub mod vga_buffer;

use linkme::distributed_slice;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_info_ptr: usize) {
    println!("Hello World!");

    paging::init(multiboot_info_ptr);
    interrupts::init();

    #[cfg(testing)]
    test_runner();

    println!("It did not crash");
    loop {}
}

#[distributed_slice(crate::tests::TESTS)]
fn test1() {
    serial_print!("Test 1");
    assert_eq!(1, 1);
}
