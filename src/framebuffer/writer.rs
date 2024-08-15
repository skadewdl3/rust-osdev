use super::{color::Color, FrameBuffer};
use crate::serial_println;
use core::ops::{Deref, DerefMut};
use noto_sans_mono_bitmap::{
    get_raster, get_raster_width, FontWeight, RasterHeight, RasterizedChar,
};

const LINE_SPACING: usize = 2;
const LETTER_SPACING: usize = 0;
const BORDER_PADDING: usize = 1;
pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;
pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);
pub const BACKUP_CHAR: char = '�';
pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;

fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}

pub struct FrameBufferWriter {
    _buffer: FrameBuffer,
    x_offset: usize,
    y_offset: usize,
    background: Color,
    foreground: Color,
}

impl FrameBufferWriter {
    pub fn new(buffer: FrameBuffer) -> Self {
        Self {
            _buffer: buffer,
            x_offset: 0,
            y_offset: 0,
            background: Color::hex(0x000000),
            foreground: Color::hex(0xffffff),
        }
    }

    fn newline(&mut self) {
        self.y_offset += CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return()
    }

    fn carriage_return(&mut self) {
        self.x_offset = BORDER_PADDING;
    }

    /// Erases all text on the screen. Resets `self.x_offset` and `self.y_offset`.
    pub fn clear(&mut self) {
        self.x_offset = BORDER_PADDING;
        self.y_offset = BORDER_PADDING;
        let color = self.background;
        self.fill(color);
    }

    pub fn write(&mut self, text: &str) {
        for char in text.chars() {
            self.write_char(char);
        }
        serial_println!("Writing: {}", text);
        // TODO:
    }

    pub fn write_char(&mut self, c: char) {
        let x_offset = self.x_offset;
        let y_offset = self.y_offset;
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                let new_xpos = x_offset + CHAR_RASTER_WIDTH;
                if new_xpos >= self.width() {
                    self.newline();
                }
                let new_ypos = y_offset + CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
                if new_ypos >= self.height() {
                    self.clear();
                }
                self.write_rendered_char(get_char_raster(c));
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                // serial_println!("Writing pixel: ({}, {})", x, y);
                let color = self.foreground;
                let x_offset = self.x_offset;
                let y_offset = self.y_offset;
                self.draw_pixel(x_offset + x, y_offset + y, color);
            }
        }
        self.x_offset += rendered_char.width() + LETTER_SPACING;
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
