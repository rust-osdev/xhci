//! USB Legacy Support Capability

use bit_field::BitField;

/// USB Legacy Support Capability
#[repr(transparent)]
pub struct UsbLegacySupportCapability(u32);
impl UsbLegacySupportCapability {
    /// Returns the value of the HC BIOS Owned Semaphore bit.
    pub fn hc_bios_owned_semaphore(&self) -> bool {
        self.0.get_bit(16)
    }

    /// Sets the value of the HC BIOS Owned Semaphore bit.
    pub fn set_hc_bios_owned_semaphore(&mut self, b: bool) {
        self.0.set_bit(16, b);
    }

    /// Returns the value of the HC OS Owned Semaphore bit.
    pub fn hc_os_owned_semaphore(&self) -> bool {
        self.0.get_bit(24)
    }

    /// Gets the value of the HC OS Owned Semaphore bit.
    pub fn set_hc_os_owned_semaphore(&mut self, b: bool) {
        self.0.set_bit(24, b);
    }
}
