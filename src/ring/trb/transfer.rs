//! Transfer TRBs.

use bit_field::BitField;
use core::convert::TryInto;

add_trb_with_default!(Normal, "Normal TRB", Type::Normal);
impl Normal {
    /// Sets the value of the Data Buffer Pointer field.
    pub fn set_data_buffer_pointer(&mut self, p: u64) -> &mut Self {
        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
        self
    }

    /// Sets the value of the TRB Transfer Length field.
    pub fn set_trb_transfer_length(&mut self, l: u32) -> &mut Self {
        self.0[2].set_bits(0..=16, l);
        self
    }

    /// Sets the value of the Interrupt On Completion field.
    pub fn set_interrupt_on_completion(&mut self, ioc: bool) -> &mut Self {
        self.0[3].set_bit(5, ioc);
        self
    }
}

add_trb!(SetupStage, "Setup Stage TRB", Type::SetupStage);
impl SetupStage {
    /// Creates a new Setup Stage TRB.
    ///
    /// This method sets the value of the TRB Type and the Immediate Data field properly. All the
    /// other fields are set to 0.
    pub fn new() -> Self {
        *Self([0; 4]).set_trb_type().set_idt()
    }

    /// Sets the value of the bmRequestType field.
    pub fn set_request_type(&mut self, t: u8) -> &mut Self {
        self.0[0].set_bits(0..=7, t.into());
        self
    }

    /// Sets the value of the bRequest field.
    pub fn set_request(&mut self, r: u8) -> &mut Self {
        self.0[0].set_bits(8..=15, r.into());
        self
    }

    /// Sets the value of the wValue field.
    pub fn set_value(&mut self, v: u16) -> &mut Self {
        self.0[0].set_bits(16..=31, v.into());
        self
    }

    /// Sets the value of the wLength field.
    pub fn set_length(&mut self, l: u16) -> &mut Self {
        self.0[1].set_bits(16..=31, l.into());
        self
    }

    /// Sets the value of the TRB Transfer Length field.
    pub fn set_trb_transfer_length(&mut self, l: u32) -> &mut Self {
        self.0[2].set_bits(0..=16, l);
        self
    }

    /// Sets the value of the Transfer Type field.
    pub fn set_transfer_type(&mut self, t: TransferType) -> &mut Self {
        self.0[3].set_bits(16..=17, t as _);
        self
    }

    fn set_idt(&mut self) -> &mut Self {
        self.0[3].set_bit(6, true);
        self
    }
}
impl Default for SetupStage {
    fn default() -> Self {
        Self::new()
    }
}

/// Transfer Type.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TransferType {
    /// No Data Stage.
    NoDataStage = 0,
    /// Out Data Stage.
    OutDataStage = 2,
    /// In Data Stage.
    InDataStage = 3,
}
