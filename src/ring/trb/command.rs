//! Command TRBs.

use bit_field::BitField;
// use core::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

macro_rules! impl_input_context_pointer {
    () => {
        param_align_16!(input_context_pointer, "Input Context Pointer");
    }
}

macro_rules! impl_subtype {
    () => {
        rw_field!(pub, self, self.0.0[3]; 16..=18, command_sub_type, "Command Sub Type", u8);
    }
}

allowed_trb!("Command TRB", {
    /// Link TRB
    Link = 6,
    /// Enable Slot Command TRB
    EnableSlot = 9,
    /// Disable Slot Command TRB
    DisableSlot = 10,
    /// Address Device Command TRB
    AddressDevice = 11,
    /// Configure Endpoint Command TRB
    ConfigureEndpoint = 12,
    /// Evaluate Context Command TRB
    EvaluateContext = 13,
    /// Reset Endpoint Command TRB
    ResetEndpoint = 14,
    /// Stop Endpoint Command TRB
    StopEndpoint = 15,
    /// Set TR Dequeue Pointer Command TRB
    SetTrDequeuePointer = 16,
    /// Reset Device Command TRB
    ResetDevice = 17,
    /// Force Event Command TRB
    ForceEvent = 18,
    /// Negotiate Bandwidth Command TRB
    NegotiateBandwidth = 19,
    /// Set Latency Tolerance Value Command TRB
    SetLatencyToleranceValue = 20,
    /// Get Port Bandwidth Command TRB
    GetPortBandwidth = 21,
    /// Force Header Command TRB
    ForceHeader = 22,
    /// NoOp Command TRB
    NoOp = 23,
    /// Get Extended Property Command TRB
    GetExtendedProperty = 24,
    /// Set Extended Property Command TRB
    SetExtendedProperty = 25,
});

impl Link {
    impl_ring_segment_pointer!();

    // impl_interrupter_target!(); // ignored in command ring

    impl_tc!();
    // impl_ch!(); // ignored in command ring
    impl_ioc!();
}
rsvdz_checking_try_from!(Link {
    [0];0..=3,
    [2];0..=21,
    [3];2..=3,
    [3];6..=9,
    [3];16..=31,
});
impl_debug_from_methods!(Link {
    ring_segment_pointer,
    // interrupter_target,
    toggle_cycle,
    // chain_bit,
    interrupt_on_completion,
});

impl NoOp {}
rsvdz_checking_try_from!(NoOp {
    [0];0..=31,
    [1];0..=31,
    [2];0..=31,
    [3];1..=9,
    [3];16..=31,
});
impl_debug_from_methods!(NoOp {});

impl EnableSlot {
    rw_field!(pub, self, self.0.0[3]; 16..=20, slot_type, "Slot Type", u8);
}
impl_debug_from_methods!(EnableSlot {
    slot_type,
});
rsvdz_checking_try_from!(EnableSlot {
    [0];0..=31,
    [1];0..=31,
    [2];0..=31,
    [3];1..=9,
    [3];21..=31,
});

impl DisableSlot {
    impl_slot_id!();
}
impl_debug_from_methods!(DisableSlot {
    slot_id,
});
rsvdz_checking_try_from!(DisableSlot {
    [0];0..=31,
    [1];0..=31,
    [2];0..=31,
    [3];1..=9,
    [3];16..=23,
});

impl AddressDevice {
    impl_input_context_pointer!();

    rw_bit!(pub, self, self.0.0[3]; 9, block_set_address_request, "Block Set Address Request");
    impl_slot_id!();
}
impl_debug_from_methods!(AddressDevice {
    input_context_pointer,
    block_set_address_request,
    slot_id,
});
rsvdz_checking_try_from!(AddressDevice {
    [0];0..=3,
    [2];0..=31,
    [3];1..=8,
    [3];16..=23,
});

impl ConfigureEndpoint {
    impl_input_context_pointer!();

