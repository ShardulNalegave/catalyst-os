
/// VGA module
pub mod vga;

// ===== Imports =====
use shared::qemu::{self, QemuExitCode};
// ===================

/// # Tests Runner
/// Runs all the defined tests.
pub fn runner(tests: &[&dyn Testable]) {
    shared::serial_println!("Running {} tests.", tests.len());
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
        shared::serial_print!("{} =>", core::any::type_name::<T>());
        self();
        shared::serial_println!(" [OK]");
    }
}