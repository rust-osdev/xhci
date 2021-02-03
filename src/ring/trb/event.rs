//! Event TRBs.

use bit_field::BitField;
use core::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

allowed! {
    /// TRBs which are allowed to be pushed to the Event Ring.
    enum {
        /// Transfer Event TRB.
        TransferEvent,
        /// Command Completion Event TRB.
        CommandCompletion,
        /// Port Status Change Event TRB.
        PortStatusChange,
        /// Bandwidth Request Event TRB.
        BandwidthRequest,
        /// Doorbell Event TRB.
        Doorbell,
        /// Host Controller Event TRB.
        HostController,
        /// Device Notification Event TRB.
        DeviceNotification,
        /// MFINDEX Wrap Event TRB.
        MfindexWrap
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
macro_rules! event {
    ($name:ident,$full:expr,$ty:expr) => {
        add_trb_with_default!($name, $full, $ty);
        completion_code!($name);
    };
}

event!(
    PortStatusChange,
    "Port Status Change Event TRB",
    Type::PortStatusChange
);
impl PortStatusChange {
    /// Returns the value of the Port ID field.
    #[must_use]
    pub fn port_id(&self) -> u8 {
        self.0[0].get_bits(24..=31).try_into().unwrap()
    }
}

event!(TransferEvent, "Transfer Event TRB", Type::TransferEvent);
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

event!(
    CommandCompletion,
    "Command Completion Event TRB",
    Type::CommandCompletion
);
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

event!(
    BandwidthRequest,
    "Bandwidth Request Event TRB",
    Type::BandwidthRequest
);
impl BandwidthRequest {
    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}

event!(Doorbell, "Doorbell Event TRB", Type::Doorbell);
impl Doorbell {
    /// Returns the value of the DB Reason field.
    #[must_use]
    pub fn db_reason(&self) -> u8 {
        self.0[0].get_bits(0..=4).try_into().unwrap()
    }
}

event!(
    HostController,
    "Host Controller Event TRB",
    Type::HostController
);

event!(
    DeviceNotification,
    "Device Notification Event TRB",
    Type::DeviceNotification
);
impl DeviceNotification {
    /// Returns the value of the Notification Type field.
    #[must_use]
    pub fn notification_type(&self) -> u8 {
        self.0[0].get_bits(4..=7).try_into().unwrap()
    }

    /// Returns the value of the Device Notification Data field.
    #[must_use]
    pub fn device_notification_data(&self) -> u64 {
        let l: u64 = self.0[0].get_bits(8..=31).into();
        let u: u64 = self.0[1].into();

        ((u << 32) | l) >> 8
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}

event!(MfindexWrap, "MFINDEX Wrap Event TRB", Type::MfindexWrap);

/// The Completion Code.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
#[non_exhaustive]
pub enum CompletionCode {
    /// The operation succeed.
    Success = 1,
}
