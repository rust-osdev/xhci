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

use bit_field::BitField;
use core::convert::TryInto;
use num_derive::FromPrimitive;
use paste::paste;

macro_rules! cx {
    ($bytes:expr) => {
        paste! {
            #[doc = $bytes "-byte Contexts."]
            /// # Examples
            ///
            /// ``` no_run
            #[doc = "use xhci::context::byte" $bytes "::Input;"]
            /// use xhci::context::InputHandler;
            ///
            /// let mut input = Input::new();
            /// let input_control = input.control_mut();
            ///
            /// input_control.set_aflag(0);
            /// input_control.set_aflag(1);
            ///
            /// # let port_id = 3;
            /// let device = input.device_mut();
            /// let slot = device.slot_mut();
            ///
            /// slot.set_context_entries(1);
            /// slot.set_root_hub_port_number(port_id);
            /// ```
            pub mod [<byte $bytes>]{
                use crate::context::InputControlHandler;
                use crate::context::EndpointHandler;
                use crate::context::SlotHandler;
                use crate::context::InputHandler;
                use crate::context::DeviceHandler;
                use crate::context::EndpointPairHandler;

                const ARRAY_LEN: usize = $bytes / 4;
                const EP_PAIR_NUM:usize=15;


                /// Input Context.
                ///
                /// See the documentation of the [`InputHandler`] for the provided methods.
                ///
                /// # Examples
                ///
                /// ```
                #[doc = "use xhci::context::byte" $bytes "::Input;"]
                /// use xhci::context::InputHandler;
                ///
                /// let mut input = Input::new();
                /// let input_control = input.control_mut();
                ///
                /// input_control.set_aflag(0);
                /// input_control.set_aflag(1);
                ///
                /// # let port_id = 3;
                /// let device = input.device_mut();
                /// let slot = device.slot_mut();
                ///
                /// slot.set_context_entries(1);
                /// slot.set_root_hub_port_number(port_id);
                /// ```
                #[repr(C)]
                #[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
                pub struct Input{
                    control:InputControl,
                    device:Device,
                }
                impl Input{
                    /// Creates a null Input Context.
                    ///
                    /// # Examples
                    ///
                    /// ```
                    #[doc = "use xhci::context::byte" $bytes "::Input;"]
                    ///
                    /// let input = Input::new();
                    /// ```
                    #[must_use]
                    pub const fn new()->Self{
                        Self{
                            control:InputControl::new(),
                            device:Device::new(),
                        }
                    }
                }
                impl InputHandler for Input{
                    fn control_mut(&mut self)->&mut dyn InputControlHandler{
                        &mut self.control
                    }

                    fn device_mut(&mut self)->&mut dyn DeviceHandler{
                        &mut self.device
                    }
                }

                #[repr(transparent)]
                #[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
                struct InputControl([u32;ARRAY_LEN]);
                impl InputControl{
                    #[must_use]
                    const fn new()->Self{
                        Self([0;ARRAY_LEN])
                    }
                }
                impl AsRef<[u32]> for InputControl{
                    fn as_ref(&self)->&[u32]{
                        &self.0
                    }
                }
                impl AsMut<[u32]> for InputControl{
                    fn as_mut(&mut self)->&mut [u32]{
                        &mut self.0
                    }
                }
                impl InputControlHandler for InputControl{}

                /// Device Context.
                ///
                /// See the documentation of the [`DeviceHandler`] for the provided methods.
                ///
                /// # Examples
                ///
                /// ```
                #[doc = "use xhci::context::byte" $bytes "::Device;"]
                /// use xhci::context::DeviceHandler;
                ///
                /// let mut device = Device::new();
                /// let slot = device.slot_mut();
                ///
                /// slot.set_context_entries(1);
                /// ```
                #[repr(C)]
                #[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
                pub struct Device{
                    slot:Slot,
                    endpoint_0:Endpoint,
                    endpoints:[EndpointPair;EP_PAIR_NUM],
                }
                impl Device{
                    /// Creates a null Device Context.
                    ///
                    /// # Examples
                    ///
                    /// ```
                    #[doc = "use xhci::context::byte" $bytes "::Device;"]
                    ///
                    /// let device = Device::new();
                    /// ```
                    #[must_use]
                    pub const fn new()->Self{
                        Self{
                            slot:Slot::new(),
                            endpoint_0:Endpoint::new(),
                            endpoints:[EndpointPair::new();EP_PAIR_NUM],
                        }
                    }
                }
                impl DeviceHandler for Device{
                    fn slot_mut(&mut self)->&mut dyn SlotHandler{
                        &mut self.slot
                    }

                    fn endpoint0_mut(&mut self)->&mut dyn EndpointHandler{
                        &mut self.endpoint_0
                    }

                    fn endpoints_mut(&mut self,i:usize)->&mut dyn EndpointPairHandler{
                        assert_ne!(i,0,"Call `endpoint0_mut` to get a reference to the Endpoint Context 0.");
                        assert!(i<=15,"There exists only 15 endpoint pairs.");

                        &mut self.endpoints[i-1]
                    }
                }

                #[repr(C)]
                #[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
                struct EndpointPair{
                    out:Endpoint,
                    input:Endpoint,
                }
                impl EndpointPair{
                    #[must_use]
                    const fn new()->Self{
                        Self{
                            out:Endpoint::new(),
                            input:Endpoint::new(),
                        }
                    }
                }
                impl EndpointPairHandler for EndpointPair{
                    fn output_mut(&mut self)->&mut dyn EndpointHandler{
                        &mut self.out
                    }

                    fn input_mut(&mut self)->&mut dyn EndpointHandler{
                        &mut self.input
                    }
                }

                #[repr(transparent)]
                #[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
                struct Endpoint([u32; ARRAY_LEN]);
                impl Endpoint {
                    #[must_use]
                    const fn new() -> Self {
                        Self([0; ARRAY_LEN])
                    }

                }
                impl From<[u32; ARRAY_LEN]> for Endpoint {
                    fn from(raw: [u32; ARRAY_LEN]) -> Self {
                        Self(raw)
                    }
                }
                impl AsRef<[u32]> for Endpoint{
                    fn as_ref(&self) ->&[u32]{
                        &self.0
                    }
                }
                impl AsMut<[u32]> for Endpoint{
                    fn as_mut(&mut self)->&mut [u32]{
                        &mut self.0
                    }
                }
                impl EndpointHandler for Endpoint{}


                #[repr(transparent)]
                #[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
                struct Slot([u32;ARRAY_LEN]);
                impl Slot{
                    #[must_use]
                    const fn new()->Self{
                        Self([0;ARRAY_LEN])
                    }
                }
                impl From<[u32;ARRAY_LEN]> for Slot{
                    fn from(raw:[u32;ARRAY_LEN])->Self{
                        Self(raw)
                    }
                }
                impl AsRef<[u32]> for Slot{
                    fn as_ref(&self)->&[u32]{
                        &self.0
                    }
                }
                impl AsMut<[u32]> for Slot{
                    fn as_mut(&mut self)->&mut [u32]{
                        &mut self.0
                    }
                }
                impl SlotHandler for Slot{}
            }
        }
    };
}
cx!(32);
cx!(64);

