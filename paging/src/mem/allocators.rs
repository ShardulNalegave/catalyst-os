
// ===== Imports =====
use x86_64::structures::paging::{FrameAllocator, Size4KiB, PhysFrame};
// ===================

/// # EmptyAllocator
/// Always returns None.
pub struct EmptyAllocator;
unsafe impl FrameAllocator<Size4KiB> for EmptyAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}