use crate::registers::Registers;
use alloc::{vec, vec::Vec};
use conquer_once::spin::OnceCell;
use qemu_print::qemu_println;
use spinning_top::Spinlock;
use xhci::ring::trb;

const NUM_OF_TRBS_IN_RING: usize = 16;

pub struct EventHandler {
    segment_table: Vec<EventRingSegmentTableEntry>,
    rings: Vec<EventRing>,

    dequeue_ptr_segment: u64,
    dequeue_ptr_ring: u64,

    cycle_bit: bool,
}
impl EventHandler {
    pub fn new(regs: &mut Registers) -> Self {
        let mut v = Self {
            segment_table: vec![EventRingSegmentTableEntry::null(); number_of_rings(regs).into()],
            rings: vec![EventRing::new(); number_of_rings(regs).into()],

            dequeue_ptr_segment: 0,
            dequeue_ptr_ring: 0,

            cycle_bit: true,
        };

        v.init(regs);

        v
    }

    fn init(&mut self, regs: &mut Registers) {
        self.register_dequeue_pointer(regs);

        self.write_rings_addresses_in_table();
        self.register_table_size(regs);
        self.enable_event_ring(regs);
    }

    fn register_dequeue_pointer(&self, regs: &mut Registers) {
        regs.interrupter_register_set
            .interrupter_mut(0)
            .erdp
            .update_volatile(|erdp| erdp.set_event_ring_dequeue_pointer(self.next_trb_addr()))
    }

    fn write_rings_addresses_in_table(&mut self) {
        let mut segment_table = self.segment_table.clone();

        for (i, ring) in self.rings.iter().enumerate() {
            segment_table[i].base_addr = ring as *const _ as u64;
            segment_table[i].segment_size = NUM_OF_TRBS_IN_RING as _;
        }
    }

    fn register_table_size(&self, regs: &mut Registers) {
        regs.interrupter_register_set
            .interrupter_mut(0)
            .erstsz
            .update_volatile(|erstsz| {
                erstsz.set(self.segment_table.len() as u16);
            })
    }

    fn enable_event_ring(&self, regs: &mut Registers) {
        regs.interrupter_register_set
            .interrupter_mut(0)
            .erstba
            .update_volatile(|erstba| erstba.set(self.segment_table.as_ptr() as u64))
    }

    fn next_trb_addr(&self) -> u64 {
        self.segment_table[self.dequeue_ptr_segment as usize].base_addr
            + self.dequeue_ptr_ring as u64 * trb::BYTES as u64
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
struct EventRingSegmentTableEntry {
    base_addr: u64,
    segment_size: u64,
}
impl EventRingSegmentTableEntry {
    fn null() -> Self {
        Self {
            base_addr: 0,
            segment_size: 0,
        }
    }
}

#[repr(C, align(64))]
#[derive(Clone, Copy, Debug)]
struct EventRing([[u32; 4]; NUM_OF_TRBS_IN_RING]);
impl EventRing {
    fn new() -> Self {
        Self([[0; 4]; NUM_OF_TRBS_IN_RING])
    }
}

fn number_of_rings(regs: &Registers) -> u16 {
    regs.capability
        .hcsparams2
        .read_volatile()
        .event_ring_segment_table_max()
}