/// A trait to handle the Input Context.
///
/// # Examples
///
/// ```
/// use xhci::context::{byte32::Input, InputHandler};
///
/// let mut input = Input::new();
/// let control = input.control_mut();
/// let device = input.device_mut();
/// ```
pub trait InputHandler {
    /// Returns a mutable reference to the Input Control Context.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Input, InputHandler};
    ///
    /// let mut input = Input::new();
    /// let control = input.control_mut();
    /// ```
    fn control_mut(&mut self) -> &mut dyn InputControlHandler;

    /// Returns a mutable reference to the Device Context.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Input, InputHandler};
    ///
    /// let mut input = Input::new();
    /// let device = input.device_mut();
    /// ```
    fn device_mut(&mut self) -> &mut dyn DeviceHandler;
}

/// A trait to handle the Device Context.
///
/// # Examples
///
/// ```
/// use xhci::context::{byte32::Device, DeviceHandler};
///
/// let mut device = Device::new();
/// let slot = device.slot_mut();
/// let ep0 = device.endpoint0_mut();
/// let ep1 = device.endpoints_mut(1);
/// ```
pub trait DeviceHandler {
    /// Returns a mutable reference to the Slot Context.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Device, DeviceHandler};
    ///
    /// let mut device = Device::new();
    /// let slot = device.slot_mut();
    /// ```
    fn slot_mut(&mut self) -> &mut dyn SlotHandler;

    /// Returns a mutable reference to the Endpoint Context 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Device, DeviceHandler};
    ///
    /// let mut device = Device::new();
    /// let ep0 = device.endpoint0_mut();
    /// ```
    fn endpoint0_mut(&mut self) -> &mut dyn EndpointHandler;

    /// Returns a mutable reference to the Endpoint Context `i`.
    ///
    /// # Panics
    ///
    /// This method panics if `i == 0` or `i > 15`.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Device, DeviceHandler};
    ///
    /// let mut device = Device::new();
    /// let ep1 = device.endpoints_mut(1);
    /// ```
    ///
    /// Do not call this method with `i == 0`. Call [`DeviceHandler::slot_mut`] instead.
    ///
    /// ```should_panic
    /// use xhci::context::{byte32::Device, DeviceHandler};
    ///
    /// let mut device = Device::new();
    /// let ep0 = device.endpoints_mut(0);
    /// ```
    fn endpoints_mut(&mut self, i: usize) -> &mut dyn EndpointPairHandler;
}

