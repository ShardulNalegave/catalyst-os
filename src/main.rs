#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::runner)]

/// Utils module
pub mod utils;

/// VGA module
pub mod vga;

/// Tests module
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
  panic!("Some panic message");
}