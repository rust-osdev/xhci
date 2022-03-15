//! xHCI Message Interrupt Capability.

use super::ExtendedCapability;
use accessor::single;
use accessor::Mapper;
use bit_field::BitField;
use core::convert::TryFrom;
use core::convert::TryInto;

/// xHCI Message Interrupt Capability.
#[derive(Debug)]
pub enum XhciMessageInterrupt<M>
where
    M: Mapper,
{
    /// xHCI Message Interrupt Capability with the 32-bit Message Address.
    Addr32(single::ReadWrite<Internal<u32>, M>),
    /// xHCI Message Interrupt Capability with the 64-bit Message Address.
    Addr64(single::ReadWrite<Internal<u64>, M>),
}
impl<M> XhciMessageInterrupt<M>
where
    M: Mapper + Clone,
{
    /// Creates an accessor to xHCI Message Interrupt Capability.
    ///
    /// # Safety
    ///
    /// `base` must be the correct address to xHCI Message Interrupt Capability.
    ///
    /// # Panics
    ///
    /// This method panics if `base` is not aligned correctly.
    pub unsafe fn new(base: usize, mapper: M) -> Self {
        let control: single::ReadWrite<MessageControl, M> =
            single::ReadWrite::new(base + 2, mapper.clone());

        if control.read_volatile().bit64_address_capable() {
            Self::Addr64(single::ReadWrite::new(base, mapper))
        } else {
            Self::Addr32(single::ReadWrite::new(base, mapper))
        }
    }
}
impl<M> From<XhciMessageInterrupt<M>> for ExtendedCapability<M>
where
    M: Mapper + Clone,
{
    fn from(x: XhciMessageInterrupt<M>) -> Self {
        ExtendedCapability::XhciMessageInterrupt(x)
    }
}

/// The actual structure of xHCI Message Interrupt Capability.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Internal<T>
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
    /// Mask Bits.
    pub mask_bits: u32,
    /// Pending Bits.
    pub pending_bits: u32,
}
impl<T> Internal<T>
where
    T: MessageAddress,
    <T as TryFrom<u64>>::Error: core::fmt::Debug,
{
    /// Sets the Message Address.
    ///
    /// # Panics
    ///
    /// This method panics if the user breaks one of the following conditions:
    ///
    /// - Bits `0..=1` of the address must be 0.
    /// - The address must fit, especially if `T = u32`.
    pub fn set_addr(&mut self, a: u64) {
        assert!(
            a.trailing_zeros() >= 2,
            "Bits 0..=1 of the Message Address must be 0."
        );

        self.address = a.try_into().expect("The address does not fit.");
    }

    /// Returns the Message Address.
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
    ro_bit!(8, per_vector_masking_capable, "Per-vector masking capable");
    ro_bit!(7, bit64_address_capable, "64 bit address capable");

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

    rw_bit!(0, msi_enable, "MSI Enable");
}
impl_debug_from_methods! {
    MessageControl {
        per_vector_masking_capable,
        bit64_address_capable,
        multiple_message_enable,
        multiple_message_capable,
        msi_enable,
    }
}
