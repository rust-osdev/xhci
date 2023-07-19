//! Transfer TRBs.

use super::{Link, Type};
use bit_field::BitField;
use core::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

allowed! {
    /// TRBs which are allowed to be pushed to the Transfer Ring.
    enum {
        /// Normal TRB.
        Normal,
        /// Setup Stage TRB.
        SetupStage,
        /// Data Stage TRB.
        DataStage,
        /// Status Stage TRB.
        StatusStage,
        /// Isoch TRB.
        Isoch,
        /// Link TRB.
        Link,
        /// Event Data TRB.
        EventData,
        /// No Op TRB.
        Noop
    }
}
impl Allowed {
    /// Sets the value of the Interrupt On Completion field.
    pub fn set_interrupt_on_completion(&mut self) {
        macro_rules! arm{
            ($($variant:ident),*)=>{
                match self {
                    $(Self::$variant(ref mut x)=>{
                        x.set_interrupt_on_completion();
                    },)*
                }
            };
        }

        arm!(
            Normal,
            SetupStage,
            DataStage,
            StatusStage,
            Isoch,
            Link,
            EventData,
            Noop
        );
    }

    /// Clears the Interrupt On Completion bit.
    pub fn clear_interrupt_on_completion(&mut self) {
        macro_rules! arm{
            ($($variant:ident),*)=>{
                match self {
                    $(Self::$variant(ref mut x)=>{
                        x.clear_interrupt_on_completion();
                    },)*
                }
            };
        }

        arm!(
            Normal,
            SetupStage,
            DataStage,
            StatusStage,
            Isoch,
            Link,
            EventData,
            Noop
        );
    }

    /// Returns the value of the Interrupt On Completion field.
    #[must_use]
    pub fn interrupt_on_completion(&self) -> bool {
        macro_rules! arm{
            ($($variant:ident),*)=>{
                match self {
                    $(Self::$variant(x)=>{
                        x.interrupt_on_completion()
                    },)*
                }
            };
        }

        arm!(
            Normal,
            SetupStage,
            DataStage,
            StatusStage,
            Isoch,
            Link,
            EventData,
            Noop
        )
    }
}
impl TryFrom<[u32; 4]> for Allowed {
    type Error = [u32; 4];

    fn try_from(raw: [u32; 4]) -> Result<Self, Self::Error> {
        try_from!(
            raw =>
            Normal,
            SetupStage,
            DataStage,
            StatusStage,
            Isoch,
            Link,
            EventData,
            Noop(Transfer),
        );
        Err(raw)
    }
}

macro_rules! interrupt_on_completion {
    ($name:ident) => {
        impl $name {
            rw_bit!([3](5), interrupt_on_completion, "Interrupt On Completion");
        }
    };
}
macro_rules! transfer_trb {
    ($name:ident,$full:expr,$type:expr) => {
        add_trb!($name, $full, $type);
        interrupt_on_completion!($name);
    };
}
macro_rules! transfer_trb_with_default {
    ($name:ident,$full:expr,$type:expr) => {
        add_trb_with_default!($name, $full, $type);
        interrupt_on_completion!($name);
    };
}
macro_rules! impl_debug_for_transfer_trb{
    ($name:ident {})=>{
        impl_debug_for_trb!($name{
            interrupt_on_completion
        });
    };
    ($name:ident {
        $($method:ident),*$(,)?
    })=>{
        impl_debug_for_trb!($name{
            interrupt_on_completion,
            $($method),*
        });
    }
}

transfer_trb_with_default!(Normal, "Normal TRB", Type::Normal);
reserved!(Normal(Type::Normal) {
    [3]7..=8;
    [3]16..=31;
});
impl Normal {
    /// Sets the value of the Data Buffer Pointer field.
    pub fn set_data_buffer_pointer(&mut self, p: u64) -> &mut Self {
        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
        self
    }

    /// Returns the value of the Data Buffer Pointer field.
    #[must_use]
    pub fn data_buffer_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    rw_field!([2](0..=16), trb_transfer_length, "TRB Transfer Length", u32);
    rw_field!([2](17..=21), td_size, "TD Size", u8);
    rw_field!([2](22..=31), interrupter_target, "Interrupter Target", u16);
    rw_bit!([3](1), evaluate_next_trb, "Evaluate Next TRB");
    rw_bit!(
        [3](2),
        interrupt_on_short_packet,
        "Interrupt-on Short Packet"
    );
    rw_bit!([3](3), no_snoop, "No Snoop");
    rw_bit!([3](4), chain_bit, "Chain bit");
    rw_bit!([3](6), immediate_data, "Immediate Data");
    rw_bit!([3](9), block_event_interrupt, "Block Event Interrupt");
}
impl_debug_for_transfer_trb! {
    Normal {
        data_buffer_pointer,
        trb_transfer_length,
        td_size,
        interrupter_target,
        cycle_bit,
        evaluate_next_trb,
        interrupt_on_short_packet,
        no_snoop,
        chain_bit,
        interrupt_on_completion,
        immediate_data,
        block_event_interrupt,
    }
}

