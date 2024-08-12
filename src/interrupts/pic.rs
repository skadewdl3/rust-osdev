use pic8259::ChainedPics;
use spin::Mutex;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl Into<u8> for InterruptIndex {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Into<usize> for InterruptIndex {
    fn into(self) -> usize {
        self as usize
    }
}
