//! Doorbell Register

use bit_field::BitField;
use core::{convert::TryInto, fmt};

/// The element of the Doorbell Array.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Register(u32);
impl Register {
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
