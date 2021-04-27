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
    pub fn get(self) -> u16 {
        self.0.get_bits(0..=15).try_into().unwrap()
    }

    /// Sets the value of the Event Ring Segment Table Size field.
    pub fn set(&mut self, sz: u16) {
        self.0.set_bits(0..=15, sz.into());
    }
}
impl_debug_from_methods! {
    EventRingSegmentTableSize {
        get,
    }
}

/// Debug Capability Event Ring Segment Table Base Address Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EventRingSegmentTableBaseAddress(u64);
impl EventRingSegmentTableBaseAddress {
    /// Returns the value of the Event Ring Segment Table Base Address field.
    #[must_use]
    pub fn get(self) -> u64 {
        self.0
    }

    /// Sets the value of the Event Ring Segment Table Base Address field.
    ///
    /// # Panics
    ///
    /// This method panics if the address is not 16-byte aligned.
    pub fn set(&mut self, a: u64) {
        assert!(
            a.trailing_zeros() >= 4,
            "The base address of the Event Ring Segment Table must be 16-byte aligned."
        );

        self.0 = a;
    }
}
impl_debug_from_methods! {
    EventRingSegmentTableBaseAddress {
        get,
    }
}

/// Debug Capability Event Ring Dequeue Pointer Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EventRingDequeuePointer(u64);
impl EventRingDequeuePointer {
    /// Returns the value of the Dequeue ERST Segment Index field.
    #[must_use]
    pub fn dequeue_erst_segment_index(self) -> u8 {
        self.0.get_bits(0..=2).try_into().unwrap()
    }

    /// Sets the value of the Dequeue ERST Segment Index field.
    pub fn set_dequeue_erst_segment_index(&mut self, i: u8) {
        self.0.set_bits(0..=2, i.into());
    }

    /// Returns the value of the Dequeue Pointer field.
    #[must_use]
    pub fn dequeue_pointer(self) -> u64 {
        self.0 & !0b1111
    }

    /// Sets the value of the Dequeue Pointer field.
    ///
    /// # Panics
    ///
    /// This method panics if the address is not 16-byte aligned.
    pub fn set_dequeue_pointer(&mut self, a: u64) {
        assert!(
            a.trailing_zeros() >= 4,
            "The Event Ring Dequeue Pointer must be 16-byte aligned."
        );

        self.0.set_bits(4..=63, a.get_bits(4..=63));
    }
}
impl_debug_from_methods! {
    EventRingDequeuePointer {
        dequeue_erst_segment_index,
        dequeue_pointer,
    }
}
