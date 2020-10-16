
/// # Tests Runner
/// Runs all the defined tests.
pub fn runner(tests: &[&dyn Testable]) {
    vga::println!("Running {} tests.", tests.len());
    for test in tests {
        test.run();
    }
}

/// # Testable Trait
/// A trait which is implemented by all the tests.
pub trait Testable {
    fn run(&self) -> ();
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        vga::print!("{} =>", core::any::type_name::<T>());
        self();
        vga::println!(" [OK]");
    }
}