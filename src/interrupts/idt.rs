use bit_field::BitField;
use x86_64::registers::segmentation::{self, Segment};
use x86_64::structures::gdt::SegmentSelector;
use x86_64::{PrivilegeLevel, VirtAddr};

pub type HandlerFunc = extern "C" fn();

pub struct Idt([Entry; 256]);

#[allow(dead_code)]
pub enum InterruptType {
    DivideError,
    Debug,
    NonMaskableInterrupt,
    Breakpoint,
    Overflow,
    BoundRangeExceeded,
    InvalidOpcode,
    DeviceNotAvailable,
    CoprocessorSegmentOverrun,
    X87FloatingPoint,
    SimdFloatingPoint,
    Virtualization,
    HvInjectionException,
    DoubleFault,
    InvaliddTss,
    SegmentNotPresent,
    GeneralProtectionFault,
    PageFault,
    AlignmentCheck,
    // MachineCheck,
    SecurityException,
}

impl Into<u8> for InterruptType {
    fn into(self) -> u8 {
        match self {
            Self::DivideError => 0,
            Self::Debug => 1,
            Self::NonMaskableInterrupt => 2,
            Self::Breakpoint => 3,
            Self::Overflow => 4,
            Self::BoundRangeExceeded => 5,
            Self::InvalidOpcode => 6,
            Self::DeviceNotAvailable => 7,
            Self::DoubleFault => 8,
            Self::CoprocessorSegmentOverrun => 9,
            Self::InvaliddTss => 10,
            Self::SegmentNotPresent => 11,
            Self::GeneralProtectionFault => 13,
            Self::PageFault => 14,
            Self::X87FloatingPoint => 16,
            Self::AlignmentCheck => 17,
            // Self::MachineCheck => 18,
            Self::SimdFloatingPoint => 19,
            Self::Virtualization => 20,
            Self::HvInjectionException => 28,
            Self::SecurityException => 30,
        }
    }
}

impl Idt {
    pub fn new() -> Idt {
        Idt([Entry::missing(); 256])
    }

    pub fn set_handler(&mut self, entry: impl Into<u8>, handler: HandlerFunc) {
        let x = entry.into();
        crate::println!("Setting handler for entry {}", x);
        self.0[x as usize] = Entry::new(segmentation::CS::get_reg(), handler);
    }

    pub fn load(&self) {
        use core::mem::size_of;
        use x86_64::instructions::tables::{lidt, DescriptorTablePointer};

        let ptr = DescriptorTablePointer {
            base: VirtAddr::from_ptr(self as *const _),
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe { lidt(&ptr) };
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Entry {
    pointer_low: u16,
    gdt_selector: SegmentSelector,
    options: EntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct EntryOptions(u16);

impl EntryOptions {
    fn minimal() -> Self {
        let mut options = 0;
        options.set_bits(9..12, 0b111);
        EntryOptions(options)
    }

    fn new() -> Self {
        let mut options = Self::minimal();
        options.set_present(true).disable_interrupts(true);
        options
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(15, present);
        self
    }

    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.0.set_bit(8, !disable);
        self
    }

    pub fn set_privilege_level(&mut self, dpl: u16) -> &mut Self {
        self.0.set_bits(13..15, dpl);
        self
    }

    pub fn set_stack_index(&mut self, index: u16) -> &mut Self {
        self.0.set_bits(0..3, index);
        self
    }
}

impl Entry {
    fn new(gdt_selector: SegmentSelector, handler: HandlerFunc) -> Self {
        let pointer = handler as u64;
        Entry {
            gdt_selector,
            pointer_low: pointer as u16,
            pointer_middle: (pointer >> 16) as u16,
            pointer_high: (pointer >> 32) as u32,
            options: EntryOptions::new(),
            reserved: 0,
        }
    }
}

impl Entry {
    fn missing() -> Self {
        Entry {
            gdt_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
            pointer_low: 0,
            pointer_middle: 0,
            pointer_high: 0,
            options: EntryOptions::minimal(),
            reserved: 0,
        }
    }
}
