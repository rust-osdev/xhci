//! Doorbell Register

use super::capability::Capability;
use accessor::array;
use accessor::Mapper;
use core::convert::TryFrom;

/// A type alias to [`Doorbell`] register for backward compability.
#[deprecated = "Use `Doorbell` instead of `Register`."]
pub type Register = Doorbell;

/// The element of the Doorbell Array.
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct Doorbell(u32);
impl Doorbell {
    /// Creates a new accessor to the Doorbell Array.
    ///
    /// # Safety
    ///
    /// Caller must ensure that the only one accessor is created, otherwise it may cause undefined
    /// behavior such as data race.
    ///
    /// # Panics
    ///
    /// This method panics if the base address of the Doorbell Array is not aligned correctly.
    pub unsafe fn new<M1, M2>(
        mmio_base: usize,
        capability: &Capability<M2>,
        mapper: M1,
    ) -> array::ReadWrite<Self, M1>
    where
        M1: Mapper,
        M2: Mapper + Clone,
    {
        let base = mmio_base + usize::try_from(capability.dboff.read_volatile().get()).unwrap();
        array::ReadWrite::new(
            base,
            capability
                .hcsparams1
                .read_volatile()
                .number_of_device_slots()
                .into(),
            mapper,
        )
    }

    rw_field!(0..=7, doorbell_target, "Doorbell Target", u8);
    rw_field!(16..=31, doorbell_stream_id, "Doorbell Stream ID", u16);
}
impl_debug_from_methods!(Doorbell {
    doorbell_target,
    doorbell_stream_id,
});
