
/// # Tests Runner
/// Runs all the defined tests.
pub fn runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}