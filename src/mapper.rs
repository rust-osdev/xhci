//! Memory mapper module.

/// A mapper trait to access physical memory.
pub trait Mapper {
    /// Maps `bytes` bytes of physical memory region starting from `phys_start` and returns the
    /// first virtual address.
    ///
    /// # Safety
    ///
    /// Caller must ensure that no more than one virtual address points to the same address,
    /// otherwise it may cause undefined behaviors such as creating multiple `&mut` references to
    /// the same object.
    unsafe fn map(phys_start: usize, bytes: usize) -> usize;

    /// Unmaps `bytes` bytes of virtual memory region starting from `virt_start`.
    fn unmap(virt_start: usize, bytes: usize);
}
