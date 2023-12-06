//! Event TRBs.

use bit_field::BitField;
// use core::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

macro_rules! impl_cc {
    (ro) => {
        /// Returns the Completion Code.
        ///
        /// # Errors
        ///
        /// This method may return an [`Err`] variant with the Completion Code that is either reserved or
        /// not implemented in this crate.
        pub fn completion_code(&self) -> Result<CompletionCode, u8> {
            let c: u8 = self.0.0[2].get_bits(24..=31).try_into().unwrap();
            CompletionCode::from_u8(c).ok_or(c)
        }
    }
}

allowed_trb!("Transfer TRB", {
    /// Transfer Event TRB
    TransferEvent = 32,
    /// Command Completion Event TRB
    CommandCompletion = 33,
    /// Port Status Change Event TRB
    PortStatusChange = 34,
    /// Bandwidth Request Event TRB
    BandwidthRequest = 35,
    /// Doorbell Event TRB
    Doorbell = 36,
    /// Host Controller Event TRB
    HostController = 37,
    /// Device Notification Event TRB
    DeviceNotification = 38,
    /// MFINDEX Wrap Event TRB
    MfindexWrap = 39,
});

impl TransferEvent {
    ro_double_field!(pub, self, self.0.0; [0, 1], trb_pointer, "TRB Pointer (or event data)", 32, u64);

    ro_field!(pub, self, self.0.0[2]; 0..=23, trb_transfer_length, "TRB Transfer Length", u32);
    impl_cc!(ro);

    ro_bit!(pub, self, self.0.0[3]; 2, event_data, "Event Data Flag");
    impl_ep_id!(ro);
    impl_slot_id!(ro);
}
impl_debug_from_methods!(TransferEvent {
    trb_pointer,
    trb_transfer_length,
    completion_code,
    event_data,
    endpoint_id,
    slot_id,
});
rsvdz_checking_try_from!(TransferEvent {
    [3];1..=1,
    [3];3..=9,
    [3];21..=23,
});

impl CommandCompletion {
    ro_double_field!(
        pub, self,
        self.0.0; [0, 1],
        command_trb_pointer,
        "Command TRB Pointer",
        32, u64
    );

    ro_field!(
        pub, self,
        self.0.0[2]; 0..=23, 
        command_completion_parameter,
        "Command Completion Parameter",
        u32
    );
    impl_cc!(ro);

    impl_vf_id!(ro);
    impl_slot_id!(ro);
}
impl_debug_from_methods!(CommandCompletion {
    command_trb_pointer,
    command_completion_parameter,
    completion_code,
    vf_id,
    slot_id,
});
rsvdz_checking_try_from!(CommandCompletion {
    [0];0..=3,
    [3];1..=9,
});

impl PortStatusChange {
    ro_field!(pub, self, self.0.0[0]; 24..=31, port_id, "Port ID", u8);

    impl_cc!(ro);
}
impl_debug_from_methods!(PortStatusChange {
    port_id,
    completion_code,
});
rsvdz_checking_try_from!(PortStatusChange {
    [0];0..=23,
    [1];0..=31,
    [2];0..=23,
    [3];1..=9,
    [3];16..=31,
});

impl BandwidthRequest {
    impl_cc!(ro);

    impl_slot_id!(ro);
}
impl_debug_from_methods!(BandwidthRequest {
    completion_code,
    slot_id,
});
rsvdz_checking_try_from!(BandwidthRequest {
    [0];0..=31,
    [1];0..=31,
    [2];0..=23,
    [3];1..=9,
    [3];16..=23,
});

impl Doorbell {
    ro_field!(pub, self, self.0.0[0]; 0..=4, db_reason, "DB Reason", u8);

    impl_cc!(ro);

    impl_vf_id!(ro);
    impl_slot_id!(ro);
}
impl_debug_from_methods!(Doorbell {
    db_reason,
    completion_code,
    vf_id,
    slot_id,
});
rsvdz_checking_try_from!(Doorbell {
    [0];5..=31,
    [1];0..=31,
    [2];0..=23,
    [3];1..=9,
});

impl HostController {
    impl_cc!(ro);
}
impl_debug_from_methods!(HostController {
    completion_code,
});
rsvdz_checking_try_from!(HostController {
    [0];0..=31,
    [1];0..=31,
    [2];0..=23,
    [3];1..=9,
    [3];16..=31,
});

impl DeviceNotification {
    ro_field!(
        pub, self,
        self.0.0[0]; 4..=7,
        notification_type,
        "Notification Type",
        u8
    );
    ro_double_field!(
        pub(self), self,
        self.0.0; [0, 1],
        device_notification_data_raw,
        "Device Notification Data Raw",
        32, u64
    );
    /// Returns the value of the Device Notification Data field.
    #[must_use]
    pub fn device_notification_data(&self) -> u64 {
        self.device_notification_data_raw() >> 8
    }

    impl_cc!(ro);

    impl_slot_id!(ro);
}
impl_debug_from_methods!(DeviceNotification {
    notification_type,
    device_notification_data,
    completion_code,
    slot_id,
});
rsvdz_checking_try_from!(DeviceNotification {
    [0];0..=3,
    [2];0..=23,
    [3];1..=9,
    [3];16..=31,
});

impl MfindexWrap {
    impl_cc!(ro);
}
impl_debug_from_methods!(MfindexWrap {
    completion_code,
});
rsvdz_checking_try_from!(MfindexWrap {
    [0];0..=31,
    [1];0..=31,
    [2];0..=23,
    [3];1..=9,
    [3];16..=31,
});


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