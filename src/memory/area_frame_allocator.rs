use crate::memory::{Frame, FrameAllocator};
use multiboot2::MemoryArea;

pub struct AreaFrameAllocator {
    next_free_frame: Frame,
    current_area: Option<&'static MemoryArea>,
    areas: [MemoryArea; 10],
    kernel_start: Frame,
    kernel_end: Frame,
    multiboot_start: Frame,
    multiboot_end: Frame,
}
