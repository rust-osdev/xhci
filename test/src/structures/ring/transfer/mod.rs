
use super::CycleBit;
use crate::transition_helper::BoxWrapper;
use alloc::vec::Vec;
use trb::Link;
use x86_64::PhysAddr;
use xhci::ring::{trb, trb::transfer};

const SIZE_OF_RING: usize = 256;

pub(crate) struct Ring {
    raw: Raw,
}
impl Ring {
    pub(crate) fn new() -> Self {
        Self { raw: Raw::new() }
    }

    pub(crate) fn phys_addr(&self) -> PhysAddr {
        self.raw.phys_addr()
    }

    pub(crate) fn enqueue(&mut self, trbs: &[transfer::Allowed]) -> Vec<PhysAddr> {
        self.raw.enqueue_trbs(trbs)
    }
}

struct Raw {
    ring: BoxWrapper<[[u32; 4]]>,
    enq_p: usize,
    c: CycleBit,
}
impl Raw {
    fn new() -> Self {
        Self {
            ring: BoxWrapper::new_slice([0; 4], SIZE_OF_RING),
            enq_p: 0,
            c: CycleBit::new(true),
        }
    }

    fn enqueue_trbs(&mut self, trbs: &[transfer::Allowed]) -> Vec<PhysAddr> {
        trbs.iter().map(|t| self.enqueue(*t)).collect()
    }

    fn enqueue(&mut self, mut trb: transfer::Allowed) -> PhysAddr {
        self.set_cycle_bit(&mut trb);
        self.write_trb_on_memory(trb);
        let addr_to_trb = self.addr_to_enqueue_ptr();
        self.increment_enqueue_ptr();

        addr_to_trb
    }

    fn write_trb_on_memory(&mut self, trb: transfer::Allowed) {
        self.ring[self.enq_p] = trb.into_raw();
    }

    fn addr_to_enqueue_ptr(&self) -> PhysAddr {
        self.phys_addr() + trb::BYTES * self.enq_p
    }

    fn phys_addr(&self) -> PhysAddr {
        self.ring.phys_addr()
    }

    fn increment_enqueue_ptr(&mut self) {
        self.enq_p += 1;
        if self.enq_p < self.len() - 1 {
            return;
        }

        self.append_link_trb();
        self.move_enqueue_ptr_to_the_beginning();
    }

    fn len(&self) -> usize {
        self.ring.len()
    }

    fn append_link_trb(&mut self) {
        let t = *Link::default().set_ring_segment_pointer(self.phys_addr().as_u64());
        let mut t = transfer::Allowed::Link(t);
        self.set_cycle_bit(&mut t);
        self.ring[self.enq_p] = t.into_raw();
    }

    fn move_enqueue_ptr_to_the_beginning(&mut self) {
        self.enq_p = 0;
        self.c.toggle();
    }

    fn set_cycle_bit(&self, trb: &mut transfer::Allowed) {
        if self.c == CycleBit::new(true) {
            trb.set_cycle_bit();
        } else {
            trb.clear_cycle_bit();
        }
    }
}
