
// ===== Imports =====
use crate::*;
use x86_64::structures::idt::InterruptStackFrame;
// ===================

/// # Breakpoint Exception Handler
pub extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame) {
    vga_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}