//! Transfer TRBs.

use super::Link;
use bit_field::BitField;
use core::convert::TryInto;
use num_derive::FromPrimitive;

/// TRBs which are allowed to be pushed to the Transfer Ring.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Allowed {
    /// Normal TRB.
    Normal(Normal),
    /// Setup Stage TRB.
    SetupStage(SetupStage),
    /// Data Stage TRB.
    DataStage(DataStage),
    /// Status Stage TRB.
    StatusStage(StatusStage),
    /// Isoch TRB.
    Isoch(Isoch),
    /// No Op TRB.
    Noop(Noop),
    /// Link TRB.
    Link(Link),
}
impl Allowed {
    /// Sets the value of the Cycle Bit.
    pub fn set_cycle_bit(&mut self, b: bool) -> &mut Self {
        match self {
            Self::Normal(ref mut n) => {
                n.set_cycle_bit(b);
            }
            Self::SetupStage(ref mut s) => {
                s.set_cycle_bit(b);
            }
            Self::DataStage(ref mut d) => {
                d.set_cycle_bit(b);
            }
            Self::StatusStage(ref mut s) => {
                s.set_cycle_bit(b);
            }
            Self::Isoch(ref mut i) => {
                i.set_cycle_bit(b);
            }
            Self::Noop(ref mut n) => {
                n.set_cycle_bit(b);
            }
            Self::Link(ref mut l) => {
                l.set_chain_bit(b);
            }
        }
        self
    }

    /// Returns the value of the Cycle Bit.
    pub fn cycle_bit(&self) -> bool {
        match self {
            Self::Normal(ref n) => n.cycle_bit(),
            Self::SetupStage(ref s) => s.cycle_bit(),
            Self::DataStage(ref d) => d.cycle_bit(),
            Self::StatusStage(ref s) => s.cycle_bit(),
            Self::Isoch(ref i) => i.cycle_bit(),
            Self::Noop(ref n) => n.cycle_bit(),
            Self::Link(ref l) => l.cycle_bit(),
        }
    }

    /// Returns the wrapped array.
    pub fn into_raw(self) -> [u32; 4] {
        match self {
            Self::Normal(n) => n.into_raw(),
            Self::SetupStage(s) => s.into_raw(),
            Self::DataStage(d) => d.into_raw(),
            Self::StatusStage(s) => s.into_raw(),
            Self::Isoch(i) => i.into_raw(),
            Self::Noop(n) => n.into_raw(),
            Self::Link(l) => l.into_raw(),
        }
    }
}
impl AsRef<[u32]> for Allowed {
    fn as_ref(&self) -> &[u32] {
        match self {
            Self::Normal(ref n) => n.as_ref(),
            Self::SetupStage(ref s) => s.as_ref(),
            Self::DataStage(ref d) => d.as_ref(),
            Self::StatusStage(ref s) => s.as_ref(),
            Self::Isoch(ref i) => i.as_ref(),
            Self::Noop(ref n) => n.as_ref(),
            Self::Link(ref l) => l.as_ref(),
        }
    }
}
impl AsMut<[u32]> for Allowed {
    fn as_mut(&mut self) -> &mut [u32] {
        match self {
            Self::Normal(ref mut n) => n.as_mut(),
            Self::SetupStage(ref mut s) => s.as_mut(),
            Self::DataStage(ref mut d) => d.as_mut(),
            Self::StatusStage(ref mut s) => s.as_mut(),
            Self::Isoch(ref mut i) => i.as_mut(),
            Self::Noop(ref mut n) => n.as_mut(),
            Self::Link(ref mut l) => l.as_mut(),
        }
    }
}

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
    #[must_use]
    pub fn new() -> Self {
        *Self([0; 4]).set_trb_type().set_idt()
    }

    /// Sets the value of the `bmRequestType` field.
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

add_trb_with_default!(DataStage, "Data Stage TRB", Type::DataStage);
impl DataStage {
    /// Sets the value of the Data Buffer Pointer field.
    pub fn set_data_buffer_pointer(&mut self, p: u64) -> &mut Self {
        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
        self
    }

