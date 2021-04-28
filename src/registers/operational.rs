//! Host Controller Operational Registers

use super::capability::{Capability, CapabilityRegistersLength};
use accessor::Mapper;
use bit_field::BitField;
use core::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

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
    rw_bit!(0, run_stop, "Run/Stop");
    rw_bit!(1, host_controller_reset, "Host Controller Reset");
    rw_bit!(2, interrupter_enable, "Interrupter Enable");
    rw_bit!(3, host_system_error_enable, "Host System Error Enable");
    rw_bit!(
        7,
        light_host_controller_reset,
        "Light Host Controller Reset"
    );
    rw_bit!(8, controller_save_state, "Controller Save State");
    rw_bit!(9, controller_restore_state, "Controller Restore State");
    rw_bit!(10, enable_wrap_event, "Enable Wrap Event");
    rw_bit!(11, enable_u3_mfindex_stop, "Enable U3 MFINDEX Stop");
    rw_bit!(13, cem_enable, "CEM Enable");
    ro_bit!(14, extended_tbc_enable, "Extended TBC Enable");
    ro_bit!(15, extended_tbc_status_enable, "Extended TBC Status Enable");
    rw_bit!(16, vtio_enable, "VTIO Enable");
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
    ro_bit!(0, hc_halted, "HC Halted");
    rw1c_bit!(2, host_system_error, "Host System Error");
    rw1c_bit!(3, event_interrupt, "Event Interrupt");
    rw1c_bit!(4, port_change_detect, "Port Change Detect");
    ro_bit!(8, save_state_status, "Save State Status");
    ro_bit!(9, restore_state_status, "Restore State Status");
    rw1c_bit!(10, save_restore_error, "Save/Restore Error");
    ro_bit!(11, controller_not_ready, "Controller Not Ready");
    ro_bit!(12, host_controller_error, "Host Controller Error");
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
    wo_bit!(0, ring_cycle_state, "Ring Cycle State");
    w1s_bit!(1, command_stop, "Command Stop");
    w1s_bit!(2, command_abort, "Command Abort");
    ro_bit!(3, command_ring_running, "Command Ring Running");

    /// Sets the value of the Command Ring Pointer field. It must be 64 byte aligned.
    ///
    /// # Panics
    ///
    /// This method panics if the given pointer is not 64 byte aligned.
    pub fn set_command_ring_pointer(&mut self, p: u64) {
        assert!(
            p.trailing_zeros() >= 6,
            "The Command Ring Pointer must be 64-byte aligned."
        );

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
        assert!(
            p.trailing_zeros() >= 6,
            "The Device Context Base Address Array Pointer must be 64-byte aligned."
        );
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

    rw_bit!(8, u3_entry_enable, "U3 Entry Enable");
    rw_bit!(
        9,
        configuration_information_enable,
        "Configuration Information Enable"
    );
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
    /// Port PM Status and Control Register
    pub portpmsc: PortPowerManagementStatusAndControlRegister,
    /// Port Link Info Register
    pub portli: PortLinkInfoRegister,
    /// Port Hardware LPM Control Register
    pub porthlpmc: PortHardwareLpmControlRegister,
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
    ro_bit!(0, current_connect_status, "Current Connect Status");
    rw1c_bit!(1, port_enabled_disabled, "Port Enabled/Disabled");
    ro_bit!(3, over_current_active, "Over-current Active");
    rw1s_bit!(4, port_reset, "Port Reset");

    /// Returns the value of the Port Link State field.
    #[must_use]
    pub fn port_link_state(self) -> u8 {
        self.0.get_bits(5..=8).try_into().unwrap()
    }

    /// Sets the value of the Port Link State field.
    pub fn set_port_link_state(&mut self, state: u8) {
        self.0.set_bits(5..=8, state.into());
    }

    rw_bit!(9, port_power, "Port Power");

    /// Returns the value of the Port Speed field.
    #[must_use]
    pub fn port_speed(self) -> u8 {
        self.0.get_bits(10..=13).try_into().unwrap()
    }

    /// Returns the value of the Port Indicator Control field.
    #[must_use]
    pub fn port_indicator_control(self) -> PortIndicator {
        let i = FromPrimitive::from_u32(self.0.get_bits(14..=15));
        i.expect("The indicator must be less than 4.")
    }

    /// Sets the value of the Port Indicator Control field.
    pub fn set_port_indicator_control(&mut self, i: PortIndicator) {
        self.0.set_bits(14..=15, i as _);
    }

    rw_bit!(
        16,
        port_link_state_write_strobe,
        "Port Link State Write Strobe"
    );
    rw1c_bit!(17, connect_status_change, "Connect Status Change");
    rw1c_bit!(
        18,
        port_enabled_disabled_change,
        "Port Enabled/Disabled Change"
    );
    rw1c_bit!(19, warm_port_reset_change, "Warm Port Reset Change");
    rw1c_bit!(20, over_current_change, "Over Current Change");
    rw1c_bit!(21, port_reset_change, "Port Reset Change");
    rw1c_bit!(22, port_link_state_change, "Port Link State Change");
    rw1c_bit!(23, port_config_error_change, "Port Config Error Change");
    ro_bit!(24, cold_attach_status, "Cold Attach Status");
    rw_bit!(25, wake_on_connect_enable, "Wake on Connect Enable");
    rw_bit!(26, wake_on_disconnect_enable, "Wake on Disconnect Enable");
    rw_bit!(
        27,
        wake_on_over_current_enable,
        "Wake on Over Current Enable"
    );
    ro_bit!(30, device_removable, "Device Removable");
    rw1s_bit!(31, warm_port_reset, "Warm Port Reset");
}
impl_debug_from_methods! {
    PortStatusAndControlRegister{
        current_connect_status,
        port_enabled_disabled,
        over_current_active,
        port_reset,
        port_link_state,
        port_power,
        port_speed,
        port_indicator_control,
        port_link_state_write_strobe,
        connect_status_change,
        port_enabled_disabled_change,
        warm_port_reset_change,
        over_current_change,
        port_reset_change,
        port_link_state_change,
        port_config_error_change,
        cold_attach_status,
        wake_on_connect_enable,
        wake_on_disconnect_enable,
        wake_on_over_current_enable,
        device_removable,
        warm_port_reset,
    }
}

