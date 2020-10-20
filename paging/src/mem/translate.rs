
// ===== Imports =====
use x86_64::{VirtAddr, PhysAddr};
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::PageTable;
use x86_64::structures::paging::page_table::FrameError;
// ===================

pub fn virt_to_phys(virt_addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    // Get the Level-4 Page table
    let (lev4_page_table, _) = Cr3::read();

    let page_table_indexes = [
        virt_addr.p4_index(), // Table Level-4 index
        virt_addr.p3_index(), // Table Level-3 index
        virt_addr.p2_index(), // Table Level-2 index
        virt_addr.p1_index(), // Table Level-1 index
    ];

    let mut curr_frame = lev4_page_table;
    for &index in &page_table_indexes {
        let virt = physical_memory_offset + curr_frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        let entry = &table[index];
        curr_frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("Huge pages are not supported"),
        };

    }

    // Return the physical address by adding page offset.
    Some(curr_frame.start_address() + u64::from(virt_addr.page_offset()))
}