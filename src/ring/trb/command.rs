//! Command TRBs.

use super::{Link, Type};
use bit_field::BitField;
use core::convert::TryInto;
use num_traits::FromPrimitive;

allowed! {
    /// TRBs which are allowed to be pushed to the Command Ring.
    enum {
        /// Link TRB
        Link,
        /// Enable Slot Command TRB
        EnableSlot,
        /// Disable Slot Command TRB
        DisableSlot,
        /// Address Device Command TRB
        AddressDevice,
        /// Configure Endpoint Command TRB
        ConfigureEndpoint,
        /// Evaluate Context Command TRB
        EvaluateContext,
        /// Reset Endpoint Command TRB
        ResetEndpoint,
        /// Stop Endpoint Command TRB
        StopEndpoint,
        /// Set TR Dequeue Pointer Command TRB
        SetTrDequeuePointer,
        /// Reset Device Command TRB
        ResetDevice,
        /// Force Event Command TRB
        ForceEvent,
        /// Negotiate Bandwidth Command TRB
        NegotiateBandwidth,
        /// Set Latency Tolerance Value Command TRB
        SetLatencyToleranceValue,
        /// Get Port Bandwidth Command TRB
        GetPortBandwidth,
        /// Force Header Command TRB
        ForceHeader,
        /// No Op Command TRB
        Noop,
        /// Get Extended Property Command TRB
        GetExtendedProperty,
        /// Set Extended Property Command TRB
        SetExtendedProperty
    }
}
impl TryFrom<[u32; 4]> for Allowed {
    type Error = [u32; 4];

    fn try_from(raw: [u32; 4]) -> Result<Self, Self::Error> {
        try_from!(
            raw =>
            Link,
            EnableSlot,
            DisableSlot,
            AddressDevice,
            ConfigureEndpoint,
            EvaluateContext,
            ResetEndpoint,
            StopEndpoint,
            SetTrDequeuePointer,
            ResetDevice,
            ForceEvent,
            NegotiateBandwidth,
            SetLatencyToleranceValue,
            GetPortBandwidth,
            ForceHeader,
            Noop(Command),
            GetExtendedProperty,
            SetExtendedProperty,
        );
        Err(raw)
    }
}

add_trb_with_default!(Noop, "No Op Command TRB", Type::NoopCommand);
reserved!(Noop(Type::NoopCommand) {
    [0]0..=31;
    [1]0..=31;
    [2]0..=31;
    [3]1..=9;
    [3]21..=31;
});
impl_debug_for_trb!(Noop {});

add_trb_with_default!(EnableSlot, "Enable Slot Command TRB", Type::EnableSlot);
reserved!(EnableSlot(Type::EnableSlot) {
    [0]0..=31;
    [1]0..=31;
    [2]0..=31;
    [3]1..=9;
    [3]21..=31;
});
impl EnableSlot {
    rw_field!([3](16..=20), slot_type, "Slot Type", u8);
}
impl_debug_for_trb!(EnableSlot { slot_type });

add_trb_with_default!(DisableSlot, "Disable Slot Command TRB", Type::DisableSlot);
reserved!(DisableSlot(Type::DisableSlot) {
    [0]0..=31;
    [1]0..=31;
    [2]0..=31;
    [3]1..=9;
    [3]16..=23;
});
impl DisableSlot {
    rw_field!([3](24..=31), slot_id, "Slot ID", u8);
}
impl_debug_for_trb!(DisableSlot { slot_id });

add_trb_with_default!(
    AddressDevice,
    "Address Device Command TRB",
    Type::AddressDevice
);
reserved!(AddressDevice(Type::AddressDevice) {
    [0]0..=3;
    [2]0..=31;
    [3]1..=8;
    [3]16..=23;
});
impl AddressDevice {
    /// Sets the value of the Input Context Pointer field.
    ///
    /// # Panics
    ///
    /// This method panics if `p` is not 16-byte aligned.
    pub fn set_input_context_pointer(&mut self, p: u64) -> &mut Self {
        assert_eq!(
            p % 16,
            0,
            "The Input Context Pointer must be 16-byte aligned."
        );

        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
        self
    }

    /// Returns the value of the Input Context Pointer field.
    #[must_use]
    pub fn input_context_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    rw_bit!(
        [3](9),
        block_set_address_request,
        "Block Set Address Request"
    );
    rw_field!([3](24..=31), slot_id, "Slot ID", u8);
}
impl_debug_for_trb!(AddressDevice {
    input_context_pointer,
    block_set_address_request,
    slot_id
});

