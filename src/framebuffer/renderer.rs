use multiboot2::FramebufferTag;

use super::{builder::FrameBufferBuilder, color::Color, FrameBuffer};

pub struct FrameBufferRenderer {
    front: FrameBuffer,
    back: FrameBuffer,
}

impl FrameBufferRenderer {
    pub fn new(front: FrameBuffer, back: FrameBuffer) -> Self {
        Self { front, back }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.back.width() || y >= self.back.height() {
            return;
        }

        let bpp = self.back.bpp();
        let pitch = self.back.pitch();
        let offset = y * pitch + x * bpp;
        let color = color.value();
        let addr = self.back.start_address() as *mut u8;

        unsafe {
            *addr.add(offset) = color[0];
            *addr.add(offset + 1) = color[1];
            *addr.add(offset + 2) = color[2];
            *addr.add(offset + 3) = color[3];
        }
    }

    pub fn fill(&mut self, color: Color) {
        self.back.fill(color)
    }

    pub fn swap(&mut self) {
        self.front.buffer().copy_from_slice(&self.back.buffer());
    }
}

impl core::ops::Deref for FrameBufferRenderer {
    type Target = FrameBuffer;
    fn deref(&self) -> &Self::Target {
        &self.back
    }
}

impl core::ops::DerefMut for FrameBufferRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.back
    }
}
