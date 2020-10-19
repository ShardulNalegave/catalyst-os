
/// # Page-Fault Exception Handler
pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;

    vga::println!("EXCEPTION: PAGE FAULT");
    vga::println!("Accessed Address: {:?}", Cr2::read());
    vga::println!("Error Code: {:?}", error_code);
    vga::println!("{:#?}", stack_frame);
    // hlt_loop();
}