//! TRB (Transfer Request Block).

use bit_field::BitField;
use core::convert::TryInto;
use num_derive::FromPrimitive;

macro_rules! add_trb {
    ($name:ident,$full:expr,$ty:expr) => {
        #[doc = $full ]
        #[repr(transparent)]
        #[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
        pub struct $name([u32; 4]);
        impl $name {
            /// Sets the value of the Cycle Bit.
            pub fn set_cycle_bit(&mut self, b: bool) -> &mut Self {
                use bit_field::BitField;
                self.0[3].set_bit(0, b);
                self
            }

            fn set_trb_type(&mut self) -> &mut Self {
                use crate::ring::trb::Type;
                use bit_field::BitField;
                self.0[3].set_bits(10..=15, $ty as _);
                self
            }
        }
        impl AsRef<[u32]> for $name {
            fn as_ref(&self) -> &[u32] {
                &self.0
            }
        }
        impl AsMut<[u32]> for $name {
            fn as_mut(&mut self) -> &mut [u32] {
                &mut self.0
            }
        }
        impl From<[u32; 4]> for $name {
            fn from(raw: [u32; 4]) -> Self {
                Self(raw)
            }
        }
    };
}
macro_rules! impl_default_simply_adds_trb_id {
    ($name:ident,$full:expr) => {
        impl $name{
            paste::paste! {
                #[doc = "Creates a new " $full ".\n\nThis method sets the sets the value of the TRB Type field properly. All the other fieldds are set to 0."]
                #[must_use]
                pub fn new()->Self{
                    *Self([0;4]).set_trb_type()
                }
            }
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
macro_rules! add_trb_with_default {
    ($name:ident,$full:expr,$type:expr) => {
        add_trb!($name, $full, $type);
        impl_default_simply_adds_trb_id!($name, $full);
    };
}

pub mod command;
pub mod event;
pub mod transfer;

add_trb_with_default!(Link, "Link TRB", Type::Link);
impl Link {
    /// Sets the value of the Ring Segment Pointer field.
    ///
    /// # Panics
    ///
    /// This method panics if `p` is not 16-byte aligned.
    pub fn set_ring_segment_pointer(&mut self, p: u64) -> &mut Self {
        assert_eq!(
            p % 16,
            0,
            "The Ring Segment Pointer must be 16-byte aligned."
        );

        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
        self
    }
}

/// TRB Type.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
#[non_exhaustive]
pub enum Type {
    /// Normal TRB, 1
    Normal = 1,
    /// Link TRB, 6
    Link = 6,
    /// Enable Slot Command TRB, 9
    EnableSlot = 9,
    /// Address Device Command TRB, 11
    AddressDevice = 11,
    /// Configure Endpoint Command TRB, 12
    ConfigureEndpoint = 12,
    /// No Op Command TRB, 23
    NoopCommand = 23,
    /// Transfer Event TRB, 32
    TransferEvent = 32,
    /// Command Completion Event TRB, 33
    CommandCompletion = 33,
    /// Port Status Change Event TRB, 34
    PortStatusChange = 34,
}
