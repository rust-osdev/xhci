//! Debug Capability.

use bit_field::BitField;
use core::convert::TryInto;

/// Debug Capability ID Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IdRegister(u32);
impl IdRegister {
    /// Returns the value of the Debug Capability Event Ring Segment Table Max field.
    #[must_use]
    pub fn debug_capability_event_ring_segment_table_max(self) -> u8 {
        self.0.get_bits(16..=20).try_into().unwrap()
    }
}
impl_debug_from_methods! {
    IdRegister {
        debug_capability_event_ring_segment_table_max,
    }
}
