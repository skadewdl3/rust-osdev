use alloc::{boxed::Box, vec::Vec};
use multiboot2::FramebufferTag;

use super::FrameBuffer;

#[derive(Default)]
pub struct FrameBufferBuilder {
    start_address: Option<usize>,
    width: Option<usize>,
    height: Option<usize>,
    pitch: Option<usize>,
    buffer: Option<&'static mut [u8]>,
    bytes_per_pixel: Option<usize>,
    paged: Option<bool>,
}

impl FrameBufferBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_tag(&mut self, tag: &FramebufferTag) -> &mut Self {
        self.start_address = Some(tag.address() as usize);
        self.width = Some(tag.width() as usize);
        self.height = Some(tag.height() as usize);
        self.pitch = Some(tag.pitch() as usize);
        self.bytes_per_pixel = Some((tag.bpp() / 8) as usize);
        self
    }

    pub fn with_start_address(&mut self, start_address: usize) -> &mut Self {
        self.start_address = Some(start_address);
        self
    }

    pub fn from_buffer(&mut self, buffer: FrameBuffer) -> &mut Self {
        self.width = Some(buffer.width);
        self.height = Some(buffer.height);
        self.pitch = Some(buffer.pitch);
        self.bytes_per_pixel = Some(buffer.bytes_per_pixel);
        self
    }

    pub fn allocate_buffer(&mut self) -> &mut Self {
        let mut vec: Vec<u8> = vec![0; self.pitch.unwrap() * self.height.unwrap()];
        let boxed_slice = vec.into_boxed_slice();
        let static_buffer: &'static mut [u8] = Box::leak(boxed_slice);
        self.start_address = Some(static_buffer.as_ptr() as usize);
        self.buffer = Some(static_buffer);
        self
    }

    pub fn build(&mut self) -> FrameBuffer {
        let start_address = self.start_address.unwrap();
        let width = self.width.unwrap();
        let height = self.height.unwrap();
        let pitch = self.pitch.unwrap();
        let bytes_per_pixel = self.bytes_per_pixel.unwrap();
        let framebuffer_size = (pitch * height) as usize;

        let buffer =
            unsafe { core::slice::from_raw_parts_mut(start_address as *mut u8, framebuffer_size) };

        FrameBuffer {
            start_address,
            width,
            height,
            pitch,
            buffer,
            bytes_per_pixel,
            paged: false,
        }
    }
}
