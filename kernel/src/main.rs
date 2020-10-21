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
use paging::mem::manager::Translator;
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

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mem_manager = unsafe { paging::mem::init(phys_mem_offset) };
    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mem_manager.virt_to_phys(virt);
        if let Some(phys) = phys {
            vga::println!("{:?} => {:?}", virt, phys);
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