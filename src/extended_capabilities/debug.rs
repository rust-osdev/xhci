//! Debug Capability.

use bit_field::BitField;
use core::convert::TryInto;

/// Debug Capability ID Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Id(u32);
impl Id {
    /// Returns the value of the Debug Capability Event Ring Segment Table Max field.
    #[must_use]
    pub fn debug_capability_event_ring_segment_table_max(self) -> u8 {
        self.0.get_bits(16..=20).try_into().unwrap()
    }
}
impl_debug_from_methods! {
    Id {
        debug_capability_event_ring_segment_table_max,
    }
}

/// Debug Capability Doorbell Register.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Doorbell(u32);
impl Doorbell {
    /// Sets the value of the Doorbell Target field.
    pub fn set_doorbell_target(&mut self, target: u8) {
        self.0.set_bits(8..=15, target.into());
    }
}

/// Debug Capability Event Ring Segment Table Size Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EventRingSegmentTableSize(u32);
impl EventRingSegmentTableSize {
    /// Returns the value of the Event Ring Segment Table Size field.
    #[must_use]
    pub fn event_ring_segment_table_size(self) -> u16 {
        self.0.get_bits(0..=15).try_into().unwrap()
    }

    /// Sets the value of the Event Ring Segment Table Size field.
    pub fn set_event_ring_segment_table_size(&mut self, sz: u16) {
        self.0.set_bits(0..=15, sz.into());
    }
}
impl_debug_from_methods! {
    EventRingSegmentTableSize {
        event_ring_segment_table_size,
    }
}