add_trb_with_default!(
    ConfigureEndpoint,
    "Configure Endpoint Command TRB",
    Type::ConfigureEndpoint
);
reserved!(ConfigureEndpoint(Type::ConfigureEndpoint) {
    [0]0..=3;
    [2]0..=31;
    [3]1..=8;
    [3]16..=23;
});
impl ConfigureEndpoint {
    /// Sets the value of the Input Context Pointer field.
    ///
    /// # Panics
    ///
    /// This method panics if `p` is not 16-byte aligned.
    pub fn set_input_context_pointer(&mut self, p: u64) -> &mut Self {
        assert_eq!(
            p % 16,
            0,
            "The Input Context Pointer must be 16-byte aligned."
        );

        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
        self
    }

    /// Returns the value of the Input Context Pointer field.
    #[must_use]
    pub fn input_context_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    rw_bit!([3](9), deconfigure, "Deconfigure");
    rw_field!([3](24..=31), slot_id, "Slot ID", u8);
}
impl_debug_for_trb!(ConfigureEndpoint {
    input_context_pointer,
    deconfigure,
    slot_id
});

add_trb_with_default!(
    EvaluateContext,
    "Evaluate Context Command TRB",
    Type::EvaluateContext
);
reserved!(EvaluateContext(Type::EvaluateContext) {
    [0]0..=3;
    [2]0..=31;
    [3]1..=8;
    [3]16..=23;
});
impl EvaluateContext {
    /// Sets the value of the Input Context Pointer field.
    ///
    /// # Panics
    ///
    /// This method panics if `p` is not 16-byte aligned.
    pub fn set_input_context_pointer(&mut self, p: u64) -> &mut Self {
        assert_eq!(
            p % 16,
            0,
            "The Input Context Pointer must be 16-byte aligned."
        );

        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
        self
    }

    /// Returns the value of the Input Context Pointer field.
    #[must_use]
    pub fn input_context_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }
    rw_field!([3](24..=31), slot_id, "Slot ID", u8);
}
impl_debug_for_trb!(EvaluateContext {
    input_context_pointer,
    slot_id
});

add_trb_with_default!(
    ResetEndpoint,
    "Reset Endpoint Command TRB",
    Type::ResetEndpoint
);
reserved!(ResetEndpoint(Type::ResetEndpoint) {
    [0]0..=31;
    [1]0..=31;
    [2]0..=31;
    [3]1..=8;
    [3]21..=23;
});
impl ResetEndpoint {
    rw_bit!([3](9), transfer_state_preserve, "Transfer State Preserve");
    rw_field!([3](16..=20), endpoint_id, "Endpoint ID", u8);
    rw_field!([3](24..=31), slot_id, "Slot ID", u8);
}
impl_debug_for_trb!(ResetEndpoint {
    transfer_state_preserve,
    endpoint_id,
    slot_id
});

add_trb_with_default!(
    StopEndpoint,
    "Stop Endpoint Command TRB",
    Type::StopEndpoint
);
reserved!(StopEndpoint(Type::StopEndpoint) {
    [0]0..=31;
    [1]0..=31;
    [2]0..=31;
    [3]1..=9;
    [3]21..=22;
});
impl StopEndpoint {
    rw_field!([3](16..=20), endpoint_id, "Endpoint ID", u8);
    rw_bit!([3](23), suspend, "Suspend");
    rw_field!([3](24..=31), slot_id, "Slot ID", u8);
}
impl_debug_for_trb!(StopEndpoint {
    endpoint_id,
    suspend,
    slot_id
});

add_trb_with_default!(
    SetTrDequeuePointer,
    "Set TR Dequeue Pointer Command TRB",
    Type::SetTrDequeuePointer
);
reserved!(SetTrDequeuePointer(Type::SetTrDequeuePointer) {
    [2]0..=15;
    [3]1..=9;
    [3]21..=23;
});
impl SetTrDequeuePointer {
    rw_bit!([0](0), dequeue_cycle_state, "Dequeue Cycle State");
    rw_field!([0](1..=3), stream_context_type, "Stream Context Type", u8);

