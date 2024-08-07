pub mod entry;
pub mod table;

use super::FrameAllocator;
use entry::{EntryFlags, Page};
use table::{Level3, Table};

use crate::println;

use super::Frame;

pub const PAGE_SIZE: u64 = 4096; // 4KB
const PAGE_TABLE_ENTRY_COUNT: usize = 512; // 512 * 8 bytes = 4KB

pub type PhysAddr = usize;
pub type VirtAddr = usize;

pub fn init() {
    println!("Initializing Paging");
}

fn translate_page(page: Page) -> Option<Frame> {
    // use self::entry::HUGE_PAGE;

    let p3 = unsafe { &*table::P4 }.next_table(page.p4_index());

    let huge_page = || {
        p3.and_then(|p3| {
            let p3_entry = &p3[page.p3_index()];
            // 1GiB page?
            if let Some(start_frame) = p3_entry.pointed_frame() {
                if p3_entry.flags().contains(entry::EntryFlags::HUGE_PAGE) {
                    // address must be 1GiB aligned
                    assert!(
                        start_frame.number
                            % ((PAGE_TABLE_ENTRY_COUNT * PAGE_TABLE_ENTRY_COUNT) as u64)
                            == 0
                    );
                    return Some(Frame {
                        number: (start_frame.number as usize
                            + page.p2_index() * PAGE_TABLE_ENTRY_COUNT
                            + page.p1_index()) as u64,
                    });
                }
            }
            if let Some(p2) = p3.next_table(page.p3_index()) {
                let p2_entry = &p2[page.p2_index()];
                // 2MiB page?
                if let Some(start_frame) = p2_entry.pointed_frame() {
                    if p2_entry.flags().contains(entry::EntryFlags::HUGE_PAGE) {
                        // address must be 2MiB aligned
                        assert!(start_frame.number % PAGE_TABLE_ENTRY_COUNT as u64 == 0);
                        return Some(Frame {
                            number: start_frame.number + page.p1_index() as u64,
                        });
                    }
                }
            }
            None
        })
    };

    p3.and_then(|p3| p3.next_table(page.p3_index()))
        .and_then(|p2| p2.next_table(page.p2_index()))
        .and_then(|p1| p1[page.p1_index()].pointed_frame())
        .or_else(huge_page)
}

pub fn translate(virtual_address: VirtAddr) -> Option<PhysAddr> {
    let offset = virtual_address as u64 % PAGE_SIZE;
    translate_page(Page::containing_address(virtual_address))
        .map(|frame| (frame.number * PAGE_SIZE + offset) as usize)
}

pub fn map_to<A: FrameAllocator>(page: Page, frame: Frame, flags: EntryFlags, allocator: &mut A) {
    let p4 = unsafe { &mut *table::P4 };
    let mut p3 = p4.next_table_create(page.p4_index(), allocator);
    let mut p2 = p3.next_table_create(page.p3_index(), allocator);
    let mut p1 = p2.next_table_create(page.p2_index(), allocator);

    assert!(p1[page.p1_index()].is_unused());
    p1[page.p1_index()].set(frame, flags | EntryFlags::PRESENT);
}
