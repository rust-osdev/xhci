//! Host Controller Operational Registers

use super::capability::{Capability, CapabilityRegistersLength};
use accessor::array;
use accessor::single;
use accessor::Mapper;
use bit_field::BitField;
use core::convert::TryFrom;
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
    pub usbcmd: single::ReadWrite<UsbCommandRegister, M>,
    /// USB Status Register
    pub usbsts: single::ReadWrite<UsbStatusRegister, M>,
    /// Page Size Register
    pub pagesize: single::ReadWrite<PageSizeRegister, M>,
    /// Device Notification Control
    pub dnctrl: single::ReadWrite<DeviceNotificationControl, M>,
    /// Command Ring Control Register
    pub crcr: single::ReadWrite<CommandRingControlRegister, M>,
    /// Device Context Base Address Array Pointer Register
    pub dcbaap: single::ReadWrite<DeviceContextBaseAddressArrayPointerRegister, M>,
    /// Configure Register
    pub config: single::ReadWrite<ConfigureRegister, M>,
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
                single::ReadWrite::new(base + $offset, mapper.clone())
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
    ro_bit!(
        15,
        extended_tbc_trb_status_enable,
        "Extended TBC TRB Status Enable"
    );
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
        extended_tbc_trb_status_enable,
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
    /// Returns the `i`th bit of the Notification Enable field. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i >= 16`.
    #[must_use]
    pub fn get(self, i: usize) -> bool {
        Self::ensure_index_is_within_range(i);

        self.0.get_bit(i)
    }

    /// Sets the `i`th bit of the Notification Enable field. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i >= 16`.
    pub fn set(&mut self, i: usize) -> &mut Self {
        Self::ensure_index_is_within_range(i);

        self.0.set_bit(i, true);
        self
    }

    /// Clears the `i`th bit of the Notification Enable field. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i >= 16`.
    pub fn clear(&mut self, i: usize) -> &mut Self {
        Self::ensure_index_is_within_range(i);

        self.0.set_bit(i, false);
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
#[derive(Copy, Clone, Debug, Default)]
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
    rw_field!(
        0..=7,
        max_device_slots_enabled,
        "Max Device Slots Enabled",
        u8
    );
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
    ) -> array::ReadWrite<Self, M1>
    where
        M1: Mapper,
        M2: Mapper + Clone,
    {
        let base = mmio_base + usize::from(capability.caplength.read_volatile().get()) + 0x400;
        array::ReadWrite::new(
            base,
            capability
                .hcsparams1
                .read_volatile()
                .number_of_ports()
                .into(),
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
    rw_field!(5..=8, port_link_state, "Port Link State", u8);
    rw_bit!(9, port_power, "Port Power");
    ro_field!(10..=13, port_speed, "Port Speed", u8);
    rw_field!(
        14..=15,
        port_indicator_control,
        "Port Indicator Control",
        PortIndicator
    );
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
    rw1c_bit!(20, over_current_change, "Over-Current Change");
    rw1c_bit!(21, port_reset_change, "Port Reset Change");
    rw1c_bit!(22, port_link_state_change, "Port Link State Change");
    rw1c_bit!(23, port_config_error_change, "Port Config Error Change");
    ro_bit!(24, cold_attach_status, "Cold Attach Status");
    rw_bit!(25, wake_on_connect_enable, "Wake on Connect Enable");
    rw_bit!(26, wake_on_disconnect_enable, "Wake on Disconnect Enable");
    rw_bit!(
        27,
        wake_on_over_current_enable,
        "Wake on Over-Current Enable"
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
/// **These methods are only valid for USB3.**
impl PortPowerManagementStatusAndControlRegister {
    rw_field!(0..=7, u1_timeout, "U1 Timeout", u8);
    rw_field!(8..=15, u2_timeout, "U2 Timeout", u8);
    rw_bit!(16, force_link_pm_accept, "Force Link PM Accept");
}
/// **These methods are only valid for USB2.**
impl PortPowerManagementStatusAndControlRegister {
    /// Returns the value of the L1 Status field.
    ///
    /// This field returns [`None`] if the value means `Reserved`.
    #[must_use]
    pub fn l1_status(self) -> Option<L1Status> {
        let s = self.0.get_bits(0..=2);
        FromPrimitive::from_u32(s)
    }

    rw_bit!(3, remote_wake_enable, "Remote Wake Enable");
    rw_field!(
        4..=7,
        best_effort_service_latency,
        "Best Effort Service Latency",
        u8
    );
    rw_field!(8..=15, l1_device_slot, "L1 Device Slot", u8);
    rw_bit!(16, hardware_lpm_enable, "Hardware LPM Enable");

    /// Returns the value of the Port Test Control field.
    ///
    /// This field returns [`None`] if the value means `Reserved`.
    #[must_use]
    pub fn port_test_control(self) -> Option<TestMode> {
        let t = self.0.get_bits(28..=31);
        FromPrimitive::from_u32(t)
    }

    /// Sets the value of the Port Test Control field.
    pub fn set_port_test_control(&mut self, m: TestMode) -> &mut Self {
        self.0.set_bits(28..=31, m as _);
        self
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
    rw_field!(0..=15, link_error_count, "Link Error Count", u16);
    ro_field!(16..=19, rx_lane_count, "Rx Lane Count", u8);
    ro_field!(20..=23, tx_lane_count, "Tx Lane Count", u8);
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
/// **This register is only valid for USB2 and is reserved for USB3.**
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PortHardwareLpmControlRegister(u32);
impl PortHardwareLpmControlRegister {
    rw_field!(
        0..=1,
        host_initiated_resume_duration_mode,
        "Host Initiated Resume Duration Mode",
        u8
    );
    rw_field!(2..=9, l1_timeout, "L1 Timeout", u8);
    rw_field!(
        10..=13,
        best_effort_service_latency_deep,
        "Best Effort Service Latency Depp",
        u8
    );
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
impl TryFrom<u32> for PortIndicator {
    type Error = u32;
    fn try_from(x: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(x).ok_or(x)
    }
}
impl From<PortIndicator> for u32 {
    fn from(i: PortIndicator) -> Self {
        i as _
    }
}

/// L1 Status.
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

/// Test Mode.
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
