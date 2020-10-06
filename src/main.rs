#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::runner)]
#![reexport_test_harness_main = "test_main"]

/// Utils module
pub mod utils;

/// VGA module
pub mod vga;

/// Tests module
#[cfg(test)]
pub mod tests;

// ===== Imports =====
#[allow(unused_imports)]
use utils::panic::panic;
// ===================

/// # Start Function
/// Entry-Point for the OS.
#[no_mangle]
pub extern "C" fn _start() -> ! {
  vga_println!("Hello, World!");
  vga_print!("How are you?");
  vga_println!(" I am fine!");

  #[cfg(test)]
  test_main();

  loop {}
}