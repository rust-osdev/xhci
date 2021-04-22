//! HCI Extended Power Management Capability.

use super::ExtendedCapability;
use accessor::Mapper;
use accessor::Single;
use bit_field::BitField;
use core::convert::TryInto;

/// HCI Extended Power Management Capability.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct HciExtendedPowerManagement([u32; 2]);
impl HciExtendedPowerManagement {
    /// Returns the value of the `PME_Support` field.
    #[must_use]
    pub fn pme_support(self) -> u8 {
        self.0[0].get_bits(27..=31).try_into().unwrap()
    }

    /// Returns the `D2_Support` bit.
    #[must_use]
    pub fn d2_support(self) -> bool {
        self.0[0].get_bit(26)
    }

    /// Returns the `D1_Support` bit.
    #[must_use]
    pub fn d1_support(self) -> bool {
        self.0[0].get_bit(25)
    }

    /// Returns the value of the `Aux_Current` field.
    #[must_use]
    pub fn aux_current(self) -> u8 {
        self.0[0].get_bits(22..=24).try_into().unwrap()
    }

    /// Returns the DSI bit.
    #[must_use]
    pub fn dsi(self) -> bool {
        self.0[0].get_bit(21)
    }

    /// Returns the PME Clock bit.
    #[must_use]
    pub fn pme_clock(self) -> bool {
        self.0[0].get_bit(19)
    }

    /// Returns the value of the Version field.
    #[must_use]
    pub fn version(self) -> u8 {
        self.0[0].get_bits(16..=18).try_into().unwrap()
    }

    /// Returns the `PME_Status` bit.
    #[must_use]
    pub fn pme_status(self) -> bool {
        self.0[1].get_bit(15)
    }

    /// Clears the `PME_Status` bit.
    pub fn clear_pme_status(&mut self) {
        self.0[1].set_bit(15, true);
    }

    /// Returns the value of the `Data_Scale` field.
    #[must_use]
    pub fn data_scale(self) -> u8 {
        self.0[1].get_bits(13..=14).try_into().unwrap()
    }

    /// Returns the value of the `Data_Select` field.
    #[must_use]
    pub fn data_select(self) -> u8 {
        self.0[1].get_bits(9..=12).try_into().unwrap()
    }

    /// Sets the value of the `Data_Select` field.
    pub fn set_data_select(&mut self, data_select: u8) {
        self.0[1].set_bits(9..=12, data_select.into());
    }

    /// Returns the `PME_En` bit.
    #[must_use]
    pub fn pme_en(self) -> bool {
        self.0[1].get_bit(8)
    }

    /// Sets the `PME_En` bit.
    pub fn set_pme_en(&mut self, b: bool) {
        self.0[1].set_bit(8, b);
    }

    /// Returns the value of the `PowerState` field.
    #[must_use]
    pub fn power_state(self) -> u8 {
        self.0[1].get_bits(0..=1).try_into().unwrap()
    }

    /// Sets the value of the `PowerState` field.
    pub fn set_power_state(&mut self, s: u8) {
        self.0[1].set_bits(0..=1, s.into());
    }

    /// Returns the `BPCC_En` bit.
    #[must_use]
    pub fn bpcc_en(self) -> bool {
        self.0[1].get_bit(23)
    }

    /// Returns the `B2_B3` bit.
    #[must_use]
    pub fn b2_b3(self) -> bool {
        self.0[1].get_bit(22)
    }

    /// Returns the Data field.
    #[must_use]
    pub fn data(self) -> u8 {
        self.0[1].get_bits(24..=31).try_into().unwrap()
    }
}
impl_debug_from_methods! {
    HciExtendedPowerManagement {
        pme_support,
        d2_support,
        d1_support,
        aux_current,
        dsi,
        pme_clock,
        version,
        pme_status,
        data_scale,
        data_select,
        pme_en,
        power_state,
        bpcc_en,
        b2_b3,
        data,
    }
}
impl<M> From<Single<HciExtendedPowerManagement, M>> for ExtendedCapability<M>
where
    M: Mapper + Clone,
{
    fn from(h: Single<HciExtendedPowerManagement, M>) -> Self {
        ExtendedCapability::HciExtendedPowerManagementCapability(h)
    }
}
