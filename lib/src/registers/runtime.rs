//! Host Controller Runtime Registers.

use super::capability::RuntimeRegisterSpaceOffset;
use accessor::marker::AccessorTypeSpecifier;
use accessor::marker::ReadOnly;
use accessor::marker::ReadWrite;
use accessor::marker::Readable;
use accessor::single;
use accessor::Mapper;
use core::convert::TryFrom;
use core::convert::TryInto;
use core::marker::PhantomData;

/// Runtime Registers
///
/// Note that this struct does not contain the interrupter register sets. Refer to
/// [`InterrupterRegisterSet`].
#[derive(Debug)]
pub struct Runtime<M>
where
    M: Mapper,
{
    /// Microframe Index Register
    pub mfindex: single::ReadWrite<MicroframeIndexRegister, M>,
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
            mfindex: single::ReadWrite::new(base, mapper),
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

/// Interrupter Register Set
#[repr(C)]
#[derive(Debug)]
pub struct InterrupterRegisterSet<M>
where
    M: Mapper + Clone,
{
    base: usize,
    mapper: M,
}

impl<M> InterrupterRegisterSet<M>
where
    M: Mapper + Clone,
{
    /// Creates an accessor to the Interrupter Register Set.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the Host Controller Runtime Registers are accessed only through
    /// this struct.
    ///
    /// # Panics
    ///
    /// This method panics if the base address of the Interrupter Register Sets is not aligned
    /// correctly.
    pub unsafe fn new(mmio_base: usize, rtoff: RuntimeRegisterSpaceOffset, mapper: M) -> Self {
        let base = mmio_base + usize::try_from(rtoff.get()).unwrap() + 0x20;
        assert!(base % 0x20 == 0, "base is not aligned");

        Self { base, mapper }
    }

    /// Returns a handler for an interrupter.
    ///
    /// # Panics
    ///
    /// This method panics if `index > 1023`.
    pub fn interrupter(&self, index: usize) -> Interrupter<'_, M, ReadOnly> {
        unsafe { Interrupter::new(self.base, index, self.mapper.clone()) }
    }

    /// Returns a mutable handler for an interrupter.
    ///
    /// # Panics
    ///
    /// This method panics if `index > 1023`.
    pub fn interrupter_mut(&mut self, index: usize) -> Interrupter<'_, M, ReadWrite> {
        unsafe { Interrupter::new(self.base, index, self.mapper.clone()) }
    }
}

/// Interrupter
#[derive(Debug)]
pub struct Interrupter<'a, M, A>
where
    M: Mapper + Clone,
    A: AccessorTypeSpecifier + Readable,
{
    /// Interrupter Management Register
    pub iman: single::Generic<InterrupterManagementRegister, M, A>,
    /// Interrupter Moderation Register
    pub imod: single::Generic<InterrupterModerationRegister, M, A>,
    /// Event Ring Segment Table Size Register
    pub erstsz: single::Generic<EventRingSegmentTableSizeRegister, M, A>,
    /// Event Ring Segment Table Base Address Register
    pub erstba: single::Generic<EventRingSegmentTableBaseAddressRegister, M, A>,
    /// Event Ring Dequeue Pointer Register
    pub erdp: single::Generic<EventRingDequeuePointerRegister, M, A>,
    // Tie the lifetime of this Interrupter to the parent InterrupterRegisterSet.
    // This prevents multiple mutable handlers from being created.
    _marker: PhantomData<&'a InterrupterRegisterSet<M>>,
}

impl<M, A> Interrupter<'_, M, A>
where
    M: Mapper + Clone,
    A: AccessorTypeSpecifier + Readable,
{
    /// Creates an accessor to an interrupter.
    ///
    /// # Safety
    ///
    /// Any mutable handlers to this Interrupter must be unique.
    ///
    /// # Panics
    ///
    /// This method panics if `index > 1023`.
    unsafe fn new(interrupter_register_set_base: usize, index: usize, mapper: M) -> Self {
        assert!(index < 1024, "index out of range");
        let base = interrupter_register_set_base + index * 0x20;
        Self {
            iman: single::Generic::new(base, mapper.clone()),
            imod: single::Generic::new(base + 0x4, mapper.clone()),
            erstsz: single::Generic::new(base + 0x8, mapper.clone()),
            erstba: single::Generic::new(base + 0x10, mapper.clone()),
            erdp: single::Generic::new(base + 0x18, mapper),
            _marker: PhantomData,
        }
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
#[derive(Copy, Clone, Default)]
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
    /// Returns the number of segments the Event Ring Segment Table supports.
    #[must_use]
    pub fn get(self) -> u16 {
        self.0.try_into().unwrap()
    }

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
    /// Returns the base address of the Event Ring Segment Table.
    #[must_use]
    pub fn get(self) -> u64 {
        self.0
    }

    /// Sets the base address of the Event Ring Segment Table. It must be 64 byte aligned.
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
#[derive(Copy, Clone, Default)]
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
        self.0 & !0b1111
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