    rw_bit!(pub, self, self.0.0[3]; 9, deconfigure, "Deconfigure");
    impl_slot_id!();
}
impl_debug_from_methods!(ConfigureEndpoint {
    input_context_pointer,
    deconfigure,
    slot_id,
});
rsvdz_checking_try_from!(ConfigureEndpoint {
    [0];0..=3,
    [2];0..=31,
    [3];1..=8,
    [3];16..=23,
});

impl EvaluateContext {
    impl_input_context_pointer!();

    // rw_bit!(pub, self, self.0.0[3]; 9, block_set_address_request, "Block Set Address Request"); // unused (no rsvdz)
    impl_slot_id!();
}
impl_debug_from_methods!(EvaluateContext {
    input_context_pointer,
    slot_id,
});
rsvdz_checking_try_from!(EvaluateContext {
    [0];0..=3,
    [2];0..=31,
    [3];1..=8,
    [3];16..=23,
});

impl ResetEndpoint {
    rw_bit!(pub, self, self.0.0[3]; 9, transfer_state_preserve, "Transfer State Preserve");
    impl_ep_id!();
    impl_slot_id!();
}
impl_debug_from_methods!(ResetEndpoint {
    transfer_state_preserve,
    endpoint_id,
    slot_id,
});
rsvdz_checking_try_from!(ResetEndpoint {
    [0];0..=31,
    [1];0..=31,
    [2];0..=31,
    [3];1..=8,
    [3];21..=23,
});

impl StopEndpoint {
    impl_ep_id!();
    rw_bit!(pub, self, self.0.0[3]; 23, suspend, "Suspend");
    impl_slot_id!();
}
impl_debug_from_methods!(StopEndpoint {
    endpoint_id,
    suspend,
    slot_id,
});
rsvdz_checking_try_from!(StopEndpoint {
    [0];0..=31,
    [1];0..=31,
    [2];0..=31,
    [3];1..=9,
    [3];21..=22,
});

impl SetTrDequeuePointer {
    rw_bit!(pub, self, self.0.0[0]; 0, dequeue_cycle_state, "Dequeue Cycle State");
    rw_field!(pub, self, self.0.0[0]; 1..=3, stream_context_type, "Stream Context Type", u8);
    param_align_16!(new_tr_dequeue_pointer, "New TR Dequeue Pointer");
    rw_field!(pub, self, self.0.0[2]; 16..=31, stream_id, "Stream ID", u16);

    impl_ep_id!();
    impl_slot_id!();
}
impl_debug_from_methods!(SetTrDequeuePointer {
    dequeue_cycle_state,
    stream_context_type,
    new_tr_dequeue_pointer,
    stream_id,
    endpoint_id,
    slot_id,
});
rsvdz_checking_try_from!(SetTrDequeuePointer {
    [2];0..=15,
    [3];1..=9,
    [3];21..=23,
});

impl ResetDevice {
    impl_slot_id!();
}
impl_debug_from_methods!(ResetDevice {
    slot_id,
});
rsvdz_checking_try_from!(ResetDevice {
    [0];0..=31,
    [1];0..=31,
    [2];0..=31,
    [3];1..=9,
    [3];16..=23,
});

impl ForceEvent {
    param_align_16!(event_trb_pointer, "Event TRB Pointer");

    rw_field!(
        pub, self,
        self.0.0[2]; 22..=31,
        vf_interrupter_target,
        "VF Interrupter Target",
        u16
    );

    impl_vf_id!();
}
impl_debug_from_methods!(ForceEvent {
    event_trb_pointer,
    vf_interrupter_target,
    vf_id
});
rsvdz_checking_try_from!(ForceEvent {
    [0];0..=3,
    [2];0..=21,
    [3];1..=9,
    [3];24..=31,
});

impl NegotiateBandwidth {
    impl_slot_id!();
}
impl_debug_from_methods!(NegotiateBandwidth {
    slot_id,
});
rsvdz_checking_try_from!(NegotiateBandwidth {
    [0];0..=31,
    [1];0..=31,
    [2];0..=31,
    [3];1..=9,
    [3];16..=23,
});

