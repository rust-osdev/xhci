//! xHCI Local Memory Capability.

use super::ExtendedCapability;
use accessor::Array;
use accessor::Mapper;
use accessor::Single;
use bit_field::BitField;
use core::convert::TryInto;

/// xHCI Local Memory Capability.
#[derive(Debug)]
pub struct XhciLocalMemory<M>
where
    M: Mapper + Clone,
{
    header: Single<Header, M>,
    memory: Array<u8, M>,
}
impl<M> XhciLocalMemory<M>
where
    M: Mapper + Clone,
{
    /// Creates an accessor to xHCI Local Memory Capability.
    ///
    /// # Safety
    ///
    /// `base` must be the correct address to xHCI Local Memory Capability.
    ///
    /// The caller must ensure that xHCI Local Memory Capability is accessed only by the created
    /// accessor.
    ///
    /// # Panics
    ///
    /// This method panics if `base` is not aligned correctly.
    pub unsafe fn new(base: usize, mapper: M) -> Self {
        let header: Single<Header, M> = Single::new(base, mapper.clone());
        let size = header.read().size();
        let memory = Array::new(base + 8, (size * 1024).try_into().unwrap(), mapper);

        Self { header, memory }
    }
}
impl<M> From<XhciLocalMemory<M>> for ExtendedCapability<M>
where
    M: Mapper + Clone,
{
    fn from(x: XhciLocalMemory<M>) -> Self {
        ExtendedCapability::XhciLocalMemory(x)
    }
}

/// The first 8 bytes of the Capability.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Header([u32; 2]);
impl Header {
    /// Returns the Local Memory Enable bit.
    #[must_use]
    pub fn local_memory_enable(self) -> bool {
        self.0[0].get_bit(16)
    }

    /// Sets the Local Memory Enable bit.
    pub fn set_local_memory_enable(&mut self, b: bool) {
        self.0[0].set_bit(16, b);
    }

    fn size(self) -> u32 {
        self.0[1]
    }
}
impl_debug_from_methods! {
    Header {
        local_memory_enable,
    }
}
