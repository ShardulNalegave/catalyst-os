
/// # Tests Runner
/// Runs all the defined tests.
pub fn runner(tests: &[&dyn Fn()]) {
    vga::println!("Running {} tests.", tests.len());
    for test in tests {
        test();
    }
}