transfer_trb!(SetupStage, "Setup Stage TRB", Type::SetupStage);
reserved!(SetupStage(Type::SetupStage) {
    [2]17..=21;
    [3]1..=4;
    [3]7..=9;
    [3]18..=31;
});
impl SetupStage {
    /// Creates a new Setup Stage TRB.
    ///
    /// This method sets the value of the TRB Type, TRB Transfer Length, and the Immediate Data field properly. All the
    /// other fields are set to 0.
    #[must_use]
    pub fn new() -> Self {
        *Self([0; 4])
            .set_trb_type()
            .set_idt()
            .set_trb_transfer_length()
    }

    rw_field!([0](0..=7), request_type, "bmRequestType", u8);
    rw_field!([0](8..=15), request, "bRequest", u8);
    rw_field!([0](16..=31), value, "wValue", u16);
    rw_field!([1](0..=15), index, "wIndex", u16);
    rw_field!([1](16..=31), length, "wLength", u16);
    rw_field!([2](22..=31), interrupter_target, "Interrupter Target", u16);

    /// Sets the value of the Transfer Type field.
    pub fn set_transfer_type(&mut self, t: TransferType) -> &mut Self {
        self.0[3].set_bits(16..=17, t as _);
        self
    }

    /// Returns the value of the Transfer Type field.
    ///
    /// # Panics
    ///
    /// This method panics if the Transfer Type field contains 1 which is reserved.
    #[must_use]
    pub fn transfer_type(&self) -> TransferType {
        FromPrimitive::from_u32(self.0[3].get_bits(16..=17)).expect("Transfer Type 1 is reserved.")
    }

    fn set_idt(&mut self) -> &mut Self {
        self.0[3].set_bit(6, true);
        self
    }

    fn set_trb_transfer_length(&mut self) -> &mut Self {
        self.0[2].set_bits(0..=16, 8);
        self
    }
}
impl Default for SetupStage {
    fn default() -> Self {
        Self::new()
    }
}
impl_debug_for_transfer_trb!(SetupStage {
    request_type,
    request,
    value,
    index,
    length,
    interrupt_on_completion,
    transfer_type,
});

transfer_trb_with_default!(DataStage, "Data Stage TRB", Type::DataStage);
reserved!(DataStage(Type::DataStage) {
    [3]7..=9;
    [3]17..=31;
});
impl DataStage {
    /// Sets the value of the Data Buffer Pointer field.
    pub fn set_data_buffer_pointer(&mut self, p: u64) -> &mut Self {
        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
        self
    }

    /// Returns the value of the Data Buffer Pointer field.
    #[must_use]
    pub fn data_buffer_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    rw_field!([2](0..=16), trb_transfer_length, "TRB Transfer Length", u32);
    rw_field!([2](17..=21), td_size, "TD Size", u8);
    rw_field!([2](22..=31), interrupter_target, "Interrupter Target", u16);
    rw_bit!([3](1), evaluate_next_trb, "Evaluate Next TRB");
    rw_bit!(
        [3](2),
        interrupt_on_short_packet,
        "Interrupt-on Short Packet"
    );
    rw_bit!([3](3), no_snoop, "No Snoop");
    rw_bit!([3](4), chain_bit, "Chain bit");
    rw_bit!([3](6), immediate_data, "Immediate Data");

    /// Sets the value of the Direction field.
    pub fn set_direction(&mut self, d: Direction) -> &mut Self {
        self.0[3].set_bit(16, d.into());
        self
    }

    /// Returns the value of the Direction field.
    #[must_use]
    pub fn direction(&self) -> Direction {
        self.0[3].get_bit(16).into()
    }
}
impl_debug_for_transfer_trb!(DataStage {
    data_buffer_pointer,
    trb_transfer_length,
    td_size,
    interrupter_target,
    evaluate_next_trb,
    interrupt_on_short_packet,
    no_snoop,
    chain_bit,
    interrupt_on_completion,
    immediate_data,
    direction
});

