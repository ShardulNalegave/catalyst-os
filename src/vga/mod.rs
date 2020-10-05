
/// Color module
mod color;
pub use color::*;

/// Buffer module
mod buffer;
pub use buffer::*;
use crate::vga::VGAChar;

/// # VGA Writer
/// Writes to the VGA Buffer
pub struct VGAWriter {
    pub column_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl VGAWriter {
    /// ## New Line
    fn new_line(&mut self) { todo!() }

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
                self.buffer.chars[row][col] = VGAChar {
                    char: byte,
                    color_code,
                };
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