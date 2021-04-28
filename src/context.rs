//! The xHC Contexts.

use bit_field::BitField;
use core::convert::TryInto;

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
impl Input<8> {
    /// Creates a new 32 bytes Input Context.
    pub fn new_32byte() -> Self {
        Self {
            control: InputControl::new(),
            device: Device::new(),
        }
    }
}
impl Input<16> {
    /// Creates a new 64 bytes Input Context.
    pub fn new_64byte() -> Self {
        Self {
            control: InputControl::new(),
            device: Device::new(),
        }
    }
}

/// Input Control Context.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InputControl<const N: usize>([u32; N]);
impl<const N: usize> InputControl<N> {
    /// Creates an empty Input Control Context.
    pub fn new() -> Self {
        Self([0; N])
    }

    /// Returns the `i`th Drop Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i < 2 || i > 31`.
    pub fn drop_context_flag(self, i: usize) -> bool {
        Self::ensure_drop_context_index_within_range(i);

        self.0[0].get_bit(i)
    }

    /// Sets the `i`th Drop Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i < 2 || i > 31`.
    pub fn set_drop_context_flag(&mut self, i: usize) {
        Self::ensure_drop_context_index_within_range(i);

        self.0[0].set_bit(i, true);
    }

    /// Clears the `i`th Drop Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i < 2 || i > 31`.
    pub fn clear_drop_context_flag(&mut self, i: usize) {
        Self::ensure_drop_context_index_within_range(i);

        self.0[0].set_bit(i, false);
    }

    /// Returns the `i`th Add Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i > 31`.
    pub fn add_context_flag(self, i: usize) -> bool {
        Self::ensure_add_context_index_within_range(i);

        self.0[1].get_bit(i)
    }

    /// Sets the `i`th Add Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i > 31`.
    pub fn set_add_context_flag(&mut self, i: usize) {
        Self::ensure_add_context_index_within_range(i);

        self.0[1].set_bit(i, true);
    }

    /// Clears the `i`th Add Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i > 31`.
    pub fn clear_add_context_flag(&mut self, i: usize) {
        Self::ensure_add_context_index_within_range(i);

        self.0[1].set_bit(i, false);
    }

    /// Returns the value of the Configuration Value field.
    pub fn configuration_value(self) -> u8 {
        self.0[7].get_bits(0..=7).try_into().unwrap()
    }

    /// Sets the value of the Configuration Value field.
    pub fn set_configuration_value(&mut self, value: u8) {
        self.0[7].set_bits(0..=7, value.into());
    }

    /// Returns the value of the Interface Number field.
    pub fn interface_number(self) -> u8 {
        self.0[7].get_bits(8..=15).try_into().unwrap()
    }

    /// Sets the value of the Interface Number field.
    pub fn set_interface_number(&mut self, number: u8) {
        self.0[7].set_bits(8..=15, number.into());
    }

    /// Returns the value of the Alternate Setting field.
    pub fn alternate_setting(self) -> u8 {
        self.0[7].get_bits(16..=23).try_into().unwrap()
    }

    /// Sets the value of the Alternate Setting field.
    pub fn set_alternate_setting(&mut self, setting: u8) {
        self.0[7].set_bits(16..=23, setting.into());
    }

    fn ensure_drop_context_index_within_range(i: usize) {
        assert!(
            i >= 2 && i <= 31,
            "The index of Drop Context flag must be within 2..=31."
        );
    }

    fn ensure_add_context_index_within_range(i: usize) {
        assert!(
            i <= 31,
            "The index of Add Context flag must be less than 32."
        )
    }
}
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
impl<const N: usize> Device<N> {
    /// Creates an empty Device Context.
    pub fn new() -> Self {
        Self {
            slot: Slot::new(),
            endpoints: [Endpoint::new(); NUM_OF_ENDPOINT_CONTEXTS],
        }
    }
}

/// Slot Context.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Slot<const N: usize>([u32; N]);
impl<const N: usize> Slot<N> {
    /// Creates an empty Slot Context.
    pub fn new() -> Self {
        Self([0; N])
    }
}
impl<const N: usize> Default for Slot<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

/// Endpoint Context.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Endpoint<const N: usize>([u32; N]);
impl<const N: usize> Endpoint<N> {
    /// Creates an empty Endpoint Context.
    pub fn new() -> Self {
        Self([0; N])
    }
}
impl<const N: usize> Default for Endpoint<N> {
    fn default() -> Self {
        Self([0; N])
    }
}
