
// ===== Imports =====
use x86_64::instructions::interrupts;
// ===================

#[test_case]
fn test_println_simple() {
    vga::println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        vga::println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    
    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = vga::WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[vga::BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.char), c);
        }
    });
}