use memory_addr::{PhysAddr, VirtAddr};
use page_table_multiarch::MappingFlags;
use page_table_multiarch::PagingHandler;

use crate::backend::Backend;
use crate::npt::NestedPageTable as PageTable;

impl Backend {
    /// Creates a new linear mapping backend.
    pub const fn new_linear(pa_va_offset: usize) -> Self {
        Self::Linear { pa_va_offset }
    }

    pub(crate) fn map_linear<H: PagingHandler>(
        &self,
        start: VirtAddr,
        size: usize,
        flags: MappingFlags,
        pt: &mut PageTable<H>,
        pa_va_offset: usize,
    ) -> bool {
        let pa_start = PhysAddr::from(start.as_usize() - pa_va_offset);
        debug!(
            "map_linear: [{:#x}, {:#x}) -> [{:#x}, {:#x}) {:?}",
            start,
            start + size,
            pa_start,
            pa_start + size,
            flags
        );
        pt.map_region(
            start,
            |va| PhysAddr::from(va.as_usize() - pa_va_offset),
            size,
            flags,
            false,
            false,
        )
        .is_ok()
    }

    pub(crate) fn unmap_linear<H: PagingHandler>(
        &self,
        start: VirtAddr,
        size: usize,
        pt: &mut PageTable<H>,
        _pa_va_offset: usize,
    ) -> bool {
        debug!("unmap_linear: [{:#x}, {:#x})", start, start + size);
        pt.unmap_region(start, size, true).is_ok()
    }
}
