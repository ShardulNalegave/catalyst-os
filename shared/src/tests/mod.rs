
// ===== Imports =====
use crate::qemu::{self, QemuExitCode};
// ===================

/// # Tests Runner
/// Runs all the defined tests.
pub fn runner(tests: &[&dyn Testable]) {
    crate::serial_println!("Running {} tests.", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit(QemuExitCode::Success);
}

/// # Testable Trait
/// A trait which is implemented by all the tests.
pub trait Testable {
    fn run(&self) -> ();
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        crate::serial_print!("{} =>", core::any::type_name::<T>());
        self();
        crate::serial_println!(" [OK]");
    }
}