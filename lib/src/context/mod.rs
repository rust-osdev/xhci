//! The xHCI Contexts.
//!
//! The size of each Context type is the same as the actual Context size.
//!
//! To make it possible to make a reference to Contexts regardless of Context's size, all Contexts
//! implement handler traits that implement methods to access and modify fields. Refer to the
//! documentation of each trait for the available methods.
//!
//! # Examples
//!
//! ```no_run
//! use xhci::{context, context::InputHandler};
//!
//! let mut input = context::Input::new_32byte();
//! let input_control = input.control_mut();
//! input_control.set_add_context_flag(0);
//! input_control.set_add_context_flag(1);
//!
//! # let port_id = 3;
//! let device = input.device_mut();
//! let slot = device.slot_mut();
//! slot.set_context_entries(1);
//! slot.set_root_hub_port_number(port_id);
//! ```

#[macro_use]
mod macros;

use bit_field::BitField;
use core::convert::TryInto;
use core::fmt;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// The number of Endpoint Contexts in a Device Context.
pub const NUM_OF_ENDPOINT_CONTEXTS: usize = 31;

/// 32 byte Input Context.
pub type Input32Byte = Input<8>;
/// 64 byte Input Context.
pub type Input64Byte = Input<16>;

/// 32 byte Input Control Context.
pub type InputControl32Byte = InputControl<8>;
/// 64 byte Input Control Context.
pub type InputControl64Byte = InputControl<16>;

/// 32 byte Device Context.
pub type Device32Byte = Device<8>;
/// 64 byte Device Context.
pub type Device64Byte = Device<16>;

/// 32 byte Slot Context.
pub type Slot32Byte = Slot<8>;
/// 64 byte Slot Context.
pub type Slot64Byte = Slot<16>;

/// 32 byte Endpoint Context.
pub type Endpoint32Byte = Endpoint<8>;
/// 64 byte Endpoint Context.
pub type Endpoint64Byte = Endpoint<16>;

/// Input Context.
///
/// Refer to [`InputHandler`] for the available methods.
#[repr(C)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Input<const N: usize> {
    control: InputControl<N>,
    device: Device<N>,
}
impl_constructor!(Input, "Input");
impl<const N: usize> Input<N> {
    const fn new() -> Self {
        Self {
            control: InputControl::new(),
            device: Device::new(),
        }
    }
}
impl<const N: usize> InputHandler for Input<N> {
    fn control(&self) -> &dyn InputControlHandler {
        &self.control
    }

    fn control_mut(&mut self) -> &mut dyn InputControlHandler {
        &mut self.control
    }

    fn device(&self) -> &dyn DeviceHandler {
        &self.device
    }

    fn device_mut(&mut self) -> &mut dyn DeviceHandler {
        &mut self.device
    }
}

/// A trait to handle Input Context.
pub trait InputHandler {
    /// Returns a handler of Input Control Context.
    fn control(&self) -> &dyn InputControlHandler;

    /// Returns a mutable handler of Input Control Context.
    fn control_mut(&mut self) -> &mut dyn InputControlHandler;

    /// Returns a handler of Device Context.
    fn device(&self) -> &dyn DeviceHandler;

    /// Returns a mutable handler of Device Context.
    fn device_mut(&mut self) -> &mut dyn DeviceHandler;
}

/// Input Control Context.
///
/// Refer to [`InputControlHandler`] for the available methods.
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InputControl<const N: usize>([u32; N]);
impl_constructor!(InputControl, "Input Control");
impl<const N: usize> InputControl<N> {
    const fn new() -> Self {
        Self([0; N])
    }
}
impl<const N: usize> AsRef<[u32]> for InputControl<N> {
    fn as_ref(&self) -> &[u32] {
        &self.0
    }
}
impl<const N: usize> AsMut<[u32]> for InputControl<N> {
    fn as_mut(&mut self) -> &mut [u32] {
        &mut self.0
    }
}
impl<const N: usize> InputControlHandler for InputControl<N> {}
impl<const N: usize> fmt::Debug for InputControl<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InputControl")
            .field("Drop Context flags", &self.0[0])
            .field("Add Context flags", &self.0[1])
            .field("configuration_value", &self.configuration_value())
            .field("interface_number", &self.interface_number())
            .field("alternate_setting", &self.alternate_setting())
            .finish()
    }
}

/// A trait to handle Input Control Context.
pub trait InputControlHandler: AsRef<[u32]> + AsMut<[u32]> {
    /// Returns the `i`th Drop Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i < 2 || i > 31`.
    #[must_use]
    fn drop_context_flag(&self, i: usize) -> bool {
        self.ensure_drop_context_index_within_range(i);

        self.as_ref()[0].get_bit(i)
    }

    /// Sets the `i`th Drop Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i < 2 || i > 31`.
    fn set_drop_context_flag(&mut self, i: usize) {
        self.ensure_drop_context_index_within_range(i);

        self.as_mut()[0].set_bit(i, true);
    }

    /// Clears the `i`th Drop Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i < 2 || i > 31`.
    fn clear_drop_context_flag(&mut self, i: usize) {
        self.ensure_drop_context_index_within_range(i);

        self.as_mut()[0].set_bit(i, false);
    }

    /// Returns the `i`th Add Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i > 31`.
    #[must_use]
    fn add_context_flag(&self, i: usize) -> bool {
        self.ensure_add_context_index_within_range(i);

        self.as_ref()[1].get_bit(i)
    }

    /// Sets the `i`th Add Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i > 31`.
    fn set_add_context_flag(&mut self, i: usize) {
        self.ensure_add_context_index_within_range(i);

        self.as_mut()[1].set_bit(i, true);
    }

    /// Clears the `i`th Add Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i > 31`.
    fn clear_add_context_flag(&mut self, i: usize) {
        self.ensure_add_context_index_within_range(i);

        self.as_mut()[1].set_bit(i, false);
    }

    rw_field_cx!([7](0..=7), configuration_value, "Configuration Value", u8);
    rw_field_cx!([7](8..=15), interface_number, "Interface Number", u8);
    rw_field_cx!([7](16..=23), alternate_setting, "Alternate Setting", u8);

    #[doc(hidden)]
    fn ensure_drop_context_index_within_range(&self, i: usize) {
        assert!(
            (2..=31).contains(&i),
            "The index of Drop Context flag must be within 2..=31."
        );
    }

    #[doc(hidden)]
    fn ensure_add_context_index_within_range(&self, i: usize) {
        assert!(
            i <= 31,
            "The index of Add Context flag must be less than 32."
        );
    }
}

/// Device Context.
///
/// Refer to [`DeviceHandler`] for the available methods.
#[repr(C)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Device<const N: usize> {
    slot: Slot<N>,
    endpoints: [Endpoint<N>; NUM_OF_ENDPOINT_CONTEXTS],
}
impl_constructor!(Device, "Device");
impl<const N: usize> Device<N> {
    const fn new() -> Self {
        Self {
            slot: Slot::new(),
            endpoints: [Endpoint::new(); NUM_OF_ENDPOINT_CONTEXTS],
        }
    }

    fn assert_dci(dci: usize) {
        assert_ne!(
            dci, 0,
            "Call `DeviceHandler::slot` to get a handler of Slot Context.`"
        );
        assert!(dci <= 31, "DCI must be less than 32.");
    }
}
impl<const N: usize> DeviceHandler for Device<N> {
    fn slot(&self) -> &dyn SlotHandler {
        &self.slot
    }

    fn slot_mut(&mut self) -> &mut dyn SlotHandler {
        &mut self.slot
    }

    fn endpoint(&self, dci: usize) -> &dyn EndpointHandler {
        Self::assert_dci(dci);

        &self.endpoints[dci - 1]
    }

    fn endpoint_mut(&mut self, dci: usize) -> &mut dyn EndpointHandler {
        Self::assert_dci(dci);

        &mut self.endpoints[dci - 1]
    }
}

/// A trait to handle Device Context.
pub trait DeviceHandler {
    /// Returns a handler of Slot Context.
    fn slot(&self) -> &dyn SlotHandler;

    /// Returns a mutable handler of Slot Context.
    fn slot_mut(&mut self) -> &mut dyn SlotHandler;

    /// Returns a handler of Endpoint Context.
    ///
    /// # Panics
    ///
    /// This method panics if `dci > 31 || dci == 0`. Call [`DeviceHandler::slot`] if you want a
    /// handler of Slot Context.
    fn endpoint(&self, dci: usize) -> &dyn EndpointHandler;

    /// Returns a mutable handler of Endpoint Context.
    ///
    /// # Panics
    ///
    /// This method panics if `dci > 31 || dci == 0`. Call [`DeviceHandler::slot_mut`] if you want
    /// a mutable handler of Slot Context.
    fn endpoint_mut(&mut self, dci: usize) -> &mut dyn EndpointHandler;
}

