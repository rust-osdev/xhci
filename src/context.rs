//! The xHC Contexts.
//!
//! The xHC supports either 32-byte or 64-byte Contexts. You must check the Context Size bit of the
//! HCCPARAMS1 register. If the bit is 0, use the [`byte32`] module. If the bit is 1, use the [`byte64`]
//! module.
//!
//! # Examples
//!
//! ```no_run
//! use xhci::{context, context::InputHandler};
//!
//! let mut input = context::byte32::Input::new();
//! let input_control = input.control_mut();
//! input_control.set_aflag(0);
//! input_control.set_aflag(1);
//!
//! # let port_id = 3;
//! let device = input.device_mut();
//! let slot = device.slot_mut();
//! slot.set_context_entries(1);
//! slot.set_root_hub_port_number(port_id);
//! ```

/// The number of Endpoint Contexts in a Device Context.
pub const NUM_OF_ENDPOINT_CONTEXTS: usize = 31;

/// Input Context.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Input<const N: usize> {
    /// Input Control Context.
    pub control: InputControl<N>,
    /// Device Context.
    pub device: Device<N>,
}

/// Input Control Context.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InputControl<const N: usize>([u32; N]);
impl<const N: usize> Default for InputControl<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

/// Device Context.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Device<const N: usize> {
    /// Slot Context.
    pub slot: Slot<N>,
    /// Endpoint Contexts.
    pub endpoints: [Endpoint<N>; NUM_OF_ENDPOINT_CONTEXTS],
}

/// Slot Context.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Slot<const N: usize>([u32; N]);
impl<const N: usize> Default for Slot<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

/// Endpoint Context.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Endpoint<const N: usize>([u32; N]);
impl<const N: usize> Default for Endpoint<N> {
    fn default() -> Self {
        Self([0; N])
    }
}
