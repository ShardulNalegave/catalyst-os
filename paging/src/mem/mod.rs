
/// Allocators module
mod allocators;
pub use allocators::*;

// ===== Imports =====
use x86_64::VirtAddr;
use x86_64::structures::paging::{PageTable, OffsetPageTable};
use x86_64::registers::control::Cr3;
// ===================

/// # Init
/// Initialises a new `OffsetPageTable`. Should be called only once.
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let lev4_page_table = active_level_4_page_table(physical_memory_offset);
    OffsetPageTable::new(lev4_page_table, physical_memory_offset)
}

/// # Active Level-4 Page Table
/// Returns a mutable reference to the currently in use level-4 page table.
unsafe fn active_level_4_page_table(
    physical_memory_offset: VirtAddr
) -> &'static mut PageTable {
    // The Cr3 register contains the address for level 4 page table
    let (lev4_page_table, _) = Cr3::read();
    let phys = lev4_page_table.start_address();

    // Virtual address will be the physical address with the offset
    let virt = physical_memory_offset + phys.as_u64();

    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    &mut *page_table_ptr // unsafe
}