mod idt;
mod pic;

use crate::{hlt_loop, println};
use core::arch::asm;
use idt::InterruptType;
use pic::InterruptIndex;

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
        extern "C" fn wrapper() {
            unsafe {
                asm!(
                    "
                    push rax
                    push rcx
                    push rdx
                    push rsi
                    push rdi
                    push r8
                    push r9
                    push r10
                    push r11

                    // to pass the exception stack frame pointer to the exception
                    // handler
                    mov rdi, rsp 
                    add rdi, 9*8 // align the stack pointer
                    call {}
                    
                    pop rax
                    pop rcx
                    pop rdx
                    pop rsi
                    pop rdi
                    pop r8
                    pop r9
                    pop r10
                    pop r11

                    iretq
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
        extern "C" fn wrapper() {
            unsafe {
                asm!(
                    "
                    push rax
                    push rcx
                    push rdx
                    push rsi
                    push rdi
                    push r8
                    push r9
                    push r10
                    push r11

                    // load the error code into rsi
                    // to pass it as second argument to the exception handler
                    mov rsi, [rsp + 9*8]

                    // to pass the exception stack frame pointer to the exception
                    // handler
                    mov rdi, rsp 
                    add rdi, 10*8 // align the stack pointer
                    call {}
                    
                    pop rax
                    pop rcx
                    pop rdx
                    pop rsi
                    pop rdi
                    pop r8
                    pop r9
                    pop r10
                    pop r11

                    iretq
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
        idt.set_handler(InterruptType::Breakpoint, handler!(breakpoint_handler));
        idt.set_handler(InterruptType::PageFault, handler_with_error_code!(page_fault_handler));
        idt.set_handler(InterruptType::DoubleFault, handler_with_error_code!(double_fault_handler));
        idt.set_handler(InterruptIndex::Timer, handler!(timer_handler));
        idt.set_handler(InterruptIndex::Keyboard, handler!(keyboard_handler));
        idt
    };
}

extern "C" fn divide_error_handler(stack_frame: &ExceptionStackFrame) {
    println!("EXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
    hlt_loop();
}
extern "C" fn breakpoint_handler(stack_frame: &ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    hlt_loop();
}

extern "C" fn invalid_opcode_handler(stack_frame: &ExceptionStackFrame) {
    println!(
        "\nEXCEPTION: INVALID OPCODE at {:#x}\n{:#?}",
        stack_frame.instruction_pointer, stack_frame
    );
    hlt_loop();
}

extern "C" fn double_fault_handler(stack_frame: &ExceptionStackFrame, error_code: u64) {
    println!(
        "\nEXCEPTION: DOUBLE FAULT\nerror code: {:?}\n{:#?}",
        error_code, stack_frame
    );
    hlt_loop();
}

extern "C" fn page_fault_handler(stack_frame: &ExceptionStackFrame, error_code: u64) -> ! {
    use x86_64::registers::control::Cr2;
    println!(
        "\nEXCEPTION: PAGE FAULT while accessing {:#x}\
        \nerror code: {:?}\n{:#?}",
        Cr2::read_raw(),
        PageFaultErrorCode::from_bits(error_code).unwrap(),
        stack_frame
    );
    hlt_loop();
}

extern "C" fn timer_handler(_stack_frame: &ExceptionStackFrame) {
    // crate::print!(".");

    unsafe {
        pic::PICS
            .lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.into());
    }
}

extern "C" fn keyboard_handler(_stack_frame: &ExceptionStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static::lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore
            ));
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => crate::print!("{}", character),
                DecodedKey::RawKey(key) => crate::print!("{:?}", key),
            }
        }
    }

    unsafe {
        pic::PICS
            .lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.into());
    }
}

pub fn init() {
    IDT.load();
    unsafe { pic::PICS.lock().initialize() }
    x86_64::instructions::interrupts::enable();
}
