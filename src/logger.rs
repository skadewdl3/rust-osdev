use core::{arch::x86_64, fmt};
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

pub fn _print_framebuffer(args: fmt::Arguments) {
    use core::fmt::Write;
    crate::framebuffer::WRITER
        .lock()
        .write_fmt(args)
        .expect("Writing to framebuffer failed");
}

pub fn _print_serial(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Writing to serial failed");
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    ::x86_64::instructions::interrupts::without_interrupts(|| {
        let mut renderer_exists = false;
        {
            let mut x = crate::framebuffer::RENDERER.lock();
            renderer_exists = x.is_some();
        }
        if renderer_exists {
            _print_framebuffer(args);
        } else {
            _print_serial(args);
        }
    });
}

// Prints to the frame buffer, if it is availabe annd mapped
// Else, falls back to serial output
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::logger::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::logger::_print_serial(format_args!($($arg)*));
    };
}

// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

// crate::test_cases! {
//     fn printing_to_vga() {
//         println!("test_println_simple output");
//     }
//
// fn chars_appearing_on_vga() {
//     use core::fmt::Write;
//     use ::x86_64::instructions::interrupts;
//
//     let s = "Some test string that fits on a single line";
//     interrupts::without_interrupts(|| {
//         let mut writer = WRITER.lock();
//         writeln!(writer, "\n{}", s).expect("writeln failed");
//         for (i, c) in s.chars().enumerate() {
//             let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
//             assert_eq!(char::from(screen_char.ascii_character), c);
//         }
//     });
// }
// }
