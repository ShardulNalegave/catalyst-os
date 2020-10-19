
// ===== Imports =====
use core::panic::PanicInfo;
// ===================

/// # Panic Function
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
  vga::println!("{}", info);
  shared::utils::halt_loop();
}