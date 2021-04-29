//! The xHC Contexts.

use core::convert::TryInto;

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
    // TODO: Define `EndpointState` enum.
    rw_field!([0](0..=2), endpoint_state, "Endpoint State", u8);
    rw_field!([0](8..=9), mult, "Mult", u8);
    rw_field!([0](10..=14), max_primary_streams, "Max Primary Streams", u8);
    rw_bit!([0](15), linear_stream_array, "Linear Stream Array");
    rw_field!([0](16..=23), interval, "Interval", u8);
    rw_field!(
        [0](24..=31),
        max_endpoint_service_time_interval_payload_high,
        "Max Endpoint Service Time Interval Payload High",
        u8
    );

    rw_field!([1](1..=2), error_count, "Error Count", u8);
    // TODO: Define `EndpointType` enum.
    rw_field!([1](3..=5), endpoint_type, "Endpoint Type", u8);
    rw_bit!([1](7), host_initiate_disable, "Host Initiate Disable");
    rw_field!([1](8..=15), max_burst_size, "Max Burst Size", u8);
    rw_field!([1](16..=31), max_packet_size, "Max Packet Size", u16);

    rw_bit!([2](0), dequeue_cycle_state, "Dequeue Cycle State");

    /// Returns the TR Dequeue Pointer.
    #[must_use]
    pub fn tr_dequeue_pointer(self) -> u64 {
        let l: u64 = self.0[2].into();
        let u: u64 = self.0[3].into();

        (u << 32) | l
    }

    /// Sets the TR Dequeue Pointer.
    ///
    /// # Panics
    ///
    /// This method panics if `addr` is not 64-byte aligned.
    pub fn set_tr_dequeue_pointer(&mut self, a: u64) -> &mut Self {
        assert_eq!(a % 64, 0, "TR Dequeue Pointer must be 64-byte aligned.");

        let l: u32 = a.get_bits(0..32).try_into().unwrap();
        let u: u32 = a.get_bits(32..64).try_into().unwrap();

        self.0[2] = l;
        self.0[3] = u;
        self
    }

    rw_field!([4](0..=15), average_trb_length, "Average TRB Length", u16);
    rw_field!(
        [4](16..=31),
        max_endpoint_service_time_interval_payload_low,
        "Max Endpoint Service Time Interval Payload Low",
        u16
    );

    const fn new() -> Self {
        Self([0; N])
    }
}
