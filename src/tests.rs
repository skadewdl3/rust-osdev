use crate::*;
use linkme::distributed_slice;

#[distributed_slice]
pub static TESTS: [fn()];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        // crate::serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        crate::serial_print!("...\t");
        crate::serial_println!("[ok]");
    }
}

pub fn test_runner() {
    crate::serial_println!("Running {} tests", TESTS.len());
    for test in TESTS {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");

    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(testing)]
#[panic_handler]
pub fn panic_handler(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
