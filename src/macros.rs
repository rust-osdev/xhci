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
}

macro_rules! bit_modifier {
    ($bit:literal,$method:ident,$name:literal) => {
        paste::paste! {
            #[doc = "Sets the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<set_ $method>](&mut self){
                use bit_field::BitField;
                self.0.set_bit($bit,true);
            }

            #[doc = "Clears the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<clear_ $method>](&mut self){
                use bit_field::BitField;
                self.0.set_bit($bit,false);
            }
        }
    };
}

macro_rules! ro_bit {
    ($bit:literal,$method:ident,$name:literal) => {
        bit_getter!($bit, $method, $name);
    };
}

macro_rules! wo_bit {
    ($bit:literal,$method:ident,$name:literal) => {
        bit_modifier!($bit, $method, $name);
    };
}

macro_rules! rw_bit {
    ($bit:literal,$method:ident,$name:literal) => {
        bit_getter!($bit, $method, $name);
        bit_modifier!($bit, $method, $name);
    };
}

macro_rules! rw1c_bit {
    ($bit:literal,$method:ident,$name:literal) => {
        bit_getter!($bit, $method, $name);
        paste::paste! {
            #[doc = "Clears the"]
            #[doc = $name]
            #[doc = "bit."]
            pub fn [<clear_ $method>](&mut self){
                use bit_field::BitField;
                self.0.set_bit($bit,true);
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
            pub fn [<set_ $method>](&mut self){
                use bit_field::BitField;
                self.0.set_bit($bit,true);
            }
        }
    };
}

macro_rules! rw1s_bit {
    ($bit:literal,$method:ident,$name:literal) => {
        bit_getter!($bit, $method, $name);
        w1s_bit!($bit, $method, $name);
    };
}
