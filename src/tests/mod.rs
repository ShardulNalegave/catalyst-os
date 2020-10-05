
// ===== Imports =====
use crate::*;
// ===================

/// # Test Runner
#[cfg(test)]
pub fn runner(tests: &[&dyn Fn()]) {
    vga_println!("Running {} tests.", tests.len());
    for test in tests {
        test();
    }
}