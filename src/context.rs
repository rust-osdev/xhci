//! The xHC Contexts.

use bit_field::BitField;

macro_rules! impl_constructor {
    ($ty:ident,$name:expr) => {
        impl $ty<8> {
            #[doc = "Creates an empty 32 byte"]
            #[doc = $name]
            #[doc = "Context."]
            #[must_use]
            pub const fn new_32byte() -> Self {
                Self::new()
            }
        }
        impl $ty<16> {
            #[doc = "Creates an empty 64 byte"]
            #[doc = $name]
            #[doc = "Context."]
            #[must_use]
            pub const fn new_64byte() -> Self {
                Self::new()
            }
        }
        impl Default for $ty<8> {
            fn default() -> Self {
                Self::new()
            }
        }
        impl Default for $ty<16> {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

/// The number of Endpoint Contexts in a Device Context.
pub const NUM_OF_ENDPOINT_CONTEXTS: usize = 31;

/// Input Context.
#[repr(C)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Input<const N: usize> {
    /// Input Control Context.
    pub control: InputControl<N>,
    /// Device Context.
    pub device: Device<N>,
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

/// Input Control Context.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InputControl<const N: usize>([u32; N]);
impl_constructor!(InputControl, "Input Control");
impl<const N: usize> InputControl<N> {
    /// Returns the `i`th Drop Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i < 2 || i > 31`.
    #[must_use]
    pub fn drop_context_flag(self, i: usize) -> bool {
        Self::ensure_drop_context_index_within_range(i);

        self.0[0].get_bit(i)
    }

    /// Sets the `i`th Drop Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i < 2 || i > 31`.
    pub fn set_drop_context_flag(&mut self, i: usize) -> &mut Self {
        Self::ensure_drop_context_index_within_range(i);

        self.0[0].set_bit(i, true);
        self
    }

    /// Clears the `i`th Drop Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i < 2 || i > 31`.
    pub fn clear_drop_context_flag(&mut self, i: usize) -> &mut Self {
        Self::ensure_drop_context_index_within_range(i);

        self.0[0].set_bit(i, false);
        self
    }

    /// Returns the `i`th Add Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i > 31`.
    #[must_use]
    pub fn add_context_flag(self, i: usize) -> bool {
        Self::ensure_add_context_index_within_range(i);

        self.0[1].get_bit(i)
    }

    /// Sets the `i`th Add Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i > 31`.
    pub fn set_add_context_flag(&mut self, i: usize) -> &mut Self {
        Self::ensure_add_context_index_within_range(i);

        self.0[1].set_bit(i, true);
        self
    }

    /// Clears the `i`th Add Context flag. `i` starts from 0.
    ///
    /// # Panics
    ///
    /// This method panics if `i > 31`.
    pub fn clear_add_context_flag(&mut self, i: usize) -> &mut Self {
        Self::ensure_add_context_index_within_range(i);

        self.0[1].set_bit(i, false);
        self
    }

    rw_field!([7](0..=7), configuration_value, "Configuration Value", u8);
    rw_field!([7](8..=15), interface_number, "Interface Number", u8);
    rw_field!([7](16..=23), alternate_setting, "Alternate Setting", u8);

    fn ensure_drop_context_index_within_range(i: usize) {
        assert!(
            (2..=31).contains(&i),
            "The index of Drop Context flag must be within 2..=31."
        );
    }

    fn ensure_add_context_index_within_range(i: usize) {
        assert!(
            i <= 31,
            "The index of Add Context flag must be less than 32."
        )
    }

    const fn new() -> Self {
        Self([0; N])
    }
}

/// Device Context.
#[repr(C)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Device<const N: usize> {
    /// Slot Context.
    pub slot: Slot<N>,
    /// Endpoint Contexts.
    pub endpoints: [Endpoint<N>; NUM_OF_ENDPOINT_CONTEXTS],
}
impl_constructor!(Device, "Device");
impl<const N: usize> Device<N> {
    const fn new() -> Self {
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
impl_constructor!(Slot, "Slot");
impl<const N: usize> Slot<N> {
    rw_field!([0](0..=19), route_string, "Route String", u32);
    rw_field!([0](20..=23), speed, "Speed", u8);
    rw_bit!([0](25), multi_tt, "Multi-TT");
    rw_bit!([0](26), hub, "Hub");
    rw_field!([0](27..=31), context_entries, "Context Entries", u8);

    rw_field!([1](0..=15), max_exit_latency, "Max Exit Latency", u16);
    rw_field!(
        [1](16..=23),
        root_hub_port_number,
        "Root Hub Port Number",
        u8
    );
    rw_field!([1](24..=31), number_of_ports, "Number of Ports", u8);

    rw_field!([2](0..=7), parent_hub_slot_id, "Parent Hub Slot ID", u8);
    rw_field!([2](8..=15), parent_port_number, "Parent Port Number", u8);
    rw_field!([2](16..=17), tt_think_time, "TT Think Time", u8);
    rw_field!([2](22..=31), interrupter_target, "Interrupter Target", u16);

    rw_field!([3](0..=7), usb_device_address, "USB Device Address", u8);
    // TODO: Define `SlotState` enum.
    rw_field!([3](27..=31), slot_state, "Slot State", u8);

    const fn new() -> Self {
        Self([0; N])
    }
}

/// Endpoint Context.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Endpoint<const N: usize>([u32; N]);
impl_constructor!(Endpoint, "Endpoint");
impl<const N: usize> Endpoint<N> {
    const fn new() -> Self {
        Self([0; N])
    }
}