/// Port Power Management Status and Control Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PortPowerManagementStatusAndControlRegister(u32);
impl PortPowerManagementStatusAndControlRegister {
    /// Returns the value of the U1 Timeout field.
    ///
    /// **This field is USB3 only.**
    #[must_use]
    pub fn u1_timeout(self) -> u8 {
        self.0.get_bits(0..=7).try_into().unwrap()
    }

    /// Sets the value of the U1 Timeout field.
    ///
    /// **This field is USB3 only.**
    pub fn set_u1_timeout(&mut self, timeout: u8) {
        self.0.set_bits(0..=7, timeout.into());
    }

    /// Returns the value of the U2 Timeout field.
    ///
    /// **This field is USB3 only.**
    #[must_use]
    pub fn u2_timeout(self) -> u8 {
        self.0.get_bits(8..=15).try_into().unwrap()
    }

    /// Sets the value of the U2 Timeout field.
    ///
    /// **This field is USB3 only.**
    pub fn set_u2_timeout(&mut self, timeout: u8) {
        self.0.set_bits(8..=15, timeout.into());
    }

    /// Returns the value of the Force Link PM Accept bit.
    ///
    /// **This field is USB3 only.**
    #[must_use]
    pub fn force_link_pm_accept(self) -> bool {
        self.0.get_bit(16)
    }

    /// Sets the value of the Force Link PM Accept bit.
    ///
    /// **This field is USB3 only.**
    pub fn set_force_link_pm_accept(&mut self, b: bool) {
        self.0.set_bit(16, b);
    }

    /// Returns the value of the L1 Status field.
    ///
    /// This field returns [`None`] if the value means `Reserved`.
    ///
    /// **This field is USB2 only.**
    #[must_use]
    pub fn l1_status(self) -> Option<L1Status> {
        let s = self.0.get_bits(0..=2);
        FromPrimitive::from_u32(s)
    }

    /// Returns the value of the Remote Wake Enable field.
    ///
    /// **This field is USB2 only.**
    #[must_use]
    pub fn remote_wake_enable(self) -> bool {
        self.0.get_bit(3)
    }

    /// Sets the value of the Remote Wake Enable field.
    ///
    /// **This field is USB2 only.**
    pub fn set_remote_wake_enable(&mut self, b: bool) {
        self.0.set_bit(3, b);
    }

    /// Returns the value of the Best Effort Service Latency field.
    ///
    /// **This field is USB2 only.**
    #[must_use]
    pub fn best_effort_service_latency(self) -> u8 {
        self.0.get_bits(4..=7).try_into().unwrap()
    }

    /// Sets the value of the Best Effort Service Latency field.
    ///
    /// **This field is USB2 only.**
    pub fn set_best_effort_service_latency(&mut self, l: u8) {
        self.0.set_bits(4..=7, l.into());
    }

    /// Returns the value of the L1 Device Slot field.
    ///
    /// **This field is USB2 only.**
    #[must_use]
    pub fn l1_device_slot(self) -> u8 {
        self.0.get_bits(8..=15).try_into().unwrap()
    }

    /// Sets the value of the L1 Device Slot field.
    ///
    /// **This field is USB2 only.**
    pub fn set_l1_device_slot(&mut self, slot: u8) {
        self.0.set_bits(8..=15, slot.into());
    }

    /// Returns the value of the Hardware LPM Enable field.
    ///
    /// **This field is USB2 only.**
    #[must_use]
    pub fn hardware_lpm_enable(self) -> bool {
        self.0.get_bit(16)
    }

    /// Sets the value of the Hardware LPM Enable field.
    ///
    /// **This field is USB2 only.**
    pub fn set_hardware_lpm_enable(&mut self, b: bool) {
        self.0.set_bit(16, b);
    }

