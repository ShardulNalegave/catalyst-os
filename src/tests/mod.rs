
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

#[test_case]
fn trivial_test() {
    vga_print!("Trivial Test: ");
    assert_eq!(1, 1);
    vga_println!("[OK]");
}