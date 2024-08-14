use crate::memory::frame::{Frame, FrameAllocator};

use super::{
    entry::EntryFlags,
    page::Page,
    table::{Level4, Table, P4},
    PhysAddr, VirtAddr, PAGE_SIZE, PAGE_TABLE_ENTRY_COUNT,
};
use core::ptr::Unique;

pub struct Mapper {
    p4: Unique<Table<Level4>>,
}

impl Mapper {
    pub unsafe fn new() -> Mapper {
        Mapper {
            p4: Unique::new_unchecked(P4),
        }
    }

    pub fn p4(&self) -> &Table<Level4> {
        unsafe { self.p4.as_ref() }
    }

    pub fn p4_mut(&mut self) -> &mut Table<Level4> {
        unsafe { self.p4.as_mut() }
    }

    pub fn translate_page(&self, page: Page) -> Option<Frame> {
        let p3 = unsafe { &*P4 }.next_table(page.p4_index());

        let huge_page = || {
            p3.and_then(|p3| {
                let p3_entry = &p3[page.p3_index()];
                // 1GiB page?
                if let Some(start_frame) = p3_entry.pointed_frame() {
                    if p3_entry.flags().contains(EntryFlags::HUGE_PAGE) {
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
                        if p2_entry.flags().contains(EntryFlags::HUGE_PAGE) {
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

        p3.and_then(|p3| {
            let tbl = p3.next_table(page.p3_index());
            tbl
        })
        .and_then(|p2| {
            let tbl = p2.next_table(page.p2_index());
            tbl
        })
        .and_then(|p1| {
            let fr = p1[page.p1_index()].pointed_frame();
            fr
        })
        .or_else(huge_page)
    }

    pub fn translate(&self, virtual_address: VirtAddr) -> Option<PhysAddr> {
        let offset = virtual_address as u64 % PAGE_SIZE;
        let page = Page::containing_address(virtual_address);
        self.translate_page(page)
            .map(|frame| (frame.number * PAGE_SIZE + offset) as usize)
    }

    pub fn map_to<A: FrameAllocator>(
        &mut self,
        page: Page,
        frame: Frame,
        flags: EntryFlags,
        allocator: &mut A,
    ) {
        let p4 = unsafe { &mut *P4 };
        let p3 = p4.next_table_create(page.p4_index(), allocator);
        let p2 = p3.next_table_create(page.p3_index(), allocator);
        let p1 = p2.next_table_create(page.p2_index(), allocator);

        assert!(p1[page.p1_index()].is_unused());
        p1[page.p1_index()].set(frame, flags | EntryFlags::PRESENT);
    }

    pub fn map<A: FrameAllocator>(&mut self, page: Page, flags: EntryFlags, allocator: &mut A) {
        let frame = allocator.allocate_frame().expect("out of memory");
        self.map_to(page, frame, flags, allocator)
    }

    pub fn identity_map<A: FrameAllocator>(
        &mut self,
        frame: Frame,
        flags: EntryFlags,
        allocator: &mut A,
    ) {
        let page = Page::containing_address(frame.start_address() as usize);
        self.map_to(page, frame, flags, allocator)
    }

    pub fn unmap<A: FrameAllocator>(&mut self, page: Page, _allocator: &mut A) {
        assert!(self.translate(page.start_address()).is_some());

        let p1 = self
            .p4_mut()
            .next_table_mut(page.p4_index())
            .and_then(|p3| p3.next_table_mut(page.p3_index()))
            .and_then(|p2| p2.next_table_mut(page.p2_index()))
            .expect("mapping code does not support huge pages");
        let _frame = p1[page.p1_index()].pointed_frame().unwrap();
        p1[page.p1_index()].set_unused();
        use x86_64::addr::VirtAddr;
        use x86_64::instructions::tlb;
        tlb::flush(VirtAddr::new(page.start_address() as u64));

        // TODO: free p(1,2,3) table if empty
        // allocator.deallocate_frame(frame);
    }
}
