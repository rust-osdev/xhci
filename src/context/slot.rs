//! Slot Context

use bit_field::BitField;
use paste::paste;

macro_rules! cx {
    ($bytes:expr,$len:expr) => {
        paste! {
            #[doc = $bytes " byte version of the Slot Context."]
            #[repr(transparent)]
            #[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
            pub struct [<Byte $bytes>]([u32;$len]);
            impl [<Byte $bytes>]{
                /// Creates a null Slot Context.
                #[must_use]
                pub const fn new()->Self{
                    Self([0;$len])
                }

                /// Converts the Slot Context into an array.
                #[must_use]
                pub fn [<into_ $bytes byte>](self)->[u32;$len]{
                    self.0
                }

                /// Sets the value of the Root Hub Port Number field.
                pub fn set_root_hub_port_number(&mut self,n:u8)->&mut Self{
                    self.0[0].set_bits(16..=23,n.into());
                    self
                }
            }
            impl From<[u32;$len]> for [<Byte $bytes>]{
                fn from(raw:[u32;$len])->Self{
                    Self(raw)
                }
            }
            impl AsRef<[u32]> for [<Byte $bytes>]{
                fn as_ref(&self)->&[u32]{
                    &self.0
                }
            }
            impl AsMut<[u32]> for [<Byte $bytes>]{
                fn as_mut(&mut self)->&mut [u32]{
                    &mut self.0
                }
            }
        }
    };
}

cx!(32, 8);
cx!(64, 16);
