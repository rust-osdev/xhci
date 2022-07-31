//! Event TRBs.

use super::Type;
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
        try_from!(
            raw =>
            TransferEvent,
            CommandCompletion,
            PortStatusChange,
            BandwidthRequest,
            Doorbell,
            HostController,
            DeviceNotification,
            MfindexWrap,
        );
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
macro_rules! impl_debug_for_event_trb{
    ($name:ident{})=>{
        impl_debug_for_trb!($name{
            completion_code
        });
    };
    ($name:ident {
        $($method:ident),*
    })=>{
        impl_debug_for_trb!($name{
            completion_code,
            $($method),*
        });
    }
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
    [3]16..=31;
});
impl PortStatusChange {
    ro_field!([0](24..=31), port_id, "Port ID", u8);
}
impl_debug_for_event_trb!(PortStatusChange { port_id });

event!(TransferEvent, "Transfer Event TRB", Type::TransferEvent);
reserved!(TransferEvent(Type::TransferEvent){
    [3]1..=1;
    [3]3..=9;
    [3]21..=23;
});
impl TransferEvent {
    /// Returns the value of the TRB Pointer field.
    #[must_use]
    pub fn trb_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    ro_field!([2](0..=23), trb_transfer_length, "TRB Transfer Length", u32);
    ro_bit!([3](2), event_data, "Event Data");
    ro_field!([3](16..=20), endpoint_id, "Endpoint ID", u8);
    ro_field!([3](24..=31), slot_id, "Slot ID", u8);
}
impl_debug_for_event_trb!(TransferEvent {
    trb_pointer,
    trb_transfer_length,
    event_data,
    endpoint_id,
    slot_id
});

event!(
    CommandCompletion,
    "Command Completion Event TRB",
    Type::CommandCompletion
);
reserved!(CommandCompletion(Type::CommandCompletion){
    [0]0..=3;
    [3]1..=9;
});
impl CommandCompletion {
    /// Returns the value of the Command TRB Pointer field.
    #[must_use]
    pub fn command_trb_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    ro_field!(
        [2](0..=23),
        command_completion_parameter,
        "Command Completion Parameter",
        u32
    );
    ro_field!([3](16..=23), vf_id, "VF ID", u8);
    ro_field!([3](24..=31), slot_id, "Slot ID", u8);
}
impl_debug_for_event_trb!(CommandCompletion {
    command_trb_pointer,
    command_completion_parameter,
    vf_id,
    slot_id
});

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
    [3]16..=23;
});
impl BandwidthRequest {
    ro_field!([3](24..=31), slot_id, "Slot ID", u8);
}
impl_debug_for_event_trb!(BandwidthRequest { slot_id });

event!(Doorbell, "Doorbell Event TRB", Type::Doorbell);
reserved!(Doorbell(Type::Doorbell){
    [0]5..=31;
    [1]0..=31;
    [2]0..=23;
    [3]1..=9;
});
impl Doorbell {
    ro_field!([0](0..=4), db_reason, "DB Reason", u8);
    ro_field!([3](16..=23), vf_id, "VF ID", u8);
    ro_field!([3](24..=31), slot_id, "Slot ID", u8);
}
impl_debug_for_event_trb!(Doorbell {
    db_reason,
    vf_id,
    slot_id
});

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
    [3]16..=31;
});
impl_debug_for_event_trb!(HostController {});

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
    [3]16..=31;
});
impl DeviceNotification {
    ro_field!([0](4..=7), notification_type, "Notification Type", u8);

    /// Returns the value of the Device Notification Data field.
    #[must_use]
    pub fn device_notification_data(&self) -> u64 {
        let l: u64 = self.0[0].get_bits(8..=31).into();
        let u: u64 = self.0[1].into();

        ((u << 32) | l) >> 8
    }

    ro_field!([3](24..=31), slot_id, "Slot ID", u8);
}
impl_debug_for_event_trb!(DeviceNotification {
    notification_type,
    device_notification_data,
    slot_id
});

event!(MfindexWrap, "MFINDEX Wrap Event TRB", Type::MfindexWrap);
reserved!(MfindexWrap(Type::MfindexWrap){
    [0]0..=3;
    [2]0..=23;
    [3]1..=9;
    [3]16..=23;
});
impl_debug_for_event_trb!(MfindexWrap {});

