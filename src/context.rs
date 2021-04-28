//! The xHC Contexts.

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
