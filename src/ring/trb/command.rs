//! Command TRBs.

use super::Link;
use bit_field::BitField;
use core::convert::TryInto;

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

add_trb_with_default!(Noop, "No Op Command TRB", Type::NoopCommand);
impl_debug_for_trb!(Noop {});

add_trb_with_default!(EnableSlot, "Enable Slot Command TRB", Type::EnableSlot);
impl EnableSlot {
    /// Sets the value of the Slot Type field.
    pub fn set_slot_type(&mut self, t: u8) -> &mut Self {
        self.0[3].set_bits(16..=20, t.into());
        self
    }

    /// Returns the value of the Slot Type field.
    #[must_use]
    pub fn slot_type(&self) -> u8 {
        self.0[3].get_bits(16..=20).try_into().unwrap()
    }
}
impl_debug_for_trb!(EnableSlot { slot_type });

add_trb_with_default!(DisableSlot, "Disable Slot Command TRB", Type::DisableSlot);
impl DisableSlot {
    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}
impl_debug_for_trb!(DisableSlot { slot_id });

add_trb_with_default!(
    AddressDevice,
    "Address Device Command TRB",
    Type::AddressDevice
);
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

    /// Sets the value of the Block Set Address Request field.
    pub fn set_block_set_address_request(&mut self, r: bool) -> &mut Self {
        self.0[3].set_bit(9, r);
        self
    }

    /// Returns the value of the Block Set Address Request.
    #[must_use]
    pub fn block_set_address_request(&self) -> bool {
        self.0[3].get_bit(9)
    }

    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
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

    /// Sets the value of the Deconfigure field.
    pub fn set_deconfigure(&mut self, d: bool) -> &mut Self {
        self.0[3].set_bit(9, d);
        self
    }

    /// Returns the value of the Deconfigure field.
    pub fn deconfigure(&mut self) -> bool {
        self.0[3].get_bit(9)
    }

    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
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

    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
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
impl ResetEndpoint {
    /// Sets the value of the Transfer State Preserve field.
    pub fn set_transfer_state_preserve(&mut self, tsp: bool) -> &mut Self {
        self.0[3].set_bit(9, tsp);
        self
    }

    /// Returns the value of the Transfer State Preserve field.
    #[must_use]
    pub fn transfer_state_preserve(&self) -> bool {
        self.0[3].get_bit(9)
    }

    /// Sets the value of the Endpoint ID field.
    pub fn set_endpoint_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(16..=20, i.into());
        self
    }

    /// Returns the value of the Endpoint ID.
    #[must_use]
    pub fn endpoint_id(&self) -> u8 {
        self.0[3].get_bits(16..=20).try_into().unwrap()
    }

    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
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
impl StopEndpoint {
    /// Sets the value of the Endpoint ID field.
    pub fn set_endpoint_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(16..=20, i.into());
        self
    }

    /// Returns the value of the Endpoint ID field.
    #[must_use]
    pub fn endpoint_id(&self) -> u8 {
        self.0[3].get_bits(16..=20).try_into().unwrap()
    }

    /// Sets the value of the Suspend field.
    pub fn set_suspend(&mut self, s: bool) -> &mut Self {
        self.0[3].set_bit(23, s);
        self
    }

    /// Returns the value of the Suspend field.
    #[must_use]
    pub fn suspend(&self) -> bool {
        self.0[3].get_bit(23)
    }

    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
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
impl SetTrDequeuePointer {
    /// Sets the value of the Dequeue Cycle State field.
    pub fn set_dequeue_cycle_state(&mut self, s: bool) -> &mut Self {
        self.0[0].set_bit(0, s);
        self
    }

    /// Returns the value of the Dequeue Cycle state field.
    #[must_use]
    pub fn dequeue_cycle_state(&self) -> bool {
        self.0[0].get_bit(0)
    }

    /// Sets the value of the Stream Context Type field.
    pub fn set_stream_context_type(&mut self, t: u8) -> &mut Self {
        self.0[0].set_bits(1..=3, t.into());
        self
    }

    /// Returns the value of the Stream Context Type field.
    #[must_use]
    pub fn stream_context_type(&self) -> u8 {
        self.0[0].get_bits(1..=3).try_into().unwrap()
    }

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

    /// Sets the value of the Stream ID field.
    pub fn set_stream_id(&mut self, i: u16) -> &mut Self {
        self.0[2].set_bits(16..=31, i.into());
        self
    }

    /// Returns the value of the Stream ID field.
    #[must_use]
    pub fn stream_id(&self) -> u16 {
        self.0[2].get_bits(16..=31).try_into().unwrap()
    }

    /// Sets the value of the Endpoint ID field.
    pub fn set_endpoint_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(16..=20, i.into());
        self
    }

    /// Returns the value of the Endpoint ID field.
    #[must_use]
    pub fn endpoint_id(&self) -> u8 {
        self.0[3].get_bits(16..=20).try_into().unwrap()
    }

    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Slot ID field.
    pub fn slot_id(&mut self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
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
impl ResetDevice {
    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}
impl_debug_for_trb!(ResetDevice { slot_id });

add_trb_with_default!(ForceEvent, "Force Event Command TRB", Type::ForceEvent);
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

    /// Sets the value of the VF Interrupter Target field.
    pub fn set_vf_interrupter_target(&mut self, t: u16) -> &mut Self {
        self.0[2].set_bits(22..=31, t.into());
        self
    }

    /// Returns the value of the VF Interrupter Target field.
    #[must_use]
    pub fn vf_interrupter_target(&self) -> u16 {
        self.0[2].get_bits(22..=31).try_into().unwrap()
    }

    /// Sets the value of the VF ID field.
    pub fn set_vf_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(16..=23, i.into());
        self
    }

    /// Returns the value of the VF ID field.
    #[must_use]
    pub fn vf_id(&self) -> u8 {
        self.0[3].get_bits(16..=23).try_into().unwrap()
    }
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
impl NegotiateBandwidth {
    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}
