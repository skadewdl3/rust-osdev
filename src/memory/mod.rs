pub mod area_frame_allocator;

use multiboot2::BootInformationHeader;

use crate::println;

pub const PAGE_SIZE: usize = 4096; // 4KB

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    fn containing_address(address: usize) -> Frame {
        Frame {
            number: address / PAGE_SIZE,
        }
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

pub fn init(multiboot_info_ptr: usize) {
    let boot_info = unsafe {
        multiboot2::BootInformation::load(multiboot_info_ptr as *const BootInformationHeader)
            .unwrap()
    };

    let multiboot_start = multiboot_info_ptr;
    let multiboot_end = multiboot_info_ptr + boot_info.total_size();

    let kernel_start = boot_info
        .elf_sections()
        .unwrap()
        .map(|s| s.start_address())
        .min()
        .unwrap();
    let kernel_end = boot_info
        .elf_sections()
        .unwrap()
        .map(|s| s.start_address())
        .max()
        .unwrap();
}
