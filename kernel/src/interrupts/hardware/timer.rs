
// ===== Imports =====
use crate::*;
use crate::interrupts::hardware::{PICS, PICSInterruptIndex};
use x86_64::structures::idt::InterruptStackFrame;
// ===================

pub extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: &mut InterruptStackFrame) {
    vga::print!(".");

    // Notify that the interrupt is now processed
    unsafe {
        PICS.lock().notify_end_of_interrupt(PICSInterruptIndex::Timer.as_u8());
    }
}