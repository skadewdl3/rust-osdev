use super::{
    active_page_table::ActivePageTable,
    table::{Level1, Table},
    PAGE_SIZE,
};
use crate::memory::{
    frame::{Frame, FrameAllocator},
    paging::entry::EntryFlags,
    tiny_frame_allocator::TinyFrameAllocator,
};

use super::VirtAddr;
#[derive(Debug, Clone, Copy)]
pub struct Page {
    number: usize,
}

impl Page {
    pub fn new(number: usize) -> Page {
        Page { number }
    }
    pub fn containing_address(address: VirtAddr) -> Page {
        assert!(
            address < 0x0000_8000_0000_0000 || address >= 0xffff_8000_0000_0000,
            "invalid address: 0x{:x}",
            address
        );
        Page {
            number: address / PAGE_SIZE as usize,
        }
    }

    pub fn start_address(&self) -> usize {
        self.number * PAGE_SIZE as usize
    }

    pub fn p4_index(&self) -> usize {
        (self.number >> 27) & 0o777
    }
    pub fn p3_index(&self) -> usize {
        (self.number >> 18) & 0o777
    }
    pub fn p2_index(&self) -> usize {
        (self.number >> 9) & 0o777
    }
    pub fn p1_index(&self) -> usize {
        (self.number >> 0) & 0o777
    }
}

pub struct TemporaryPage {
    pub page: Page,
    allocator: TinyFrameAllocator,
}

impl TemporaryPage {
    pub fn new<A: FrameAllocator>(page: Page, allocator: &mut A) -> TemporaryPage {
        TemporaryPage {
            page,
            allocator: TinyFrameAllocator::new(allocator),
        }
    }

    pub fn map(&mut self, frame: Frame, active_table: &mut ActivePageTable) -> VirtAddr {
        assert!(
            active_table.translate_page(self.page).is_none(),
            "temporary page is already mapped"
        );
        crate::serial_println!("From map {}", self.page.p4_index());
        active_table.map_to(self.page, frame, EntryFlags::WRITABLE, &mut self.allocator);
        self.page.start_address()
    }

    pub fn map_table_frame(
        &mut self,
        frame: Frame,
        active_table: &mut ActivePageTable,
    ) -> &mut Table<Level1> {
        unsafe { &mut *(self.map(frame, active_table) as *mut Table<Level1>) }
    }

    pub fn unmap(&mut self, active_table: &mut ActivePageTable) {
        active_table.unmap(self.page, &mut self.allocator);
    }
}
