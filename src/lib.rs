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
//! o.usbcmd.update(|u| {
//!     u.set_run_stop();
//! });
//! while o.usbsts.read().hc_halted() {}
//! ```

#![no_std]
#![deny(
    rustdoc::all,
    missing_docs,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    missing_copy_implementations,
    meta_variable_misuse,
    non_ascii_idents,
    private_doc_tests,
    single_use_lifetimes,
    unreachable_pub,
    unused_crate_dependencies,
    unused_extern_crates,
    trivial_casts,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    missing_debug_implementations
)]
#![allow(clippy::missing_panics_doc)]

pub use accessor;
pub use extended_capabilities::ExtendedCapability;
pub use registers::Registers;

#[macro_use]
mod macros;

pub mod context;
pub mod extended_capabilities;
pub mod registers;
pub mod ring;
