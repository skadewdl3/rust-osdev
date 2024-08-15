use core::{arch::x86_64, fmt};
use lazy_static::lazy_static;
use spin::Mutex;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::logger::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// #[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    ::x86_64::instructions::interrupts::without_interrupts(|| {
        let mut x = crate::framebuffer::WRITER.lock();
        if let Some(c) = x.as_mut() {
            if c.paged() {
                c.write_fmt(args).expect("Writing to framebuffer failed");
            } else {
                crate::serial::SERIAL1
                    .lock()
                    .write_fmt(args)
                    .expect("Writing to serial failed");
            }
        } else {
            crate::serial::SERIAL1
                .lock()
                .write_fmt(args)
                .expect("Writing to serial failed");
        }
    });
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
