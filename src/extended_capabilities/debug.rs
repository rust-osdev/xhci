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

/// Debug Capability Control Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Control(u32);
impl Control {
    /// Returns the DbC Run bit.
    #[must_use]
    pub fn dbc_run(self) -> bool {
        self.0.get_bit(0)
    }

    /// Returns the Link Status Event Enable bit.
    #[must_use]
    pub fn link_status_event_enable(self) -> bool {
        self.0.get_bit(1)
    }

    /// Sets the Link Status Event Enable bit.
    pub fn set_link_status_event_enable(&mut self, b: bool) {
        self.0.set_bit(1, b);
    }

    /// Returns the Halt OUT TR bit.
    #[must_use]
    pub fn halt_out_tr(self) -> bool {
        self.0.get_bit(2)
    }

    /// Sets the Halt OUT TR bit.
    ///
    /// This bit is RW1S.
    pub fn set_halt_out_tr(&mut self) {
        self.0.set_bit(2, true);
    }

    /// Returns the Halt IN TR bit.
    #[must_use]
    pub fn halt_in_tr(self) -> bool {
        self.0.get_bit(3)
    }

    /// Sets the Halt IN TR bit.
    ///
    /// This bit is RW1S.
    pub fn set_halt_in_tr(&mut self) {
        self.0.set_bit(3, true);
    }

    /// Returns the DbC Run Change bit.
    #[must_use]
    pub fn dbc_run_change(self) -> bool {
        self.0.get_bit(4)
    }

    /// Clears the DbC Run Change bit.
    pub fn clear_dbc_run_change(&mut self) {
        self.0.set_bit(4, true);
    }

    /// Returns the value of the Debug Max Burst Size field.
    #[must_use]
    pub fn debug_max_burst_size(self) -> u8 {
        self.0.get_bits(16..=23).try_into().unwrap()
    }

    /// Returns the value of the Device Address field.
    #[must_use]
    pub fn device_address(self) -> u8 {
        self.0.get_bits(24..=30).try_into().unwrap()
    }

    /// Returns the Debug Capability Enable bit.
    #[must_use]
    pub fn debug_capability_enable(self) -> bool {
        self.0.get_bit(31)
    }

    /// Sets the Debug Capability Enable bit.
    pub fn set_debug_capability_enable(&mut self, b: bool) {
        self.0.set_bit(31, b);
    }
}
impl_debug_from_methods! {
    Control {
        dbc_run,
        link_status_event_enable,
        halt_out_tr,
        halt_in_tr,
        dbc_run_change,
        debug_max_burst_size,
        device_address,
        debug_capability_enable,
    }
}
