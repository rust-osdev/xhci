//! Event TRBs.

use bit_field::BitField;
use core::convert::{TryFrom, TryInto};
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
impl TryFrom<[u32; 4]> for Allowed {
    type Error = [u32; 4];

    fn try_from(raw: [u32; 4]) -> Result<Self, Self::Error> {
        macro_rules! try_from {
            ($name:ident) => {
                if let Ok(t) = $name::try_from(raw) {
                    return Ok(Self::$name(t));
                }
            };
        }

        try_from!(TransferEvent);
        try_from!(CommandCompletion);
        try_from!(PortStatusChange);
        try_from!(BandwidthRequest);
        try_from!(Doorbell);
        try_from!(HostController);
        try_from!(DeviceNotification);
        try_from!(MfindexWrap);

        Err(raw)
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
reserved!(PortStatusChange(Type::PortStatusChange){
    [0]0..=23;
    [1]0..=31;
    [2]0..=23;
    [3]1..=9;
    [3]16..=31
});
impl PortStatusChange {
    /// Returns the value of the Port ID field.
    #[must_use]
    pub fn port_id(&self) -> u8 {
        self.0[0].get_bits(24..=31).try_into().unwrap()
    }
}

event!(TransferEvent, "Transfer Event TRB", Type::TransferEvent);
reserved!(TransferEvent(Type::TransferEvent){
    [3]1..=1;
    [3]3..=9;
    [3]21..=23
});
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
reserved!(CommandCompletion(Type::CommandCompletion){
    [0]0..=3;
    [3]1..=9
});
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
reserved!(BandwidthRequest(Type::BandwidthRequest){
    [0]0..=31;
    [1]0..=31;
    [2]0..=23;
    [3]1..=9;
    [3]16..=23
});
impl BandwidthRequest {
    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}

event!(Doorbell, "Doorbell Event TRB", Type::Doorbell);
reserved!(Doorbell(Type::Doorbell){
    [0]5..=31;
    [1]0..=31;
    [2]0..=23;
    [3]1..=9
});
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
reserved!(HostController(Type::HostController){
    [0]0..=31;
    [1]0..=31;
    [2]0..=23;
    [3]1..=9;
    [3]16..=31
});

event!(
    DeviceNotification,
    "Device Notification Event TRB",
    Type::DeviceNotification
);
reserved!(DeviceNotification(Type::DeviceNotification){
    [0]0..=31;
    [1]0..=31;
    [2]0..=23;
    [3]1..=9;
    [3]16..=31
});
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
reserved!(MfindexWrap(Type::MfindexWrap){
    [0]0..=3;
    [2]0..=23;
    [3]1..=9;
    [3]16..=23
});

/// The Completion Code.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
#[non_exhaustive]
pub enum CompletionCode {
    /// The operation succeed.
    Success = 1,
}
