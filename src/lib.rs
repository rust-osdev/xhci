//! A library to handle xHCI.
//!
//! This crate provides types of the xHCI structures, such as the Registers and Contexts.
//! Users can use this library to implement a USB device deriver on your own OS.
//!
//! This crate is `#![no_std]` compatible.
//!
//! # Examples
//!
//! ```no_run
//! # use core::num::NonZeroUsize;
//! # use xhci::accessor::Mapper;
//! #
//! # const MMIO_BASE: usize = 0x1000;
//! #
//! # #[derive(Clone)]
//! # struct MemoryMapper;
//! # impl Mapper for MemoryMapper {
//! #     unsafe fn map(&mut self, phys_base: usize, bytes: usize) -> NonZeroUsize {
//! #         unimplemented!()
//! #     }
//! #
//! #     fn unmap(&mut self, virt_base: usize, bytes: usize) {
//! #         unimplemented!()
//! #     }
//! # }
//! #
//! # let mapper = MemoryMapper;
//! #
//! let mut r = unsafe { xhci::Registers::new(MMIO_BASE, mapper) };
//! let o = &mut r.operational;
//!
//! o.usbcmd.update(|u| u.set_run_stop(true));
//! while o.usbsts.read().hc_halted() {}
//! ```

#![no_std]
#![deny(
    warnings,
    rustdoc,
    missing_docs,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    missing_copy_implementations,
    meta_variable_misuse,
    non_ascii_idents,
    private_doc_tests,
    single_use_lifetimes,
    unaligned_references,
    unreachable_pub,
    unused_crate_dependencies,
    unused_extern_crates,
    trivial_casts,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    pointer_structural_match,
    missing_debug_implementations
)]
#![allow(clippy::missing_panics_doc)]

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

pub use accessor;
pub use extended_capabilities::ExtendedCapability;
pub use registers::Registers;

pub mod context;
pub mod extended_capabilities;
pub mod registers;
pub mod ring;
