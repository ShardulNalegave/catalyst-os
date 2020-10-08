
// ===== Imports =====
use pic8259_simple::ChainedPics;
use spin;
// ===================

/// The Start IO-port for PIC. (Starts at 32 because all the previous ones are used for CPU-Exceptions)
pub const PIC_1_OFFSET: u8 = 32;
/// The End IO-port for PIC
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

/// # PICS
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });