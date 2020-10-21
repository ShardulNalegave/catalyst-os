
// ===== Imports =====
use x86_64::structures::paging::{Page, FrameAllocator};
use x86_64::{PhysAddr};
// ===================

pub trait Translator {}
pub trait Mapper {
    fn map<T>(page: Page, to: PhysAddr, allocation: &impl FrameAllocator<T>);
}

pub struct Manager {
    //
}

impl Manager {}
impl ManagerTrait for Manager {}