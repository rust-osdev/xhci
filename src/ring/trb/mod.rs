//! TRB (Transfer Request Block).

use bit_field::BitField;
use core::convert::TryInto;
use num_derive::FromPrimitive;

/// Link TRB.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Link([u32; 4]);
impl Link {
    /// Creates a new Link TRB.
    ///
    /// This method sets the TRB Type field with the correct Type. All other fields are 0.
    #[must_use]
    pub fn new() -> Self {
        *Self([0; 4]).set_trb_type()
    }

    /// Sets the value of the Cycle Bit.
    pub fn set_cycle_bit(&mut self, b: bool) -> &mut Self {
        self.0[3].set_bit(0, b);
        self
    }

    /// Sets the value of the Ring Segment Pointer field.
    ///
    /// # Panics
    ///
    /// This method panics if `p` is not 16-byte aligned.
    pub fn set_ring_segment_pointer(&mut self, p: u64) -> &mut Self {
        assert_eq!(
            p % 16,
            0,
            "The Ring Segment Pointer must be 16-byte aligned."
        );

        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
        self
    }

    fn set_trb_type(&mut self) -> &mut Self {
        self.0[3].set_bits(10..=15, Type::Link as _);
        self
    }
}
impl Default for Link {
    fn default() -> Self {
        Self::new()
    }
}
impl AsRef<[u32]> for Link {
    fn as_ref(&self) -> &[u32] {
        &self.0
    }
}
impl AsMut<[u32]> for Link {
    fn as_mut(&mut self) -> &mut [u32] {
        &mut self.0
    }
}
impl From<[u32; 4]> for Link {
    fn from(raw: [u32; 4]) -> Self {
        Self(raw)
    }
}

/// TRB Type.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
pub enum Type {
    /// 6
    Link = 6,
    /// 23
    Noop = 23,
}
