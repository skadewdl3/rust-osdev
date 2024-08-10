use crate::memory::frame::{Frame, FrameAllocator};
use multiboot2::MemoryArea;

pub struct AreaFrameAllocator<'a> {
    next_free_frame: Frame,
    current_area: Option<&'a MemoryArea>,
    areas: &'a [MemoryArea],
    kernel_start: Frame,
    kernel_end: Frame,
    multiboot_start: Frame,
    multiboot_end: Frame,
}

impl<'a> FrameAllocator for AreaFrameAllocator<'a> {
    fn allocate_frame(&mut self) -> Option<Frame> {
        if let Some(area) = self.current_area {
            let frame = Frame::new(self.next_free_frame.number);
            let current_area_last_frame = {
                let address = area.start_address() + area.size() - 1;
                Frame::containing_address(address)
            };

            if frame > current_area_last_frame {
                // all frames of current area are used, switch to next area
                self.choose_next_area();
            } else if frame >= self.kernel_start && frame <= self.kernel_end {
                // `frame` is used by the kernel
                self.next_free_frame = Frame {
                    number: self.kernel_end.number + 1,
                };
            } else if frame >= self.multiboot_start && frame <= self.multiboot_end {
                // `frame` is used by the multiboot information structure
                self.next_free_frame = Frame {
                    number: self.multiboot_end.number + 1,
                };
            } else {
                // frame is unused, increment `next_free_frame` and return it
                self.next_free_frame.number += 1;
                return Some(frame);
            }
            // `frame` was not valid, try it again with the updated `next_free_frame`
            self.allocate_frame()
        } else {
            None // no free frames left
        }
    }

    fn deallocate_frame(&mut self, _frame: Frame) {
        unimplemented!()
    }
}

impl<'a> AreaFrameAllocator<'a> {
    pub fn new(
        kernel_start: u64,
        kernel_end: u64,
        multiboot_start: u64,
        multiboot_end: u64,
        memory_areas: &'a [MemoryArea],
    ) -> Self {
        let mut allocator = AreaFrameAllocator {
            next_free_frame: Frame::containing_address(0),
            current_area: None,
            areas: memory_areas,
            kernel_start: Frame::containing_address(kernel_start),
            kernel_end: Frame::containing_address(kernel_end),
            multiboot_start: Frame::containing_address(multiboot_start),
            multiboot_end: Frame::containing_address(multiboot_end),
        };
        allocator.choose_next_area();
        allocator
    }

    fn choose_next_area(&mut self) {
        self.current_area = self
            .areas
            .into_iter()
            .filter(|area| {
                let address = area.start_address() + area.size() - 1;
                Frame::containing_address(address) >= self.next_free_frame
            })
            .min_by_key(|area| area.start_address());

        if let Some(area) = self.current_area {
            let start_frame = Frame::containing_address(area.start_address());
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }
    }
}