transfer_trb_with_default!(StatusStage, "Status Stage TRB", Type::StatusStage);
reserved!(StatusStage(Type::StatusStage) {
    [0]0..=31;
    [1]0..=31;
    [2]0..=21;
    [3]2..=3;
    [3]6..=9;
    [3]17..=31;
});
impl StatusStage {
    rw_field!([2](22..=31), interrupter_target, "Interrupter Target", u16);
    rw_bit!([3](1), evaluate_next_trb, "Evaluate Next TRB");
    rw_bit!([3](4), chain_bit, "Chain bit");
    rw_bit!([3](16), direction, "Direction");
}
impl_debug_for_transfer_trb! {
    StatusStage {
        interrupter_target,
        evaluate_next_trb,
        chain_bit,
        interrupt_on_completion,
        direction,
    }
}

transfer_trb_with_default!(Isoch, "Isoch TRB", Type::Isoch);
reserved!(Isoch(Type::Isoch) {});
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
    #[must_use]
    pub fn data_buffer_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    rw_field!([2](0..=16), trb_transfer_length, "TRB Transfer Length", u32);
    rw_field!([2](17..=21), td_size_or_tbc, "TD Size/TBC", u8);
    rw_field!([2](22..=31), interrupter_target, "Interrupter Target", u16);
    rw_bit!([3](1), evaluate_next_trb, "Evaluate Next TRB");
    rw_bit!(
        [3](2),
        interrupt_on_short_packet,
        "Interrupt on Short Packet"
    );
    rw_bit!([3](3), no_snoop, "No Snoop");
    rw_bit!([3](4), chain_bit, "Chain bit");
    rw_bit!([3](6), immediate_data, "Immediate Data");
    rw_field!([3](7..=8), transfer_burst_count, "Transfer Burst Count", u8);
    rw_bit!([3](9), block_event_interrupt, "Block Event Interrupt");
    rw_field!(
        [3](16..=19),
        transfer_last_burst_packet_count,
        "Transfer Last Burst Packet Count",
        u8
    );
    rw_field!([3](20..=30), frame_id, "Frame ID", u16);
    rw_bit!([3](31), start_isoch_asap, "Start Isoch ASAP");
}
impl_debug_for_transfer_trb!(Isoch {
    data_buffer_pointer,
    trb_transfer_length,
    td_size_or_tbc,
    interrupter_target,
    evaluate_next_trb,
    interrupt_on_short_packet,
    no_snoop,
    chain_bit,
    immediate_data,
    transfer_burst_count,
    block_event_interrupt,
    transfer_last_burst_packet_count,
    frame_id,
    start_isoch_asap
});

transfer_trb_with_default!(EventData, "Event Data TRB", Type::EventData);
reserved!(EventData(Type::EventData) {
    [2]0..=21;
    [3]2..=3;
    [3]6..=8;
    [3]16..=31;
});
impl EventData {
    /// Sets the value of the Event Data field.
    pub fn set_event_data(&mut self, d: u64) -> &mut Self {
        let l = d.get_bits(0..32);
        let u = d.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
        self
    }

    /// Returns the value of the Event Data field.
    #[must_use]
    pub fn event_data(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    rw_field!([2](22..=31), interrupter_target, "Interrupter Target", u16);
    rw_bit!([3](1), evaluate_next_trb, "Evaluate Next TRB");
    rw_bit!([3](4), chain_bit, "Chain bit");
    rw_bit!([3](9), block_event_interrupt, "Block Event Interrupt");
}
impl_debug_for_transfer_trb!(EventData {
    event_data,
    interrupter_target,
    evaluate_next_trb,
    chain_bit,
    block_event_interrupt
});

transfer_trb_with_default!(Noop, "No Op TRB", Type::NoopTransfer);
reserved!(Noop(Type::NoopTransfer) {
    [0]0..=31;
    [1]0..=31;
    [2]0..=21;
    [3]2..=3;
    [3]6..=9;
    [3]16..=31;
});
impl Noop {
    rw_field!([2](22..=31), interrupter_target, "Interrupter Target", u16);
    rw_bit!([3](1), evaluate_next_trb, "Evaluate Next TRB");
    rw_bit!([3](4), chain_bit, "Chain bit");
}
impl_debug_for_transfer_trb!(Noop {
    interrupter_target,
    evaluate_next_trb,
    chain_bit
});

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
impl From<bool> for Direction {
    fn from(b: bool) -> Self {
        if b {
            Direction::In
        } else {
            Direction::Out
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normal_data_buffer_pointer() {
        let mut normal = Normal::new();
        let pointer = 0x12345678_9abcdef0;
        normal.set_data_buffer_pointer(pointer);
        let pointer_read = normal.data_buffer_pointer();
        assert_eq!(pointer, pointer_read);
    }

    #[test]
    fn isoch_data_buffer_pointer() {
        let mut isoch = Isoch::new();
        let pointer = 0xabcd1234_567890ef;
        isoch.set_data_buffer_pointer(pointer);
        let pointer_read = isoch.data_buffer_pointer();
        assert_eq!(pointer, pointer_read);
    }
}
