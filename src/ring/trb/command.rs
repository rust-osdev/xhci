//! Command TRBs.

use super::Type;
use bit_field::BitField;

/// No Op Command TRB.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Noop([u32; 4]);
impl Noop {
    /// Creates a new No Op Command TRB.
    ///
    /// This method sets the TRB Type field. All other fields are 0.
    #[must_use]
    pub fn new() -> Self {
        *Self([0; 4]).set_trb_type()
    }

    /// Sets the value of the Cycle Bit.
    pub fn set_cycle_bit(&mut self, b: bool) -> &mut Self {
        self.0[3].set_bit(0, b);
        self
    }

    fn set_trb_type(&mut self) -> &mut Self {
        self.0[3].set_bits(10..=15, Type::NoopCommand as _);
        self
    }
}
impl Default for Noop {
    fn default() -> Self {
        Self::new()
    }
}
impl AsRef<[u32]> for Noop {
    fn as_ref(&self) -> &[u32] {
        &self.0
    }
}
impl AsMut<[u32]> for Noop {
    fn as_mut(&mut self) -> &mut [u32] {
        &mut self.0
    }
}
impl From<[u32; 4]> for Noop {
    fn from(raw: [u32; 4]) -> Self {
        Self(raw)
    }
}
