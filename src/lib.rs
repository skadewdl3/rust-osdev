#![no_std]
#![no_main]
#![feature(
    abi_x86_interrupt,
    naked_functions,
    lang_items,
    ptr_internals,
    allocator_api
)]
#![allow(internal_features)]

#[macro_use]
extern crate alloc;

pub mod framebuffer;
pub mod interrupts;
pub mod memory;
pub mod panic;
pub mod serial;
pub mod vga_buffer;

#[cfg(testing)]
#[allow(unused_imports)]
pub mod tests;

use alloc::{boxed::Box, rc::Rc, vec::Vec};
use core::arch::asm;
use linkme::distributed_slice;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_info_ptr: usize) {
    println!("Hello World!");

    interrupts::init();
    memory::init(multiboot_info_ptr);

    // TODO: Get framebuffer working
    // framebuffer::init(multiboot_info_ptr);

    // Testing the heap
    let heap_value = Box::new(42);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    #[cfg(testing)]
    tests::test_runner();

    println!("It did not crash");

    loop {}
}
