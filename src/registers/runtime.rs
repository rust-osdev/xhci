//! Host Controller Runtime Registers.

use super::capability::RuntimeRegisterSpaceOffset;
use accessor::Mapper;
use bit_field::BitField;
use core::convert::TryFrom;
use core::convert::TryInto;

/// Runtime Registers
///
/// Note that this struct does not contain the interrupt register sets. Refer to
/// [`InterruptRegisterSet`].
#[derive(Debug)]
pub struct RuntimeRegisters<M>
where
    M: Mapper + Clone,
{
    /// Microframe Index Register
    pub mfindex: accessor::Single<MicroframeIndexRegister, M>,
}
impl<M> RuntimeRegisters<M>
where
    M: Mapper + Clone,
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
    pub unsafe fn new(mmio_base: usize, mapper: M) -> Self {
        Self {
            mfindex: accessor::Single::new(mmio_base, mapper),
        }
    }
}

/// Microframe Index Register
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MicroframeIndexRegister(u32);
impl MicroframeIndexRegister {
    /// Returns the value of the Microframe Index field.
    pub fn microframe_index(self) -> u16 {
        self.0.get_bits(0..=13).try_into().unwrap()
    }
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
    /// Returns the value of the Interrupter Pending bit.
    #[must_use]
    pub fn interrupt_pending(self) -> bool {
        self.0.get_bit(0)
    }

    /// Sets the value of the Interrupt Pending bit.
    pub fn set_interrupt_pending(&mut self, b: bool) {
        self.0.set_bit(0, b);
    }

    /// Returns the value of the Interrupt Enable bit.
    #[must_use]
    pub fn interrupt_enable(self) -> bool {
        self.0.get_bit(1)
    }

    /// Sets the value of the Interrupt Enable bit.
    pub fn set_interrupt_enable(&mut self, b: bool) {
        self.0.set_bit(1, b);
    }
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
    /// Returns the value of the Interrupt Moderation Interval field.
    #[must_use]
    pub fn interrupt_moderation_interval(self) -> u16 {
        self.0.get_bits(0..=15).try_into().unwrap()
    }

    /// Sets the value of the Interrupt Moderation Interval field.
    pub fn set_interrupt_moderation_interval(&mut self, interval: u16) {
        self.0.set_bits(0..=15, interval.into());
    }

    /// Returns the value of the Interrupt Moderation Counter field.
    pub fn interrupt_moderation_counter(self) -> u16 {
        self.0.get_bits(16..=31).try_into().unwrap()
    }

    /// Sets the value of the Interrupt Moderation Counter field.
    pub fn set_interrupt_moderation_counter(&mut self, counter: u16) {
        self.0.set_bits(16..=31, counter.into());
    }
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
        assert!(a.trailing_zeros() >= 6);
        self.0 = a;
    }
}

/// Event Ring Dequeue Pointer Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EventRingDequeuePointerRegister(u64);
impl EventRingDequeuePointerRegister {
    /// Returns the value of the Dequeue ERST Segment Index field.
    pub fn dequeue_erst_segment_index(self) -> u8 {
        self.0.get_bits(0..=2).try_into().unwrap()
    }

    /// Sets the value of the Dequeue ERST Segment Index field.
    pub fn set_dequeue_erst_segment_index(&mut self, i: u8) {
        self.0.set_bits(0..=2, i.into());
    }

    /// Returns the value of the Event Handler Busy bit.
    pub fn event_handler_busy(self) -> bool {
        self.0.get_bit(3)
    }

    /// Clears the Event Handler Busy bit.
    pub fn clear_event_handler_busy(&mut self) {
        self.0.set_bit(3, true);
    }

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
        assert!(p.trailing_zeros() >= 4);
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
