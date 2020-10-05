
// ===== Imports =====
use crate::vga::ColorCode;
// ===================

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

/// # VGA Character
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VGAChar {
    ascii_character: u8,
    color_code: ColorCode,
}

/// # The VGA Buffer
pub struct Buffer {}