/// A trait to handle a pair of the Endpoint Context.
///
/// # Examples
///
/// ```
/// use xhci::context::{byte32::Device, DeviceHandler};
///
/// let mut device = Device::new();
/// let ep1 = device.endpoints_mut(1);
/// let ep1_in = ep1.input_mut();
/// let ep1_out = ep1.output_mut();
/// ```
pub trait EndpointPairHandler {
    /// Returns a mutable reference to the Output Endpoint Context.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Device, DeviceHandler};
    ///
    /// let mut device = Device::new();
    /// let ep1 = device.endpoints_mut(1);
    /// let ep1_out = ep1.output_mut();
    /// ```
    fn output_mut(&mut self) -> &mut dyn EndpointHandler;

    /// Returns a mutable reference to the Input Endpoint Context.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Device, DeviceHandler};
    ///
    /// let mut device = Device::new();
    /// let ep1 = device.endpoints_mut(1);
    /// let ep1_in = ep1.input_mut();
    /// ```
    fn input_mut(&mut self) -> &mut dyn EndpointHandler;
}

/// A trait to handle the Slot Context.
///
/// # Examples
///
/// ```
/// use xhci::context::{byte32::Device, DeviceHandler};
///
/// let mut device = Device::new();
/// let slot = device.slot_mut();
/// # let port_number = 1;
/// slot.set_context_entries(1);
/// slot.set_root_hub_port_number(port_number);
/// ```
pub trait SlotHandler: AsMut<[u32]> {
    /// Sets the value of the Context Entries field.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Device, DeviceHandler};
    ///
    /// let mut device = Device::new();
    /// let slot = device.slot_mut();
    ///
    /// slot.set_context_entries(1);
    /// ```
    fn set_context_entries(&mut self, e: u8) {
        self.as_mut()[0].set_bits(27..=31, e.into());
    }

    /// Sets the value of the Root Hub Port Number field.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Device, DeviceHandler};
    ///
    /// let mut device = Device::new();
    /// let slot = device.slot_mut();
    /// # let port_number= 1;
    ///
    /// slot.set_root_hub_port_number(port_number);
    /// ```
    fn set_root_hub_port_number(&mut self, n: u8) {
        self.as_mut()[1].set_bits(16..=23, n.into());
    }
}

/// A trait to handle the Input Control Context.
///
/// # Examples
///
/// ```
/// use xhci::context::{byte32::Input, InputHandler};
///
/// let mut input = Input::new();
/// let control = input.control_mut();
///
/// control.set_aflag(0);
/// control.set_aflag(1);
///
/// control.clear_aflag(1);
/// ```
pub trait InputControlHandler: AsMut<[u32]> {
    /// Sets the `i`th Add Context flag.
    ///
    /// # Panics
    ///
    /// This method panics if `i >= 32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Input, InputHandler};
    ///
    /// let mut input = Input::new();
    /// let control = input.control_mut();
    ///
    /// control.set_aflag(0);
    /// control.set_aflag(1);
    /// ```
    fn set_aflag(&mut self, i: usize) {
        assert!(i < 32, "There exists only 0..=31 Add Context flags.");
        self.as_mut()[1].set_bit(i, true);
    }

    /// Clears the `i`th Add Context flag.
    ///
    /// # Panics
    ///
    /// This method panics if `i >= 32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Input, InputHandler};
    ///
    /// let mut input = Input::new();
    /// let control = input.control_mut();
    ///
    /// control.clear_aflag(1);
    /// ```
    fn clear_aflag(&mut self, i: usize) {
        assert!(i < 32, "There exists only 0..=31 Add Context flags.");
        self.as_mut()[1].set_bit(i, false);
    }
}

/// A trait to handle the Endpoint Context.
///
/// # Examples
///
/// ```
/// use xhci::context::{byte32::Device, DeviceHandler};
///
/// let mut device = Device::new();
/// let ep0 = device.endpoint0_mut();
///
/// ep0.set_mult(0);
/// ```
pub trait EndpointHandler: AsMut<[u32]> {
    /// Sets the value of the Mult field.
    ///
    /// # Panics
    ///
    /// This method panics if `m >= 4`.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Device, DeviceHandler};
    ///
    /// let mut device = Device::new();
    /// let ep0 = device.endpoint0_mut();
    ///
    /// ep0.set_mult(0);
    /// ```
    fn set_mult(&mut self, m: u8) {
        assert!(m < 4, "Mult must be less than 4.");

        self.as_mut()[0].set_bits(8..=9, m.into());
    }

