//! Event TRBs.

use bit_field::BitField;
use core::convert::TryInto;

add_trb_with_default!(
    PortStatusChange,
    "Port Status Change Event TRB",
    Type::PortStatusChange
);
impl PortStatusChange {
    /// Returns the value of the Port ID field.
    pub fn port_id(&self) -> u8 {
        self.0[0].get_bits(24..=31).try_into().unwrap()
    }
}
