//! Transfer TRBs.

use bit_field::BitField;
// use core::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

macro_rules! impl_data_buffer_pointer {
    () => {
        rw_double_field!(
            pub, self,
            self.0.0; [0, 1],
            data_buffer_pointer,
            "Data Buffer Pointer",
            32, u64
        );
    }
}

// 17-bit trb transfer length only.
macro_rules! impl_trb_transfer_length {
    () => {
        rw_field!(pub, self, self.0.0[2]; 0..=16, trb_transfer_length, "TRB Transfer Length", u32);
    };
    (prv) => {
        rw_field!(pub(self), self, self.0.0[2]; 0..=16, trb_transfer_length, "TRB Transfer Length", u32);
    };
}

macro_rules! impl_td_size {
    () => {
        rw_field!(pub, self, self.0.0[2]; 17..=21, td_size, "TD Size", u8);
    };
}

macro_rules! impl_interrupter_target {
    () => {
        rw_field!(pub, self, self.0.0[2]; 22..=31, interrupter_target, "Interrupter Target", u16);
    };
}

macro_rules! impl_ent {
    () => {
        rw_bit!(pub, self, self.0.0[3]; 1, evaluate_next_trb, "Evaluate Next TRB");
    };
}
macro_rules! impl_isp {
    () => {
        rw_bit!(
            pub, self,
            self.0.0[3]; 2,
            interrupt_on_short_packet,
            "Interrupt on Short Packet"
        );
    };
}
macro_rules! impl_ns {
    () => {
        rw_bit!(pub, self, self.0.0[3]; 3, no_snoop, "No Snoop");
    };
}
macro_rules! impl_ch {
    () => {
        rw_bit!(pub, self, self.0.0[3]; 4, chain_bit, "Chain");
    };
}
macro_rules! impl_idt {
    () => {
        rw_bit!(pub, self, self.0.0[3]; 6, immediate_data, "Immediate Data");
    };
    (prv) => {
        rw1s_bit!(pub(self), self, self.0.0[3]; 6, immediate_data, "Immediate Data");
    };
}
macro_rules! impl_bei {
    () => {
        rw_bit!(pub, self, self.0.0[3]; 9, block_event_interrupt, "Block Event Interrupt");
    };
}
macro_rules! impl_dir {
    () => {
        rw_bit!(pub, self, self.0.0[3]; 16, direction, "Direction (is-in)");
    }
}

allowed_trb!("Transfer TRB", {
    /// Normal TRB
    Normal = 1,
    /// Setup Stage TRB
    "no-new" SetupStage = 2,
    /// Data Stage TRB
    DataStage = 3,
    /// Status Stage TRB
    StatusStage = 4,
    /// Isoch TRB
    Isoch = 5,
    /// Link TRB
    Link = 6,
    /// Event Data TRB
    EventData = 7,
    /// No Op Transfer TRB
    NoOp = 8,
});
impl TRB {
    ro_bit!(pub, self, self.0[3]; 4, chain_bit, "Chain");
}

impl Normal {
    impl_data_buffer_pointer!();

    impl_trb_transfer_length!();
    impl_td_size!();
    impl_interrupter_target!();

    impl_ent!();
    impl_isp!();
    impl_ns!();
    impl_ch!();
    impl_ioc!();
    impl_idt!();
    impl_bei!();
}
impl_debug_from_methods!(Normal {
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
    block_event_interrupt,
});
rsvdz_checking_try_from!(Normal {
    [3];7..=8,
    [3];16..=31,
});

impl SetupStage {
    /// Creates a new Setup Stage TRB.
    ///
    /// This method sets the value of the TRB Type, TRB Transfer Length, and the Immediate Data field properly. All the other fields are set to 0.
    #[must_use]
    pub fn new() -> Self {
        *Self(TRB::new(AllowedType::SetupStage))
            .set_immediate_data()
            .set_trb_transfer_length(8)
    }

    rw_field!(pub, self, self.0.0[0]; 0..=7, request_type, "bmRequestType", u8);
    rw_field!(pub, self, self.0.0[0]; 8..=15, request, "bRequest", u8);
    rw_field!(pub, self, self.0.0[0]; 16..=31, value, "wValue", u16);
    rw_field!(pub, self, self.0.0[1]; 0..=15, index, "wIndex", u16);
    rw_field!(pub, self, self.0.0[1]; 16..=31, length, "wLength", u16);

    impl_trb_transfer_length!(prv);
    impl_interrupter_target!();

    impl_ioc!();
    impl_idt!(prv);

    /// Sets the value of the Transfer Type field.
    pub fn set_transfer_type(&mut self, t: TransferType) -> &mut Self {
        self.0 .0[3].set_bits(16..=17, t as _);
        self
    }