    /// Sets the value of the TRB Tranfer Length field.
    pub fn set_trb_transfer_length(&mut self, l: u32) -> &mut Self {
        self.0[2].set_bits(0..=16, l);
        self
    }

    /// Sets the value of the Direction field.
    pub fn set_direction(&mut self, d: Direction) -> &mut Self {
        self.0[3].set_bit(16, d.into());
        self
    }
}

add_trb_with_default!(StatusStage, "Status Stage TRB", Type::StatusStage);
impl StatusStage {
    /// Sets the value of the Interrupt On Completion bit.
    pub fn set_interrupt_on_completion(&mut self, i: bool) -> &mut Self {
        self.0[3].set_bit(5, i);
        self
    }
}

add_trb_with_default!(Isoch, "Isoch TRB", Type::Isoch);
impl Isoch {
    /// Sets the value of the Data Buffer Pointer.
    pub fn set_data_buffer_pointer(&mut self, p: u64) -> &mut Self {
        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
        self
    }

    /// Returns the value of the Data Buffer Pointer.
    pub fn data_buffer_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    /// Sets the value of the TRB Transfer Length field.
    pub fn set_trb_transfer_length(&mut self, l: u32) -> &mut Self {
        self.0[2].set_bits(0..=16, l);
        self
    }

    /// Returns the value of the TRB Transfer Length field.
    pub fn trb_transfer_length(&self) -> u32 {
        self.0[2].get_bits(0..=16)
    }

    /// Sets the value of the TD Size/TBC field.
    pub fn set_td_size_or_tbc(&mut self, t: u8) -> &mut Self {
        self.0[2].set_bits(17..=21, t.into());
        self
    }

    /// Returns the value of the TD Size/TBC field.
    pub fn td_size_or_tbc(&self) -> u8 {
        self.0[2].get_bits(17..=21).try_into().unwrap()
    }

    /// Sets the value of the Interrupter Target.
    pub fn set_interrupter_target(&mut self, t: u16) -> &mut Self {
        self.0[2].set_bits(22..=31, t.into());
        self
    }

    /// Returns the value of the Interrupter Target.
    pub fn interrupter_target(&self) -> u16 {
        self.0[2].get_bits(22..=31).try_into().unwrap()
    }

    /// Sets the value of the Evaluate Next TRB field.
    pub fn set_evaluate_next_trb(&mut self, ent: bool) -> &mut Self {
        self.0[3].set_bit(1, ent);
        self
    }

    /// Returns the value of the Evaluate Next TRB field.
    pub fn evaluate_next_trb(&self) -> bool {
        self.0[3].get_bit(1)
    }

    /// Sets the value of the Interrupt-on Short Packet field.
    pub fn set_interrupt_on_short_packet(&mut self, isp: bool) -> &mut Self {
        self.0[3].set_bit(2, isp);
        self
    }

    /// Returns the value of the Interrupt-on Short Packet field.
    pub fn interrupt_on_short_packet(&self) -> bool {
        self.0[3].get_bit(2)
    }

    /// Sets the value of the No Snoop field.
    pub fn set_no_snoop(&mut self, s: bool) -> &mut Self {
        self.0[3].set_bit(3, s);
        self
    }

    /// Returns the value of the No Snoop field.
    pub fn no_snoop(&self) -> bool {
        self.0[3].get_bit(3)
    }

    /// Sets the value of the Chain Bit field.
    pub fn set_chain_bit(&mut self, b: bool) -> &mut Self {
        self.0[3].set_bit(4, b);
        self
    }

    /// Returns the value of the Chain Bit field.
    pub fn chain_bit(&self) -> bool {
        self.0[3].get_bit(4)
    }

    /// Sets the value of the Interrupt On Completion field.
    pub fn set_interrupt_on_completion(&mut self, ioc: bool) -> &mut Self {
        self.0[3].set_bit(5, ioc);
        self
    }

    /// Returns the value of the Interrupt On Completion field.
    pub fn interrupt_on_completion(&self) -> bool {
        self.0[3].get_bit(5)
    }

    /// Sets the value of the Immediate Data field.
    pub fn set_immediate_data(&mut self, idt: bool) -> &mut Self {
        self.0[3].set_bit(6, idt);
        self
    }

