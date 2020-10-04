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

static HELLO: &[u8] = b"Hello World!";

/// # Start Function
/// Entry-Point for the OS.
#[no_mangle]
pub extern "C" fn _start() -> ! {
  let vga_buffer = 0xb8000 as *mut u8;

  for (i, &byte) in HELLO.iter().enumerate() {
    unsafe {
      *vga_buffer.offset(i as isize * 2) = byte;
      *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    }
  }

  loop {}
}