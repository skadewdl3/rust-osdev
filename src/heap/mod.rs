pub mod bump_allocator;
pub mod linked_list_allocator;
pub mod utils;

use alloc::{boxed::Box, rc::Rc, vec::Vec};
use bump_allocator::BumpAllocator;
use linked_list_allocator::LinkedListAllocator;
use utils::Locked;

use crate::{
    memory::{frame::FrameAllocator, MemoryError},
    paging::{entry::EntryFlags, mapper::Mapper, page::Page},
};

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 400 * 1024; // 100 KiB

#[global_allocator]
// static ALLOCATOR: BumpAllocator = BumpAllocator;
static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

pub fn init(mapper: &mut Mapper, allocator: &mut impl FrameAllocator) -> Result<(), MemoryError> {
    use x86_64::instructions::tlb;

    let page_range = {
        let heap_start = HEAP_START;
        let heap_end = heap_start + HEAP_SIZE - 1;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = allocator
            .allocate_frame()
            .ok_or(MemoryError::FrameAllocationFailed)?;
        let flags = EntryFlags::PRESENT | EntryFlags::WRITABLE;
        mapper.map_to(page, frame, flags, allocator);
        tlb::flush_all();
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

crate::test_cases! {
    fn box_allocation() {
        let heap_value_1 = Box::new(41);
        let heap_value_2 = Box::new(13);
        assert_eq!(*heap_value_1, 41);
        assert_eq!(*heap_value_2, 13);
    }

    fn vector_allocation() {
        let n = 1000;
        let mut vec = Vec::new();
        for i in 0..n {
            vec.push(i);
        }
        assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
    }


    fn multiple_boxes_causing_reallocation() {
        for i in 0..HEAP_SIZE {
            let x = Box::new(i);
            assert_eq!(*x, i);
        }
    }

    fn reference_counting() {
        let rc = Rc::new(42);
        assert_eq!(Rc::strong_count(&rc), 1);
        let cloned_rc = rc.clone();
        assert_eq!(Rc::strong_count(&rc), 2);
        assert_eq!(Rc::strong_count(&cloned_rc), 2);
        core::mem::drop(rc);
        assert_eq!(Rc::strong_count(&cloned_rc), 1);
    }
}