/// The TRB Completion Codes.
///
/// The description of each error is quoted from eXtensible Host Controller Interface for Universal
/// Serial Bus (xHCI) Requirements Specification May 2019 Revision 1.2, Section 6.4.5, Table 6-90.
/// Refer to this specification for more detail.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
pub enum CompletionCode {
    /// Indicates that the Completion Code field has not been updated by the TRB producer.
    Invalid = 0,
    /// Indicates successful completion of the TRB operation.
    Success = 1,
    /// Indicates that the Host Controller is unable to keep up the reception of incoming data
    /// (overrun) or is unable to supply data fast enough during transmission (underrun).
    DataBufferError = 2,
    /// Asserted when "babbling" is detected during the transaction generated by this TRB.
    BabbleDetectedError = 3,
    /// Asserted in the case where the host did not receive a valid response from the device.
    UsbTransactionError = 4,
    /// Asserted when a TRB parameter error condition is detected in a TRB.
    TrbError = 5,
    /// Asserted when a Stall condition is detected for a TRB.
    StallError = 6,
    /// Asserted by a Configure Endpoint Command or an Address Device Command if there are not
    /// adequate xHC resources available to successfully complete the command.
    ResourceError = 7,
    /// Asserted by a Configure Endpoint Command if periodic endpoints are declared and the xHC is
    /// not able to allocate the required Bandwidth.
    BandwidthError = 8,
    /// Asserted if a adding one more device would result in the host controller to exceed the
    /// maximum Number of Device Slots for this implementation.
    NoSlotsAvailableError = 9,
    /// Asserted if an invalid Stream Context Type value is detected.
    InvalidStreamTypeError = 10,
    /// Asserted if a command is issued to a Device Slot that is in the Disabled state.
    SlotNotEnabledError = 11,
    /// Asserted if a doorbell is rung for an endpoint that is in the Disabled state.
    EndpointNotEnabledError = 12,
    /// Asserted if the number of bytes received was less than the TD Transfer Size.
    ShortPacket = 13,
    /// Asserted in a Transfer Event TRB if the Transfer Ring is empty when an enabled Isoch
    /// endpoint is scheduled to transmit data.
    RingUnderrun = 14,
    /// Asserted in a Transfer Event TRB if the Transfer Ring is empty when an enabled Isoch
    /// endpoint is scheduled to receive data.
    RingOverrun = 15,
    /// Asserted by a Force Event command if the target VF's Event Ring is full.
    VfEventRingFullError = 16,
    /// Asserted by a command if a Context parameter is invalid.
    ParameterError = 17,
    /// Asserted during an Isoch transfer if the TD exceeds the bandwidth allocated to the
    /// endpoint.
    BandwidthOverrunError = 18,
    /// Asserted if a command is issued to transition from an illegal context state.
    ContextStateError = 19,
    /// Asserted if the xHC was unable to complete a periodic data transfer associated within the
    /// ESIT, because it did not receive a PING_RESPONSE in time.
    NoPingResponseError = 20,
    /// Asserted if the Event Ring is full, the xHC is unable to post an Event to the ring.
    EventRingFullError = 21,
    /// Asserted if the xHC detects a problem with a device that does not allow it to be
    /// successfully accessed.
    IncompatibleDeviceError = 22,
    /// Asserted if the xHC was unable to service a Isochronous endpoint within the Interval time.
    MissedServiceError = 23,
    /// Asserted in a Command Completion Event due to a Command Stop operation.
    CommandRingStopped = 24,
    /// Asserted in a Command Completion Event of an aborted command if the command was terminated
    /// by a Command Abort (CA) operation.
    CommandAborted = 25,
    /// Asserted in a Transfer Event if the transfer was terminated by a Stop Endpoint Command.
    Stopped = 26,
    /// Asserted in a Transfer Event if the transfer was terminated by a Stop Endpoint Command and
    /// the Transfer Event TRB Transfer Length field is invalid.
    StoppedLengthInvalid = 27,
    /// Asserted in a Transfer Event if the transfer was terminated by a Stop Endpoint Command, and
    /// the transfer was stopped after Short Packet conditions were met, but before the end of the
    /// TD was reached.
    StoppedShortPacket = 28,
    /// Asserted by the Evaluate Context Command if the proposed Max Exit Latency would not allow
    /// the periodic endpoints of the Device Slot to be scheduled.
    MaxExitLatencyTooLargeError = 29,
    /// Asserted if the data buffer defined by an Isoch TD on an IN endpoint is less than the Max
    /// ESIT Payload in size and the device attempts to send more data than it can hold.
    IsochBufferOverrun = 31,
    /// Asserted if the xHC internal event overrun condition.
    EventLostError = 32,
    /// May be reported by an event when other error codes do not apply.
    UndefinedError = 33,
    /// Asserted if an invalid Stream ID is received.
    InvalidStreamIdError = 34,
    /// Asserted by a Configure Endpoint Command if periodic endpoints are declared and the xHC is
    /// not able to allocate the required Bandwidth due to a Secondary Bandwidth Domain.
    SecondaryBandwidthError = 35,
    /// Asserted if an error is detected on a USB2 protocol endpoint for a split transaction.
    SplitTransactionError = 36,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_from_macro() {
        let raw = [
            0x00c8_8800, // address low
            0x0000_0000, // address high
            0x01_000000, // completion code 1, transfer length 0
            0x0000_8401, // endpoint id 0, slot id 0, type 33, event data 0, cycle bit 1
        ];
        assert_eq!(
            Allowed::try_from(raw),
            Ok(Allowed::CommandCompletion(
                CommandCompletion::try_from(raw).unwrap()
            )),
        );
    }
}
