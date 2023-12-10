//! Host Controller Capability Registers

use accessor::single;
use accessor::Mapper;
use bit_field::BitField;

/// Host Controller Capability Registers
#[derive(Debug)]
pub struct Capability<M>
where
    M: Mapper + Clone,
{
    /// Capability Registers Length
    pub caplength: single::ReadOnly<CapabilityRegistersLength, M>,
    /// Host Controller Interface Version Number
    pub hciversion: single::ReadOnly<InterfaceVersionNumber, M>,
    /// Structural Parameters 1
    pub hcsparams1: single::ReadOnly<StructuralParameters1, M>,
    /// Structural Parameters 2
    pub hcsparams2: single::ReadOnly<StructuralParameters2, M>,
    /// Structural Parameters 3
    pub hcsparams3: single::ReadOnly<StructuralParameters3, M>,
    /// Capability Parameters 1
    pub hccparams1: single::ReadOnly<CapabilityParameters1, M>,
    /// Doorbell Offset
    pub dboff: single::ReadOnly<DoorbellOffset, M>,
    /// Runtime Register Space Offset
    pub rtsoff: single::ReadOnly<RuntimeRegisterSpaceOffset, M>,
    /// Capability Parameters 2
    pub hccparams2: single::ReadOnly<CapabilityParameters2, M>,
    /// Virtualization Based Trusted IO Register Space Offset
    pub vtiosoff: single::ReadOnly<VirtualizationBasedTrustedIoRegisterSpaceOffset, M>,
}
impl<M> Capability<M>
where
    M: Mapper + Clone,
{
    /// Creates a new accessor to the Host Controller Capability Registers.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the Host Controller Capability Registers are accessed only
    /// through this struct.
    ///
    /// # Panics
    ///
    /// This method panics if `mmio_base` is not aligned correctly.
    pub unsafe fn new(mmio_base: usize, mapper: &M) -> Self
    where
        M: Mapper,
    {
        macro_rules! m {
            ($offset:expr) => {
                single::ReadOnly::new(mmio_base + $offset, mapper.clone())
            };
        }

        Self {
            caplength: m!(0x00),
            hciversion: m!(0x02),
            hcsparams1: m!(0x04),
            hcsparams2: m!(0x08),
            hcsparams3: m!(0x0c),
            hccparams1: m!(0x10),
            dboff: m!(0x14),
            rtsoff: m!(0x18),
            hccparams2: m!(0x1c),
            vtiosoff: m!(0x20),
        }
    }
}

/// Capability Registers Length
#[repr(transparent)]
#[allow(clippy::module_name_repetitions)]
#[derive(Copy, Clone, Debug)]
pub struct CapabilityRegistersLength(u8);
impl CapabilityRegistersLength {
    /// Returns the length of the Capability Registers.
    #[must_use]
    pub fn get(self) -> u8 {
        self.0
    }
}

/// Interface Version Number
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct InterfaceVersionNumber(u16);
impl InterfaceVersionNumber {
    /// Returns a BCD encoding of the xHCI specification revision number supported by HC.
    ///
    /// The most significant byte of the value represents a major version and the least significant
    /// byte contains the minor revision extensions.
    ///
    /// For example, 0x0110 means xHCI version 1.1.0.
    #[must_use]
    pub fn get(self) -> u16 {
        self.0
    }
}

/// Structural Parameters 1
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct StructuralParameters1(u32);
impl StructuralParameters1 {
    ro_field!(0..=7, number_of_device_slots, "Number of Device Slots", u8);
    ro_field!(8..=18, number_of_interrupts, "Number of Interrupts", u16);
    ro_field!(24..=31, number_of_ports, "Number of Ports", u8);
}
impl_debug_from_methods! {
    StructuralParameters1{
        number_of_device_slots,
        number_of_interrupts,
        number_of_ports
    }
}

/// Structural Parameters 2
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct StructuralParameters2(u32);
impl StructuralParameters2 {
    ro_field!(
        0..=3,
        isochronous_scheduling_threshold,
        "Isochronous Scheduling Threshold",
        u8
    );

    /// Returns the maximum number of the elements the Event Ring Segment Table can contain.
    ///
    /// Note that the `ERST Max` field of the Structural Parameters 2 register contains the exponential
    /// value, but this method returns the calculated value.
    #[must_use]
    pub fn event_ring_segment_table_max(self) -> u16 {
        2_u16.pow(self.erst_max())
    }

    /// Returns the number of scratchpads that xHC needs.
    #[must_use]
    pub fn max_scratchpad_buffers(self) -> u32 {
        let h = self.max_scratchpad_buffers_hi();
        let l = self.max_scratchpad_buffers_lo();

        h << 5 | l
    }

    ro_bit!(26, scratchpad_restore, "Scratchpad Restore");

    fn erst_max(self) -> u32 {
        self.0.get_bits(4..=7)
    }

    fn max_scratchpad_buffers_hi(self) -> u32 {
        self.0.get_bits(21..=25)
    }

    fn max_scratchpad_buffers_lo(self) -> u32 {
        self.0.get_bits(27..=31)
    }
}
impl_debug_from_methods! {
    StructuralParameters2{
        isochronous_scheduling_threshold,
        event_ring_segment_table_max,
        max_scratchpad_buffers,
        scratchpad_restore
    }
}