    /// Sets the value of the New TR Dequeue Pointer field.
    ///
    /// # Panics
    ///
    /// This method panics if `p` is not 16-byte aligned.
    pub fn set_new_tr_dequeue_pointer(&mut self, p: u64) -> &mut Self {
        assert_eq!(
            p % 16,
            0,
            "The New TR Dequeue Pointer must be 16-byte aligned."
        );

        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0].set_bits(4..32, l.get_bits(4..32).try_into().unwrap());
        self.0[1] = u.try_into().unwrap();
        self
    }

    /// Returns the value of the New TR Dequeue Pointer field.
    #[must_use]
    pub fn new_tr_dequeue_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        ((u << 32) | l) & 0xffff_fff0
    }

    rw_field!([2](16..=31), stream_id, "Stream ID", u16);
    rw_field!([3](16..=20), endpoint_id, "Endpoint ID", u8);
    rw_field!([3](24..=31), slot_id, "Slot ID", u8);
}
impl_debug_for_trb!(SetTrDequeuePointer {
    dequeue_cycle_state,
    stream_context_type,
    new_tr_dequeue_pointer,
    stream_id,
    endpoint_id,
    slot_id
});

add_trb_with_default!(ResetDevice, "Reset Device Command TRB", Type::ResetDevice);
reserved!(ResetDevice(Type::ResetDevice) {
    [0]0..=31;
    [1]0..=31;
    [2]0..=31;
    [3]1..=9;
    [3]16..=23;
});
impl ResetDevice {
    rw_field!([3](24..=31), slot_id, "Slot ID", u8);
}
impl_debug_for_trb!(ResetDevice { slot_id });

add_trb_with_default!(ForceEvent, "Force Event Command TRB", Type::ForceEvent);
reserved!(ForceEvent(Type::ForceEvent) {
    [0]0..=3;
    [2]0..=21;
    [3]1..=9;
    [3]24..=31;
});
impl ForceEvent {
    /// Sets the value of the Event TRB Pointer field.
    ///
    /// # Panics
    ///
    /// This method panics if the `p` is not 16-byte aligned.
    pub fn set_event_trb_pointer(&mut self, p: u64) -> &mut Self {
        assert_eq!(p % 16, 0, "The Event TRB Pointer must be 16-byte aligned.");

        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();

        self
    }

    /// Returns the value of the Event TRB Pointer field.
    #[must_use]
    pub fn event_trb_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    rw_field!(
        [2](22..=31),
        vf_interrupter_target,
        "VF Interrupter Target",
        u16
    );
    rw_field!([3](16..=23), vf_id, "VF ID", u8);
}
impl_debug_for_trb!(ForceEvent {
    event_trb_pointer,
    vf_interrupter_target,
    vf_id
});

add_trb_with_default!(
    NegotiateBandwidth,
    "Negotiate Bandwidth Command TRB",
    Type::NegotiateBandwidth
);
reserved!(NegotiateBandwidth(Type::NegotiateBandwidth) {
    [0]0..=31;
    [1]0..=31;
    [2]0..=31;
    [3]1..=9;
    [3]16..=23;
});
impl NegotiateBandwidth {
    rw_field!([3](24..=31), slot_id, "Slot ID", u8);
}
impl_debug_for_trb!(NegotiateBandwidth { slot_id });

add_trb_with_default!(
    SetLatencyToleranceValue,
    "Set Latency Tolerance Value Command TRB",
    Type::SetLatencyToleranceValue
);
reserved!(SetLatencyToleranceValue(Type::SetLatencyToleranceValue) {
    [0]0..=31;
    [1]0..=31;
    [2]0..=31;
    [3]1..=9;
    [3]28..=31;
});
impl SetLatencyToleranceValue {
    rw_field!(
        [3](16..=27),
        best_effort_latency_tolerance_value,
        "Best Effort Latency Tolerance Value",
        u16
    );
}
impl_debug_for_trb!(SetLatencyToleranceValue {
    best_effort_latency_tolerance_value
});

add_trb_with_default!(
    GetPortBandwidth,
    "Get Port Bandwidth Command TRB",
    Type::GetPortBandwidth
);
reserved!(GetPortBandwidth(Type::GetPortBandwidth) {
    [0]0..=3;
    [2]0..=31;
    [3]1..=9;
    [3]20..=23;
});
impl GetPortBandwidth {
    /// Sets the value of the Port Bandwidth Context Pointer field.
    ///
    /// # Panics
    ///
    /// This method panics if the `p` is not 16-byte aligned.
    pub fn set_port_bandwidth_context_pointer(&mut self, p: u64) -> &mut Self {
        assert_eq!(
            p % 16,
            0,
            "The Port Bandwidth Context Pointer must be 16-byte aligned."
        );

        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
        self
    }

