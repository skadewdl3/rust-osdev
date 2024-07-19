mod idt;

use crate::println;
use core::arch::asm;
use idt::InterruptType;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
struct ExceptionStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

bitflags::bitflags! {
    #[derive(Debug)]
    struct PageFaultErrorCode: u64 {
        const PROTECTION_VIOLATION = 1 << 0;
        const CAUSED_BY_WRITE = 1 << 1;
        const USER_MODE = 1 << 2;
        const MALFORMED_TABLE = 1 << 3;
        const INSTRUCTION_FETCH = 1 << 4;
    }
}

// At the end of this function, a ud2 instruction is added
// This is because of the options(noreturn) in the asm! macro
// To return from the exception, we must use ther iretq instruction
// But this will jump to the old instruction pointer, which still contains the
// instruction that caused the exception
// TODO: Find a way to return from the exception

macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!(
                    "
                    mov rdi, rsp
                    call {}
                    ",
                    sym $name,
                    options(noreturn)
                )
            }
        }
        wrapper
    }}
}

macro_rules! handler_with_error_code {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!(
                    "
                    pop rsi
                    mov rdi, rsp
                    call {}
                    ",
                    sym $name,
                    options(noreturn)
                )
            }
        }
        wrapper
    }}
}

lazy_static::lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();
        idt.set_handler(InterruptType::DivideError, handler!(divide_error_handler));
        idt.set_handler(InterruptType::InvalidOpcode, handler!(invalid_opcode_handler));
        idt.set_handler(InterruptType::PageFault, handler_with_error_code!(page_fault_handler));
        idt
    };
}

extern "C" fn divide_error_handler(stack_frame: &ExceptionStackFrame) {
    println!("EXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
    loop {}
}

extern "C" fn invalid_opcode_handler(stack_frame: &ExceptionStackFrame) {
    println!(
        "\nEXCEPTION: INVALID OPCODE at {:#x}\n{:#?}",
        stack_frame.instruction_pointer, stack_frame
    );
    loop {}
}

extern "C" fn page_fault_handler(stack_frame: &ExceptionStackFrame, error_code: u64) -> ! {
    use x86_64::registers::control::Cr2;
    println!(
        "\nEXCEPTION: PAGE FAULT while accessing {:#x}\
        \nerror code: {:?}\n{:#?}",
        unsafe { Cr2::read_raw() },
        PageFaultErrorCode::from_bits(error_code).unwrap(),
        unsafe { &*stack_frame }
    );
    loop {}
}

pub fn init() {
    IDT.load();
}
