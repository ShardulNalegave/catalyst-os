
/// # Tests Runner
/// Runs all the defined tests.
pub fn tests_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}