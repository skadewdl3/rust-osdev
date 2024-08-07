pub mod active_page_table;
pub mod entry;
pub mod table;

use super::FrameAllocator;
use active_page_table::ActivePageTable;
use entry::{EntryFlags, Page};
use table::{Level3, Table};

use crate::println;

use super::Frame;

pub const PAGE_SIZE: u64 = 4096; // 4KB
const PAGE_TABLE_ENTRY_COUNT: usize = 512; // 512 * 8 bytes = 4KB

pub type PhysAddr = usize;
pub type VirtAddr = usize;

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

pub fn init() {
    println!("Initializing Paging");
}