    /// Returns the value of the Port Bandwidth Context Pointer field.
    #[must_use]
    pub fn port_bandwidth_context_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }
    rw_field!([3](16..=19), dev_speed, "Dev Speed", u8);
    rw_field!([3](24..=31), hub_slot_id, "Hub Slot ID", u8);
}
impl_debug_for_trb!(GetPortBandwidth {
    port_bandwidth_context_pointer,
    dev_speed,
    hub_slot_id
});

add_trb_with_default!(ForceHeader, "Force Header Command TRB", Type::ForceHeader);
reserved!(ForceHeader(Type::ForceHeader) {
    [3]1..=9;
    [3]16..=23;
});
impl ForceHeader {
    rw_field!([0](0..=4), packet_type, "Packet Type", u8);

    /// Sets the value of the Header Info field.
    ///
    /// # Panics
    ///
    /// This method panics if the lowest 5 bits of the `i[0]` are not 0.
    pub fn set_header_info(&mut self, info: [u32; 3]) -> &mut Self {
        assert!(
            info[0].trailing_zeros() >= 5,
            "The lowest 5 bits of the Header Info Low must be 0."
        );

        self.0[0].set_bits(5..=31, info[0].get_bits(5..=31));
        self.0[1] = info[1];
        self.0[2] = info[2];
        self
    }

    /// Returns the value of the Header Info field.
    #[must_use]
    pub fn header_info(&self) -> [u32; 3] {
        [self.0[0] & 0xffff_ffe0, self.0[1], self.0[2]]
    }

    rw_field!(
        [3](24..=31),
        root_hub_port_number,
        "Root Hub Port Number",
        u8
    );
}
impl_debug_for_trb!(ForceHeader {
    packet_type,
    header_info,
    root_hub_port_number
});

add_trb_with_default!(
    GetExtendedProperty,
    "Get Extended Property Command TRB",
    Type::GetExtendedProperty
);
reserved!(GetExtendedProperty(Type::GetExtendedProperty) {
    [0]0..=3;
    [2]16..=31;
    [3]1..=9;
});
impl GetExtendedProperty {
    /// Sets the value of the Extended Property Context Pointer field.
    ///
    /// # Panics
    ///
    /// This method panics if the `p` is not 16-byte aligned.
    pub fn set_extended_property_context_pointer(&mut self, p: u64) -> &mut Self {
        assert_eq!(
            p % 16,
            0,
            "The Extended Property Context Pointer must be 16-byte aligned."
        );

        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
        self
    }

    /// Returns the value of the Extended Property Context Pointer field.
    #[must_use]
    pub fn extended_property_context_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    rw_field!(
        [2](0..=15),
        extended_capability_identifier,
        "Extended Capability Identifier",
        u16
    );
    rw_field!([3](16..=18), command_sub_type, "Command Sub Type", u8);
    rw_field!([3](19..=23), endpoint_id, "Endpoint ID", u8);
    rw_field!([3](24..=31), slot_id, "Slot ID", u8);
}
impl_debug_for_trb!(GetExtendedProperty {
    extended_property_context_pointer,
    extended_capability_identifier,
    command_sub_type,
    endpoint_id,
    slot_id
});

add_trb_with_default!(
    SetExtendedProperty,
    "Set Extended Property Command TRB",
    Type::SetExtendedProperty
);
reserved!(SetExtendedProperty(Type::SetExtendedProperty) {
    [0]0..=31;
    [1]0..=31;
    [2]24..=31;
    [3]1..=9;
});
impl SetExtendedProperty {
    rw_field!(
        [2](0..=15),
        extended_capability_identifier,
        "Extended Cpaability Identifier",
        u16
    );
    rw_field!(
        [2](16..=23),
        capability_parameter,
        "Capability Parameter",
        u8
    );
    rw_field!([3](16..=18), command_sub_type, "Command Sub Type", u8);
    rw_field!([3](19..=23), endpoint_id, "Endpoint ID", u8);
    rw_field!([3](24..=31), slot_id, "Slot ID", u8);
}
impl_debug_for_trb!(SetExtendedProperty {
    extended_capability_identifier,
    capability_parameter,
    command_sub_type,
    endpoint_id,
    slot_id
});
