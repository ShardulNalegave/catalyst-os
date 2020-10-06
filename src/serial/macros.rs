
/// Prints to the host through the serial 1 interface.
#[macro_export]
macro_rules! serial1_print {
    ($($arg:tt)*) => {
        $crate::serial::_print_serial_1(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial 1 interface, appending a newline.
#[macro_export]
macro_rules! serial1_println {
    () => ($crate::serial1_print!("\n"));
    ($fmt:expr) => ($crate::serial1_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial1_print!(
        concat!($fmt, "\n"), $($arg)*));
}