//! Event TRBs.

use bit_field::BitField;
use core::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// TRBs which are allowed to be pushed to the Event Ring.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Allowed {
    /// Port Status Change Event TRB.
    PortStatusChange(PortStatusChange),
    /// Transfer Event TRB.
    TransferEvent(TransferEvent),
    /// Command Completion Event TRB.
    CommandCompletion(CommandCompletion),
}
impl Allowed {
    /// Returns the field of the Command Completion field.
    ///
    /// # Errors
    ///
    /// This method may return an [`Err`] value with the Completion Code that is either reserved or
    /// not implemented by this crate.
    pub fn completion_code(&self) -> Result<CompletionCode, u8> {
        match self {
            Self::PortStatusChange(p) => p.completion_code(),
            Self::TransferEvent(t) => t.completion_code(),
            Self::CommandCompletion(c) => c.completion_code(),
        }
    }
}
impl AsRef<[u32]> for Allowed {
    fn as_ref(&self) -> &[u32] {
        match self {
            Self::PortStatusChange(p) => p.as_ref(),
            Self::TransferEvent(t) => t.as_ref(),
            Self::CommandCompletion(c) => c.as_ref(),
        }
    }
}
impl AsMut<[u32]> for Allowed {
    fn as_mut(&mut self) -> &mut [u32] {
        match self {
            Self::PortStatusChange(ref mut p) => p.as_mut(),
            Self::TransferEvent(ref mut t) => t.as_mut(),
            Self::CommandCompletion(ref mut c) => c.as_mut(),
        }
    }
}

macro_rules! completion_code {
    ($name:ident) => {
        impl $name {
            /// Returns the Completion Code.
            ///
            /// # Errors
            ///
            /// This method may return an [`Err`] value with the Completion Code that is either reserved or
            /// not implemented by this crate.
            #[must_use]
            pub fn completion_code(&self) -> Result<CompletionCode, u8> {
                let c: u8 = self.0[2].get_bits(24..=31).try_into().unwrap();
                CompletionCode::from_u8(c).ok_or(c)
            }
        }
    };
}

add_trb_with_default!(
    PortStatusChange,
    "Port Status Change Event TRB",
    Type::PortStatusChange
);
completion_code!(PortStatusChange);
impl PortStatusChange {
    /// Returns the value of the Port ID field.
    #[must_use]
    pub fn port_id(&self) -> u8 {
        self.0[0].get_bits(24..=31).try_into().unwrap()
    }
}

add_trb_with_default!(TransferEvent, "Transfer Event TRB", Type::TransferEvent);
completion_code!(TransferEvent);
impl TransferEvent {
    /// Returns the value of the TRB Pointer field.
    #[must_use]
    pub fn trb_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }
}

add_trb_with_default!(
    CommandCompletion,
    "Command Completion Event TRB",
    Type::CommandCompletion
);
completion_code!(CommandCompletion);
impl CommandCompletion {
    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }

    /// Returns the value of the Command TRB Pointer field.
    #[must_use]
    pub fn command_trb_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }
}

/// The Completion Code.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
#[non_exhaustive]
pub enum CompletionCode {
    /// The operation succeed.
    Success = 1,
}
