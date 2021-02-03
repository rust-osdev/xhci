//! Event TRBs.

use bit_field::BitField;
use core::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

allowed! {
    /// TRBs which are allowed to be pushed to the Event Ring.
    enum {
        /// Port Status Change Event TRB.
        PortStatusChange,
        /// Transfer Event TRB.
        TransferEvent,
        /// Command Completion Event TRB.
        CommandCompletion
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

    /// Returns the value of the TRB Transfer Length field.
    #[must_use]
    pub fn trb_transfer_length(&self) -> u32 {
        self.0[2].get_bits(0..=23)
    }

    /// Returns the value of the Event Data field.
    #[must_use]
    pub fn event_data(&self) -> bool {
        self.0[3].get_bit(2)
    }

    /// Returns the value of the Endpoint ID field.
    #[must_use]
    pub fn endpoint_id(&self) -> u8 {
        self.0[3].get_bits(16..=20).try_into().unwrap()
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}

add_trb_with_default!(
    CommandCompletion,
    "Command Completion Event TRB",
    Type::CommandCompletion
);
completion_code!(CommandCompletion);
impl CommandCompletion {
    /// Returns the value of the Command TRB Pointer field.
    #[must_use]
    pub fn command_trb_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    /// Returns the value of the Command Completion Parameter field.
    #[must_use]
    pub fn command_completion_parameter(&self) -> u32 {
        self.0[2].get_bits(0..=23)
    }

    /// Returns the value of the VF (Virtual Function) ID field.
    #[must_use]
    pub fn vf_id(&self) -> u8 {
        self.0[3].get_bits(16..=23).try_into().unwrap()
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}

add_trb_with_default!(
    BandwidthRequest,
    "Bandwidth Request Event TRB",
    Type::BandwidthRequest
);
completion_code!(BandwidthRequest);
impl BandwidthRequest {
    /// Returns the value of the Slot ID field.
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}

add_trb_with_default!(Doorbell, "Doorbell Event TRB", Type::Doorbell);
completion_code!(Doorbell);
impl Doorbell {
    /// Returns the value of the DB Reason field.
    pub fn db_reason(&self) -> u8 {
        self.0[0].get_bits(0..=4).try_into().unwrap()
    }
}

add_trb_with_default!(
    HostController,
    "Host Controller Event TRB",
    Type::HostController
);
completion_code!(HostController);

add_trb_with_default!(
    DeviceNotification,
    "Device Notification Event TRB",
    Type::DeviceNotification
);
completion_code!(DeviceNotification);
impl DeviceNotification {
    /// Returns the value of the Notification Type field.
    pub fn notification_type(&self) -> u8 {
        self.0[0].get_bits(4..=7).try_into().unwrap()
    }

    /// Returns the value of the Device Notification Data field.
    pub fn device_notification_data(&self) -> u64 {
        let l: u64 = self.0[0].get_bits(8..=31).into();
        let u: u64 = self.0[1].into();

        ((u << 32) | l) >> 8
    }

    /// Returns the value of the Slot ID field.
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}

add_trb_with_default!(MfindexWrap, "MFINDEX Wrap Event TRB", Type::MfindexWrap);
completion_code!(MfindexWrap);

/// The Completion Code.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
#[non_exhaustive]
pub enum CompletionCode {
    /// The operation succeed.
    Success = 1,
}
