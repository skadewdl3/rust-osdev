#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Color([u8; 4]);

impl Color {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self([red, green, blue, 0])
    }

    pub fn hex(hex: u32) -> Self {
        Self([
            ((hex >> 16) & 0xFF) as u8,
            ((hex >> 8) & 0xFF) as u8,
            (hex & 0xFF) as u8,
            0,
        ])
    }

    pub fn value(&self) -> [u8; 4] {
        self.0
    }
}
