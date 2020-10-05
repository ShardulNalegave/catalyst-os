#![no_std]
#![no_main]

/// Utils module
pub mod utils;

/// VGA module
pub mod vga;

// ===== Imports =====
#[allow(unused_imports)]
use utils::panic::panic;
// ===================

/// # Start Function
/// Entry-Point for the OS.
#[no_mangle]
pub extern "C" fn _start() -> ! {
  vga::WRITER.lock().write_string("Hello, World! How are you?");
  loop {}
}