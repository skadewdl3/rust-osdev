pub mod area_frame_allocator;
pub mod frame;
pub mod paging;
pub mod tiny_frame_allocator;

use crate::println;
use area_frame_allocator::AreaFrameAllocator;
use frame::FrameAllocator;
use multiboot2::BootInformationHeader;

pub const PAGE_SIZE: u64 = 4096; // 4KB

pub fn init(multiboot_info_ptr: usize) {
    let boot_info = unsafe {
        multiboot2::BootInformation::load(multiboot_info_ptr as *const BootInformationHeader)
            .unwrap()
    };

    let multiboot_start = multiboot_info_ptr as u64;
    let multiboot_end = (multiboot_info_ptr + boot_info.total_size()) as u64;

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

    let memory_areas = boot_info.memory_map_tag().unwrap().memory_areas();

    let mut frame_allocator = AreaFrameAllocator::new(
        kernel_start,
        kernel_end,
        multiboot_start,
        multiboot_end,
        memory_areas,
    );

    println!("{:?}", frame_allocator.allocate_frame());

    paging::init(&mut frame_allocator, &boot_info)
}
