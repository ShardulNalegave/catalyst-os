
// ===== Imports =====
use crate::*;
use crate::interrupts::hardware::{PICS, PICSInterruptIndex};
use x86_64::structures::idt::InterruptStackFrame;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;
use lazy_static::lazy_static;
// ===================

lazy_static! {
    /// # Keyboard
    /// A static reference to the Keyboard struct.
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1,
        HandleControl::Ignore)
    );
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    match get_key() {
        Some(key) => match key {
            DecodedKey::Unicode(ch) => vga::print!("{}", ch),
            DecodedKey::RawKey(key) => vga::print!("{:?}", key),
        },
        None => (),
    }

    // Notify that the interrupt is now processed
    unsafe {
        PICS.lock().notify_end_of_interrupt(PICSInterruptIndex::Keyboard.as_u8());
    }
}

/// # Get Key
/// Returns the PC-Keyboard Decoded Key.
fn get_key() -> Option<DecodedKey> {
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);
    let scancode = unsafe { port.read() };

    match keyboard.add_byte(scancode) {
        Ok(Some(key_event)) => keyboard.process_keyevent(key_event),
        _ => None,
    }
}