    /// Sets the value of the Max Primary Streams field.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Device, DeviceHandler};
    ///
    /// let mut device = Device::new();
    /// let ep0 = device.endpoint0_mut();
    ///
    /// ep0.set_max_primary_streams(0);
    /// ```
    fn set_max_primary_streams(&mut self, s: u8) {
        self.as_mut()[0].set_bits(10..=14, s.into());
    }

    /// Sets the value of the Interval field.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Device, DeviceHandler};
    ///
    /// let mut device = Device::new();
    /// let ep0 = device.endpoint0_mut();
    ///
    /// ep0.set_interval(0);
    /// ```
    fn set_interval(&mut self, i: u8) {
        self.as_mut()[0].set_bits(16..=23, i.into());
    }

    /// Sets the value of the Error Count field.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Device, DeviceHandler};
    ///
    /// let mut device = Device::new();
    /// let ep0 = device.endpoint0_mut();
    ///
    /// ep0.set_error_count(3);
    /// ```
    fn set_error_count(&mut self, c: u8) {
        self.as_mut()[1].set_bits(1..=2, c.into());
    }

    /// Sets the type of the Endpoint.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Device, DeviceHandler, EndpointType};
    ///
    /// let mut device = Device::new();
    /// let ep0 = device.endpoint0_mut();
    ///
    /// ep0.set_endpoint_type(EndpointType::Control);
    /// ```
    fn set_endpoint_type(&mut self, t: EndpointType) {
        self.as_mut()[1].set_bits(3..=5, t as _);
    }

    /// Sets the value of the Max Burst Size field.
    ///
    /// # Panics
    ///
    /// This method panics if `s > 15`.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Device, DeviceHandler, EndpointType};
    ///
    /// let mut device = Device::new();
    /// let ep0 = device.endpoint0_mut();
    ///
    /// ep0.set_max_burst_size(0);
    /// ```
    fn set_max_burst_size(&mut self, s: u8) {
        assert!(
            s <= 15,
            "The valid values of the Max Burst Size field is 0..=15."
        );

        self.as_mut()[1].set_bits(8..=15, s.into());
    }

    /// Sets the value of the Max Packet Size field.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Device, DeviceHandler, EndpointType};
    ///
    /// let mut device = Device::new();
    /// let ep0 = device.endpoint0_mut();
    /// # let max_packet_size = 0;
    ///
    /// ep0.set_max_packet_size(max_packet_size);
    /// ```
    fn set_max_packet_size(&mut self, s: u16) {
        self.as_mut()[1].set_bits(16..=31, s.into());
    }

    /// Sets the value of the Dequeue Cycle State field.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Device, DeviceHandler, EndpointType};
    ///
    /// let mut device = Device::new();
    /// let ep0 = device.endpoint0_mut();
    ///
    /// ep0.set_dequeue_cycle_state(true);
    /// ```
    fn set_dequeue_cycle_state(&mut self, c: bool) {
        self.as_mut()[2].set_bit(0, c);
    }

    /// Sets the value of the Transfer Ring Dequeue pointer field.
    ///
    /// # Panics
    ///
    /// This method panics if `p` is not 16 byte aligned.
    ///
    /// # Examples
    ///
    /// ```
    /// use xhci::context::{byte32::Device, DeviceHandler, EndpointType};
    ///
    /// let mut device = Device::new();
    /// let ep0 = device.endpoint0_mut();
    /// # let ring_addr = 0x1000;
    ///
    /// ep0.set_transfer_ring_dequeue_pointer(ring_addr);
    /// ```
    fn set_transfer_ring_dequeue_pointer(&mut self, p: u64) {
        assert_eq!(
            p % 16,
            0,
            "The Transfer Ring Dequeue Pointer must be 16-byte aligned."
        );

        let l: u32 = (p & 0xffff_ffff).try_into().unwrap();
        let u: u32 = (p >> 32).try_into().unwrap();

        self.as_mut()[2] = l | self.as_mut()[2].get_bit(0) as u32;
        self.as_mut()[3] = u;
    }
}

/// Endpoint Type.
///
/// # Examples
///
/// ```
/// use xhci::context::{byte32::Device, DeviceHandler, EndpointType};
///
/// let mut device = Device::new();
/// let ep0 = device.endpoint0_mut();
///
/// ep0.set_endpoint_type(EndpointType::Control);
/// ```
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
pub enum EndpointType {
    /// Not Valid N/A
    NotValid = 0,
    /// Isoch Out.
    IsochronousOut = 1,
    /// Bulk Out.
    BulkOut = 2,
    /// Interrupt Out.
    InterruptOut = 3,
    /// Control Bidirectional.
    Control = 4,
    /// Isoch In.
    IsochronousIn = 5,
    /// Bulk In.
    BulkIn = 6,
    /// Interrupt In.
    InterruptIn = 7,
}
