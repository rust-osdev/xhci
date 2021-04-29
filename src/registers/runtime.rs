//! Host Controller Runtime Registers.

use super::capability::RuntimeRegisterSpaceOffset;
use accessor::Mapper;
use core::convert::TryFrom;

/// Runtime Registers
///
/// Note that this struct does not contain the interrupt register sets. Refer to
/// [`InterruptRegisterSet`].
#[derive(Debug)]
pub struct Runtime<M>
where
    M: Mapper,
{
    /// Microframe Index Register
    pub mfindex: accessor::Single<MicroframeIndexRegister, M>,
}
impl<M> Runtime<M>
where
    M: Mapper,
{
    /// Creates a new accessor to the Host Controller Runtime Registers.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the Host Controller Runtime Registers are accessed only through
    /// this struct.
    ///
    /// # Panics
    ///
    /// This method panics if `mmio_base` is not aligned correctly.
    pub unsafe fn new(mmio_base: usize, rtoff: RuntimeRegisterSpaceOffset, mapper: M) -> Self {
        let base = mmio_base + usize::try_from(rtoff.get()).unwrap();

        Self {
            mfindex: accessor::Single::new(base, mapper),
        }
    }
}

/// Microframe Index Register
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MicroframeIndexRegister(u32);
impl MicroframeIndexRegister {
    ro_field!(0..=13, microframe_index, "Microframe Index", u16);
}
impl_debug_from_methods! {
    MicroframeIndexRegister {
        microframe_index,
    }
}

/// Interrupt Register Set
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct InterruptRegisterSet {
    /// Interrupt Management Register
    pub iman: InterrupterManagementRegister,
    /// Interrupt Moderation Register
    pub imod: InterrupterModerationRegister,
    /// Event Ring Segment Table Size Register
    pub erstsz: EventRingSegmentTableSizeRegister,
    _rsvd: u32,
    /// Event Ring Segment Table Base Address Register
    pub erstba: EventRingSegmentTableBaseAddressRegister,
    /// Event Ring Dequeue Pointer Register
    pub erdp: EventRingDequeuePointerRegister,
}
impl InterruptRegisterSet {
    /// Creates an accessor to the Interrupt Register Set.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the Host Controller Runtime Registers are accessed only through
    /// this struct.
    ///
    /// # Panics
    ///
    /// This method panics if the base address of the Interrupt Register Sets is not aligned
    /// correctly.
    pub unsafe fn new<M>(
        mmio_base: usize,
        rtoff: RuntimeRegisterSpaceOffset,
        mapper: M,
    ) -> accessor::Array<Self, M>
    where
        M: Mapper,
    {
        const NUM_INTERRUPT_REGISTER_SET: usize = 1024;

        let base = mmio_base + usize::try_from(rtoff.get()).unwrap() + 0x20;

        accessor::Array::new(base, NUM_INTERRUPT_REGISTER_SET, mapper)
    }
}

/// Interrupter Management Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct InterrupterManagementRegister(u32);
impl InterrupterManagementRegister {
    rw1c_bit!(0, interrupt_pending, "Interrupt Pending");
    rw_bit!(1, interrupt_enable, "Interrupt Enable");
}
impl_debug_from_methods! {
    InterrupterManagementRegister {
        interrupt_pending,
        interrupt_enable,
    }
}

/// Interrupter Moderation Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct InterrupterModerationRegister(u32);
impl InterrupterModerationRegister {
    rw_field!(
        0..=15,
        interrupt_moderation_interval,
        "Interrupt Moderation Interval",
        u16
    );
    rw_field!(
        16..=31,
        interrupt_moderation_counter,
        "Interrupt Moderation Counter",
        u16
    );
}
impl_debug_from_methods! {
    InterrupterModerationRegister{
        interrupt_moderation_interval,
        interrupt_moderation_counter,
    }
}

/// Event Ring Segment Table Size Register.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct EventRingSegmentTableSizeRegister(u32);
impl EventRingSegmentTableSizeRegister {
    /// Sets the number of segments the Event Ring Segment Table supports.
    pub fn set(&mut self, s: u16) {
        self.0 = s.into();
    }
}

/// Event Ring Segment Table Base Address Register.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct EventRingSegmentTableBaseAddressRegister(u64);
impl EventRingSegmentTableBaseAddressRegister {
    /// Sets the address of the Event Ring Segment Table. It must be 64 byte aligned.
    ///
    /// # Panics
    ///
    /// This method panics if the address is not 64 byte aligned.
    pub fn set(&mut self, a: u64) {
        assert!(
            a.trailing_zeros() >= 6,
            "The Event Ring Segment Table Base Address must be 64-byte aligned."
        );
        self.0 = a;
    }
}

/// Event Ring Dequeue Pointer Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EventRingDequeuePointerRegister(u64);
impl EventRingDequeuePointerRegister {
    rw_field!(
        0..=2,
        dequeue_erst_segment_index,
        "Dequeue ERST Segment Index",
        u8
    );
    rw1c_bit!(3, event_handler_busy, "Event Handler Busy");

    /// Returns the address of the current Event Ring Dequeue Pointer.
    #[must_use]
    pub fn event_ring_dequeue_pointer(self) -> u64 {
        self.0 & 0b1111
    }

    /// Sets the address of the current Event Ring Dequeue Pointer. It must be 16 byte aligned.
    ///
    /// # Panics
    ///
    /// This method panics if the address is not 16 byte aligned.
    pub fn set_event_ring_dequeue_pointer(&mut self, p: u64) {
        assert!(
            p.trailing_zeros() >= 4,
            "The Event Ring Dequeue Pointer must be 16-byte aligned."
        );
        self.0 = p;
    }
}
impl_debug_from_methods! {
    EventRingDequeuePointerRegister{
        dequeue_erst_segment_index,
        event_handler_busy,
        event_ring_dequeue_pointer
    }
}
