mod area_frame_allocator;
mod tiny_frame_allocator;

pub use area_frame_allocator::*;
pub use tiny_frame_allocator::*;

use super::paging::PAGE_SIZE;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    pub number: u64,
}

impl Frame {
    pub fn new(number: u64) -> Self {
        Self { number }
    }
    pub fn containing_address(address: u64) -> Frame {
        Frame {
            number: address / PAGE_SIZE,
        }
    }

    pub fn start_address(&self) -> u64 {
        self.number * PAGE_SIZE
    }
    pub fn clone(&self) -> Frame {
        Frame {
            number: self.number,
        }
    }

    pub fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter { start, end }
    }
}

pub struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        } else {
            None
        }
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}
