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
use x86_64::{VirtAddr, PhysAddr};
use paging::mem::manager::{Translator, MemMapper};
use x86_64::structures::paging::{Page, Size4KiB, PhysFrame, PageTableFlags};
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
    let mut mem_manager = unsafe { paging::mem::init(phys_mem_offset) };

    let page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(0));
    mem_manager.map(
        page,
        PhysFrame::containing_address(PhysAddr::new(0xb8000)),
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        &mut paging::mem::EmptyAllocator,
    );

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

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