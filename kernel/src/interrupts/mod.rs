
/// GDT module
pub mod gdt;
/// GDT
pub use gdt::GDT;

/// Hardware module
pub mod hardware;

/// Breakpoint Exception module
pub mod breakpoint;

/// Double-Fault Exception module
pub mod double_fault;

// ===== Imports =====
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
// ===================

lazy_static! {
    /// # IDT
    /// Initializes the Interrupt Descriptor Table and sets the handlers.
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // Breakpoint exception
        idt.breakpoint.set_handler_fn(breakpoint::breakpoint_handler);

        // Page-Fault exception
        idt.page_fault.set_handler_fn(paging::page_fault::page_fault_handler);

        // Double-Fault exception
        unsafe { idt.double_fault.set_handler_fn(double_fault::double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); }

        // Timer interrupt
        idt[hardware::PICSInterruptIndex::Timer.as_usize()]
            .set_handler_fn(hardware::timer::timer_interrupt_handler);

        // Keyboard interrupt
        idt[hardware::PICSInterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(hardware::keyboard::keyboard_interrupt_handler);

        // Return the idt
        idt
    };
}

/// # Load IDT
pub fn load_idt() {
    gdt::init();
    IDT.load();
    unsafe { hardware::PICS.lock().initialize(); };
    x86_64::instructions::interrupts::enable();
}