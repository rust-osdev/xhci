macro_rules! bit_getter_cx {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        #[doc = "Returns the"]
        #[doc = $name]
        #[doc = "bit."]
        #[must_use]
        pub fn $method(self) -> bool {
            use bit_field::BitField;
            self.as_ref()[$offset].get_bit($bit)
        }
    };
    ($bit:literal,$method:ident,$name:literal) => {
        #[doc = "Returns the"]
        #[doc = $name]
        #[doc = "bit."]
        #[must_use]
        pub fn $method(self) -> bool {
            use bit_field::BitField;
            self.as_ref().get_bit($bit)
        }
    };
}

macro_rules! bit_modifier_cx {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Sets the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<set_ $method>](&mut self){
                use bit_field::BitField;
                self.as_mut()[$offset].set_bit($bit,true);
            }

            #[doc = "Clears the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<clear_ $method>](&mut self){
                use bit_field::BitField;
                self.as_mut()[$offset].set_bit($bit,false);
            }
        }
    };
    ($bit:literal,$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Sets the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<set_ $method>](&mut self){
                use bit_field::BitField;
                self.as_mut().set_bit($bit,true);
            }

            #[doc = "Clears the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<clear_ $method>](&mut self){
                use bit_field::BitField;
                self.as_mut().set_bit($bit,false);
            }
        }
    };
}

macro_rules! ro_bit_cx {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter!([$offset]($bit), $method, $name);
    };
    ($bit:literal,$method:ident,$name:literal) => {
        bit_getter!($bit, $method, $name);
    };
}

macro_rules! wo_bit_cx {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_modifier!([$offset]($bit), $method, $name);
    };
    ($bit:literal,$method:ident,$name:literal) => {
        bit_modifier!($bit, $method, $name);
    };
}

macro_rules! rw_bit_cx {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter!([$offset]($bit), $method, $name);
        bit_modifier!([$offset]($bit), $method, $name);
    };
    ($bit:literal,$method:ident,$name:literal) => {
        bit_getter!($bit, $method, $name);
        bit_modifier!($bit, $method, $name);
    };
}

macro_rules! rw1c_bit_cx {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter!([$offset]($bit), $method, $name);
        paste::paste! {
            #[doc = "Clears the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<clear_ $method>](&mut self){
                use bit_field::BitField;
                self.as_mut()[$offset].set_bit($bit,true);
            }
        }
    };
    ($bit:literal,$method:ident,$name:literal) => {
        bit_getter!($bit, $method, $name);
        paste::paste! {
            #[doc = "Clears the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<clear_ $method>](&mut self){
                use bit_field::BitField;
                self.as_mut().set_bit($bit,true);
            }
        }
    };
}

macro_rules! w1s_bit_cx {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Sets the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<set_ $method>](&mut self){
                use bit_field::BitField;
                self.as_mut()[$offset].set_bit($bit,true);
            }
        }
    };
    ($bit:literal,$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Sets the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<set_ $method>](&mut self){
                use bit_field::BitField;
                self.as_mut().set_bit($bit,true);
            }
        }
    };
}

macro_rules! rw1s_bit_cx {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter!([$offset]($bit), $method, $name);
        w1s_bit!([$offset]($bit), $method, $name);
    };
    ($bit:literal,$method:ident,$name:literal) => {
        bit_getter!($bit, $method, $name);
        w1s_bit!($bit, $method, $name);
    };
}

macro_rules! field_getter_cx {
    ([$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        #[doc = "Returns the value of the"]
        #[doc = $name]
        #[doc = "field."]
        #[must_use]
        pub fn $method(self) -> $ty {
            use bit_field::BitField;
            use core::convert::TryInto;
            self.as_ref()[$offset].get_bits($range).try_into().unwrap()
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
            self.as_ref().get_bits($range).try_into().unwrap()
        }
    };
}

macro_rules! field_setter_cx {
    ([$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        paste::paste! {
            #[doc = "Sets the value of the"]
            #[doc = $name]
            #[doc = "field."]
            pub fn [<set_ $method>](&mut self,value:$ty){
                use bit_field::BitField;
                use core::convert::TryInto;
                self.as_mut()[$offset].set_bits($range,value.try_into().unwrap());
            }
        }
    };
    ($range:expr,$method:ident,$name:literal,$ty:ty) => {
        paste::paste! {
            #[doc = "Sets the value of the"]
            #[doc = $name]
            #[doc = "field."]
            pub fn [<set_ $method>](&mut self,value:$ty){
                use bit_field::BitField;
                use core::convert::TryInto;
                self.as_mut().set_bits($range,value.try_into().unwrap());
            }
        }
    };
}

macro_rules! ro_field_cx {
    ([$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        field_getter!([$offset]($range), $method, $name, $ty);
    };
    ($range:expr,$method:ident,$name:literal,$ty:ty) => {
        field_getter!($range, $method, $name, $ty);
    };
}

macro_rules! rw_field_cx {
    ([$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        field_getter!([$offset]($range), $method, $name, $ty);
        field_setter!([$offset]($range), $method, $name, $ty);
    };
    ($range:expr,$method:ident,$name:literal,$ty:ty) => {
        field_getter!($range, $method, $name, $ty);
        field_setter!($range, $method, $name, $ty);
    };
}
