//! USB Legacy Support Capability

use super::ExtendedCapability;
use accessor::Mapper;
use accessor::Single;
use bit_field::BitField;

/// USB Legacy Support Capability.
#[derive(Debug)]
pub struct UsbLegacySupport<M>
where
    M: Mapper + Clone,
{
    usblegsup: Single<LegSup, M>,
    usblegctlsts: Single<UsbLegacySupportControlStatus, M>,
}
impl<M> UsbLegacySupport<M>
where
    M: Mapper + Clone,
{
    /// Creates an instance of [`UsbLegacySupport`].
    ///
    /// # Safety
    ///
    /// `base` must be the correct address to USB Legacy Support Capability.
    ///
    /// The caller must ensure that the capability is only accessed through the returned accessor.
    ///
    /// # Panics
    ///
    /// This method panics if `base` is not aligned correctly.
    pub unsafe fn new(base: usize, m: M) -> Self {
        let usblegsup = Single::new(base, m.clone());
        let usblegctlsts = Single::new(base, m);

        Self {
            usblegsup,
            usblegctlsts,
        }
    }
}
impl<M> From<UsbLegacySupport<M>> for ExtendedCapability<M>
where
    M: Mapper + Clone,
{
    fn from(u: UsbLegacySupport<M>) -> Self {
        ExtendedCapability::UsbLegacySupport(u)
    }
}

/// The first 4-byte of the USB Legacy Support Capability.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct LegSup(u32);
impl LegSup {
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
    LegSup {
        hc_bios_owned_semaphore,
        hc_os_owned_semaphore,
    }
}

/// USB Legacy Support Control/Status.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct UsbLegacySupportControlStatus(u32);
impl UsbLegacySupportControlStatus {
    /// Returns the USB SMI Enable bit.
    #[must_use]
    pub fn usb_smi_enable(self) -> bool {
        self.0.get_bit(0)
    }

    /// Sets the USB SMI Enable bit.
    pub fn set_usb_smi_enable(&mut self, b: bool) {
        self.0.set_bit(0, b);
    }

    /// Returns the SMI on Host System Error Enable bit.
    #[must_use]
    pub fn smi_on_host_system_error_enable(self) -> bool {
        self.0.get_bit(4)
    }

    /// Sets the SMI on Host System Error Enable bit.
    pub fn set_smi_on_host_system_error_enable(&mut self, b: bool) {
        self.0.set_bit(4, b);
    }

    /// Returns the SMI on OS Ownership Enable bit.
    #[must_use]
    pub fn smi_on_os_ownership_enable(self) -> bool {
        self.0.get_bit(13)
    }

    /// Sets the SMi on OS Ownership Enable bit.
    pub fn set_smi_on_os_ownership_enable(&mut self, b: bool) {
        self.0.set_bit(13, b);
    }

    /// Returns the SMI on PCI Command Enable bit.
    #[must_use]
    pub fn smi_on_pci_command_enable(self) -> bool {
        self.0.get_bit(14)
    }

    /// Sets the SMI on PCI Command Enable bit.
    pub fn set_smi_on_pci_command_enable(&mut self, b: bool) {
        self.0.set_bit(14, b);
    }

    /// Returns the SMI on BAR Enable bit.
    #[must_use]
    pub fn smi_on_bar_enable(self) -> bool {
        self.0.get_bit(15)
    }

    /// Sets the SMI on BAR Enable bit.
    pub fn set_smi_on_bar_enable(&mut self, b: bool) {
        self.0.set_bit(15, b);
    }

    /// Returns the SMI on Event Interrupt bit.
    #[must_use]
    pub fn smi_on_event_interrupt(self) -> bool {
        self.0.get_bit(16)
    }

    /// Returns the SMI on Host System Error bit.
    #[must_use]
    pub fn smi_on_host_system_error(self) -> bool {
        self.0.get_bit(20)
    }

    /// Returns the SMI on OS Ownership Change bit.
    #[must_use]
    pub fn smi_on_os_ownership_change(self) -> bool {
        self.0.get_bit(29)
    }

    /// Clears the SMI on OS Ownership Change bit.
    pub fn clear_smi_on_os_ownership(&mut self) {
        self.0.set_bit(29, true);
    }

    /// Returns the SMI on PCI Command bit.
    #[must_use]
    pub fn smi_on_pci_command(self) -> bool {
        self.0.get_bit(30)
    }

    /// Clears the SMI on PCI Command bit.
    pub fn clear_smi_on_pci_command(&mut self) {
        self.0.set_bit(30, true);
    }

    /// Returns the SMI on BAR bit.
    #[must_use]
    pub fn smi_on_bar(self) -> bool {
        self.0.get_bit(31)
    }

    /// Clears the SMI on BAR bit.
    pub fn clear_smi_on_bar(&mut self) {
        self.0.set_bit(31, true);
    }
}
impl_debug_from_methods! {
    UsbLegacySupportControlStatus {
        usb_smi_enable,
        smi_on_host_system_error_enable,
        smi_on_os_ownership_enable,
        smi_on_pci_command_enable,
        smi_on_bar_enable,
        smi_on_event_interrupt,
        smi_on_host_system_error,
        smi_on_os_ownership_change,
        smi_on_pci_command,
        smi_on_bar,
    }
}
