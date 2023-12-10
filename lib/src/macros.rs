macro_rules! impl_debug_from_methods {
    ($name:ident {
        $($method:ident),*$(,)?
    }) => {
        impl core::fmt::Debug for $name {
            fn fmt(&self, f:&mut core::fmt::Formatter<'_>) -> core::fmt::Result{
                f.debug_struct(core::stringify!($name))
                    $(.field(core::stringify!($method), &self.$method()))*
                    .finish()
            }
        }
    };
}

macro_rules! bit_getter {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        #[doc = "Returns the"]
        #[doc = $name]
        #[doc = "bit."]
        #[must_use]
        pub fn $method(self) -> bool {
            use bit_field::BitField;
            self.0[$offset].get_bit($bit)
        }
    };
    ($bit:literal,$method:ident,$name:literal) => {
        #[doc = "Returns the"]
        #[doc = $name]
        #[doc = "bit."]
        #[must_use]
        pub fn $method(self) -> bool {
            use bit_field::BitField;
            self.0.get_bit($bit)
        }
    };
}

macro_rules! bit_modifier {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Sets the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<set_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0[$offset].set_bit($bit,true);
                self
            }

            #[doc = "Clears the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<clear_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0[$offset].set_bit($bit,false);
                self
            }
        }
    };
    ($bit:literal,$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Sets the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<set_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0.set_bit($bit,true);
                self
            }

            #[doc = "Clears the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<clear_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0.set_bit($bit,false);
                self
            }
        }
    };
}

macro_rules! ro_bit {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter!([$offset]($bit), $method, $name);
    };
    ($bit:literal,$method:ident,$name:literal) => {
        bit_getter!($bit, $method, $name);
    };
}

macro_rules! wo_bit {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_modifier!([$offset]($bit), $method, $name);
    };
    ($bit:literal,$method:ident,$name:literal) => {
        bit_modifier!($bit, $method, $name);
    };
}

macro_rules! rw_bit {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter!([$offset]($bit), $method, $name);
        bit_modifier!([$offset]($bit), $method, $name);
    };
    ($bit:literal,$method:ident,$name:literal) => {
        bit_getter!($bit, $method, $name);
        bit_modifier!($bit, $method, $name);
    };
}

macro_rules! rw1c_bit {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter!([$offset]($bit), $method, $name);
        paste::paste! {
            #[doc = "Assigns 1 to the"]
            #[doc = $name]
            #[doc = "bit. On register write, this results in clearing the bit."]
            pub fn [<clear_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0[$offset].set_bit($bit,true);
                self
            }

            #[doc = "Assigns 0 to the"]
            #[doc = $name]
            #[doc = "bit, preventing the bit from being cleared on write."]
            pub fn [<set_0_ $method>](&mut self) -> &mut Self {
                use bit_field::BitField;
                self.0[$offset].set_bit($bit,false);
                self
            }
        }
    };
    ($bit:literal,$method:ident,$name:literal) => {
        bit_getter!($bit, $method, $name);
        paste::paste! {
            #[doc = "Assigns 1 to the"]
            #[doc = $name]
            #[doc = "bit. On register write, this results in clearing the bit."]
            pub fn [<clear_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0.set_bit($bit,true);
                self
            }

            #[doc = "Assigns 0 to the"]
            #[doc = $name]
            #[doc = "bit, preventing the bit from being cleared on write."]
            pub fn [<set_0_ $method>](&mut self) -> &mut Self {
                use bit_field::BitField;
                self.0.set_bit($bit,false);
                self
            }
        }
    };
}

macro_rules! w1s_bit {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Sets the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<set_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0[$offset].set_bit($bit,true);
                self
            }
        }
    };
    ($bit:literal,$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Sets the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<set_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0.set_bit($bit,true);
                self
            }
        }
    };
}

macro_rules! rw1s_bit {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter!([$offset]($bit), $method, $name);
        w1s_bit!([$offset]($bit), $method, $name);
    };
    ($bit:literal,$method:ident,$name:literal) => {
        bit_getter!($bit, $method, $name);
        w1s_bit!($bit, $method, $name);
    };
}

macro_rules! field_getter {
    ([$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        #[doc = "Returns the value of the"]
        #[doc = $name]
        #[doc = "field."]
        #[must_use]
        pub fn $method(self) -> $ty {
            use bit_field::BitField;
            use core::convert::TryInto;
            self.0[$offset].get_bits($range).try_into().unwrap()
        }
    };
    ($range:expr,$method:ident,$name:literal,$ty:ty) => {
        #[doc = "Returns the value of the"]
        #[doc = $name]
        #[doc = "field."]
        #[must_use]
        pub fn $method(self) -> $ty {
            use bit_field::BitField;
            use core::convert::TryInto;
            self.0.get_bits($range).try_into().unwrap()
        }
    };
}

macro_rules! field_setter {
    ([$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        paste::paste! {
            #[doc = "Sets the value of the"]
            #[doc = $name]
            #[doc = "field."]
            pub fn [<set_ $method>](&mut self,value:$ty) -> &mut Self {
                use bit_field::BitField;
                use core::convert::TryInto;
                self.0[$offset].set_bits($range,value.try_into().unwrap());
                self
            }
        }
    };
    ($range:expr,$method:ident,$name:literal,$ty:ty) => {
        paste::paste! {
            #[doc = "Sets the value of the"]
            #[doc = $name]
            #[doc = "field."]
            pub fn [<set_ $method>](&mut self,value:$ty) -> &mut Self {
                use bit_field::BitField;
                use core::convert::TryInto;
                self.0.set_bits($range,value.try_into().unwrap());
                self
            }
        }
    };
}

macro_rules! ro_field {
    ([$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        field_getter!([$offset]($range), $method, $name, $ty);
    };
    ($range:expr,$method:ident,$name:literal,$ty:ty) => {
        field_getter!($range, $method, $name, $ty);
    };
}

macro_rules! rw_field {
    ([$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        field_getter!([$offset]($range), $method, $name, $ty);
        field_setter!([$offset]($range), $method, $name, $ty);
    };
    ($range:expr,$method:ident,$name:literal,$ty:ty) => {
        field_getter!($range, $method, $name, $ty);
        field_setter!($range, $method, $name, $ty);
    };
}
