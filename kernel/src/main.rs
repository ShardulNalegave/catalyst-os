#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

/// Utils module
pub mod utils;

/// VGA module
// pub mod vga;

/// Interrupts module
pub mod interrupts;

// ===== Imports =====
#[allow(unused_imports)]
use utils::panic::panic;
// ===================

/// # Start Function
/// Entry-Point for the OS.
#[no_mangle]
pub extern "C" fn _start() -> ! {
  // Initialize all modules
  init();

  vga_println!("Hello, World!");
  vga_print!("How are you?");
  vga_println!(" I am fine!");

  vga_println!("It did not crash!!");

  utils::halt_loop();
}

/// # Init
/// All modules are initialized here
fn init() {
  interrupts::load_idt();
}