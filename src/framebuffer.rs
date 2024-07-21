use crate::{println, serial_println};
use multiboot2::{BootInformation, BootInformationHeader};

pub fn init(multiboot_info_ptr: usize) {
    let boot_info = unsafe {
        BootInformation::load(multiboot_info_ptr as *const BootInformationHeader).unwrap()
    };

    let framebuffer_tag = boot_info
        .framebuffer_tag()
        .expect("Could not find framebuffer tag");

    serial_println!("Framebuffer: {:#?}", framebuffer_tag);
}
