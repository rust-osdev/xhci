//! USB Legacy Support Capability

use super::ExtendedCapability;
use accessor::Mapper;
use accessor::Single;
use bit_field::BitField;

/// USB Legacy Support Capability
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct UsbLegacySupportCapability(u32);
impl UsbLegacySupportCapability {
    /// Returns the value of the HC BIOS Owned Semaphore bit.
    #[must_use]
    pub fn hc_bios_owned_semaphore(self) -> bool {
        self.0.get_bit(16)
    }

    /// Sets the value of the HC BIOS Owned Semaphore bit.
    pub fn set_hc_bios_owned_semaphore(&mut self, b: bool) {
        self.0.set_bit(16, b);
    }

    /// Returns the value of the HC OS Owned Semaphore bit.
    #[must_use]
    pub fn hc_os_owned_semaphore(self) -> bool {
        self.0.get_bit(24)
    }

    /// Gets the value of the HC OS Owned Semaphore bit.
    pub fn set_hc_os_owned_semaphore(&mut self, b: bool) {
        self.0.set_bit(24, b);
    }
}
impl_debug_from_methods! {
    UsbLegacySupportCapability {
        hc_bios_owned_semaphore,
        hc_os_owned_semaphore,
    }
}
impl<M> From<Single<UsbLegacySupportCapability, M>> for ExtendedCapability<M>
where
    M: Mapper + Clone,
{
    fn from(l: Single<UsbLegacySupportCapability, M>) -> Self {
        ExtendedCapability::UsbLegacySupportCapability(l)
    }
}
