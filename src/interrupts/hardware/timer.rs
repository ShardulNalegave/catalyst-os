
// ===== Imports =====
use crate::*;
use x86_64::structures::idt::InterruptStackFrame;
// ===================

pub extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: &mut InterruptStackFrame) {
    vga_println!("timer interrupt.");
}