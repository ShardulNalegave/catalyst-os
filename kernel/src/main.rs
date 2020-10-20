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
use x86_64::VirtAddr;
// ===================

// Declare main function as entry-point
bootloader::entry_point!(main);

/// # Main Function
/// Entry-Point for the OS.
fn main(boot_info: &'static BootInfo) -> ! {
    // Initialize all modules
    init();

    vga::println!("Hello, World!");
    vga::print!("How are you?");
    vga::println!(" I am fine!");

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let lev4_page_table = unsafe { paging::mem::active_level_4_page_table(physical_memory_offset) };
    for (i, entry) in lev4_page_table.iter().enumerate() {
        if !entry.is_unused() {
            vga::println!("Level-4 Page Table entry {}: {:?}", i, entry);
        }
    }

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