    /// Returns the value of the Port Test Control field.
    ///
    /// This field returns [`None`] if the value means `Reserved`.
    ///
    /// **This field is USB2 only.**
    #[must_use]
    pub fn port_test_control(self) -> Option<TestMode> {
        let t = self.0.get_bits(28..=31);
        FromPrimitive::from_u32(t)
    }

    /// Sets the value of the Port Test Control field.
    ///
    /// **This field is USB2 only.**
    pub fn set_port_test_control(&mut self, m: TestMode) {
        self.0.set_bits(28..=31, m as _);
    }
}
impl_debug_from_methods! {
    PortPowerManagementStatusAndControlRegister{
        u1_timeout,
        u2_timeout,
        force_link_pm_accept,
        l1_status,
        remote_wake_enable,
        best_effort_service_latency,
        l1_device_slot,
        hardware_lpm_enable,
        port_test_control,
    }
}

/// Port Link Info Register.
///
/// **This register is only valid for USB3 and is reserved for USB2.**
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PortLinkInfoRegister(u32);
impl PortLinkInfoRegister {
    /// Returns the value of the Link Error Count field.
    #[must_use]
    pub fn link_error_count(self) -> u16 {
        self.0.get_bits(0..=15).try_into().unwrap()
    }

    /// Sets the value of the Link Error Count field.
    pub fn set_link_error_count(&mut self, c: u16) {
        self.0.set_bits(0..=15, c.into());
    }

    /// Returns the value of the Rx Lane Count field.
    #[must_use]
    pub fn rx_lane_count(self) -> u8 {
        self.0.get_bits(16..=19).try_into().unwrap()
    }

    /// Returns the value of the Tx Lane Count field.
    #[must_use]
    pub fn tx_lane_count(self) -> u8 {
        self.0.get_bits(20..=23).try_into().unwrap()
    }
}
impl_debug_from_methods! {
    PortLinkInfoRegister{
        link_error_count,
        rx_lane_count,
        tx_lane_count,
    }
}

/// Port Hardware LPM Control Register
///
/// **This register is onlyvalid for USB2 and is reserved for USB3.**
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PortHardwareLpmControlRegister(u32);
impl PortHardwareLpmControlRegister {
    /// Returns the value of the Host Initiated Resume Duration Mode field.
    #[must_use]
    pub fn host_initiated_resume_duration_mode(self) -> u8 {
        self.0.get_bits(0..=1).try_into().unwrap()
    }

    /// Sets the value of the Host Initiated Resume Duration Mode field.
    pub fn set_host_initiated_resume_duration_mode(&mut self, mode: u8) {
        self.0.set_bits(0..=1, mode.into());
    }

    /// Returns the value of the L1 Timeout field.
    #[must_use]
    pub fn l1_timeout(self) -> u8 {
        self.0.get_bits(2..=9).try_into().unwrap()
    }

    /// Sets the value of the L1 Timeout field.
    pub fn set_l1_timeout(&mut self, timeout: u8) {
        self.0.set_bits(2..=9, timeout.into());
    }

    /// Returns the value of the Best Effort Service Latency Deep field.
    #[must_use]
    pub fn best_effort_service_latency_deep(self) -> u8 {
        self.0.get_bits(10..=13).try_into().unwrap()
    }

    /// Sets the value of the Best Effort Service Latency Deep field.
    pub fn set_best_effort_service_latency_deep(&mut self, latency: u8) {
        self.0.set_bits(10..=13, latency.into());
    }
}
impl_debug_from_methods! {
    PortHardwareLpmControlRegister {
        host_initiated_resume_duration_mode,
        l1_timeout,
        best_effort_service_latency_deep,
    }
}

/// A type returned by [`PortStatusAndControlRegister::port_indicator_control`].
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, FromPrimitive)]
pub enum PortIndicator {
    /// Port Indicators are off.
    Off = 0,
    /// Amber.
    Amber = 1,
    /// Green.
    Green = 2,
    /// Undefined.
    Undefined = 3,
}

/// A type returned by [`PortPowerManagementStatusAndControlRegister::l1_status`].
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, FromPrimitive)]
pub enum L1Status {
    /// The L1 Status field shall be ignored by software.
    Invalid = 0,
    /// Port successfully transitioned to L1 (ACK).
    Success = 1,
    /// Device is unable to enter L1 at this time (NYET).
    NotYet = 2,
    /// Device does not support L1 transitions (STALL).
    NotSupported = 3,
    /// Device failed to respond to the LPM Transaction or an error occurred.
    TimeOutOrError = 4,
}

/// A type returned by [`PortPowerManagementStatusAndControlRegister::port_test_control`].
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, FromPrimitive)]
pub enum TestMode {
    /// Test mode not enabled.
    NotEnabled = 0,
    /// Test J_STATE.
    JState = 1,
    /// Test K_STATE.
    KState = 2,
    /// Test SE0_NAK.
    Se0Nak = 3,
    /// Test Packet.
    Pakcet = 4,
    /// Test FORCE_ENABLE.
    ForceEnable = 5,
    /// Port Test Control Error.
    PortTestControlError = 15,
}
