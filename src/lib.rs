#![no_std]

//! A library which is useful to handle xHCI.

mod accessor;
pub mod error;
pub mod mapper;
pub mod registers;

/// A struct which initializes the host controller.
pub struct HcInitializer;
