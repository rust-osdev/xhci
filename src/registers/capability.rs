//! Host Controller Capability Registers

use accessor::Mapper;
use bit_field::BitField;
use core::{convert::TryInto, fmt};

/// Host Controller Capability Registers
#[derive(Debug)]
pub struct Capability<M>
where
    M: Mapper + Clone,
{
    /// Capability Registers Length
    pub caplength: accessor::Single<CapabilityRegistersLength, M>,
    /// Host Controller Interface Version Number
    pub hciversion: accessor::Single<InterfaceVersionNumber, M>,
    /// Structural Parameters 1
    pub hcsparams1: accessor::Single<StructuralParameters1, M>,
    /// Structural Parameters 2
    pub hcsparams2: accessor::Single<StructuralParameters2, M>,
    /// Structural Parameters 3
    pub hcsparams3: accessor::Single<StructuralParameters3, M>,
    /// Capability Parameters 1
    pub hccparams1: accessor::Single<CapabilityParameters1, M>,
    /// Doorbell Offset
    pub dboff: accessor::Single<DoorbellOffset, M>,
    /// Runtime Register Space Offset
    pub rtsoff: accessor::Single<RuntimeRegisterSpaceOffset, M>,
    /// Capability Parameters 2
    pub hccparams2: accessor::Single<CapabilityParameters2, M>,
    /// Virtualization Based Trusted IO Register Space Offset
    pub vtiosoff: accessor::Single<VirtualizationBasedTrustedIoRegisterSpaceOffset, M>,
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
                accessor::Single::new(mmio_base + $offset, mapper.clone())
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
    /// Returns the number of available device slots.
    #[must_use]
    pub fn number_of_device_slots(self) -> u8 {
        self.0.get_bits(0..=7).try_into().unwrap()
    }

    /// Returns the number of ports.
    #[must_use]
    pub fn number_of_ports(self) -> u8 {
        self.0.get_bits(24..=31).try_into().unwrap()
    }
}
impl fmt::Debug for StructuralParameters1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StructuralParameters1")
            .field("number_of_device_slots", &self.number_of_device_slots())
            .field("number_of_ports", &self.number_of_ports())
            .finish()
    }
}

/// Structural Parameters 2
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct StructuralParameters2(u32);
impl StructuralParameters2 {
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

    fn erst_max(self) -> u32 {
        self.0.get_bits(4..=7)
    }

    fn max_scratchpad_buffers_hi(self) -> u32 {
        self.0.get_bits(20..=25)
    }

    fn max_scratchpad_buffers_lo(self) -> u32 {
        self.0.get_bits(27..=31)
    }
}
impl fmt::Debug for StructuralParameters2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StructuralParameters2")
            .field(
                "event_ring_segment_table_max",
                &self.event_ring_segment_table_max(),
            )
            .field("max_scratchpad_buffers", &self.max_scratchpad_buffers())
            .finish()
    }
}

/// Structural Parameters 3
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct StructuralParameters3(u32);
impl StructuralParameters3 {
    /// Returns the value of the U1 Device Exit Latency field.
    #[must_use]
    pub fn u1_device_exit_latency(self) -> u8 {
        self.0.get_bits(0..=7).try_into().unwrap()
    }

    /// Returns the value of the U2 Device Exit Latency field.
    #[must_use]
    pub fn u2_device_exit_latency(self) -> u16 {
        self.0.get_bits(16..=31).try_into().unwrap()
    }
}
impl fmt::Debug for StructuralParameters3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StructuralParameters3")
            .field("u1_device_exit_latency", &self.u1_device_exit_latency())
            .field("u2_device_exit_latency", &self.u2_device_exit_latency())
            .finish()
    }
}

/// Capability Parameters 1
#[repr(transparent)]
#[derive(Copy, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct CapabilityParameters1(u32);
impl CapabilityParameters1 {
    /// Returns `true` if the xHC uses 64 byte Context data structures, and `false` if the xHC uses
    /// 32 byte Context data structures.
    #[must_use]
    pub fn context_size(self) -> bool {
        self.0.get_bit(2)
    }

    /// Returns the offset of the xHCI extended capability list from the MMIO base. If this value is
    /// zero, the list does not exist.
    /// The base address can be calculated by `(MMIO base) + (xECP) << 2`
    #[must_use]
    pub fn xhci_extended_capabilities_pointer(self) -> u16 {
        self.0.get_bits(16..=31).try_into().unwrap()
    }
}
impl fmt::Debug for CapabilityParameters1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CapabilityParameters1")
            .field("context_size", &self.context_size())
            .field("xhci_extended_capabilities_pointer", &self.context_size())
            .finish()
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
    /// Returns the value of the U3 Entry Capability field.
    #[must_use]
    pub fn u3_entry_capability(self) -> bool {
        self.0.get_bit(0)
    }

    /// Returns the value of the Configure Endpoint Command Max Exit Latency Too Large Capability
    /// field.
    #[must_use]
    pub fn configure_endpoint_command_max_exit_latency_too_large_capability(self) -> bool {
        self.0.get_bit(1)
    }

    /// Returns the value of the Force Save Context Capability field.
    #[must_use]
    pub fn force_save_context_capability(self) -> bool {
        self.0.get_bit(2)
    }

    /// Returns the value of the Compliance Transition Capability field.
    #[must_use]
    pub fn compliance_transition_capability(self) -> bool {
        self.0.get_bit(3)
    }

    /// Returns the value of the Large ESIT Payload Capability field.
    #[must_use]
    pub fn large_esit_payload_capability(self) -> bool {
        self.0.get_bit(4)
    }

    /// Returns the value of the Configuration Information Capability field.
    #[must_use]
    pub fn configuration_information_capability(self) -> bool {
        self.0.get_bit(5)
    }

    /// Returns the value of the Extended TBC Capability field.
    #[must_use]
    pub fn extended_tbc_capability(self) -> bool {
        self.0.get_bit(6)
    }

    /// Returns the value of the Extended TBC TRB Status Capability field.
    #[must_use]
    pub fn extended_tbc_trb_status_capability(self) -> bool {
        self.0.get_bit(7)
    }

    /// Returns the value of the Get/Set Extended Property Capability field.
    #[must_use]
    pub fn get_set_extended_property_capability_field(self) -> bool {
        self.0.get_bit(8)
    }

    /// Returns the value of the Virtualization Based Trusted I/O Capability field.
    #[must_use]
    pub fn virtualization_based_trusted_io_capability(self) -> bool {
        self.0.get_bit(9)
    }
}
impl fmt::Debug for CapabilityParameters2 {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CapabilityParameters2")
            .field("u3_entry_capability", &self.u3_entry_capability())
            .field(
                "configure_endpoint_command_max_exit_latency_too_large_capability",
                &self.configure_endpoint_command_max_exit_latency_too_large_capability(),
            )
            .field(
                "force_save_context_capability",
                &self.force_save_context_capability(),
            )
            .field(
                "compliance_transition_capability",
                &self.compliance_transition_capability(),
            )
            .field(
                "large_esit_payload_capability",
                &self.large_esit_payload_capability(),
            )
            .field(
                "configuration_information_capability",
                &self.configuration_information_capability(),
            )
            .field("extended_tbc_capability", &self.extended_tbc_capability())
            .field(
                "extended_tbc_trb_status_capability",
                &self.extended_tbc_trb_status_capability(),
            )
            .field(
                "get_set_extended_property_capability_field",
                &self.get_set_extended_property_capability_field(),
            )
            .field(
                "virtualization_based_trusted_io_capability",
                &self.virtualization_based_trusted_io_capability(),
            )
            .finish()
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
