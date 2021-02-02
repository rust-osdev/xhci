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
        /// No Op Command TRB
        Noop
    }
}

add_trb_with_default!(Noop, "No Op Command TRB", Type::NoopCommand);

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

add_trb_with_default!(ResetDevice, "Reset Device Command TRB", Type::ResetDevice);
impl ResetDevice {
    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Slot ID field.
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}

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
    pub fn vf_interrupter_target(&self) -> u16 {
        self.0[2].get_bits(22..=31).try_into().unwrap()
    }

    /// Sets the value of the VF ID field.
    pub fn set_vf_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(16..=23, i.into());
        self
    }

    /// Returns the value of the VF ID field.
    pub fn vf_id(&self) -> u8 {
        self.0[3].get_bits(16..=23).try_into().unwrap()
    }
}

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
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}

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
    pub fn best_effort_latency_tolerance_value(&self) -> u16 {
        self.0[3].get_bits(16..=27).try_into().unwrap()
    }
}

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
    pub fn dev_speed(&self) -> u8 {
        self.0[3].get_bits(16..=19).try_into().unwrap()
    }

    /// Sets the value of the Hub Slot ID field.
    pub fn set_hub_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Hub Slot ID field.
    pub fn hub_slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}
