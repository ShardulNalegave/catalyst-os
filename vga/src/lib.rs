
/// Color module
mod color;
pub use color::*;

/// Buffer module
mod buffer;
pub use buffer::*;

/// Macros module
pub mod macros;

// ===== Imports =====
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
// ===================

lazy_static! {
    /// Global Static Instance of VGAWriter
    pub static ref WRITER: Mutex<VGAWriter> = Mutex::new(VGAWriter {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/// # VGA Writer
/// Writes to the VGA Buffer
pub struct VGAWriter {
    pub column_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl VGAWriter {
    /// ## New Line
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// ## Clear row
    fn clear_row(&mut self, row: usize) {
        let blank = VGAChar {
            char: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    /// ## Write (Byte)
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => { self.new_line() },
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(VGAChar {
                    char: byte,
                    color_code,
                });
                self.column_position += 1;
            },
        }
    }

    /// ## Write (String)
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }
}

/// Rust formatting support for VGAWriter
impl fmt::Write for VGAWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}