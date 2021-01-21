//! Host Controller Capability Registers

use bit_field::BitField;
use core::convert::TryInto;

/// Host Controller Capability Registers.
#[repr(C, packed)]
pub struct Capability {
    capability_registers_length: CapabilityRegistersLength,
    rsvd: u8,
    host_controller_interface_version_number: u16,
    structural_parameters_1: StructuralParameters1,
    structural_parameters_2: StructuralParameters2,
    structural_parameters_3: u32,
    capability_parameters_1: CapabilityParameters1,
    doorbell_offset: DoorbellOffset,
    runtime_register_space_offset: RuntimeRegisterSpaceOffset,
}
impl Capability {
    /// Returns the accessor to the Capability Registers Length.
    pub fn capability_registers_length(&self) -> &CapabilityRegistersLength {
        &self.capability_registers_length
    }

    /// Returns the accessor to the Structural Parameters 1.
    pub fn structural_parameters_1(&self) -> &StructuralParameters1 {
        &self.structural_parameters_1
    }

    /// Returns the accessor to the Structural Parameters 2.
    pub fn structural_parameters_2(&self) -> &StructuralParameters2 {
        &self.structural_parameters_2
    }

    /// Returns the accessor to the Capability Parameters 1.
    pub fn capability_parameters_1(&self) -> &CapabilityParameters1 {
        &self.capability_parameters_1
    }

    /// Returns the accessor to the Doorbell Offset.
    pub fn doorbell_offset(&self) -> &DoorbellOffset {
        &self.doorbell_offset
    }
}

#[repr(transparent)]
/// Capability Registers Length
pub struct CapabilityRegistersLength(u8);
impl CapabilityRegistersLength {
    /// Returns the length of the Capability Registers.
    pub fn get(&self) -> u8 {
        self.0
    }
}

/// Structural Parameters 1
#[repr(transparent)]
pub struct StructuralParameters1(u32);
impl StructuralParameters1 {
    /// Returns the number of available device slots.
    pub fn number_of_device_slots(&self) -> u8 {
        self.0.get_bits(0..=7).try_into().unwrap()
    }

    /// Returns the number of ports.
    pub fn number_of_ports(&self) -> u8 {
        self.0.get_bits(24..=31).try_into().unwrap()
    }
}

/// Structural Parameters 2
#[repr(transparent)]
pub struct StructuralParameters2(u32);
impl StructuralParameters2 {
    /// Returns the maximum number of the elements the Event Ring Segment Table can contain.
    ///
    /// Note that the `ERST Max` field of the Structural Parameters 2 register contains the exponential
    /// value, but this method returns the calculated value.
    pub fn event_ring_segment_table_max(&self) -> u16 {
        2_u16.pow(self.erst_max())
    }

    /// Returns the number of scratchpads that xHC needs.
    pub fn max_scratchpad_buffers(&self) -> u32 {
        let h = self.max_scratchpad_buffers_hi();
        let l = self.max_scratchpad_buffers_lo();

        h << 5 | l
    }

    fn erst_max(&self) -> u32 {
        self.0.get_bits(4..=7)
    }

    fn max_scratchpad_buffers_hi(&self) -> u32 {
        self.0.get_bits(20..=25)
    }

    fn max_scratchpad_buffers_lo(&self) -> u32 {
        self.0.get_bits(27..=31)
    }
}

/// Capability Parameters 1
#[repr(transparent)]
pub struct CapabilityParameters1(u32);
impl CapabilityParameters1 {
    /// Returns `true` if the xHC uses 64 byte Context data structures, and `false` if the xHC uses
    /// 32 byte Context data structures.
    pub fn context_size(&self) -> bool {
        self.0.get_bit(2)
    }

    /// Returns the offset of the xHCI extended capability list from the MMIO base. If this value is
    /// zero, the list does not exist.
    /// The base address can be calculated by `(MMIO base) + (xECP) << 2`
    pub fn xhci_extended_capabilities_pointer(&self) -> u16 {
        self.0.get_bits(16..=31).try_into().unwrap()
    }
}

/// Doorbell Offset
#[repr(transparent)]
pub struct DoorbellOffset(u32);
impl DoorbellOffset {
    /// Returns the offset of the Doorbell Array from the MMIO base.
    pub fn get(&self) -> u32 {
        self.0
    }
}

/// Runtime Register Space Offset
#[repr(transparent)]
pub struct RuntimeRegisterSpaceOffset(u32);
impl RuntimeRegisterSpaceOffset {
    /// Returns the offset of the Runtime Registers from the MMIO base.
    pub fn get(&self) -> u32 {
        self.0
    }
}
