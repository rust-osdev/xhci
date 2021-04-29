macro_rules! impl_debug_from_methods_cx {
    ($name:ident {
        $($method:ident),*$(,)?
    }) => {
        impl<const N:usize> core::fmt::Debug for $name<N> {
            fn fmt(&self, f:&mut core::fmt::Formatter<'_>) -> core::fmt::Result{
                f.debug_struct(core::stringify!($name))
                    $(.field(core::stringify!($method), &self.$method()))*
                    .finish()
            }
        }
    };
}

macro_rules! bit_getter_cx {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        #[doc = "Returns the"]
        #[doc = $name]
        #[doc = "bit."]
        #[must_use]
        fn $method(&self) -> bool {
            use bit_field::BitField;
            self.as_ref()[$offset].get_bit($bit)
        }
    };
}

macro_rules! bit_modifier_cx {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Sets the"]
            #[doc = $name]
            #[doc = "bit."]
             fn [<set_ $method>](&mut self){
                use bit_field::BitField;
                self.as_mut()[$offset].set_bit($bit,true);
            }

            #[doc = "Clears the"]
            #[doc = $name]
            #[doc = "bit."]
             fn [<clear_ $method>](&mut self){
                use bit_field::BitField;
                self.as_mut()[$offset].set_bit($bit,false);
            }
        }
    };
}

macro_rules! rw_bit_cx {
    ([$offset:literal]($bit:literal),$method:ident,$name:literal) => {
        bit_getter_cx!([$offset]($bit), $method, $name);
        bit_modifier_cx!([$offset]($bit), $method, $name);
    };
}

macro_rules! field_getter_cx {
    ([$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        #[doc = "Returns the value of the"]
        #[doc = $name]
        #[doc = "field."]
        #[must_use]
        fn $method(&self) -> $ty {
            use bit_field::BitField;
            use core::convert::TryInto;
            self.as_ref()[$offset].get_bits($range).try_into().unwrap()
        }
    };
}

macro_rules! field_setter_cx {
    ([$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        paste::paste! {
            #[doc = "Sets the value of the"]
            #[doc = $name]
            #[doc = "field."]
             fn [<set_ $method>](&mut self,value:$ty){
                use bit_field::BitField;
                use core::convert::TryInto;
                self.as_mut()[$offset].set_bits($range,value.try_into().unwrap());
            }
        }
    };
}

macro_rules! rw_field_cx {
    ([$offset:literal]($range:expr),$method:ident,$name:literal,$ty:ty) => {
        field_getter_cx!([$offset]($range), $method, $name, $ty);
        field_setter_cx!([$offset]($range), $method, $name, $ty);
    };
}

macro_rules! impl_constructor_for_bytes {
    ($name:ident,$full:literal,$bytes:literal) => {
        paste::paste! {
            impl [<$name $bytes Byte>]{
                #[doc = "Creates an empty"]
                #[doc = $bytes]
                #[doc = "byte"]
                #[doc = $full]
                #[doc = "Context."]
                #[must_use]
                pub const fn [<new_ $bytes byte>]()->Self{
                    Self::new()
                }
            }
            impl Default for [<$name $bytes Byte>]{
                fn default()->Self{
                    Self::new()
                }
            }
        }
    };
}
macro_rules! impl_constructor {
    ($name:ident,$full:literal) => {
        paste::paste! {
            impl_constructor_for_bytes!($name,$full,"32");
            impl_constructor_for_bytes!($name,$full,"64");
        }
    };
}