impl SetLatencyToleranceValue {
    rw_field!(
        pub, self,
        self.0.0[3]; 16..=27,
        best_effort_latency_tolerance_value,
        "Best Effort Latency Tolerance Value",
        u16
    );
}
impl_debug_from_methods!(SetLatencyToleranceValue {
    best_effort_latency_tolerance_value,
});
rsvdz_checking_try_from!(SetLatencyToleranceValue {
    [0];0..=31,
    [1];0..=31,
    [2];0..=31,
    [3];1..=9,
    [3];28..=31,
});

impl GetPortBandwidth {
    param_align_16!(
        port_bandwidth_context_pointer,
        "Port Bandwidth Context Pointer"
    );

    rw_field!(pub, self, self.0.0[3]; 16..=19, dev_speed, "Dev Speed", u8);
    rw_field!(pub, self, self.0.0[3]; 24..=31, hub_slot_id, "Hub Slot ID", u8);
}
impl_debug_from_methods!(GetPortBandwidth {
    port_bandwidth_context_pointer,
    dev_speed,
    hub_slot_id
});
rsvdz_checking_try_from!(GetPortBandwidth {
    [0];0..=3,
    [2];0..=31,
    [3];1..=9,
    [3];20..=23,
});

impl ForceHeader {
    rw_field!(pub, self, self.0.0[0]; 0..=4, packet_type, "Packet Type", u8);

    /// Sets the value of the Header Info field.
    ///
    /// # Panics
    ///
    /// This method panics if the lowest 5 bits of the `info[0]` are not 0.
    pub fn set_header_info(&mut self, info: [u32; 3]) -> &mut Self {
        assert!(
            info[0].trailing_zeros() >= 5,
            "The lowest 5 bits of the Header Info Low must be 0."
        );

        self.0.0[0].set_bits(5.., info[0].get_bits(5..));
        self.0.0[1] = info[1];
        self.0.0[2] = info[2];
        self
    }

    /// Returns the value of the Header Info field.
    #[must_use]
    pub fn header_info(&self) -> [u32; 3] {
        [self.0.0[0] >> 5 << 5, self.0.0[1], self.0.0[2]]
    }

    rw_field!(
        pub, self,
        self.0.0[3]; 24..=31,
        root_hub_port_number,
        "Root Hub Port Number",
        u8
    );
}
impl_debug_from_methods!(ForceHeader {
    packet_type,
    header_info,
    root_hub_port_number,
});
rsvdz_checking_try_from!(ForceHeader {
    [3];1..=9,
    [3];16..=23,
});

impl GetExtendedProperty {
    param_align_16!(
        extended_property_context_pointer,
        "Extended Property Context Pointer"
    );

    rw_field!(
        pub, self,
        self.0.0[2]; 0..=15,
        extended_capability_identifier,
        "Extended Capability Identifier",
        u16
    );

    impl_subtype!();
    impl_ep_id!();
    impl_slot_id!();
}
impl_debug_from_methods!(GetExtendedProperty {
    extended_property_context_pointer,
    extended_capability_identifier,
    command_sub_type,
    endpoint_id,
    slot_id,
});
rsvdz_checking_try_from!(GetExtendedProperty {
    [0];0..=3,
    [2];16..=31,
    [3];1..=9,
});

impl SetExtendedProperty {
    rw_field!(
        pub, self,
        self.0.0[2]; 0..=15,
        extended_capability_identifier,
        "Extended Capability Identifier",
        u16
    );
    rw_field!(
        pub, self,
        self.0.0[2]; 16..=23,
        capability_parameter,
        "Capability Parameter",
        u8
    );

    impl_subtype!();
    impl_ep_id!();
    impl_slot_id!();
}
impl_debug_from_methods!(SetExtendedProperty {
    extended_capability_identifier,
    capability_parameter,
    command_sub_type,
    endpoint_id,
    slot_id,
});
rsvdz_checking_try_from!(SetExtendedProperty {
    [0];0..=3,
    [2];16..=31,
    [3];1..=9,
});