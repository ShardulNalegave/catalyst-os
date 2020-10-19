#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::runner)]
#![reexport_test_harness_main = "tests_main"]

/// Panic module
pub mod panic;

/// Interrupts module
pub mod interrupts;

/// Tests module
#[cfg(test)]
pub mod tests;

// ===== Imports =====
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

    // Page-fault exception
    let ptr = 0xdeadbeaf as *mut u32;
    unsafe { *ptr = 42; }

    vga::println!("It did not crash!!");

    #[cfg(test)]
    tests_main();

    shared::utils::halt_loop();
}

/// # Init
/// All modules are initialized here
fn init() {
    interrupts::load_idt();
}