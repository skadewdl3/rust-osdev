use crate::serial_println;
use crate::*;
use linkme::distributed_slice;
use panic::{exit_qemu, QemuExitCode};

#[distributed_slice]
pub static TESTS: [fn()];

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
        serial_print!("...\t");
        serial_println!("[ok]");
    }
}

pub fn test_runner() {
    serial_println!("Running {} tests", TESTS.len());
    for test in TESTS {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}