/// Slot Context.
///
/// Refer to [`SlotHandler`] for the available methods.
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Slot<const N: usize>([u32; N]);
impl_constructor!(Slot, "Slot");
impl<const N: usize> Slot<N> {
    const fn new() -> Self {
        Self([0; N])
    }
}
impl<const N: usize> AsRef<[u32]> for Slot<N> {
    fn as_ref(&self) -> &[u32] {
        &self.0
    }
}
impl<const N: usize> AsMut<[u32]> for Slot<N> {
    fn as_mut(&mut self) -> &mut [u32] {
        &mut self.0
    }
}
impl<const N: usize> SlotHandler for Slot<N> {}
impl_debug_from_methods_cx! {
    Slot {
        route_string,
        speed,
        multi_tt,
        hub,
        context_entries,
        max_exit_latency,
        root_hub_port_number,
        number_of_ports,
        parent_hub_slot_id,
        parent_port_number,
        tt_think_time,
        interrupter_target,
        usb_device_address,
        slot_state,
    }
}

/// A trait to handle Slot Context.
pub trait SlotHandler: AsRef<[u32]> + AsMut<[u32]> {
    rw_field_cx!([0](0..=19), route_string, "Route String", u32);
    rw_field_cx!([0](20..=23), speed, "Speed", u8);
    rw_bit_cx!([0](25), multi_tt, "Multi-TT");
    rw_bit_cx!([0](26), hub, "Hub");
    rw_field_cx!([0](27..=31), context_entries, "Context Entries", u8);

    rw_field_cx!([1](0..=15), max_exit_latency, "Max Exit Latency", u16);
    rw_field_cx!(
        [1](16..=23),
        root_hub_port_number,
        "Root Hub Port Number",
        u8
    );
    rw_field_cx!([1](24..=31), number_of_ports, "Number of Ports", u8);

    rw_field_cx!([2](0..=7), parent_hub_slot_id, "Parent Hub Slot ID", u8);
    rw_field_cx!([2](8..=15), parent_port_number, "Parent Port Number", u8);
    rw_field_cx!([2](16..=17), tt_think_time, "TT Think Time", u8);
    rw_field_cx!([2](22..=31), interrupter_target, "Interrupter Target", u16);

    rw_field_cx!([3](0..=7), usb_device_address, "USB Device Address", u8);
    /// Returns Slot State.
    ///
    /// # Panics
    ///
    /// This method panics if the Slot State represents Reserved.
    #[must_use]
    fn slot_state(&self) -> SlotState {
        let v = self.as_ref()[3].get_bits(27..=31);
        let s = FromPrimitive::from_u32(v);
        s.expect("Slot State represents Reserved.")
    }

    /// Sets Slot State.
    fn set_slot_state(&mut self, state: SlotState) {
        self.as_mut()[3].set_bits(27..=31, state as _);
    }
}

/// Endpoint Context.
///
/// Refer to [`EndpointHandler`] for the available methods.
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Endpoint<const N: usize>([u32; N]);
impl_constructor!(Endpoint, "Endpoint");
impl<const N: usize> Endpoint<N> {
    const fn new() -> Self {
        Self([0; N])
    }
}
impl<const N: usize> AsRef<[u32]> for Endpoint<N> {
    fn as_ref(&self) -> &[u32] {
        &self.0
    }
}
impl<const N: usize> AsMut<[u32]> for Endpoint<N> {
    fn as_mut(&mut self) -> &mut [u32] {
        &mut self.0
    }
}
impl<const N: usize> EndpointHandler for Endpoint<N> {}
impl_debug_from_methods_cx! {
    Endpoint {
        endpoint_state,
        mult,
        max_primary_streams,
        linear_stream_array,
        interval,
        max_endpoint_service_time_interval_payload_high,
        error_count,
        endpoint_type,
        host_initiate_disable,
        max_burst_size,
        max_packet_size,
        dequeue_cycle_state,
        tr_dequeue_pointer,
        average_trb_length,
        max_endpoint_service_time_interval_payload_low,
    }
}

/// A trait to handle Endpoint Context.
pub trait EndpointHandler: AsRef<[u32]> + AsMut<[u32]> {
    /// Returns Endpoint State.
    ///
    /// # Panics
    ///
    /// This method panics if the Endpoint State represents Reserved.
    #[must_use]
    fn endpoint_state(&self) -> EndpointState {
        let v = self.as_ref()[0].get_bits(0..=2);
        let s = FromPrimitive::from_u32(v);
        s.expect("Endpoint State represents Reserved.")
    }

