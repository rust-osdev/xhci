//! The xHC Contexts.
//!
//! The xHC supports either 32-byte or 64-byte Contexts. You must check the Context Size bit of the
//! HCCPARAMS register. If the bit is 0, use [`byte32`] module. If the bit is 1, use [`byte64`]
//! module.

use paste::paste;

macro_rules! cx {
    ($bytes:expr) => {
        paste! {
            #[doc = $bytes " byte Contexts."]
            pub mod [<byte $bytes>]{
                use bit_field::BitField;
                use crate::context::EndpointType;
                use core::convert::TryInto;

                const ARRAY_LEN: usize = $bytes / 4;
                const EP_PAIR_NUM:usize=15;

                /// Device Context.
                #[repr(C)]
                #[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
                pub struct Device{
                    /// Slot Context.
                    pub slot:Slot,
                    /// Endpoint Context 0.
                    pub endpoint_0:Endpoint,
                    /// Endpoint Context 1..=15.
                    pub endpoints:[EndpointPair;EP_PAIR_NUM],
                }
                impl Device{
                    /// Creates a null Device Context.
                    pub const fn new()->Self{
                        Self{
                            slot:Slot::new(),
                            endpoint_0:Endpoint::new(),
                            endpoints:[EndpointPair::new();EP_PAIR_NUM],
                        }
                    }
                }

                /// A struct containing the pair of the Endpoint Context.
                #[repr(C)]
                #[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
                pub struct EndpointPair{
                    /// Output.
                    pub out:Endpoint,
                    /// Input.
                    pub input:Endpoint,
                }
                impl EndpointPair{
                    /// Creates a null instance of the Endpoint Pair.
                    pub const fn new()->Self{
                        Self{
                            out:Endpoint::new(),
                            input:Endpoint::new(),
                        }
                    }
                }

                /// Endpoint Context.
                #[repr(transparent)]
                #[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
                pub struct Endpoint([u32; ARRAY_LEN]);
                impl Endpoint {
                    /// Creates a null Endpoint Context.
                    #[must_use]
                    pub const fn new() -> Self {
                        Self([0; ARRAY_LEN])
                    }

                    /// Sets the value of the Mult field.
                    ///
                    /// # Panics
                    ///
                    /// This method panics if `m >= 4`.
                    pub fn set_mult(&mut self, m: u8) -> &mut Self {
                        assert!(m < 4, "Mult must be less than 4.");

                        self.0[0].set_bits(8..=9, m.into());
                        self
                    }

                    /// Sets the value of the Max Primary Streams field.
                    pub fn set_max_primary_streams(&mut self, s: u8) -> &mut Self {
                        self.0[0].set_bits(10..=14, s.into());
                        self
                    }

                    /// Sets the value of the Interval field.
                    pub fn set_interval(&mut self, i: u8) -> &mut Self {
                        self.0[0].set_bits(16..=23, i.into());
                        self
                    }

                    /// Sets the value of the Error Count field.
                    pub fn set_error_count(&mut self, c: u8) -> &mut Self {
                        self.0[1].set_bits(1..=2, c.into());
                        self
                    }

                    /// Sets the type of the Endpoint.
                    pub fn set_endpoint_type(&mut self, t: EndpointType) -> &mut Self {
                        self.0[1].set_bits(3..=5, t as _);
                        self
                    }

                    /// Sets the value of the Max Burst Size field.
                    ///
                    /// # Panics
                    ///
                    /// This method panics if `s > 15`.
                    pub fn set_max_burst_size(&mut self, s: u8) -> &mut Self {
                        assert!(
                            s <= 15,
                            "The valid values of the Max Burst Size field is 0..=15."
                        );

                        self.0[1].set_bits(8..=15, s.into());
                        self
                    }

                    /// Sets the value of the Max Packet Size field.
                    pub fn set_max_packet_size(&mut self, s: u16) -> &mut Self {
                        self.0[1].set_bits(16..=31, s.into());
                        self
                    }

                    /// Sets the value of the Dequeue Cycle State field.
                    pub fn set_dequeue_cycle_state(&mut self, c: bool) -> &mut Self {
                        self.0[2].set_bit(0, c.into());
                        self
                    }

                    /// Sets the value of the Transfer Ring Dequeue pointer field.
                    ///
                    /// # Panics
                    ///
                    /// This method panics if `p` is not 16 byte aligned.
                    pub fn set_transfer_ring_dequeue_pointer(&mut self, p: u64) -> &mut Self {
                        assert_eq!(p % 16, 0);

                        let l: u32 = (p & 0xffff_ffff).try_into().unwrap();
                        let u: u32 = (p >> 32).try_into().unwrap();

                        self.0[2] = l | self.0[2].get_bit(0) as u32;
                        self.0[3] = u;
                        self
                    }
                }
                impl From<[u32; ARRAY_LEN]> for Endpoint {
                    fn from(raw: [u32; ARRAY_LEN]) -> Self {
                        Self(raw)
                    }
                }

                #[repr(transparent)]
                #[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
                /// Slot Context.
                pub struct Slot([u32;ARRAY_LEN]);
                impl Slot{
                    /// Creates a null Slot Context.
                    #[must_use]
                    pub const fn new()->Self{
                        Self([0;ARRAY_LEN])
                    }

                    /// Sets the value of the Root Hub Port Number field.
                    pub fn set_root_hub_port_number(&mut self,n:u8)->&mut Self{
                        self.0[0].set_bits(16..=23,n.into());
                        self
                    }
                }
                impl From<[u32;ARRAY_LEN]> for Slot{
                    fn from(raw:[u32;ARRAY_LEN])->Self{
                        Self(raw)
                    }
                }
            }
        }
    };
}

cx!(32);
cx!(64);

/// Endpoint Type.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
