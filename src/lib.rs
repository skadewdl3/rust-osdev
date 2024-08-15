#![no_std]
#![no_main]
#![feature(
    abi_x86_interrupt,
    naked_functions,
    lang_items,
    ptr_internals,
    allocator_api,
    const_mut_refs
)]
#![allow(internal_features)]

#[macro_use]
extern crate alloc;

#[macro_use]
pub mod interrupts;
pub mod framebuffer;
pub mod heap;
pub mod logger;
pub mod memory;
pub mod paging;
pub mod panic;
pub mod serial;

#[allow(unused_imports)]
#[macro_use]
pub mod tests;

use framebuffer::{color::Color, WRITER};
use multiboot2::BootInformationHeader;
use x86_64::instructions::hlt;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_info_ptr: usize) {
    // Parse the multiboot information header passed by grub
    let boot_info = unsafe {
        multiboot2::BootInformation::load(multiboot_info_ptr as *const BootInformationHeader)
            .unwrap()
    };

    // Initialize interrupts
    interrupts::init();

    // Initialize frame buffer
    framebuffer::init(&boot_info);

    // Create a frame allocator, and setup paging and heap
    let mut frame_allocator = memory::init(&boot_info);
    let mut active_page_table = paging::init(&mut frame_allocator, &boot_info);
    heap::init(&mut active_page_table, &mut frame_allocator);

    framebuffer::fill_bg();

    #[cfg(testing)]
    tests::test_runner();

    println!("It did not crash");

    hlt_loop();
}

fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt()
    }
}
