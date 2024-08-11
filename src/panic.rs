use core::panic::PanicInfo;

use crate::{println, serial_println};

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

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);

    #[cfg(testing)]
    serial_println!("{}", info);

    // exit_qemu(QemuExitCode::Failed);
    loop {}
}

// TODO: Implement panic_print from https://github.com/thepowersgang/rust-barebones-kernel/blob/master/Kernel/unwind.rs

// fn panic_print(fmt: core::fmt::Arguments, file: &'static str, line: u32) {
//     #[cfg(testing)]
//     {
//         use crate::serial_print;
//         serial_print!("\n\n PANIC in {} a line {}:", file, line);
//         serial_println!("    {}", fmt);
//     }
//
//     #[cfg(not(testing))]
//     {
//         use crate::println;
//         println!("\n\n PANIC in {} a line {}:", file, line);
//         println!("    {}", fmt);
//     }
// }
