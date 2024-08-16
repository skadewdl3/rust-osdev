pub mod builder;
pub mod color;
pub mod renderer;
pub mod writer;

use crate::{
    memory::{
        frame::{Frame, FrameAllocator},
        MemoryError,
    },
    paging::{active_page_table::ActivePageTable, entry::EntryFlags, page::Page},
    serial_println,
};
use alloc::vec::Vec;
use builder::FrameBufferBuilder;
use color::Color;
use multiboot2::{
    BootInformation, BootInformationHeader, FramebufferField, FramebufferTag, TagTrait,
};
use renderer::FrameBufferRenderer;
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
    paged: bool,
}

impl FrameBuffer {
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

    pub fn start_address(&self) -> usize {
        self.start_address
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
    pub static ref WRITER: Mutex<FrameBufferWriter> = Mutex::new(FrameBufferWriter::new());
    pub static ref RENDERER: Mutex<Option<FrameBufferRenderer>> = Mutex::new(None);
}

pub fn init<'a>(boot_info: &BootInformation) {
    let tag = boot_info.framebuffer_tag().unwrap().unwrap();
    let framebuffer = FrameBufferBuilder::new().from_tag(&tag).build();

    let front = FrameBufferBuilder::new().from_tag(&tag).build();
    let back = FrameBufferBuilder::new()
        .from_tag(&tag)
        .allocate_buffer()
        .build();

    let renderer = FrameBufferRenderer::new(front, back);

    *RENDERER.lock() = Some(renderer);

    // *WRITER.lock() = Some(FrameBufferWriter::new(framebuffer));
}

pub fn fill_bg() {
    let mut x = crate::framebuffer::RENDERER.lock();
    let mut c = x.as_mut().unwrap();
    c.fill(Color::hex(0xff0000));
    c.swap();
}

// Initialize the framebuffer writer font
// pub fn init() {
//     let mut c = WRITER.lock();
//     let w = c.as_mut();
//     let mut w = w.unwrap();
//     w.load_font();
// }
