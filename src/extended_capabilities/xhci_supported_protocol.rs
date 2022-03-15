//! xHCI Supported Protocol Capability

use super::ExtendedCapability;
use accessor::{array, single, Mapper};
use bit_field::BitField;
use core::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// The entry point to xHCI Supported Protocol Capability.
#[derive(Debug)]
pub struct XhciSupportedProtocol<M>
where
    M: Mapper + Clone,
{
    /// The first 16 bytes of xHCI Supported Protocol Capability.
    pub header: single::ReadWrite<Header, M>,
    /// Protocol Speed IDs.
    ///
    /// This field is `None` is `PSIC == 0`. Refer to 7.2.2.1.2 of the xHCI requirements
    /// specification for more information.
    pub psis: Option<array::ReadWrite<ProtocolSpeedId, M>>,
}
impl<M> XhciSupportedProtocol<M>
where
    M: Mapper + Clone,
{
    /// Creates an accessor to xHCI Supported Protocol Capability.
    ///
    /// # Safety
    ///
    /// `base` must be the correct address to xHCI Supported Protocol Capability.
    ///
    /// # Panics
    ///
    /// This method panics if `base` is not aligned correctly.
    pub unsafe fn new(base: usize, mapper: M) -> Self {
        let header: single::ReadWrite<Header, M> = single::ReadWrite::new(base, mapper.clone());
        let len = header.read_volatile().protocol_speed_id_count();
        let psis = if len > 0 {
            Some(array::ReadWrite::new(base + 0x10, len.into(), mapper))
        } else {
            None
        };

        Self { header, psis }
    }
}
impl<M> From<XhciSupportedProtocol<M>> for ExtendedCapability<M>
where
    M: Mapper + Clone,
{
    fn from(x: XhciSupportedProtocol<M>) -> Self {
        ExtendedCapability::XhciSupportedProtocol(x)
    }
}

/// The first 16 bytes of xHCI Supported Protocol Capability.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Header([u32; 4]);
impl Header {
    /// Returns the value of the Minor Revision field.
    #[must_use]
    pub fn minor_revision(self) -> u8 {
        self.0[0].get_bits(16..=23).try_into().unwrap()
    }

    /// Returns the value of the Major Revision field.
    #[must_use]
    pub fn major_revision(self) -> u8 {
        self.0[0].get_bits(24..=31).try_into().unwrap()
    }

    /// Returns the value of the Name String field.
    #[must_use]
    pub fn name_string(&self) -> u32 {
        self.0[1]
    }

    /// Returns the value of the Compatible Port Offset field.
    #[must_use]
    pub fn compatible_port_offset(self) -> u8 {
        self.0[2].get_bits(0..=7).try_into().unwrap()
    }

    /// Returns the value of the Compatible Port Count field.
    #[must_use]
    pub fn compatible_port_count(self) -> u8 {
        self.0[2].get_bits(8..=15).try_into().unwrap()
    }

    /// Returns the Link Soft Error Count Capability bit.
    ///
    /// **This bit is only valid for USB3.**
    #[must_use]
    pub fn link_soft_error_count_capability(self) -> bool {
        self.0[2].get_bit(24)
    }

    /// Returns the High-speed Only bit.
    ///
    /// **This bit is only valid for USB2.**
    #[must_use]
    pub fn high_speed_only(self) -> bool {
        self.0[2].get_bit(17)
    }

    /// Returns the Integrated Hub Implemented bit.
    ///
    /// **This bit is only valid for USB2.**
    #[must_use]
    pub fn integrated_hub_implemented(self) -> bool {
        self.0[2].get_bit(18)
    }

    /// Returns the Hardware LPM Capability bit.
    ///
    /// **This bit is only valid for USB2.**
    #[must_use]
    pub fn hardware_lpm_capability(self) -> bool {
        self.0[2].get_bit(19)
    }

    /// Returns the BESL LPM Capability bit.
    ///
    /// **This bit is only valid for USB2.**
    #[must_use]
    pub fn besl_lpm_capability(self) -> bool {
        self.0[2].get_bit(20)
    }

