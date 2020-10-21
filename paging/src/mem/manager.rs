
// ===== Imports =====
use x86_64::structures::paging::{Page, FrameAllocator, OffsetPageTable, MapperAllSizes, PageSize, PhysFrame, PageTableFlags, Size4KiB, Mapper};
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
pub trait MemMapper {
    /// Creates a mapping of a virtual page to a physical address using the provided frame allocator.
    fn map<T: PageSize>(
        &mut self,
        page: Page<T>,
        frame: PhysFrame<T>,
        flags: PageTableFlags,
        allocator: &mut impl FrameAllocator<Size4KiB>,
    );
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

impl MemMapper for Manager {
    fn map<T: PageSize>(
        &mut self,
        page: Page<T>,
        frame: PhysFrame<T>,
        flags: PageTableFlags,
        allocator: &mut impl FrameAllocator<Size4KiB>,
    ) {
        let result = unsafe {
            self.offset_page_table.map_to(page, frame, flags, allocator)
        };
        result.unwrap_or_else(|_| panic!("Failed to map!")).flush();
    }
}