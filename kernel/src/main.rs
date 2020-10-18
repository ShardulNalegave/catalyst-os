#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(shared::tests::runner)]
#![reexport_test_harness_main = "tests_main"]

/// Utils module
pub mod utils;

/// Interrupts module
pub mod interrupts;

/// Tests module
#[cfg(test)]
pub mod tests;

// ===== Imports =====
#[allow(unused_imports)]
use utils::panic::panic;
use bootloader::BootInfo;
// ===================

// Declare main function as entry-point
bootloader::entry_point!(main);

/// # Main Function
/// Entry-Point for the OS.
fn main(_boot_info: &'static BootInfo) -> ! {
    // Initialize all modules
    init();

    vga::println!("Hello, World!");
    vga::print!("How are you?");
    vga::println!(" I am fine!");

    vga::println!("It did not crash!!");

    #[cfg(test)]
    tests_main();

    utils::halt_loop();
}

/// # Init
/// All modules are initialized here
fn init() {
    interrupts::load_idt();
}