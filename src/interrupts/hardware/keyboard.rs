
// ===== Imports =====
use crate::*;
use crate::interrupts::hardware::{PICS, PICSInterruptIndex};
use x86_64::structures::idt::InterruptStackFrame;
// ===================

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    vga_print!("k");

    // Notify that the interrupt is now processed
    unsafe {
        PICS.lock().notify_end_of_interrupt(PICSInterruptIndex::Keyboard.as_u8());
    }
}