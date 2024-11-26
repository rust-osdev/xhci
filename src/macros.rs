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
    ($vis:vis,$self_:ident,$from:expr;$bit:literal,$method:ident,$name:literal) => {
        #[doc = "Returns the"]
        #[doc = $name]
        #[doc = "bit."]
        #[must_use]
        $vis fn $method(&$self_) -> bool {
            use bit_field::BitField;
            $from.get_bit($bit)
        }
    };
}

macro_rules! bit_raiser {
    ($vis:vis,$self_:ident,$from:expr;$bit:literal,$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Sets the"]
            #[doc = $name]
            #[doc = "bit."]
            $vis fn [<set_ $method>](&mut $self_)->&mut Self{
                use bit_field::BitField;
                $from.set_bit($bit,true);
                $self_
            }
        }
    };
}

macro_rules! bit_clearer {
    ($vis:vis,$self_:ident,$from:expr;$bit:literal,$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Clears the"]
            #[doc = $name]
            #[doc = "bit."]
            $vis fn [<clear_ $method>](&mut $self_)->&mut Self{
                use bit_field::BitField;
                $from.set_bit($bit,false);
                $self_
            }
        }
    };
}

macro_rules! bit_modifier_w1c {
    ($vis:vis,$self_:ident,$from:expr;$bit:literal,$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Assigns 1 to the"]
            #[doc = $name]
            #[doc = "bit. On register write, this results in clearing the bit."]
            $vis fn [<clear_ $method>](&mut $self_)->&mut Self{
                use bit_field::BitField;
                $from.set_bit($bit,true);
                $self_
            }

            #[doc = "Assigns 0 to the"]
            #[doc = $name]
            #[doc = "bit, preventing the bit from being cleared on write."]
            $vis fn [<set_0_ $method>](&mut $self_) -> &mut Self {
                use bit_field::BitField;
                $from.set_bit($bit,false);
                $self_
            }
        }
    };
}

macro_rules! ro_bit {
    ($vis:vis,$self_:ident,$from:expr;$bit:literal,$method:ident,$name:literal) => {
        bit_getter!($vis, $self_, $from;$bit, $method, $name);
    };
}

macro_rules! wo_bit {
    ($vis:vis,$self_:ident,$from:expr;$bit:literal,$method:ident,$name:literal) => {
        bit_raiser!($vis, $self_, $from;$bit, $method, $name);
        bit_clearer!($vis, $self_, $from;$bit, $method, $name);
    };
}

macro_rules! w1s_bit {
    ($vis:vis,$self_:ident,$from:expr;$bit:literal,$method:ident,$name:literal) => {
        bit_raiser!($vis, $self_, $from;$bit, $method, $name);
    };
}

macro_rules! rw_bit {
    ($vis:vis,$self_:ident,$from:expr;$bit:literal,$method:ident,$name:literal) => {
        bit_getter!($vis, $self_, $from;$bit, $method, $name);
        bit_raiser!($vis, $self_, $from;$bit, $method, $name);
        bit_clearer!($vis, $self_, $from;$bit, $method, $name);
    };
}

macro_rules! rw1s_bit {
    ($vis:vis,$self_:ident,$from:expr;$bit:literal,$method:ident,$name:literal) => {
        bit_getter!($vis, $self_, $from;$bit, $method, $name);
        bit_raiser!($vis, $self_, $from;$bit, $method, $name);
    };
}

macro_rules! rw1c_bit {
    ($vis:vis,$self_:ident,$from:expr;$bit:literal,$method:ident,$name:literal) => {
        bit_getter!($vis, $self_, $from;$bit, $method, $name);
        bit_modifier_w1c!($vis, $self_, $from;$bit, $method, $name);
    };
}

macro_rules! field_getter {
    ($vis:vis,$self_:ident,$from:expr;$range:expr,$method:ident,$name:literal,$ty:ty) => {
        #[doc = "Returns the value of the"]
        #[doc = $name]
        #[doc = "field."]
        #[must_use]
        $vis fn $method(&$self_) -> $ty {
            use bit_field::BitField;
            use core::convert::TryInto;
            $from.get_bits($range).try_into().unwrap()
        }
    };
}

