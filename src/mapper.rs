//! Abstractions for accessing xHCI registers on memory.

/// An interface for mapping and unmapping physical memory.
pub unsafe trait Mapper {
    /// Maps `bytes` bytes from the physical address `phys_start` and returns the mapped virtual
    /// address.
    ///
    /// # Safety
    ///
    /// Caller must not call this function more than once for a region, otherwise more than once
    /// mutable references to the same region will be created.
    unsafe fn map_pages(&mut self, phys_start: usize, bytes: usize) -> usize;

    /// Unmaps `bytes` bytes from the virtual address `virt_start`.
    fn unmap_pages(&mut self, virt_start: usize, bytes: usize);
}
