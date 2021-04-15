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
    /// Device Notification Control
    pub dnctrl: accessor::Single<DeviceNotificationControl, M>,
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
            dnctrl: m!(0x14),
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

    /// Returns the value of the Interrupter Enable bit.
    #[must_use]
    pub fn interrupter_enable(self) -> bool {
        self.0.get_bit(2)
    }

    /// Sets the value of the Interrupter Enable bit.
    pub fn set_interrupter_enable(&mut self, b: bool) {
        self.0.set_bit(2, b);
    }

    /// Returns the value of the Host System Error Enable bit.
    #[must_use]
    pub fn host_system_error_enable(self) -> bool {
        self.0.get_bit(3)
    }

    /// Sets the value of the Host System Error Enable bit.
    pub fn set_host_system_error_enable(&mut self, b: bool) {
        self.0.set_bit(3, b);
    }

    /// Returns the value of the Light Host Controller Reset bit.
    #[must_use]
    pub fn light_host_controller_reset(self) -> bool {
        self.0.get_bit(7)
    }

    /// Sets the value of the Light Host Controller Reset bit.
    pub fn set_light_host_controller_reset(&mut self, b: bool) {
        self.0.set_bit(7, b);
    }

    /// Returns the value of the Controller Save State bit.
    #[must_use]
    pub fn controller_save_state(self) -> bool {
        self.0.get_bit(8)
    }

    /// Sets the value of the Controller Save State bit.
    pub fn set_controller_save_state(&mut self, b: bool) {
        self.0.set_bit(8, b);
    }

    /// Returns the value of the Controller Restore State bit.
    #[must_use]
    pub fn controller_restore_state(self) -> bool {
        self.0.get_bit(9)
    }

    /// Sets the value of the Controller Restore State bit.
    pub fn set_controller_restore_state(&mut self, b: bool) {
        self.0.set_bit(9, b);
    }

    /// Returns the value of the Enable Wrap Event bit.
    #[must_use]
    pub fn enable_wrap_event(self) -> bool {
        self.0.get_bit(10)
    }

    /// Sets the value of the Enable Wrap Event bit.
    pub fn set_enable_wrap_event(&mut self, b: bool) {
        self.0.set_bit(10, b);
    }

    /// Returns the value of the Enable U3 MFINDEX Stop bit.
    #[must_use]
    pub fn enable_u3_mfindex_stop(self) -> bool {
        self.0.get_bit(11)
    }

    /// Sets the value of the Enable U3 MFINDEX Stop bit.
    pub fn set_enable_u3_mfindex_stop(&mut self, b: bool) {
        self.0.set_bit(11, b);
    }

    /// Returns the value of the CEM Enable bit.
    #[must_use]
    pub fn cem_enable(self) -> bool {
        self.0.get_bit(13)
    }

    /// Sets the value of the CEM Enable bit.
    pub fn set_cem_enable(&mut self, b: bool) {
        self.0.set_bit(13, b);
    }

    /// Returns the value of the Extended TBC Enable bit.
    #[must_use]
    pub fn extended_tbc_enable(self) -> bool {
        self.0.get_bit(14)
    }

    /// Sets the value of the Extended TBC Enable bit.
    pub fn set_extended_tbc_enable(&mut self, b: bool) {
        self.0.set_bit(14, b);
    }

    /// Returns the value of the Extended TBC Status Enable bit.
    #[must_use]
    pub fn extended_tbc_status_enable(self) -> bool {
        self.0.get_bit(15)
    }

    /// Sets the value of the Extended TBC Status Enable bit.
    pub fn set_extended_tbc_status_enable(&mut self, b: bool) {
        self.0.set_bit(15, b);
    }

    /// Returns the value of the VTIO Enable bit.
    #[must_use]
    pub fn vtio_enable(self) -> bool {
        self.0.get_bit(16)
    }

    /// Sets the value of the VTIO Enable bit.
    pub fn set_vtio_enable(&mut self, b: bool) {
        self.0.set_bit(16, b);
    }
}
impl_debug_from_methods! {
    UsbCommandRegister{
        run_stop,
        host_controller_reset,
        interrupter_enable,
        host_system_error_enable,
        light_host_controller_reset,
        controller_save_state,
        controller_restore_state,
        enable_wrap_event,
        enable_u3_mfindex_stop,
        cem_enable,
        extended_tbc_enable,
        extended_tbc_status_enable,
        vtio_enable,
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

    /// Clears the Host System Error bit.
    pub fn clear_host_system_error(&mut self) {
        self.0.set_bit(2, true);
    }

    /// Returns the value of the Event Interrupt bit.
    #[must_use]
    pub fn event_interrupt(self) -> bool {
        self.0.get_bit(3)
    }

    /// Clears the Event Interrupt bit.
    pub fn clear_event_interrupt(&mut self) {
        self.0.set_bit(3, true);
    }

    /// Returns the value of the Port Change Detect bit.
    #[must_use]
    pub fn port_change_detect(self) -> bool {
        self.0.get_bit(4)
    }

    /// Clears the Port Change Detect bit.
    pub fn clear_port_change_detect(&mut self) {
        self.0.set_bit(4, true);
    }

    /// Returns the value of the Save State Status field.
    #[must_use]
    pub fn save_state_status(self) -> bool {
        self.0.get_bit(8)
    }

    /// Returns the value of the Restore State Status field.
    #[must_use]
    pub fn restore_state_status(self) -> bool {
        self.0.get_bit(9)
    }

    /// Returns the value of the Save/Restore Error field.
    #[must_use]
    pub fn save_restore_error(self) -> bool {
        self.0.get_bit(10)
    }

    /// Clears the Save/Restore Error field.
    pub fn clear_save_restore_error(&mut self) {
        self.0.set_bit(10, true);
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
        event_interrupt,
        port_change_detect,
        save_state_status,
        restore_state_status,
        save_restore_error,
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

/// Device Notification Control
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceNotificationControl(u32);
impl DeviceNotificationControl {
    /// Returns the value of the `i`th of the Notification Enable field. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i >= 16`.
    #[must_use]
    pub fn get(self, i: usize) -> bool {
        Self::ensure_index_is_within_range(i);

        self.0.get_bit(i)
    }

    /// Sets the value of the `i`th of the Notification Enable field. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i >= 16`.
    pub fn set(&mut self, i: usize, ne: bool) -> &mut Self {
        Self::ensure_index_is_within_range(i);

        self.0.set_bit(i, ne);
        self
    }

    fn ensure_index_is_within_range(i: usize) {
        assert!(
            i < 16,
            "The index of the Notification Enable field must be less than 16."
        );
    }
}

/// Command Ring Controller Register
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CommandRingControlRegister(u64);
impl CommandRingControlRegister {
    /// Sets the Ring Cycle State bit.
    pub fn set_ring_cycle_state(&mut self, s: bool) {
        self.0.set_bit(0, s);
    }

    /// Sets the Command Stop bit.
    pub fn set_command_stop(&mut self) {
        self.0.set_bit(1, true);
    }

    /// Sets the Command Abort bit.
    pub fn set_command_abort(&mut self) {
        self.0.set_bit(2, true);
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
    /// Returns the value of the Device Context Base Address Array Pointer.
    #[must_use]
    pub fn get(self) -> u64 {
        self.0
    }

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

    /// Returns the value of the U3 Entry Enable bit.
    #[must_use]
    pub fn u3_entry_enable(self) -> bool {
        self.0.get_bit(8)
    }

    /// Sets the value of the U3 Entry Enable bit.
    pub fn set_u3_entry_enable(&mut self, b: bool) {
        self.0.set_bit(8, b);
    }

    /// Returns the value of the Configuration Information Enable bit.
    pub fn configuration_information_enable(self) -> bool {
        self.0.get_bit(9)
    }

    /// Sets the value of the Configuration Information Enable bit.
    pub fn set_configuration_information_enable(&mut self, b: bool) {
        self.0.set_bit(9, b);
    }
}
impl_debug_from_methods! {
    ConfigureRegister {
        max_device_slots_enabled,
        u3_entry_enable,
        configuration_information_enable,
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
