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
}

macro_rules! bit_modifier {
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
}

macro_rules! ro_bit {
    ($bit:literal,$method:ident,$name:literal) => {
        bit_getter!($bit, $method, $name);
    };
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter!([$offset]($bit), $method, $name);
    };
}

macro_rules! wo_bit {
    ($bit:literal,$method:ident,$name:literal) => {
        bit_modifier!($bit, $method, $name);
    };
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_modifier!([$offset]($bit), $method, $name);
    };
}

macro_rules! rw_bit {
    ($bit:literal,$method:ident,$name:literal) => {
        bit_getter!($bit, $method, $name);
        bit_modifier!($bit, $method, $name);
    };
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter!([$offset]($bit), $method, $name);
        bit_modifier!([$offset]($bit), $method, $name);
    };
}

macro_rules! rw1c_bit {
    ($bit:literal,$method:ident,$name:literal) => {
        bit_getter!($bit, $method, $name);
        paste::paste! {
            #[doc = "Clears the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<clear_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0.set_bit($bit,true);
                self
            }
        }
    };
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter!([$offset]($bit), $method, $name);
        paste::paste! {
            #[doc = "Clears the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<clear_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0[$offset].set_bit($bit,true);
                self
            }
        }
    };
}

macro_rules! w1s_bit {
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
}

macro_rules! rw1s_bit {
    ($bit:literal,$method:ident,$name:literal) => {
        bit_getter!($bit, $method, $name);
        w1s_bit!($bit, $method, $name);
    };
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter!([$offset]($bit), $method, $name);
        w1s_bit!([$offset]($bit), $method, $name);
    };
}

macro_rules! field_getter {
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
}

macro_rules! field_setter {
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
}

macro_rules! ro_field {
    ($range:expr,$method:ident,$name:literal,$ty:ty) => {
        field_getter!($range, $method, $name, $ty);
    };
    ([$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        field_getter!([$offset]($range), $method, $name, $ty);
    };
}

macro_rules! rw_field {
    ($range:expr,$method:ident,$name:literal,$ty:ty) => {
        field_getter!($range, $method, $name, $ty);
        field_setter!($range, $method, $name, $ty);
    };
    ([$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        field_getter!([$offset]($range), $method, $name, $ty);
        field_setter!([$offset]($range), $method, $name, $ty);
    };
}
