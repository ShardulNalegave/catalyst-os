
// ===== Imports =====
use volatile::Volatile;
use crate::ColorCode;
// ===================

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

/// # VGA Character
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VGAChar {
    pub char: u8,
    pub color_code: ColorCode,
}

/// # The VGA Buffer
pub struct Buffer {
    pub chars: [[Volatile<VGAChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}