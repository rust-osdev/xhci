//! xHCI Extended Message Interrupt Capability.

use super::ExtendedCapability;
use accessor::Mapper;
use accessor::Single;
use bit_field::BitField;
use core::convert::TryInto;

/// xHCI Extended Message Interrupt Capability.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XhciExtendedMessageInterrupt {
    _id: u8,
    _next: u8,
    /// Message Control.
    pub control: MessageControl,
    /// Message Upper Address.
    pub upper_address: u32,
    /// Table Offset and BIR.
    pub table_offset: TableOffset,
}
impl<M> From<Single<XhciExtendedMessageInterrupt, M>> for ExtendedCapability<M>
where
    M: Mapper + Clone,
{
    fn from(x: Single<XhciExtendedMessageInterrupt, M>) -> Self {
        ExtendedCapability::XhciExtendedMessageInterrupt(x)
    }
}

/// Message Control.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MessageControl(u16);
impl MessageControl {
    /// Returns the MSI-X Enable bit.
    #[must_use]
    pub fn msi_x_enable(self) -> bool {
        self.0.get_bit(15)
    }

    /// Sets the MSI-X Enable bit.
    pub fn set_msi_x_enable(&mut self, b: bool) {
        self.0.set_bit(15, b);
    }

    /// Returns the value of the Table Size field.
    #[must_use]
    pub fn table_size(self) -> u16 {
        self.0.get_bits(0..=10)
    }
}
impl_debug_from_methods! {
    MessageControl {
        msi_x_enable,
        table_size,
    }
}

/// Table Offset and BIR.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct TableOffset(u32);
impl TableOffset {
    /// Returns the 8-byte aligned offset.
    #[must_use]
    pub fn offset(self) -> u32 {
        self.0 & !0b111
    }

    /// Returns the BIR value.
    #[must_use]
    pub fn bir(self) -> u8 {
        self.0.get_bits(0..=2).try_into().unwrap()
    }
}
impl_debug_from_methods! {
    TableOffset {
        offset,
        bir,
    }
}
