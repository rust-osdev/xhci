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
    ($vis:vis,[$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        #[doc = "Returns the"]
        #[doc = $name]
        #[doc = "bit."]
        #[must_use]
        $vis fn $method(self) -> bool {
            use bit_field::BitField;
            self.0[$offset].get_bit($bit)
        }
    };
    ($vis:vis,$bit:literal,$method:ident,$name:literal) => {
        #[doc = "Returns the"]
        #[doc = $name]
        #[doc = "bit."]
        #[must_use]
        $vis fn $method(self) -> bool {
            use bit_field::BitField;
            self.0.get_bit($bit)
        }
    };
}

macro_rules! bit_modifier {
    ($vis:vis,[$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Sets the"]
            #[doc = $name]
            #[doc = "bit."]
            $vis fn [<set_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0[$offset].set_bit($bit,true);
                self
            }

            #[doc = "Clears the"]
            #[doc = $name]
            #[doc = "bit."]
            $vis fn [<clear_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0[$offset].set_bit($bit,false);
                self
            }
        }
    };
    ($vis:vis,$bit:literal,$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Sets the"]
            #[doc = $name]
            #[doc = "bit."]
            $vis fn [<set_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0.set_bit($bit,true);
                self
            }

            #[doc = "Clears the"]
            #[doc = $name]
            #[doc = "bit."]
            $vis fn [<clear_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0.set_bit($bit,false);
                self
            }
        }
    };
}

macro_rules! ro_bit {
    ($vis:vis,[$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter!($vis, [$offset]($bit), $method, $name);
    };
    ($vis:vis,$bit:literal,$method:ident,$name:literal) => {
        bit_getter!($vis, $bit, $method, $name);
    };
}

macro_rules! wo_bit {
    ($vis:vis,[$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_modifier!($vis, [$offset]($bit), $method, $name);
    };
    ($vis:vis,$bit:literal,$method:ident,$name:literal) => {
        bit_modifier!($vis, $bit, $method, $name);
    };
}

macro_rules! rw_bit {
    ($vis:vis,[$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter!($vis, [$offset]($bit), $method, $name);
        bit_modifier!($vis, [$offset]($bit), $method, $name);
    };
    ($vis:vis,$bit:literal,$method:ident,$name:literal) => {
        bit_getter!($vis, $bit, $method, $name);
        bit_modifier!($vis, $bit, $method, $name);
    };
}

macro_rules! rw1c_bit {
    ($vis:vis,[$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter!($vis, [$offset]($bit), $method, $name);
        paste::paste! {
            #[doc = "Assigns 1 to the"]
            #[doc = $name]
            #[doc = "bit. On register write, this results in clearing the bit."]
            $vis fn [<clear_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0[$offset].set_bit($bit,true);
                self
            }

            #[doc = "Assigns 0 to the"]
            #[doc = $name]
            #[doc = "bit, preventing the bit from being cleared on write."]
            $vis fn [<set_0_ $method>](&mut self) -> &mut Self {
                use bit_field::BitField;
                self.0[$offset].set_bit($bit,false);
                self
            }
        }
    };
    ($vis:vis,$bit:literal,$method:ident,$name:literal) => {
        bit_getter!($vis, $bit, $method, $name);
        paste::paste! {
            #[doc = "Assigns 1 to the"]
            #[doc = $name]
            #[doc = "bit. On register write, this results in clearing the bit."]
            $vis fn [<clear_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0.set_bit($bit,true);
                self
            }

            #[doc = "Assigns 0 to the"]
            #[doc = $name]
            #[doc = "bit, preventing the bit from being cleared on write."]
            $vis fn [<set_0_ $method>](&mut self) -> &mut Self {
                use bit_field::BitField;
                self.0.set_bit($bit,false);
                self
            }
        }
    };
}

macro_rules! w1s_bit {
    ($vis:vis,[$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Sets the"]
            #[doc = $name]
            #[doc = "bit."]
            $vis fn [<set_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0[$offset].set_bit($bit,true);
                self
            }
        }
    };
    ($vis:vis,$bit:literal,$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Sets the"]
            #[doc = $name]
            #[doc = "bit."]
            $vis fn [<set_ $method>](&mut self)->&mut Self{
                use bit_field::BitField;
                self.0.set_bit($bit,true);
                self
            }
        }
    };
}

macro_rules! rw1s_bit {
    ($vis:vis,[$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter!($vis, [$offset]($bit), $method, $name);
        w1s_bit!($vis, [$offset]($bit), $method, $name);
    };
    ($vis:vis,$bit:literal,$method:ident,$name:literal) => {
        bit_getter!($vis, $bit, $method, $name);
        w1s_bit!($vis, $bit, $method, $name);
    };
}

macro_rules! field_getter {
    ($vis:vis,[$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        #[doc = "Returns the value of the"]
        #[doc = $name]
        #[doc = "field."]
        #[must_use]
        $vis fn $method(self) -> $ty {
            use bit_field::BitField;
            use core::convert::TryInto;
            self.0[$offset].get_bits($range).try_into().unwrap()
        }
    };
    ($vis:vis,$range:expr,$method:ident,$name:literal,$ty:ty) => {
        #[doc = "Returns the value of the"]
        #[doc = $name]
        #[doc = "field."]
        #[must_use]
        $vis fn $method(self) -> $ty {
            use bit_field::BitField;
            use core::convert::TryInto;
            self.0.get_bits($range).try_into().unwrap()
        }
    };
}

macro_rules! field_setter {
    ($vis:vis,[$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        paste::paste! {
            #[doc = "Sets the value of the"]
            #[doc = $name]
            #[doc = "field."]
            $vis fn [<set_ $method>](&mut self,value:$ty) -> &mut Self {
                use bit_field::BitField;
                use core::convert::TryInto;
                self.0[$offset].set_bits($range,value.try_into().unwrap());
                self
            }
        }
    };
    ($vis:vis,$range:expr,$method:ident,$name:literal,$ty:ty) => {
        paste::paste! {
            #[doc = "Sets the value of the"]
            #[doc = $name]
            #[doc = "field."]
            $vis fn [<set_ $method>](&mut self,value:$ty) -> &mut Self {
                use bit_field::BitField;
                use core::convert::TryInto;
                self.0.set_bits($range,value.try_into().unwrap());
                self
            }
        }
    };
}

macro_rules! ro_field {
    ($vis:vis,[$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        field_getter!($vis, [$offset]($range), $method, $name, $ty);
    };
    ($vis:vis,$range:expr,$method:ident,$name:literal,$ty:ty) => {
        field_getter!($vis, $range, $method, $name, $ty);
    };
}

macro_rules! rw_field {
    ($vis:vis,[$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        field_getter!($vis, [$offset]($range), $method, $name, $ty);
        field_setter!($vis, [$offset]($range), $method, $name, $ty);
    };
    ($vis:vis, $range:expr,$method:ident,$name:literal,$ty:ty) => {
        field_getter!($vis, $range, $method, $name, $ty);
        field_setter!($vis, $range, $method, $name, $ty);
    };
}
