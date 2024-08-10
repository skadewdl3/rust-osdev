use super::frame::{Frame, FrameAllocator};

pub struct TinyFrameAllocator([Option<Frame>; 3]);

impl FrameAllocator for TinyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        for frame_option in &mut self.0 {
            if frame_option.is_some() {
                return frame_option.take();
            }
        }
        None
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        for frame_option in &mut self.0 {
            if frame_option.is_none() {
                *frame_option = Some(frame);
                return;
            }
        }
        panic!("Tiny allocator can hold only 3 frames.");
    }
}

impl TinyFrameAllocator {
    pub fn new<A: FrameAllocator>(allocator: &mut A) -> Self {
        let mut f = || allocator.allocate_frame();
        let frames = [f(), f(), f()];
        Self(frames)
    }
}