impl_debug_for_trb!(NegotiateBandwidth { slot_id });

add_trb_with_default!(
    SetLatencyToleranceValue,
    "Set Latency Tolerance Value Command TRB",
    Type::SetLatencyToleranceValue
);
impl SetLatencyToleranceValue {
    /// Sets the value of the Best Effort Latency Tolerance Value field.
    pub fn set_best_effort_latency_tolerance_value(&mut self, v: u16) -> &mut Self {
        self.0[3].set_bits(16..=27, v.into());
        self
    }

    /// Returns the value of the Best Effort Latency Tolerance Value field.
    #[must_use]
    pub fn best_effort_latency_tolerance_value(&self) -> u16 {
        self.0[3].get_bits(16..=27).try_into().unwrap()
    }
}
impl_debug_for_trb!(SetLatencyToleranceValue {
    best_effort_latency_tolerance_value
});

add_trb_with_default!(
    GetPortBandwidth,
    "Get Port Bandwidth Command TRB",
    Type::GetPortBandwidth
);
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

    /// Sets the value of the Dev Speed field.
    pub fn set_dev_speed(&mut self, s: u8) -> &mut Self {
        self.0[3].set_bits(16..=19, s.into());
        self
    }

    /// Returns the value of the Dev Speed field.
    #[must_use]
    pub fn dev_speed(&self) -> u8 {
        self.0[3].get_bits(16..=19).try_into().unwrap()
    }

    /// Sets the value of the Hub Slot ID field.
    pub fn set_hub_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Hub Slot ID field.
    #[must_use]
    pub fn hub_slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}
impl_debug_for_trb!(GetPortBandwidth {
    port_bandwidth_context_pointer,
    dev_speed,
    hub_slot_id
});

add_trb_with_default!(ForceHeader, "Force Header Command TRB", Type::ForceHeader);
impl ForceHeader {
    /// Sets the value of the Packet Type field.
    pub fn set_packet_type(&mut self, t: u8) -> &mut Self {
        self.0[0].set_bits(0..=4, t.into());
        self
    }

    /// Returns the value of the Packet Type field.
    #[must_use]
    pub fn packet_type(&self) -> u8 {
        self.0[0].get_bits(0..=4).try_into().unwrap()
    }

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

    /// Sets the value of the Root Hub Port Number.
    pub fn set_root_hub_port_number(&mut self, n: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, n.into());
        self
    }

    /// Returns the value of the Root Hub Port Number.
    #[must_use]
    pub fn root_hub_port_number(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
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

    /// Sets the value of the Extended Capability Identifier field.
    pub fn set_extended_capability_identifier(&mut self, eci: u16) -> &mut Self {
        self.0[2].set_bits(0..=15, eci.into());
        self
    }

    /// Returns the value of the Extended Capability Identifier field.
    #[must_use]
    pub fn extended_capability_identifier(&self) -> u16 {
        self.0[2].get_bits(0..=15).try_into().unwrap()
    }

    /// Sets the value of the Command Sub Type field.
    pub fn set_command_sub_type(&mut self, t: u8) -> &mut Self {
        self.0[3].set_bits(16..=18, t.into());
        self
    }

    /// Returns the value of the Command Sub Type field.
    #[must_use]
    pub fn command_sub_type(&self) -> u8 {
        self.0[3].get_bits(16..=18).try_into().unwrap()
    }

    /// Sets the value of the Endpoint ID field.
    pub fn set_endpoint_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(19..=23, i.into());
        self
    }

    /// Returns the value of the Endpoint ID field.
    #[must_use]
    pub fn endpoint_id(&self) -> u8 {
        self.0[3].get_bits(19..=23).try_into().unwrap()
    }

    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
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
impl SetExtendedProperty {
    /// Sets the value of the Extended Capability Identifier field.
    pub fn set_extended_capability_identifier(&mut self, eci: u16) -> &mut Self {
        self.0[2].set_bits(0..=15, eci.into());
        self
    }

    /// Returns the value of the Extended Capability Identifier field.
    #[must_use]
    pub fn extended_capability_identifier(&self) -> u16 {
        self.0[2].get_bits(0..=15).try_into().unwrap()
    }

    /// Sets the value of the Capability Parameter field.
    pub fn set_capability_parameter(&mut self, p: u8) -> &mut Self {
        self.0[2].set_bits(15..=23, p.into());
        self
    }

    /// Returns the value of the Capability Parameter field.
    #[must_use]
    pub fn capability_parameter(&self) -> u8 {
        self.0[2].get_bits(15..=23).try_into().unwrap()
    }

    /// Sets the value of the Command Sub Type field.
    pub fn set_command_sub_type(&mut self, t: u8) -> &mut Self {
        self.0[3].set_bits(16..=18, t.into());
        self
    }

    /// Returns the value of the Command Sub Type field.
    #[must_use]
    pub fn command_sub_type(&self) -> u8 {
        self.0[3].get_bits(16..=18).try_into().unwrap()
    }

    /// Sets the value of the Endpoint ID field.
    pub fn set_endpoint_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(19..=23, i.into());
        self
    }

    /// Returns the value of the Endpoint ID field.
    #[must_use]
    pub fn endpoint_id(&self) -> u8 {
        self.0[3].get_bits(19..=23).try_into().unwrap()
    }

    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}
impl_debug_for_trb!(SetExtendedProperty {
    extended_capability_identifier,
    capability_parameter,
    command_sub_type,
    endpoint_id,
    slot_id
});
