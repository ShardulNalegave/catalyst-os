
// ===== Imports =====
use x86_64::structures::paging::{Page, FrameAllocator, OffsetPageTable, Size4KiB, MapperAllSizes, PageSize};
use x86_64::{PhysAddr, VirtAddr};
// ===================

/// # Translator Trait
/// Methods related to translation of addresses.
pub trait Translator {
    /// Translated virtual addresses to physical ones.
    fn virt_to_phys(&self, virt: VirtAddr) -> Option<PhysAddr>;
}

/// # Mapper Trait
/// Methods related to mapping.
pub trait Mapper {
    /// Creates a mapping of a virtual page to a physical address using the provided frame allocator.
    fn map<T: PageSize>(&mut self, page: Page, to: PhysAddr, allocator: &impl FrameAllocator<T>);
}

// ===========================================================================================

/// # Manager
/// Memory manager. Performs all memory related stuff.
pub struct Manager {
    offset_page_table: OffsetPageTable<'static>,
}

impl Manager {
    /// ## Constructor
    pub(crate) fn create(offset_page_table: OffsetPageTable<'static>) -> Self {
        Self { offset_page_table }
    }
}

impl Translator for Manager {
    fn virt_to_phys(&self, virt: VirtAddr) -> Option<PhysAddr> {
        self.offset_page_table.translate_addr(virt)
    }
}

impl Mapper for Manager {
    fn map<T: PageSize>(&mut self, _page: Page<Size4KiB>, _to: PhysAddr, _allocator: &impl FrameAllocator<T>) {
        unimplemented!()
    }
}