//! TRB (Transfer Request Block).

/// The bytes of a TRB.
pub const BYTES: usize = 16;

macro_rules! param_align_16 {
    ($method:ident,$name:literal) => {
        rw_double_zero_trailing!(
            pub, self,
            self.0.0; [0, 1]; 4~; "16-byte aligned",
            $method, $name, 32, u64
        );
    }
}

macro_rules! impl_ring_segment_pointer {
    () => {
        param_align_16!(ring_segment_pointer, "Ring Segment Pointer");
    }
}

macro_rules! impl_tc {
    () => {
        rw_bit!(pub, self, self.0.0[3]; 1, toggle_cycle, "Toggle Cycle");
    }
}

macro_rules! impl_ioc {
    () => {
        rw_bit!(
            pub, self,
            self.0.0[3]; 5,
            interrupt_on_completion,
            "Interrupt On Completion"
        );
    }
}

macro_rules! impl_ep_id {
    () => {
        rw_field!(pub, self, self.0.0[3]; 16..=20, endpoint_id, "Endpoint ID", u8);
    };
    (ro) => {
        ro_field!(pub, self, self.0.0[3]; 16..=20, endpoint_id, "Endpoint ID", u8);
    };
}

macro_rules! impl_vf_id {
    () => {
        rw_field!(pub, self, self.0.0[3]; 16..=23, vf_id, "VF ID", u8);
    };
    (ro) => {
        ro_field!(pub, self, self.0.0[3]; 16..=23, vf_id, "VF ID", u8);
    };
}

macro_rules! impl_slot_id {
    () => {
        rw_field!(pub, self, self.0.0[3]; 24..=31, slot_id, "Slot ID", u8);
    };
    (ro) => {
        ro_field!(pub, self, self.0.0[3]; 24..=31, slot_id, "Slot ID", u8);
    };
}

macro_rules! allowed_trb {
    ($name:literal, {
        $($(#[$docs:meta])* $($deco:literal)? $variant:ident = $val:literal),+ $(,)?
    }) => {
        // defining AllowedType
        paste::paste!(
            #[doc = "Allowed TRB Type for " $name "."]
            #[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
            pub enum AllowedType {
                $(
                    $(#[$docs])*
                    #[doc = ", " $val ""]
                    $variant = $val
                ),+
            }
        );
        
        // defining common block
        paste::paste!(
            #[doc = "A raw " $name " Block."]
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
            pub struct TRB(pub(crate) [u32; 4]);
            impl TRB {
                pub(crate) fn new(ty: AllowedType) -> Self {
                    *Self([0; 4])
                        .set_trb_type(ty)
                }

                pub(crate) fn trb_type(&self) -> Option<AllowedType> {
                    AllowedType::from_u32(self.0[3].get_bits(10..=15))
                }

                fn set_trb_type(&mut self, ty: AllowedType) -> &mut Self {
                    self.0[3].set_bits(10..=15, ty as u32);
                    self
                }

                rw_bit!(pub, self, self.0[3]; 0, cycle_bit, "Cycle");
            }
        );
        
        // defining individual TRB types
        // all TRB types require `Self::new()`. Derive by simple default or manually implement it.
        $(
            paste::paste! {
                #[doc = "A "]
                $(#[$docs])*
                #[doc = "."]
                #[repr(transparent)]
                #[derive(Clone, Copy, PartialEq, Eq, Hash)]
                pub struct $variant(TRB);

                impl From<$variant> for TRB {
                    fn from(v: $variant) -> Self {
                        v.0
                    }
                }

                impl Default for $variant {
                    fn default() -> Self {
                        Self::new()
                    }
                }

                simple_default!($(#[$docs])* $($deco)? $variant); // this branches whether $def is "default" or empty.
            }
        )+
    }
}

macro_rules! simple_default {
    ($(#[$docs:meta])* $variant:ident) => {
        impl $variant{
            #[doc = "Creates a new "]
            $(#[$docs])*
            #[doc = ".\n\nThis method sets the value of the TRB Type field properly. "]
            #[doc = "All the other fields are set to 0."]
            #[must_use]
            pub fn new() -> Self{
                Self(TRB::new(AllowedType::$variant))
            }
        }
    };
    ($(#[$docs:meta])* "no-new" $variant:ident) => {};
}

macro_rules! rsvdz_checking_try_from {
    ($name:ident {
        $([$index:expr];$range:expr),* $(,)?
    }) => {
        impl TryFrom<TRB> for $name {
            type Error = TRB;

            fn try_from(block: TRB) -> Result<Self, Self::Error> {
                if block.trb_type() != Some(AllowedType::$name)
                $(|| block.0[$index].get_bits($range) != 0 )* {
                    return Err(block);
                }
                Ok(Self(block))
            }
        }
    }
}

pub mod transfer;
pub mod event;
pub mod command;