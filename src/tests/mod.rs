
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
    serial1_print!("Trivial Test: ");
    vga_print!("Trivial Test: ");
    assert_eq!(1, 1);
    serial1_println!("[OK]");
    vga_println!("[OK]");
}