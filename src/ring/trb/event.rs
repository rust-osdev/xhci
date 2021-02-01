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

add_trb_with_default!(TransferEvent, "Transfer Event TRB", Type::TransferEvent);
impl TransferEvent {
    /// Returns the value of the TRB Pointer field.
    pub fn trb_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    /// Returns the value of the Completion Code field.
    pub fn completion_code(&self) -> u8 {
        self.0[2].get_bits(24..=31).try_into().unwrap()
    }
}

add_trb_with_default!(
    CommandCompletion,
    "Command Completion Event TRB",
    Type::CommandCompletion
);
impl CommandCompletion {
    /// Returns the value of the Slot ID field.
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }

    /// Returns the value of the Command TRB Pointer field.
    pub fn command_trb_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    /// Returns the value of the Completion Code field.
    pub fn completion_code(&self) -> u8 {
        self.0[2].get_bits(24..=31).try_into().unwrap()
    }
}
