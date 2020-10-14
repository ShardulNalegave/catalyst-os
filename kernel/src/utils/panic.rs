
// ===== Imports =====
use core::panic::PanicInfo;
// ===================

/// # Panic Function
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
  vga::println!("{}", info);
  crate::utils::halt_loop();
}