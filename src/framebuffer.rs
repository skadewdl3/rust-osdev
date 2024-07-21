use crate::{println, serial_println};
use multiboot2::{BootInformation, BootInformationHeader};

/*
TODO: Look into the following for implementing a frame buffer

1. https://github.com/rust-osdev/bootloader/blob/main/uefi/src/main.rs#L462

*/

pub fn init(multiboot_info_ptr: usize) {
    let boot_info = unsafe {
        BootInformation::load(multiboot_info_ptr as *const BootInformationHeader).unwrap()
    };

    let framebuffer_tag = boot_info
        .framebuffer_tag()
        .expect("Could not find framebuffer tag");

    serial_println!("Framebuffer: {:#?}", framebuffer_tag);
}
