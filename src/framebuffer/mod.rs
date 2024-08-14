use crate::{
    memory::{
        frame::{Frame, FrameAllocator},
        MemoryError,
    },
    paging::{active_page_table::ActivePageTable, entry::EntryFlags, page::Page},
    serial_println,
};
use multiboot2::{BootInformation, BootInformationHeader, TagTrait};
use x86_64::instructions::tlb;

const FRAMEBUFFER_ADDR: usize = 0xfd000000;
const FRAMEBUFFER_SIZE: usize = 0x3e8000;
const FRAMEBUFFER_PITCH: usize = 5120;
const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 720;
const BYTES_PER_PIXEL: usize = 4;

#[repr(C)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
    _reserved: u8,
}

pub fn draw_pixel(x: usize, y: usize, color: RGB) {
    if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
        return;
    }

    let framebuffer = FRAMEBUFFER_ADDR as *mut u8;
    let offset = y * FRAMEBUFFER_PITCH + x * BYTES_PER_PIXEL;
    unsafe {
        // Write the color to the framebuffer at the calculated offset
        let pixel = framebuffer.add(offset) as *mut RGB;
        *pixel = color;
    }
}

pub fn init(
    boot_info: &BootInformation,
    mapper: &mut ActivePageTable,
    allocator: &mut impl FrameAllocator,
) -> Result<(), MemoryError> {
    let framebuffer_tag = boot_info
        .framebuffer_tag()
        .expect("Could not find framebuffer tag")
        .unwrap();

    let framebuffer_start = framebuffer_tag.address() as usize;
    let width = framebuffer_tag.width();
    let height = framebuffer_tag.height();
    let pitch = framebuffer_tag.pitch();
    let bytes_per_pixel = (framebuffer_tag.bpp() / 8) as usize;
    let framebuffer_size = (pitch * height) as usize;

    let frame_range = {
        let framebuffer_start_frame = Frame::containing_address(framebuffer_start as u64);
        let framebuffer_end_frame =
            Frame::containing_address((framebuffer_start + framebuffer_size - 1) as u64);
        Frame::range_inclusive(framebuffer_start_frame, framebuffer_end_frame)
    };

    for frame in frame_range {
        let flags = EntryFlags::PRESENT | EntryFlags::WRITABLE;
        mapper.identity_map(frame, flags, allocator);
    }

    tlb::flush_all();

    let framebuffer: &mut [u8] =
        unsafe { core::slice::from_raw_parts_mut(framebuffer_start as *mut u8, framebuffer_size) };

    serial_println!("Framebuffer start: {:x}", framebuffer_start);
    serial_println!("Framebuffer size: {:x}", framebuffer_size);
    serial_println!(
        "bpp - {}, pitch - {}, type = {:?}",
        bytes_per_pixel,
        pitch,
        framebuffer_tag.buffer_type()
    );

    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            draw_pixel(
                x,
                y,
                RGB {
                    red: 255,
                    green: 0,
                    blue: 0,
                    _reserved: 0,
                },
            );
        }
    }

    // self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
    //     .copy_from_slice(&color[..bytes_per_pixel]);
    // let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };

    Ok(())
}
