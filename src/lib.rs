//! A library which is useful to handle xHCI.

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
    missing_debug_implementations,
    missing_doc_code_examples
)]

pub use accessor;
pub use extended_capabilities::ExtendedCapability;
pub use registers::Registers;

pub mod context;
pub mod extended_capabilities;
pub mod registers;
