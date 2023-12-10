//! HCI Extended Power Management Capability.

use super::ExtendedCapability;
use accessor::{single, Mapper};

/// HCI Extended Power Management Capability.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct HciExtendedPowerManagement {
    _id: u8,
    _next: u8,
    /// Power Management Capabilities.
    pub pmc: PowerManagementCapabilities,
    /// Power Management Control Status Register.
    pub pmcsr: PowerManagementControlStatusRegister,
    /// PMESR_BSE.
    pub pmcsr_bse: PmesrBse,
    /// Data.
    pub data: Data,
}
impl<M> From<single::ReadWrite<HciExtendedPowerManagement, M>> for ExtendedCapability<M>
where
    M: Mapper + Clone,
{
    fn from(h: single::ReadWrite<HciExtendedPowerManagement, M>) -> Self {
        ExtendedCapability::HciExtendedPowerManagementCapability(h)
    }
}

/// Power Management Capabilities.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PowerManagementCapabilities(u16);
impl PowerManagementCapabilities {
    ro_field!(11..=15, pme_support, "PME_Support", u8);
    ro_bit!(10, d2_support, "D2_Support");
    ro_bit!(9, d1_support, "D1_Support");
    ro_field!(6..=8, aux_current, "Aux_Current", u8);
    ro_bit!(5, dsi, "DSI");
    ro_bit!(3, pme_clock, "PME Clock");
    ro_field!(0..=2, version, "Version", u8);
}
impl_debug_from_methods! {
    PowerManagementCapabilities {
        pme_support,
        d2_support,
        d1_support,
        aux_current,
        dsi,
        pme_clock,
        version,
    }
}

/// Power Management Control/Status Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PowerManagementControlStatusRegister(u16);
impl PowerManagementControlStatusRegister {
    rw1c_bit!(15, pme_status, "PME_Status");
    ro_field!(13..=14, data_scale, "Data_Scale", u8);
    rw_field!(9..=12, data_select, "Data_Select", u8);
    rw_bit!(8, pme_en, "PME_En");
    rw_field!(0..=1, power_state, "PowerState", u8);
}
impl_debug_from_methods! {
    PowerManagementControlStatusRegister {
        pme_status,
        data_scale,
        data_select,
        pme_en,
        power_state,
    }
}

/// `PMESR_BSE` Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PmesrBse(u8);
impl PmesrBse {
    ro_bit!(7, bpcc_en, "BPCC_En");
    ro_bit!(6, b2_b3, "B2_B3");
}
impl_debug_from_methods! {
    PmesrBse {
        bpcc_en,
        b2_b3,
    }
}

/// Data.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Data(u8);
impl Data {
    /// Returns the wrapped data.
    #[must_use]
    pub fn get(self) -> u8 {
        self.0
    }
}
