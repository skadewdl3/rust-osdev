use core::ops::{Deref, DerefMut};

use super::{color::Color, FrameBuffer};

pub struct FrameBufferWriter {
    _buffer: FrameBuffer,
}

impl FrameBufferWriter {
    pub fn new(buffer: FrameBuffer) -> Self {
        Self { _buffer: buffer }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width() || y >= self.height() {
            return;
        }

        let offset = y * self.pitch() + x * self.bpp();

        let color = color.value();
        let bpp = self.bpp();
        // self.buffer()[offset..(offset + bpp)].copy_from_slice(&color[..bpp]);
        let addr = self.start_address as *mut u8;
        unsafe {
            *addr.add(offset) = color[0];
            *addr.add(offset + 1) = color[1];
            *addr.add(offset + 2) = color[2];
        }
    }
}

impl Deref for FrameBufferWriter {
    type Target = FrameBuffer;

    fn deref(&self) -> &FrameBuffer {
        &self._buffer
    }
}

impl DerefMut for FrameBufferWriter {
    fn deref_mut(&mut self) -> &mut FrameBuffer {
        &mut self._buffer
    }
}