    /// Returns the value of the Transfer Type field.
    ///
    /// # Panics
    ///
    /// This method panics if the Transfer Type field contains 1 which is reserved.
    #[must_use]
    pub fn transfer_type(&self) -> TransferType {
        FromPrimitive::from_u32(self.0 .0[3].get_bits(16..=17))
            .expect("Transfer Type 1 is reserved.")
    }
}
impl_debug_from_methods!(SetupStage {
    request_type,
    request,
    value,
    index,
    length,
    trb_transfer_length,
    interrupter_target,
    interrupt_on_completion,
    immediate_data, // always true
    transfer_type,
});
rsvdz_checking_try_from!(SetupStage { // this won't check IDT and transfer length field.
    [2];17..=21,
    [3];1..=4,
    [3];7..=9,
    [3];18..=31,
});

impl DataStage {
    impl_data_buffer_pointer!();

    impl_trb_transfer_length!();
    impl_td_size!();
    impl_interrupter_target!();

    impl_ent!();
    impl_isp!();
    impl_ns!();
    impl_ch!();
    impl_ioc!();
    impl_idt!();
    impl_dir!();
}
impl_debug_from_methods!(DataStage {
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
    direction,
});
rsvdz_checking_try_from!(DataStage {
    [3];7..=9,
    [3];17..=31,
});

impl StatusStage {
    impl_interrupter_target!();

    impl_ent!();
    impl_ch!();
    impl_ioc!();
    impl_dir!();
}
impl_debug_from_methods!(StatusStage {
    interrupter_target,
    evaluate_next_trb,
    chain_bit,
    interrupt_on_completion,
    direction,
});
rsvdz_checking_try_from!(StatusStage {
    [0];0..=31,
    [1];0..=31,
    [2];0..=21,
    [3];2..=3,
    [3];6..=9,
    [3];17..=31,
});

impl Isoch {
    impl_data_buffer_pointer!();

    impl_trb_transfer_length!();
    rw_field!(pub, self, self.0.0[2]; 17..=21, td_size_or_tbc, "TD Size/TBC", u8);
    impl_interrupter_target!();

    impl_ent!();
    impl_isp!();
    impl_ns!();
    impl_ch!();
    impl_ioc!();
    impl_idt!();
    rw_field!(pub, self, self.0.0[3]; 7..=8, tbc_or_sts, "TBC/TRBSts", u8);
    impl_bei!();
    rw_field!(
        pub, self,
        self.0.0[3]; 16..=19,
        transfer_last_burst_packet_count,
        "Transfer Last Burst Packet Count",
        u8
    );
    rw_field!(pub, self, self.0.0[3]; 20..=30, frame_id, "Frame ID", u16);
    rw_bit!(pub, self, self.0.0[3]; 31, start_isoch_asap, "Start Isoch ASAP");
}
impl_debug_from_methods!(Isoch {
    data_buffer_pointer,
    trb_transfer_length,
    td_size_or_tbc,
    interrupter_target,
    evaluate_next_trb,
    interrupt_on_short_packet,
    no_snoop,
    chain_bit,
    interrupt_on_completion,
    immediate_data,
    tbc_or_sts,
    block_event_interrupt,
    transfer_last_burst_packet_count,
    frame_id,
    start_isoch_asap,
});
rsvdz_checking_try_from!(Isoch {});

impl Link {
    impl_ring_segment_pointer!();

    impl_interrupter_target!();

    impl_tc!();
    impl_ch!();
    impl_ioc!();
}
rsvdz_checking_try_from!(Link {
    [0];0..=3,
    [2];0..=21,
    [3];2..=3,
    [3];6..=9,
    [3];16..=31,
});
impl_debug_from_methods!(Link {
    ring_segment_pointer,
    interrupter_target,
    toggle_cycle,
    chain_bit,
    interrupt_on_completion,
});

impl EventData {
    rw_double_field!(
        pub, self,
        self.0.0; [0, 1],
        event_data,
        "Event Data",
        32, u64
    );
    impl_interrupter_target!();
    impl_ent!();
    impl_ch!();
    impl_ioc!();
    impl_bei!();
}
impl_debug_from_methods!(EventData {
    event_data,
    interrupter_target,
    evaluate_next_trb,
    chain_bit,
    interrupt_on_completion,
    block_event_interrupt,
});
rsvdz_checking_try_from!(EventData {
    [2];0..=21,
    [3];2..=3,
    [3];6..=8,
    [3];16..=31,
});

impl NoOp {
    impl_interrupter_target!();
    impl_ent!();
    impl_ch!();
    impl_ioc!();
}
impl_debug_from_methods!(NoOp {
    interrupter_target,
    evaluate_next_trb,
    chain_bit,
    interrupt_on_completion,
});
rsvdz_checking_try_from!(NoOp {
    [0];0..=31,
    [1];0..=31,
    [2];0..=21,
    [3];2..=3,
    [3];6..=9,
    [3];16..=31,
});

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
