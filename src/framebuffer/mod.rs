pub mod color;
pub mod writer;

use crate::{
    memory::{
        frame::{Frame, FrameAllocator},
        MemoryError,
    },
    paging::{active_page_table::ActivePageTable, entry::EntryFlags, page::Page},
    serial_println,
};
use color::Color;
use multiboot2::{BootInformation, BootInformationHeader, FramebufferTag, TagTrait};
use spin::Mutex;
use writer::FrameBufferWriter;
use x86_64::instructions::tlb;

pub struct FrameBuffer {
    start_address: usize,
    width: usize,
    height: usize,
    pitch: usize,
    buffer: &'static mut [u8],
    bytes_per_pixel: usize,
}

impl FrameBuffer {
    pub fn new(tag: &FramebufferTag) -> Self {
        let framebuffer_start = tag.address() as usize;
        let width = tag.width() as usize;
        let height = tag.height() as usize;
        let pitch = tag.pitch() as usize;
        let bytes_per_pixel = (tag.bpp() / 8) as usize;
        let framebuffer_size = (pitch * height) as usize;

        let buffer: &mut [u8] = unsafe {
            core::slice::from_raw_parts_mut(framebuffer_start as *mut u8, framebuffer_size)
        };

        Self {
            start_address: framebuffer_start,
            width,
            height,
            pitch,
            buffer,
            bytes_per_pixel,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pitch(&self) -> usize {
        self.pitch
    }

    pub fn bpp(&self) -> usize {
        self.bytes_per_pixel
    }

    pub fn buffer(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = y * self.pitch + x * self.bytes_per_pixel;
        let color = color.value();
        let bpp = self.bytes_per_pixel;
        let addr = self.start_address as *mut u8;
        unsafe {
            *addr.add(offset) = color[0];
            *addr.add(offset + 1) = color[1];
            *addr.add(offset + 2) = color[2];
            *addr.add(offset + 3) = color[3];
        }
    }

    pub fn fill(&mut self, color: Color) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.draw_pixel(x, y, color);
            }
        }
    }
}

lazy_static::lazy_static! {
    pub static ref WRITER: Mutex<Option<FrameBufferWriter>> = Mutex::new(None);
}

pub fn init<'a>(boot_info: &'a BootInformation) -> &'a FramebufferTag {
    let tag = boot_info.framebuffer_tag().unwrap().unwrap();
    let framebuffer = FrameBuffer::new(&tag);
    *WRITER.lock() = Some(FrameBufferWriter::new(framebuffer));
    &tag
}

pub fn test() {
    let mut c = WRITER.lock();
    let w = c.as_mut();
    let mut w = w.unwrap();
    serial_println!("Writing to: {:x}", w.start_address);
    w.fill(Color::rgb(255, 0, 0));

    w.write("A");
}