macro_rules! zero_trailing_getter {
    ($vis:vis,$self_:ident,$from:expr;$start:literal~,$method:ident,$name:literal,$ty:ty) => {
        #[doc = "Returns the value of the"]
        #[doc = $name]
        #[doc = "field."]
        #[must_use]
        $vis fn $method(&$self_) -> $ty {
            use core::convert::TryInto;
            ($from >> $start << $start).try_into().unwrap()
        }
    };
}

macro_rules! field_setter {
    ($vis:vis,$self_:ident,$from:expr;$range:expr,$method:ident,$name:literal,$ty:ty) => {
        paste::paste! {
            #[doc = "Sets the value of the"]
            #[doc = $name]
            #[doc = "field."]
            $vis fn $method(&mut $self_,value:$ty) -> &mut Self {
                use bit_field::BitField;
                use core::convert::TryInto;
                $from.set_bits($range,value.try_into().unwrap());
                $self_
            }
        }
    };
}

macro_rules! zero_trailing_setter {
    ($vis:vis,$self_:ident,$from:expr;$start:literal~;$expect:literal,$method:ident,$name:literal,$ty:ty) => {
        paste::paste! {
            #[doc = "Sets the value of the"]
            #[doc = $name]
            #[doc = "field."]
            #[doc = "\n\n# Panics \n\n"]
            #[doc = "This method panics if the given value is not"]
            #[doc = $expect]
            #[doc = "."]
            $vis fn $method(&mut $self_,value:$ty) -> &mut Self {
                use bit_field::BitField;

                assert!(value.trailing_zeros() >= $start, "The {} must be {}.", $name, $expect);

                $from.set_bits($start.., value.get_bits($start..));
                $self_
            }
        }
    };
}

macro_rules! ro_field {
    ($vis:vis,$self_:ident,$from:expr;$range:expr,$method:ident,$name:literal,$ty:ty) => {
        field_getter!($vis, $self_, $from;$range, $method, $name, $ty);
    };
    ($vis:vis,$self_:ident,$from:expr,$range:expr,$name:literal,$ty:ty) => {
        field_getter!($vis, $self_, $from;$range, get, $name, $ty);
    };
}

macro_rules! rw_field {
    ($vis:vis,$self_:ident,$from:expr;$range:expr,$method:ident,$name:literal,$ty:ty) => {
        paste::paste!{
            field_getter!($vis, $self_, $from;$range, $method, $name, $ty);
            field_setter!($vis, $self_, $from;$range, [<set_ $method>], $name, $ty);
        }
    };
    ($vis:vis,$self_:ident,$from:expr;$range:expr,$name:literal,$ty:ty) => {
        field_getter!($vis, $self_, $from;$range, get, $name, $ty);
        field_setter!($vis, $self_, $from;$range, set, $name, $ty);
    };
}

macro_rules! rw_zero_trailing {
    ($vis:vis,$self_:ident,$from:expr;$start:literal~;$expect:literal,$method:ident,$name:literal,$ty:ty) => {
        paste::paste!{
            zero_trailing_getter!($vis, $self_, $from;$start~, $method, $name, $ty);
            zero_trailing_setter!($vis, $self_, $from;$start~;$expect, [<set_ $method>], $name, $ty);
        }
    };
    ($vis:vis,$self_:ident,$from:expr;$start:literal~;$expect:literal,$name:literal,$ty:ty) => {
        zero_trailing_getter!($vis, $self_, $from;$start~, get, $name, $ty);
        zero_trailing_setter!($vis, $self_, $from;$start~;$expect, set, $name, $ty);
    };
}

macro_rules! double_field_getter {
    ($vis:vis,$self_:ident,$arr:expr;[$off_lo:literal,$off_hi:literal],$method:ident,$name:literal,$bits:literal,$ty:ty) => {
        #[doc = "Returns the value of the"]
        #[doc = $name]
        #[doc = "field."]
        #[must_use]
        $vis fn $method(&$self_) -> $ty {
            let lo = $arr[$off_lo] as $ty;
            let hi = $arr[$off_hi] as $ty;

            (hi << $bits) | lo
        }
    };
}

