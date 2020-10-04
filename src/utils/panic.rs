
// ===== Imports =====
use core::panic::PanicInfo;
// ===================

/// # Panic Function
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
  loop {}
}