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
        $vis fn $method(&self) -> bool {
            use bit_field::BitField;
            self.0[$offset].get_bit($bit)
        }
    };
    ($vis:vis,$bit:literal,$method:ident,$name:literal) => {
        #[doc = "Returns the"]
        #[doc = $name]
        #[doc = "bit."]
        #[must_use]
        $vis fn $method(&self) -> bool {
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

macro_rules! bit_modifier_w1c {
    ($vis:vis,[$offset:literal]($bit:literal),$method:ident,$name:literal) => {
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
        bit_modifier_w1c!($vis, [$offset]($bit), $method, $name);
    };
    ($vis:vis,$bit:literal,$method:ident,$name:literal) => {
        bit_getter!($vis, $bit, $method, $name);
        bit_modifier_w1c!($vis, $bit, $method, $name);
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
        $vis fn $method(&self) -> $ty {
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
        $vis fn $method(&self) -> $ty {
            use bit_field::BitField;
            use core::convert::TryInto;
            self.0.get_bits($range).try_into().unwrap()
        }
    };
    ($vis:vis,$range:expr,$name:literal,$ty:ty) => {
        #[doc = "Returns the value of the"]
        #[doc = $name]
        #[doc = "field."]
        #[must_use]
        $vis fn get(&self) -> $ty {
            use bit_field::BitField;
            use core::convert::TryInto;
            self.0.get_bits($range).try_into().unwrap()
        }
    };
    ($vis:vis,[]{$start:literal},$method:ident,$name:literal,$ty:ty) => {
        #[doc = "Returns the value of the"]
        #[doc = $name]
        #[doc = "field."]
        #[must_use]
        $vis fn $method(&self) -> $ty {
            // use bit_field::BitField;
            use core::convert::TryInto;
            (self.0 >> $start << $start).try_into().unwrap()
        }
    };
    ($vis:vis,[]{$start:literal},$name:literal,$ty:ty) => {
        #[doc = "Returns the value of the"]
        #[doc = $name]
        #[doc = "field."]
        #[must_use]
        $vis fn get(&self) -> $ty {
            // use bit_field::BitField;
            use core::convert::TryInto;
            (self.0 >> $start << $start).try_into().unwrap()
        }
    }
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
    ($vis:vis,$range:expr,$name:literal,$ty:ty) => {
        paste::paste! {
            #[doc = "Sets the value of the"]
            #[doc = $name]
            #[doc = "field."]
            $vis fn set(&mut self,value:$ty) -> &mut Self {
                use bit_field::BitField;
                use core::convert::TryInto;
                self.0.set_bits($range,value.try_into().unwrap());
                self
            }
        }
    };
    ($vis:vis,[]{$start:literal,$expect:literal},$method:ident,$name:literal,$ty:ty) => {
        paste::paste! {
            #[doc = "Sets the value of the"]
            #[doc = $name]
            #[doc = "field."]
            #[doc = "\n\n# Panics \n\n"]
            #[doc = "This method panics if the given value is not"]
            #[doc = $expect]
            #[doc = "."]
            $vis fn [<set_ $method>](&mut self,value:$ty) -> &mut Self {
                use bit_field::BitField;

                assert!(value.trailing_zeros() >= $start, "The {} must be {}.", $name, $expect);

                self.0.set_bits($start.., value.get_bits($start..));
                self
            }
        }
    };
    ($vis:vis,[]{$start:literal,$expect:literal},$name:literal,$ty:ty) => {
        paste::paste! {
            #[doc = "Sets the value of the"]
            #[doc = $name]
            #[doc = "field."]
            #[doc = "\n\n# Panics \n\n"]
            #[doc = "This method panics if the given value is not"]
            #[doc = $expect]
            #[doc = "."]
            $vis fn set(&mut self,value:$ty) -> &mut Self {
                use bit_field::BitField;

                assert!(value.trailing_zeros() >= $start, "The {} must be {}.", $name, $expect);

                self.0.set_bits($start.., value.get_bits($start..));
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
    ($vis:vis,$range:expr,$name:literal,$ty:ty) => {
        field_getter!($vis, $range, $name, $ty);
    };
    ($vis:vis,[]{$start:literal},$method:ident,$name:literal,$ty:ty) => {
        field_getter!($vis, []{$start}, $method, $name, $ty);
    };
    ($vis:vis,[]{$start:literal},$name:literal,$ty:ty) => {
        field_getter!($vis, []{$start}, $name, $ty);
    };
}

macro_rules! rw_field {
    ($vis:vis,[$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        field_getter!($vis, [$offset]($range), $method, $name, $ty);
        field_setter!($vis, [$offset]($range), $method, $name, $ty);
    };
    ($vis:vis,$range:expr,$method:ident,$name:literal,$ty:ty) => {
        field_getter!($vis, $range, $method, $name, $ty);
        field_setter!($vis, $range, $method, $name, $ty);
    };
    ($vis:vis,$range:expr,$name:literal,$ty:ty) => {
        field_getter!($vis, $range, $name, $ty);
        field_setter!($vis, $range, $name, $ty);
    };
    ($vis:vis,[]{$start:literal,$expect:literal},$method:ident,$name:literal,$ty:ty) => {
        field_getter!($vis, []{$start}, $method, $name, $ty);
        field_setter!($vis, []{$start, $expect}, $method, $name, $ty);
    };
    ($vis:vis,[]{$start:literal,$expect:literal},$name:literal,$ty:ty) => {
        field_getter!($vis, []{$start}, $name, $ty);
        field_setter!($vis, []{$start, $expect}, $name, $ty);
    };
}

macro_rules! double_field_getter {
    ($vis:vis,[$off_lo:literal,$off_hi:literal],$method:ident,$name:literal,$bits:literal,$ty:ty) => {
        #[doc = "Returns the value of the"]
        #[doc = $name]
        #[doc = "field."]
        #[must_use]
        $vis fn $method(&self) -> $ty {
            let lo = self.0[$off_lo] as $ty;
            let hi = self.0[$off_hi] as $ty;
            
            (hi << $bits) | lo
        }
    };
    ($vis:vis,[$off_lo:literal,$off_hi:literal]{$start:literal},$method:ident,$name:literal,$bits:literal,$ty:ty) => {
        #[doc = "Returns the value of the"]
        #[doc = $name]
        #[doc = "field."]
        #[must_use]
        $vis fn $method(&self) -> $ty {
            let lo = (self.0[$off_lo] as $ty) >> $start << $start;
            let hi = self.0[$off_hi] as $ty;
            
            (hi << $bits) | lo
        }
    };
}

macro_rules! double_field_setter {
    ($vis:vis,[$off_lo:literal,$off_hi:literal],$method:ident,$name:literal,$bits:literal,$ty:ty) => {
        paste::paste! {
            #[doc = "Sets the value of the"]
            #[doc = $name]
            #[doc = "field."]
            $vis fn [<set_ $method>](&mut self, value: $ty) -> &mut Self {
                use bit_field::BitField;
                use core::convert::TryInto;

                let lo = value.get_bits(..$bits);
                let hi = value.get_bits($bits..);

                self.0[$off_lo] = lo.try_into().unwrap();
                self.0[$off_hi] = hi.try_into().unwrap();
                self
            }
        }
    };
    ($vis:vis,[$off_lo:literal,$off_hi:literal]{$start:literal,$expect:literal},$method:ident,$name:literal,$bits:literal,$ty:ty) => {
        paste::paste! {
            #[doc = "Sets the value of the"]
            #[doc = $name]
            #[doc = "field."]
            #[doc = "\n\n# Panics \n\n"]
            #[doc = "This method panics if the given value is not"]
            #[doc = $expect]
            #[doc = "."]
            $vis fn [<set_ $method>](&mut self, value: $ty) -> &mut Self {
                use bit_field::BitField;
                use core::convert::TryInto;

                let lo = value.get_bits(..$bits);
                let hi = value.get_bits($bits..);

                assert!(lo.trailing_zeros() >= $start, "The {} must be {}.", $name, $expect);

                self.0[$off_lo].set_bits(
                    $start..,
                    lo.get_bits($start..).try_into().unwrap()
                );
                self.0[$off_hi] = hi.try_into().unwrap();
                self
            }
        }
    };
}

macro_rules! ro_double_field {
    ($vis:vis,[$off_lo:literal,$off_hi:literal],$method:ident,$name:literal,$bits:literal,$ty:ty) => {
        double_field_getter!($vis, [$off_lo, $off_hi], $method, $name, $bits, $ty);
    };
    ($vis:vis,[$off_lo:literal,$off_hi:literal]{$start:literal},$method:ident,$name:literal,$bits:literal,$ty:ty) => {
        double_field_getter!($vis, [$off_lo, $off_hi]{$start}, $method, $name, $bits, $ty);
    };
}

macro_rules! rw_double_field {
    ($vis:vis,[$off_lo:literal,$off_hi:literal],$method:ident,$name:literal,$bits:literal,$ty:ty) => {
        double_field_getter!($vis, [$off_lo, $off_hi], $method, $name, $bits, $ty);
        double_field_setter!($vis, [$off_lo, $off_hi], $method, $name, $bits, $ty);
    };
    ($vis:vis,[$off_lo:literal,$off_hi:literal]{$start:literal,$expect:literal},$method:ident,$name:literal,$bits:literal,$ty:ty) => {
        double_field_getter!($vis, [$off_lo, $off_hi]{$start}, $method, $name, $bits, $ty);
        double_field_setter!($vis, [$off_lo, $off_hi]{$start, $expect}, $method, $name, $bits, $ty);
    };
}