macro_rules! double_zero_trailing_getter {
    ($vis:vis,$self_:ident,$arr:expr;[$off_lo:literal,$off_hi:literal];$start:literal~,$method:ident,$name:literal,$bits:literal,$ty:ty) => {
        #[doc = "Returns the value of the"]
        #[doc = $name]
        #[doc = "field."]
        #[must_use]
        $vis fn $method(&$self_) -> $ty {
            let lo = ($arr[$off_lo] as $ty) >> $start << $start;
            let hi = $arr[$off_hi] as $ty;

            (hi << $bits) | lo
        }
    };
}

macro_rules! double_field_setter {
    ($vis:vis,$self_:ident,$arr:expr;[$off_lo:literal,$off_hi:literal],$method:ident,$name:literal,$bits:literal,$ty:ty) => {
        #[doc = "Sets the value of the"]
        #[doc = $name]
        #[doc = "field."]
        $vis fn $method(&mut $self_, value: $ty) -> &mut Self {
            use bit_field::BitField;
            use core::convert::TryInto;

            let lo = value.get_bits(..$bits);
            let hi = value.get_bits($bits..);

            $arr[$off_lo] = lo.try_into().unwrap();
            $arr[$off_hi] = hi.try_into().unwrap();
            $self_
        }
    };
}

macro_rules! double_zero_trailing_setter {
    ($vis:vis,$self_:ident,$arr:expr;[$off_lo:literal,$off_hi:literal];$start:literal~;$expect:literal,$method:ident,$name:literal,$bits:literal,$ty:ty) => {
        #[doc = "Sets the value of the"]
        #[doc = $name]
        #[doc = "field."]
        #[doc = "\n\n# Panics \n\n"]
        #[doc = "This method panics if the given value is not"]
        #[doc = $expect]
        #[doc = "."]
        $vis fn $method(&mut $self_, value: $ty) -> &mut Self {
            use bit_field::BitField;
            use core::convert::TryInto;

            let lo = value.get_bits(..$bits);
            let hi = value.get_bits($bits..);

            assert!(lo.trailing_zeros() >= $start, "The {} must be {}.", $name, $expect);

            $arr[$off_lo].set_bits(
                $start..,
                lo.get_bits($start..).try_into().unwrap()
            );
            $arr[$off_hi] = hi.try_into().unwrap();
            $self_
        }
    };
}

macro_rules! ro_double_field {
    ($vis:vis,$self_:ident,$arr:expr;[$off_lo:literal,$off_hi:literal],$method:ident,$name:literal,$bits:literal,$ty:ty) => {
        double_field_getter!($vis, $self_, $arr; [$off_lo, $off_hi], $method, $name, $bits, $ty);
    };
    ($vis:vis,$self_:ident,$arr:expr;[$off_lo:literal,$off_hi:literal],$name:literal,$bits:literal,$ty:ty) => {
        double_field_getter!($vis, $self_, $arr; [$off_lo, $off_hi], get, $name, $bits, $ty);
    };
}

macro_rules! rw_double_field {
    ($vis:vis,$self_:ident,$arr:expr;[$off_lo:literal,$off_hi:literal],$method:ident,$name:literal,$bits:literal,$ty:ty) => {
        paste::paste! {
            double_field_getter!($vis, $self_, $arr; [$off_lo, $off_hi], $method, $name, $bits, $ty);
            double_field_setter!($vis, $self_, $arr; [$off_lo, $off_hi], [<set_ $method>], $name, $bits, $ty);
        }
    };
    ($vis:vis,$self_:ident,$arr:expr;[$off_lo:literal,$off_hi:literal],$name:literal,$bits:literal,$ty:ty) => {
        double_field_getter!($vis, $self_, $arr; [$off_lo, $off_hi], get, $name, $bits, $ty);
        double_field_setter!($vis, $self_, $arr; [$off_lo, $off_hi], set, $name, $bits, $ty);
    };
}

macro_rules! rw_double_zero_trailing {
    ($vis:vis,$self_:ident,$arr:expr;[$off_lo:literal,$off_hi:literal];$start:literal~;$expect:literal,$method:ident,$name:literal,$bits:literal,$ty:ty) => {
        paste::paste! {
            double_zero_trailing_getter!($vis, $self_, $arr; [$off_lo, $off_hi]; $start~, $method, $name, $bits, $ty);
            double_zero_trailing_setter!($vis, $self_, $arr; [$off_lo, $off_hi]; $start~; $expect, [<set_ $method>], $name, $bits, $ty);
        }
    };
}
