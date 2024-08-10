pub mod area_frame_allocator;
pub mod frame;
pub mod paging;
pub mod tiny_frame_allocator;

use crate::println;
use area_frame_allocator::AreaFrameAllocator;
use frame::FrameAllocator;
use multiboot2::BootInformationHeader;

pub const PAGE_SIZE: u64 = 4096; // 4KB

fn enable_bits() {
    // Enable nxe bit in the efer register
    // This bit is set to prevent the execution of code on the stack
    use x86_64::registers::model_specific::Efer;

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = Efer::read_raw();
        // let efer = rdmsr(IA32_EFER);
        // wrmsr(IA32_EFER, efer | nxe_bit);
        Efer::write_raw(efer | nxe_bit);
    }

    // Enable write protect bit in the cr0 register

    use x86_64::registers::control::{Cr0, Cr0Flags};
    unsafe { Cr0::write(Cr0Flags::all() | Cr0Flags::WRITE_PROTECT) }
}

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

    enable_bits();
    paging::init(&mut frame_allocator, &boot_info)
}