/// Structural Parameters 3
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct StructuralParameters3(u32);
impl StructuralParameters3 {
    ro_field!(0..=7, u1_device_exit_latency, "U1 Device Exit Latency", u8);
    ro_field!(
        16..=31,
        u2_device_exit_latency,
        "U2 Device Exit Latency",
        u16
    );
}
impl_debug_from_methods! {
    StructuralParameters3{
        u1_device_exit_latency,
        u2_device_exit_latency
    }
}

/// Capability Parameters 1
#[repr(transparent)]
#[derive(Copy, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct CapabilityParameters1(u32);
impl CapabilityParameters1 {
    ro_bit!(0, addressing_capability, "64-bit Addressing Capability");
    ro_bit!(1, bw_negotiation_capability, "BW Negotiation Capability");
    ro_bit!(2, context_size, "Context Size");
    ro_bit!(3, port_power_control, "Port Power Control");
    ro_bit!(4, port_indicators, "Port Indicators");
    ro_bit!(5, light_hc_reset_capability, "Light HC Reset Capability");
    ro_bit!(
        6,
        latency_tolerance_messaging_capability,
        "Latency Tolerance Messaging Capability"
    );
    ro_bit!(7, no_secondary_sid_support, "No Secondary SID Support");
    ro_bit!(8, parse_all_event_data, "Parse All Event Data");
    ro_bit!(
        9,
        stopped_short_packet_capability,
        "Stopped - Short Packet Capability"
    );
    ro_bit!(10, stopped_edtla_capability, "Stopped EDTLA Capability");
    ro_bit!(
        11,
        contiguous_frame_id_capability,
        "Contiguous Frame ID Capability"
    );
    ro_field!(
        12..=15,
        maximum_primary_stream_array_size,
        "Maximum Primary Stream Array Size",
        u8
    );
    ro_field!(
        16..=31,
        xhci_extended_capabilities_pointer,
        "xHCI Extended Capabilities Pointer",
        u16
    );
}
impl_debug_from_methods! {
    CapabilityParameters1 {
        addressing_capability,
        bw_negotiation_capability,
        context_size,
        port_power_control,
        port_indicators,
        light_hc_reset_capability,
        latency_tolerance_messaging_capability,
        no_secondary_sid_support,
        parse_all_event_data,
        stopped_short_packet_capability,
        stopped_edtla_capability,
        contiguous_frame_id_capability,
        maximum_primary_stream_array_size,
        xhci_extended_capabilities_pointer
    }
}

/// Doorbell Offset
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct DoorbellOffset(u32);
impl DoorbellOffset {
    /// Returns the offset of the Doorbell Array from the MMIO base.
    #[must_use]
    pub fn get(self) -> u32 {
        self.0
    }
}

/// Runtime Register Space Offset
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct RuntimeRegisterSpaceOffset(u32);
impl RuntimeRegisterSpaceOffset {
    /// Returns the offset of the Runtime Registers from the MMIO base.
    #[must_use]
    pub fn get(self) -> u32 {
        self.0
    }
}

/// Capability Parameters 2
#[repr(transparent)]
#[allow(clippy::module_name_repetitions)]
#[derive(Copy, Clone)]
pub struct CapabilityParameters2(u32);
impl CapabilityParameters2 {
    ro_bit!(0, u3_entry_capability, "U3 Entry Capability");
    ro_bit!(
        1,
        configure_endpoint_command_max_exit_latency_too_large_capability,
        "Configure Endpoint Command Max Exit Latency Too Large Capability"
    );
    ro_bit!(
        2,
        force_save_context_capability,
        "Force Save Context Capability"
    );
    ro_bit!(
        3,
        compliance_transition_capability,
        "Compliance Transition Capability"
    );
    ro_bit!(
        4,
        large_esit_payload_capability,
        "Large ESIT Payload Capability"
    );
    ro_bit!(
        5,
        configuration_information_capability,
        "Configuration Information Capability"
    );
    ro_bit!(6, extended_tbc_capability, "Extended TBC Capability");
    ro_bit!(
        7,
        extended_tbc_trb_status_capability,
        "Extended TBC TRB Status Capability"
    );
    ro_bit!(
        8,
        get_set_extended_property_capability,
        "Get/Set Extended Property Capability"
    );
    ro_bit!(
        9,
        virtualization_based_trusted_io_capability,
        "Virtualization Based Trusted I/O Capability"
    );
}
impl_debug_from_methods! {
    CapabilityParameters2 {
        u3_entry_capability,
        configure_endpoint_command_max_exit_latency_too_large_capability,
        force_save_context_capability,
        compliance_transition_capability,
        large_esit_payload_capability,
        configuration_information_capability,
        extended_tbc_capability,
        extended_tbc_trb_status_capability,
        get_set_extended_property_capability,
        virtualization_based_trusted_io_capability
    }
}

/// Virtualization Based Trusted IO Register Space Offset
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct VirtualizationBasedTrustedIoRegisterSpaceOffset(u32);
impl VirtualizationBasedTrustedIoRegisterSpaceOffset {
    /// Returns the offset of the VTIO Registers from the MMIO base.
    #[must_use]
    pub fn get(self) -> u32 {
        self.0
    }
}