    /// Returns the value of the Immediate Data.
    pub fn immediate_data(&self) -> bool {
        self.0[3].get_bit(6)
    }

    /// Sets the value of the Transfer Burst Count field.
    pub fn set_transfer_burst_count(&mut self, c: u8) -> &mut Self {
        self.0[3].set_bits(7..=8, c.into());
        self
    }

    /// Returns the value of the Transfer Burst Count field.
    pub fn transfer_burst_count(&self) -> u8 {
        self.0[3].get_bits(7..=8).try_into().unwrap()
    }

    /// Sets the value of the Block Event Interrupt field.
    pub fn set_block_event_interrupt(&mut self, bei: bool) -> &mut Self {
        self.0[3].set_bit(9, bei);
        self
    }

    /// Returns the value of the Block Event Interrupt field.
    pub fn block_event_interrupt(&self) -> bool {
        self.0[3].get_bit(9)
    }

    /// Sets the value of the Transfer Last Burst Packet Count field.
    pub fn set_transfer_last_burst_packet_count(&mut self, c: u8) -> &mut Self {
        self.0[3].set_bits(16..=19, c.into());
        self
    }

    /// Returns the value of the Transfer Last Burst Packet Count field.
    pub fn transfer_last_burst_packet_count(&self) -> u8 {
        self.0[3].get_bits(16..=19).try_into().unwrap()
    }

    /// Sets the value of the Frame ID field.
    pub fn set_frame_id(&mut self, id: u16) -> &mut Self {
        self.0[3].set_bits(20..=30, id.into());
        self
    }

    /// Returns the value of the Frame ID field.
    pub fn frame_id(&self) -> u16 {
        self.0[3].get_bits(20..=30).try_into().unwrap()
    }

    /// Sets the value of the Start Isoch ASAP field.
    pub fn set_start_isoch_asap(&mut self, sia: bool) -> &mut Self {
        self.0[3].set_bit(31, sia);
        self
    }

    /// Returns the value of the Start Isoch ASAP field.
    pub fn start_isoch_asap(&self) -> bool {
        self.0[3].get_bit(31)
    }
}

add_trb_with_default!(Noop, "No Op TRB", Type::NoopTransfer);
impl Noop {
    /// Sets the value of the Interrupter Target.
    pub fn set_interrupter_target(&mut self, t: u16) -> &mut Self {
        self.0[2].set_bits(22..=31, t.into());
        self
    }

    /// Returns the value of the Interrupter Target.
    pub fn interrupter_target(&self) -> u16 {
        self.0[2].get_bits(22..=31).try_into().unwrap()
    }

    /// Sets the value of the Evaluate Next TRB field.
    pub fn set_evaluate_next_trb(&mut self, ent: bool) -> &mut Self {
        self.0[3].set_bit(1, ent);
        self
    }

    /// Returns the value of the Evaluate Next TRB field.
    pub fn evaluate_next_trb(&self) -> bool {
        self.0[3].get_bit(1)
    }

    /// Sets the value of the Chain Bit field.
    pub fn set_chain_bit(&mut self, b: bool) -> &mut Self {
        self.0[3].set_bit(4, b);
        self
    }

    /// Returns the value of the Chain Bit field.
    pub fn chain_bit(&self) -> bool {
        self.0[3].get_bit(4)
    }

    /// Sets the value of the Interrupt On Completion field.
    pub fn set_interrupt_on_completion(&mut self, ioc: bool) -> &mut Self {
        self.0[3].set_bit(5, ioc);
        self
    }

    /// Returns the value of the Interrupt On Completion field.
    pub fn interrupt_on_completion(&mut self) -> bool {
        self.0[3].get_bit(5)
    }
}

/// The direction of the data transfer.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
pub enum Direction {
    /// Out (Write Data)
    Out = 0,
    /// In (Read Data)
    In = 1,
}
impl From<Direction> for bool {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Out => false,
            Direction::In => true,
        }
    }
}

/// Transfer Type.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive)]
#[allow(clippy::module_name_repetitions)]
pub enum TransferType {
    /// No Data Stage.
    No = 0,
    /// Out Data Stage.
    Out = 2,
    /// In Data Stage.
    In = 3,
}
