
// ===== Imports =====
use crate::*;
use core::panic::PanicInfo;
// ===================

/// # Panic Function
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
  vga_println!("{}", info);
  crate::utils::halt_loop();
}