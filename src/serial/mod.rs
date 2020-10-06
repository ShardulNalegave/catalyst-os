
/// Macros module
pub mod macros;

// ===== Imports =====
use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;
// ===================

lazy_static! {
    /// Serial 1
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print_serial_1(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}