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

extern "C" fn divide_error_handler() -> ! {
    crate::println!("EXCEPTION: DIVIDE BY ZERO");
    loop {}
}

extern "C" fn breakpoint_handler() -> ! {
    crate::println!("EXCEPTION: BREAKPOINT");
    loop {}
}

pub fn init() {
    IDT.load();
}
