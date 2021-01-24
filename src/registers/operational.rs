//! Host Controller Operational Registers

use super::capability::Capability;
use super::capability::CapabilityRegistersLength;
use crate::error::Error;
use accessor::Mapper;
use bit_field::BitField;
use core::{convert::TryInto, fmt};

/// Host Controller Operational Registers
///
/// This struct does not contain the Port Register set.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Operational {
    /// USB Command Register
    pub usbcmd: UsbCommandRegister,
    /// USB Status Register
    pub usbsts: UsbStatusRegister,
    /// Page Size Register
    pub pagesize: PageSizeRegister,
    _rsvd: [u8; 8],
    _dnctrl: u32,
    _rsvd2: [u8; 16],
    /// Device Context Base Address Array Pointer Register
    pub dcbaap: DeviceContextBaseAddressArrayPointerRegister,
    /// Configure Register
    pub config: ConfigureRegister,
}
impl Operational {
    /// Creates a new accessor to the Host Controller Operational Registers.
    ///
    /// # Safety
    ///
    /// Caller must ensure that only one accessor is created, otherwise undefined behaviors such as
    /// data race may occur.
    ///
    /// # Errors
    ///
    /// This method may return an [`accessor::Error::NotAligned`] error if `mmio_base` is not aligned.
    pub unsafe fn new<M>(
        mmio_base: usize,
        caplength: CapabilityRegistersLength,
        mapper: M,
    ) -> Result<accessor::Single<Self, M>, accessor::Error>
    where
        M: Mapper,
    {
        accessor::Single::new(mmio_base + usize::from(caplength.get()), mapper)
    }
}

/// USB Command Register
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct UsbCommandRegister(u32);
impl UsbCommandRegister {
    /// Returns the value of the Run/Stop bit.
    #[must_use]
    pub fn run_stop(self) -> bool {
        self.0.get_bit(0)
    }

    /// Sets the value of the Run/Stop bit.
    pub fn set_run_stop(&mut self, b: bool) {
        self.0.set_bit(0, b);
    }

    /// Returns the value of the Host Controller Reset bit.
    #[must_use]
    pub fn host_controller_reset(self) -> bool {
        self.0.get_bit(1)
    }

    /// Sets the value of the Host Controller Reset bit.
    pub fn set_host_controller_reset(&mut self, b: bool) {
        self.0.set_bit(1, b);
    }
}
impl fmt::Debug for UsbCommandRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UsbCommandRegister")
            .field("run_stop", &self.run_stop())
            .field("host_controller_reset", &self.host_controller_reset())
            .finish()
    }
}

/// USB Status Register
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct UsbStatusRegister(u32);
impl UsbStatusRegister {
    #[allow(clippy::doc_markdown)]
    /// Returns the value of the HCHalted bit.
    #[must_use]
    pub fn hc_halted(self) -> bool {
        self.0.get_bit(0)
    }

    /// Returns the value of the Host System Error bit.
    #[must_use]
    pub fn host_system_error(self) -> bool {
        self.0.get_bit(2)
    }

    /// Returns the value of the Controller Not Ready bit.
    #[must_use]
    pub fn controller_not_ready(self) -> bool {
        self.0.get_bit(11)
    }

    /// Returns the value of the Host Controller Error bit.
    #[must_use]
    pub fn host_controller_error(self) -> bool {
        self.0.get_bit(12)
    }
}
impl fmt::Debug for UsbStatusRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UsbStatusRegister")
            .field("hc_halted", &self.hc_halted())
            .field("host_system_error", &self.host_system_error())
            .field("controller_not_ready", &self.controller_not_ready())
            .field("host_controller_error", &self.host_controller_error())
            .finish()
    }
}

/// Page Size Register
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct PageSizeRegister(u32);
impl PageSizeRegister {
    /// Returns the value of the page size supported by xHC.
    #[must_use]
    pub fn get(self) -> u16 {
        self.0.try_into().unwrap()
    }
}

/// Command Ring Controller Register
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CommandRingControlRegister(u64);
impl CommandRingControlRegister {
    /// Sets the value of the Ring Cycle State bit.
    pub fn set_ring_cycle_state(&mut self, s: bool) {
        self.0.set_bit(0, s);
    }

    /// Returns the bit of the Command Ring Running bit.
    #[must_use]
    pub fn command_ring_running(self) -> bool {
        self.0.get_bit(3)
    }

