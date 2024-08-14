pub mod frame;

use frame::AreaFrameAllocator;
use multiboot2::{BootInformation, BootInformationHeader};

use crate::paging::active_page_table::ActivePageTable;

pub enum MemoryError {
    FrameAllocationFailed,
}

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

pub fn init<'a>(boot_info: &'a BootInformation) -> AreaFrameAllocator<'a> {
    let multiboot_start = boot_info.start_address() as u64;
    let multiboot_end = (multiboot_start + boot_info.total_size() as u64);

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

    enable_bits();

    // Initialzie 4-level paging
    // let mut active_page_table = paging::init(&mut frame_allocator, &boot_info);

    // Initialize the heap
    // let _ = heap::init(&mut active_page_table, &mut frame_allocator);

    frame_allocator
}
