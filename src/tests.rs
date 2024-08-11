// pub mod vga_buffer;

use crate::serial_println;
use crate::*;
use panic::{exit_qemu, QemuExitCode};
use vga_buffer::*;

#[linkme::distributed_slice]
pub static TESTS: [fn()];

pub fn test_runner() {
    serial_println!("Running {} tests\n", TESTS.len());
    for test in TESTS {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[macro_export]
macro_rules! test_cases {
    (
        $(
            fn $test_name:ident() $body:block
        )*
    ) => {
        $(
            #[linkme::distributed_slice(crate::tests::TESTS)]
            fn $test_name() {
                crate::serial_print!("{}...\t", stringify!($test_name));
                $body
                crate::serial_println!("[ok]\n");
            }
        )*
    };
}
