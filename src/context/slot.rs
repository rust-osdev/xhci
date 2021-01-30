//! Slot Context

use bit_field::BitField;

/// Slot Context.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Slot([u32; 8]);
impl Slot {
    /// Creates a null Slot Context.
    pub const fn new() -> Self {
        Self([0; 8])
    }

    /// Converts [`Slot`] into an array.
    ///
    /// Use this method if the Context Size bit of the HCCPARAMS1 register is 0.
    pub fn into_32byte(self) -> [u32; 8] {
        self.0
    }

    /// Converts [`Slot`] into an array.
    ///
    /// Use this method if the Context Size bit of the HCCPARAMS1 register is 1.
    pub fn into_64byte(self) -> [u32; 16] {
        [
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7],
            0, 0, 0, 0, 0, 0, 0, 0,
        ]
    }

    /// Sets the value of the Context Entries field.
    pub fn set_context_entries(&mut self, e: u8) -> &mut Self {
        self.0[0].set_bits(27..=31, e.into());
        self
    }

    /// Sets the value of the Root Hub Port Number field.
    pub fn set_root_hub_port_number(&mut self, n: u8) -> &mut Self {
        self.0[0].set_bits(16..=23, n.into());
        self
    }
}
impl From<[u32; 8]> for Slot {
    fn from(raw: [u32; 8]) -> Self {
        Self(raw)
    }
}
impl From<[u32; 16]> for Slot {
    fn from(raw: [u32; 16]) -> Self {
        Self([
            raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7],
        ])
    }
}
