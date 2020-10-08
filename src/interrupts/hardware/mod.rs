
/// Timer Interrupt module
pub mod timer;

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

/// # PICS Interrupt Index
/// IO-port indexes for different devices.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PICSInterruptIndex {
    Timer = PIC_1_OFFSET,
}

impl PICSInterruptIndex {
    /// ## As u8
    /// Return as `u8`
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// ## As usize
    /// Return as `usize`
    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}