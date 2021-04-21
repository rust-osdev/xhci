//! xHCI Supported Protocol Capability

use bit_field::BitField;
use core::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Protocol Speed ID
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ProtocolSpeedId(u32);
impl ProtocolSpeedId {
    /// Returns the value of the Protocol Speed ID Value field.
    pub fn protocol_speed_id_value(self) -> u8 {
        self.0.get_bits(0..=3).try_into().unwrap()
    }

    /// Returns the value of the Protocol Speed ID Exponent field.
    pub fn protocol_speed_id_exponent(self) -> BitRate {
        let r = FromPrimitive::from_u32(self.0.get_bits(4..=5));
        r.expect("The value must be less than 4.")
    }

    /// Returns the value of the PSI Type field.
    pub fn psi_type(self) -> PsiType {
        let r = FromPrimitive::from_u32(self.0.get_bits(6..=7));
        r.expect("The PSI Type must not take the reserved value.")
    }

    /// Returns the PSI Full-duplex bit.
    pub fn psi_full_duplex(self) -> bool {
        self.0.get_bit(8)
    }

    /// Returns the value of the Link Protocol field.
    pub fn link_protocol(self) -> LinkProtocol {
        let r = FromPrimitive::from_u32(self.0.get_bits(14..=15));
        r.expect("The Link Protocol field must not take the reserved value.")
    }

    /// Returns the value of the Protocol Speed ID Mantissa field.
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
