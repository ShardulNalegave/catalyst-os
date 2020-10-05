#![no_std]
#![no_main]

/// Utils module
pub mod utils;

/// VGA module
pub mod vga;

// ===== Imports =====
#[allow(unused_imports)]
use utils::panic::panic;
use vga::VGAWriter;
use crate::vga::{ColorCode, Color, Buffer};
// ===================

/// # Start Function
/// Entry-Point for the OS.
#[no_mangle]
pub extern "C" fn _start() -> ! {
  print_test();
  loop {}
}

fn print_test() {
  let mut writer = VGAWriter {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
  };
  writer.write_string("Hello, World! How are you?");
}