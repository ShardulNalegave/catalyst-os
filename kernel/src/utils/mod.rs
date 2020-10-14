
/// Panic module
pub mod panic;

/// # Halt Loop
pub fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}