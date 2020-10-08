
/// GDT module
pub mod gdt;

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
        idt.breakpoint.set_handler_fn(breakpoint::breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault::double_fault_handler);
        idt
    };
}

/// # Load IDT
pub fn load_idt() {
    IDT.load();
}