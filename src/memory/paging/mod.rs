pub mod active_page_table;
pub mod entry;
pub mod inactive_page_table;
pub mod mapper;
pub mod page;
pub mod table;

use super::frame::{Frame, FrameAllocator};
use crate::println;
use active_page_table::ActivePageTable;
use entry::EntryFlags;
use inactive_page_table::InactivePageTable;
use multiboot2::BootInformation;
use page::{Page, TemporaryPage};

pub const PAGE_SIZE: u64 = 4096; // 4KB
const PAGE_TABLE_ENTRY_COUNT: usize = 512; // 512 * 8 bytes = 4KB

pub type PhysAddr = usize;
pub type VirtAddr = usize;

pub fn remap_kernel<A: FrameAllocator>(allocator: &mut A, boot_info: &BootInformation) {
    let mut temporary_page = TemporaryPage::new(Page::new(0xcafebabe), allocator);

    let mut active_table = unsafe { ActivePageTable::new() };
    let mut new_table = {
        let frame = allocator.allocate_frame().expect("no more frames");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };

    active_table.with(&mut new_table, &mut temporary_page, |mapper| {
        let elf_sections = boot_info.elf_sections().unwrap();

        // Identity map elf sections
        for section in elf_sections {
            use self::entry::EntryFlags;

            if !section.is_allocated() {
                // section is not loaded to memory
                continue;
            }
            crate::serial_println!(
                "Section start addr is: {:#x}, size is: {:#x}, % is {}",
                section.start_address(),
                section.size(),
                section.start_address() % 4096
            );
            assert!(
                section.start_address() % PAGE_SIZE == 0,
                "sections need to be page aligned"
            );

            crate::serial_println!(
                "mapping section at addr: {:#x}, size: {:#x}",
                section.start_address(),
                section.size()
            );

            let flags = EntryFlags::from_elf_section_flags(&section);
            let start_frame = Frame::containing_address(section.start_address());
            let end_frame = Frame::containing_address(section.end_address() - 1);
            for frame in Frame::range_inclusive(start_frame, end_frame) {
                mapper.identity_map(frame, flags, allocator);
            }
        }

        // Identity map VGA buffer
        let vga_buffer_frame = Frame::containing_address(0xb8000);
        mapper.identity_map(vga_buffer_frame, EntryFlags::WRITABLE, allocator);

        // Identity map the Multiboot info struct
        let multiboot_start = Frame::containing_address(boot_info.start_address() as u64);
        let multiboot_end = Frame::containing_address((boot_info.end_address() - 1) as u64);
        for frame in Frame::range_inclusive(multiboot_start, multiboot_end) {
            mapper.identity_map(frame, EntryFlags::PRESENT, allocator);
        }
    });

    let old_table = active_table.switch(new_table);

    // Turn the old P4 table into a guard page
    let old_p4_page = Page::containing_address(old_table.p4_frame.start_address() as usize);
    active_table.unmap(old_p4_page, allocator);
    println!("Guard page at {:#x}", old_p4_page.start_address());
    println!("Switched to new page table!");
}

pub fn test_paging<A: FrameAllocator>(allocator: &mut A) {
    let mut page_table = unsafe { ActivePageTable::new() };

    let addr = 42 * 512 * 512 * 4096; // 42th P3 entry
    let page = Page::containing_address(addr);
    let frame = allocator.allocate_frame().expect("no more frames");
    println!(
        "None = {:?}, map to {:?}",
        page_table.translate(addr),
        frame
    );
    page_table.map_to(page, frame, EntryFlags::empty(), allocator);
    println!("Some = {:?}", page_table.translate(addr));
    println!("next free frame: {:?}", allocator.allocate_frame());

    println!("{:#x}", unsafe {
        *(Page::containing_address(addr).start_address() as *const u64)
    });

    page_table.unmap(Page::containing_address(addr), allocator);
    println!("None = {:?}", page_table.translate(addr));

    println!("{:#x}", unsafe {
        *(Page::containing_address(addr).start_address() as *const u64)
    });
}

pub fn init(allocator: &mut impl FrameAllocator, boot_info: &BootInformation) {
    remap_kernel(allocator, boot_info);
}
