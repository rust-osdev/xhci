//! xHCI Message Interrupt Capability.

use bit_field::BitField;
use core::convert::TryFrom;
use core::convert::TryInto;

/// xHCI Message Interrupt Capability.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XhciMessageInterrupt<T>
where
    T: MessageAddress,
    <T as TryFrom<u64>>::Error: core::fmt::Debug,
{
    _id: u8,
    _next: u8,
    /// Message Control.
    pub control: MessageControl,
    address: T,
    /// Data.
    pub data: u16,
}
impl<T> XhciMessageInterrupt<T>
where
    T: MessageAddress,
    <T as TryFrom<u64>>::Error: core::fmt::Debug,
{
    /// Sets the value of the Message Address.
    ///
    /// # Panics
    ///
    /// This method panics if one of the following conditions is met.
    ///
    /// - The passed address does not fit.
    /// - Bits `0..=1` of the Address are not 0.
    pub fn set_addr(&mut self, a: u64) {
        assert!(
            a.trailing_zeros() >= 2,
            "Bits 0..=1 of the Message Address must be 0."
        );

        self.address = a.try_into().expect("The address does not fit.");
    }

    /// Returns the value of the Message Address.
    pub fn get_addr(&self) -> u64 {
        self.address.into()
    }
}

/// A marker trait for the Message Address.
pub trait MessageAddress: Into<u64> + TryFrom<u64> + Copy
where
    <Self as TryFrom<u64>>::Error: core::fmt::Debug,
{
}
impl MessageAddress for u32 {}
impl MessageAddress for u64 {}

/// Message Control.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MessageControl(u16);
impl MessageControl {
    /// Returns the 64 bit address capable bit.
    #[must_use]
    pub fn bit64_address_capable(self) -> bool {
        self.0.get_bit(7)
    }

    /// Returns the value of the Multiple Message Enable field.
    #[must_use]
    pub fn multiple_message_enable(self) -> u8 {
        self.0.get_bits(4..=6).try_into().unwrap()
    }

    /// Sets the value of the Multiple Message Enable field.
    pub fn set_multiple_message_enable(&mut self, m: u8) {
        self.0.set_bits(4..=6, m.into());
    }

    /// Returns the value of the Multiple Message Capable field.
    #[must_use]
    pub fn multiple_message_capable(self) -> u8 {
        self.0.get_bits(1..=3).try_into().unwrap()
    }

    /// Returns the MSI Enable bit.
    #[must_use]
    pub fn msi_enable(self) -> bool {
        self.0.get_bit(0)
    }

    /// Sets the MSI Enable bit.
    pub fn set_msi_enable(&mut self, b: bool) {
        self.0.set_bit(0, b);
    }
}
impl_debug_from_methods! {
    MessageControl {
        bit64_address_capable,
        multiple_message_enable,
        multiple_message_capable,
        msi_enable,
    }
}
