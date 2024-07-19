use core::arch::asm;

use idt::InterruptType;

mod idt;

lazy_static::lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();
        idt.set_handler(InterruptType::DivideError, divide_error_handler);
        idt.set_handler(InterruptType::Breakpoint, breakpoint_handler);
        idt
    };
}
extern "x86-interrupt" fn divide_error_handler() -> ! {
    let mut x: u64 = 0;
    unsafe {
        asm!("mov {}, rsp", out(reg) x);
        let stack_frame = *(x as *const ExceptionStackFrame);
        crate::println!("EXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
    }
    loop {}
}

extern "x86-interrupt" fn breakpoint_handler() -> ! {
    crate::println!("EXCEPTION: BREAKPOINT");
    loop {}
}

pub fn init() {
    IDT.load();
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
struct ExceptionStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}