    /// Sets the value of the Command Ring Pointer field. It must be 64 byte aligned.
    ///
    /// # Errors
    ///
    /// This method may return a `NotAligned` error if the given pointer is not 64
    /// byte aligned.
    pub fn set_command_ring_pointer(&mut self, p: u64) -> Result<(), Error> {
        if p.trailing_zeros() >= 6 {
            let p = p >> 6;
            self.0.set_bits(6..=63, p);
            Ok(())
        } else {
            Err(Error::NotAligned {
                alignment: 64,
                address: p,
            })
        }
    }
}
impl fmt::Debug for CommandRingControlRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandRingControlRegister")
            .field("command_ring_running", &self.command_ring_running())
            .finish()
    }
}

/// Device Context Base Address Array Pointer Register
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceContextBaseAddressArrayPointerRegister(u64);
impl DeviceContextBaseAddressArrayPointerRegister {
    /// Sets the value of the Device Context Base Address Array Pointer. It must be 64 byte aligned.
    ///
    /// # Errors
    ///
    /// This method may return a `NotAligned` error if the given pointer is not 64 byte aligned.
    pub fn set(&mut self, p: u64) -> Result<(), Error> {
        if p.trailing_zeros() >= 6 {
            self.0 = p;
            Ok(())
        } else {
            Err(Error::NotAligned {
                alignment: 64,
                address: p,
            })
        }
    }
}

/// Configure Register
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ConfigureRegister(u32);
impl ConfigureRegister {
    /// Returns the value of the Max Device Slots Enabled field.
    #[must_use]
    pub fn max_device_slots_enabled(self) -> u8 {
        self.0.get_bits(0..=7).try_into().unwrap()
    }

    /// Sets the value of the Max Device Slots Enabled field.
    pub fn set_max_device_slots_enabled(&mut self, s: u8) {
        self.0.set_bits(0..=7, s.into());
    }
}
impl fmt::Debug for ConfigureRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConfigureRegister")
            .field("max_device_slots_enabled", &self.max_device_slots_enabled())
            .finish()
    }
}

/// Port Register Set
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PortRegisterSet {
    pub portsc: PortStatusAndControlRegister,
    _portpmsc: u32,
    portli: u32,
    porthlpmc: u32,
}
impl PortRegisterSet {
    /// Creates a new accessor to the array of the Port Register Set.
    ///
    /// # Safety
    ///
    /// Caller must ensure that only one accessor is created, otherwise it may cause undefined
    /// behavior such as data race.
    ///
    /// # Errors
    ///
    /// This method may return a [`accessor::Error::NotAligned`] error if `mmio_base` is not
    /// aligned properly.
    pub unsafe fn new<M>(
        mmio_base: usize,
        capability: &Capability,
        mapper: M,
    ) -> Result<accessor::Array<Self, M>, accessor::Error>
    where
        M: Mapper,
    {
        let base = mmio_base + usize::from(capability.caplength.get()) + 0x400;
        accessor::Array::new(base, capability.hcsparams1.number_of_ports().into(), mapper)
    }
}

/// Port Status and Control Register
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PortStatusAndControlRegister(u32);
impl PortStatusAndControlRegister {
    /// Returns the value of the Current Connect Status bit.
    #[must_use]
    pub fn current_connect_status(self) -> bool {
        self.0.get_bit(0)
    }

    /// Returns the value of the Port Reset bit.
    #[must_use]
    pub fn port_reset(self) -> bool {
        self.0.get_bit(4)
    }

    /// Sets the value of the Port Reset bit.
    pub fn set_port_reset(&mut self, b: bool) {
        self.0.set_bit(4, b);
    }

    /// Returns the value of the Port Speed field.
    #[must_use]
    pub fn port_speed(self) -> u8 {
        self.0.get_bits(10..=13).try_into().unwrap()
    }

    /// Returns the value of the Port Reset Changed bit.
    #[must_use]
    pub fn port_reset_changed(self) -> bool {
        self.0.get_bit(21)
    }
}
impl fmt::Debug for PortStatusAndControlRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PortStatusAndControlRegister")
            .field("current_connect_status", &self.current_connect_status())
            .field("port_reset", &self.port_reset())
            .field("port_speed", &self.port_speed())
            .field("port_reset_changed", &self.port_reset_changed())
            .finish()
    }
}
