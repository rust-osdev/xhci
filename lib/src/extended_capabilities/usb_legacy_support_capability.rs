//! USB Legacy Support Capability

use super::ExtendedCapability;
use accessor::{single, Mapper};

/// USB Legacy Support Capability.
#[derive(Debug)]
pub struct UsbLegacySupport<M>
where
    M: Mapper + Clone,
{
    /// The first 4 byte of USB Legacy Support Capability.
    pub usblegsup: single::ReadWrite<LegSup, M>,
    /// USB Legacy Support Control/Status.
    pub usblegctlsts: single::ReadWrite<UsbLegacySupportControlStatus, M>,
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
        let usblegsup = single::ReadWrite::new(base, m.clone());
        let usblegctlsts = single::ReadWrite::new(base, m);

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
    rw_bit!(16, hc_bios_owned_semaphore, "HC BIOS Owned Semaphore");
    rw_bit!(24, hc_os_owned_semaphore, "HC OS Owned Semaphore");
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
    rw_bit!(0, usb_smi_enable, "USB SMI Enable");
    rw_bit!(
        4,
        smi_on_host_system_error_enable,
        "SMI on Host System Error Enable"
    );
    rw_bit!(13, smi_on_os_ownership_enable, "SMI on OS Ownership Enable");
    rw_bit!(14, smi_on_pci_command_enable, "SMI on PCI Command Enable");
    rw_bit!(15, smi_on_bar_enable, "SMI on BAR Enable");
    ro_bit!(16, smi_on_event_interrupt, "SMI on Event Interrupt");
    ro_bit!(20, smi_on_host_system_error, "SMI on Host System Error");
    rw1c_bit!(29, smi_on_os_ownership_change, "SMI on OS Ownership Change");
    rw1c_bit!(30, smi_on_pci_command, "SMI on PCI Command");
    rw1c_bit!(31, smi_on_bar, "SMI on BAR");
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