    /// Sets Endpoint State.
    fn set_endpoint_state(&mut self, s: EndpointState) {
        self.as_mut()[0].set_bits(0..=2, s as _);
    }

    rw_field_cx!([0](8..=9), mult, "Mult", u8);
    rw_field_cx!([0](10..=14), max_primary_streams, "Max Primary Streams", u8);
    rw_bit_cx!([0](15), linear_stream_array, "Linear Stream Array");
    rw_field_cx!([0](16..=23), interval, "Interval", u8);
    rw_field_cx!(
        [0](24..=31),
        max_endpoint_service_time_interval_payload_high,
        "Max Endpoint Service Time Interval Payload High",
        u8
    );

    rw_field_cx!([1](1..=2), error_count, "Error Count", u8);
    /// Returns Endpoint Type.
    #[must_use]
    fn endpoint_type(&self) -> EndpointType {
        let v = self.as_ref()[1].get_bits(3..=5);
        let t = FromPrimitive::from_u32(v);
        t.expect("Invalid Endpoint Type.")
    }

    /// Sets Endpoint Type.
    fn set_endpoint_type(&mut self, t: EndpointType) {
        self.as_mut()[1].set_bits(3..=5, t as _);
    }

    rw_bit_cx!([1](7), host_initiate_disable, "Host Initiate Disable");
    rw_field_cx!([1](8..=15), max_burst_size, "Max Burst Size", u8);
    rw_field_cx!([1](16..=31), max_packet_size, "Max Packet Size", u16);

    rw_bit_cx!([2](0), dequeue_cycle_state, "Dequeue Cycle State");

    /// Returns the TR Dequeue Pointer.
    #[must_use]
    fn tr_dequeue_pointer(&self) -> u64 {
        let l: u64 = self.as_ref()[2].into();
        let u: u64 = self.as_ref()[3].into();

        (u << 32) | l
    }

    /// Sets the TR Dequeue Pointer.
    ///
    /// # Panics
    ///
    /// This method panics if `addr` is not 64-byte aligned.
    fn set_tr_dequeue_pointer(&mut self, a: u64) {
        assert_eq!(a % 64, 0, "TR Dequeue Pointer must be 64-byte aligned.");

        let l: u32 = a.get_bits(0..32).try_into().unwrap();
        let u: u32 = a.get_bits(32..64).try_into().unwrap();

        self.as_mut()[2] = l;
        self.as_mut()[3] = u;
    }

    rw_field_cx!([4](0..=15), average_trb_length, "Average TRB Length", u16);
    rw_field_cx!(
        [4](16..=31),
        max_endpoint_service_time_interval_payload_low,
        "Max Endpoint Service Time Interval Payload Low",
        u16
    );
}

/// Slot State.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
pub enum SlotState {
    /// Disabled/Enabled.
    DisabledEnabled = 0,
    /// Default.
    Default = 1,
    /// Addressed.
    Addressed = 2,
    /// Configured.
    Configured = 3,
}

/// Endpoint State.
///
/// The descriptions of each variant are taken from Table 6-8 of eXtensible Host Controller Interface for Universal Serial Bus(xHCI) Requirements Specification May2019 Revision 1.2.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
pub enum EndpointState {
    /// The endpoint is not operational.
    Disabled = 0,
    /// The endpoint is operational, either waiting for a doorbell ring or processing TDs.
    Running = 1,
    /// The endpoint is halted due to a Halt condition detected on the USB.
    Halted = 2,
    /// The endpoint is not running due to a Stop Endpoint Command or recovering from a Halt
    /// condition.
    Stopped = 3,
    /// The endpoint is not running due to a TRB Erorr.
    Error = 4,
}

/// Endpoint Type.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
pub enum EndpointType {
    /// Not Valid.
    NotValid = 0,
    /// Isoch Out.
    IsochOut = 1,
    /// Bulk Out.
    BulkOut = 2,
    /// Interrupt Out.
    InterruptOut = 3,
    /// Control Bidirectional.
    Control = 4,
    /// Isoch In.
    IsochIn = 5,
    /// Bulk In.
    BulkIn = 6,
    /// Interrupt In.
    InterruptIn = 7,
}