    /// Returns the value of the Hub Depth field.
    #[must_use]
    pub fn hub_depth(self) -> u8 {
        self.0[2].get_bits(25..=27).try_into().unwrap()
    }

    /// Returns the value of the Protocol Speed ID Count field.
    #[must_use]
    pub fn protocol_speed_id_count(self) -> u8 {
        self.0[2].get_bits(28..=31).try_into().unwrap()
    }

    /// Returns the value of the Protocol Slot Type field.
    #[must_use]
    pub fn protocol_slot_type(self) -> u8 {
        self.0[3].get_bits(0..=4).try_into().unwrap()
    }
}
impl_debug_from_methods! {
    Header {
        minor_revision,
        major_revision,
        name_string,
        compatible_port_offset,
        compatible_port_count,
        link_soft_error_count_capability,
        high_speed_only,
        integrated_hub_implemented,
        hardware_lpm_capability,
        besl_lpm_capability,
        hub_depth,
        protocol_speed_id_count,
        protocol_slot_type,
    }
}

/// Protocol Speed ID
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ProtocolSpeedId(u32);
impl ProtocolSpeedId {
    /// Returns the value of the Protocol Speed ID Value field.
    #[must_use]
    pub fn protocol_speed_id_value(self) -> u8 {
        self.0.get_bits(0..=3).try_into().unwrap()
    }

    /// Returns the value of the Protocol Speed ID Exponent field.
    #[must_use]
    pub fn protocol_speed_id_exponent(self) -> BitRate {
        let r = FromPrimitive::from_u32(self.0.get_bits(4..=5));
        r.expect("The value must be less than 4.")
    }

    /// Returns the value of the PSI Type field.
    #[must_use]
    pub fn psi_type(self) -> PsiType {
        let r = FromPrimitive::from_u32(self.0.get_bits(6..=7));
        r.expect("The PSI Type must not take the reserved value.")
    }

    /// Returns the PSI Full-duplex bit.
    #[must_use]
    pub fn psi_full_duplex(self) -> bool {
        self.0.get_bit(8)
    }

    /// Returns the value of the Link Protocol field.
    #[must_use]
    pub fn link_protocol(self) -> LinkProtocol {
        let r = FromPrimitive::from_u32(self.0.get_bits(14..=15));
        r.expect("The Link Protocol field must not take the reserved value.")
    }

    /// Returns the value of the Protocol Speed ID Mantissa field.
    #[must_use]
    pub fn protocol_speed_id_mantissa(self) -> u16 {
        self.0.get_bits(16..=31).try_into().unwrap()
    }
}
impl_debug_from_methods! {
    ProtocolSpeedId {
        protocol_speed_id_value,
        protocol_speed_id_exponent,
        psi_type,
        psi_full_duplex,
        link_protocol,
        protocol_speed_id_mantissa,
    }
}

/// Bit Rate
///
/// [`ProtocolSpeedId::protocol_speed_id_exponent`] returns a value of this type.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, FromPrimitive)]
pub enum BitRate {
    /// Bits Per Second
    Bits = 0,
    /// Kb/s
    Kb = 1,
    /// Mb/s
    Mb = 2,
    /// Gb/s
    Gb = 3,
}

/// PSI Type
///
/// [`ProtocolSpeedId::psi_type`] returns a value of this type.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, FromPrimitive)]
pub enum PsiType {
    /// Symmetric.
    ///
    /// Single DSI Dword.
    Symmetric = 0,
    /// Asymmetric Rx.
    ///
    /// Paired with Asymmetric Tx PSI Dword.
    AsymmetricRx = 2,
    /// Asymmetric Tx.
    ///
    /// Immediately follows Rx Asymmetric PSI Dword.
    AsymmetricTx = 3,
}

/// Link-level protocol
///
/// [`ProtocolSpeedId::link_protocol`] returns a value of this type.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, FromPrimitive)]
pub enum LinkProtocol {
    /// Super Speed
    SuperSpeed = 0,
    /// Super Speed Plus
    SuperSpeedPlus = 1,
}
