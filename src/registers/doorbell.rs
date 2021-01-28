//! Doorbell Register

use super::capability::Capability;
use accessor::Mapper;
use bit_field::BitField;
use core::{
    convert::{TryFrom, TryInto},
    fmt,
};

/// The element of the Doorbell Array.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Register(u32);
impl Register {
    /// Creates a new accessor to the Doorbell Array.
    ///
    /// # Safety
    ///
    /// Caller must ensure that the only one accessor is created, otherwise it may cause undefined
    /// behavior such as data race.
    ///
    /// # Errors
    ///
    /// This method may return a [`accessor::Error::NotAligned`] error if the start of the Doorbell
    /// Array is not aligned.
    pub unsafe fn new<M1, M2>(
        mmio_base: usize,
        capability: &Capability<M2>,
        mapper: M1,
    ) -> Result<accessor::Array<Self, M1>, accessor::Error>
    where
        M1: Mapper,
        M2: Mapper + Clone,
    {
        let base = mmio_base + usize::try_from(capability.dboff.read().get()).unwrap();
        accessor::Array::new(
            base,
            capability.hcsparams1.read().number_of_device_slots().into(),
            mapper,
        )
    }
    /// Get a doorbell target.
    #[must_use]
    pub fn doorbell_target(self) -> u8 {
        self.0.get_bits(0..=7).try_into().unwrap()
    }

    /// Set a doorbell target.
    pub fn set_doorbell_target(&mut self, target: u8) {
        self.0.set_bits(0..=7, target.into());
    }
}
impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("doorbell::Register")
            .field("doorbell_target", &self.doorbell_target())
            .finish()
    }
}
