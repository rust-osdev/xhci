//! Host Controller Operational Registers

use super::capability::{Capability, CapabilityRegistersLength};
use accessor::Mapper;
use bit_field::BitField;
use core::convert::TryInto;

/// Host Controller Operational Registers
///
/// This struct does not contain the Port Register set.
#[derive(Debug)]
pub struct Operational<M>
where
    M: Mapper + Clone,
{
    /// USB Command Register
    pub usbcmd: accessor::Single<UsbCommandRegister, M>,
    /// USB Status Register
    pub usbsts: accessor::Single<UsbStatusRegister, M>,
    /// Page Size Register
    pub pagesize: accessor::Single<PageSizeRegister, M>,
    /// Command Ring Control Register
    pub crcr: accessor::Single<CommandRingControlRegister, M>,
    /// Device Context Base Address Array Pointer Register
    pub dcbaap: accessor::Single<DeviceContextBaseAddressArrayPointerRegister, M>,
    /// Configure Register
    pub config: accessor::Single<ConfigureRegister, M>,
}
impl<M> Operational<M>
where
    M: Mapper + Clone,
{
    /// Creates a new accessor to the Host Controller Operational Registers.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the Host Controller Operational Registers are accessed only
    /// through this struct.
    ///
    /// # Panics
    ///
    /// This method panics if the base address of the Host Controller Operational Registers is not
    /// aligned correctly.
    pub unsafe fn new(mmio_base: usize, caplength: CapabilityRegistersLength, mapper: &M) -> Self
    where
        M: Mapper,
    {
        let base = mmio_base + usize::from(caplength.get());

        macro_rules! m {
            ($offset:expr) => {
                accessor::Single::new(base + $offset, mapper.clone())
            };
        }

        Self {
            usbcmd: m!(0x00),
            usbsts: m!(0x04),
            pagesize: m!(0x08),
            crcr: m!(0x18),
            dcbaap: m!(0x30),
            config: m!(0x38),
        }
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
impl_debug_from_methods! {
    UsbCommandRegister{
        run_stop,
        host_controller_reset,
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
impl_debug_from_methods! {
    UsbStatusRegister{
        hc_halted,
        host_system_error,
        controller_not_ready,
        host_controller_error,
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
    /// # Panics
    ///
    /// This method panics if the given pointer is not 64 byte aligned.
    pub fn set_command_ring_pointer(&mut self, p: u64) {
        assert!(p.trailing_zeros() >= 6);

        let p = p >> 6;
        self.0.set_bits(6..=63, p);
    }
}
impl_debug_from_methods! {
    CommandRingControlRegister{
        command_ring_running
    }
}

/// Device Context Base Address Array Pointer Register
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceContextBaseAddressArrayPointerRegister(u64);
impl DeviceContextBaseAddressArrayPointerRegister {
    /// Sets the value of the Device Context Base Address Array Pointer. It must be 64 byte aligned.
    ///
    /// # Panics
    ///
    /// This method panics if the given pointer is not 64 byte aligned.
    pub fn set(&mut self, p: u64) {
        assert!(p.trailing_zeros() >= 6);
        self.0 = p;
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
impl_debug_from_methods! {
    ConfigureRegister {
        max_device_slots_enabled
    }
}

/// Port Register Set
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PortRegisterSet {
    /// Port Status and Control Register
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
    /// # Panics
    ///
    /// This method panics if the base address of the Port Register Sets is not aligned correctly.
    pub unsafe fn new<M1, M2>(
        mmio_base: usize,
        capability: &Capability<M2>,
        mapper: M1,
    ) -> accessor::Array<Self, M1>
    where
        M1: Mapper,
        M2: Mapper + Clone,
    {
        let base = mmio_base + usize::from(capability.caplength.read().get()) + 0x400;
        accessor::Array::new(
            base,
            capability.hcsparams1.read().number_of_ports().into(),
            mapper,
        )
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
impl_debug_from_methods! {
    PortStatusAndControlRegister{
        current_connect_status,
        port_reset,
        port_speed,
        port_reset_changed,
    }
}
