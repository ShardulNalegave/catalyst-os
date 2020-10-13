
// ===== Imports =====
use crate::*;
use crate::interrupts::hardware::{PICS, PICSInterruptIndex};
use x86_64::structures::idt::InterruptStackFrame;
// ===================

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    vga_print!("{}", get_scancode());

    // Notify that the interrupt is now processed
    unsafe {
        PICS.lock().notify_end_of_interrupt(PICSInterruptIndex::Keyboard.as_u8());
    }
}

/// # Get Scancode
/// Scans the keyboard port to receive a scancode i.e. keycode basically.
fn get_scancode() -> u8 {
    use x86_64::instructions::port::Port;
    unsafe { Port::new(0x60).read() }
}