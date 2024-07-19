use core::arch::asm;

use idt::InterruptType;

mod idt;

lazy_static::lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();
        idt.set_handler(InterruptType::DivideError, divide_error_wrapper);
        idt.set_handler(InterruptType::Breakpoint, breakpoint_handler);
        idt
    };
}

#[naked]
extern "C" fn divide_error_wrapper() -> ! {
    unsafe {
        asm!(
            "
            mov rdi, rsp
            call {}
            ",
            sym divide_error_handler,
            options(noreturn)
        );
    }
}

extern "C" fn divide_error_handler(stack_frame: &ExceptionStackFrame) -> ! {
    // crate::println!("Got frame as: {:#?}", stack_frame);
    crate::println!("EXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
    // let mut x: u64 = 0;
    // unsafe {
    //     let stack_frame = *(x as *const exceptionstackframe);
    //     crate::println!("exception: divide by zero\n{:#?}", stack_frame);
    // }
    loop {}
}

extern "C" fn breakpoint_handler() -> ! {
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
