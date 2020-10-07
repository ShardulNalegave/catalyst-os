
// ===== Imports =====
use x86_64::structures::idt::InterruptDescriptorTable;
// ===================

/// # Init
/// Initializes the Interrupt Descriptor Table
pub fn init() {
    let mut idt = InterruptDescriptorTable::new();
}