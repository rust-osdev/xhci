use super::CycleBit;
use crate::{page_box::PageBox, registers};
use trb::Link;
use x86_64::{
    structures::paging::{PageSize, Size4KiB},
    PhysAddr,
};
use xhci::ring::{trb, trb::command};

#[allow(clippy::cast_possible_truncation)]
const NUM_OF_TRBS: usize = Size4KiB::SIZE as usize / trb::BYTES;

pub(crate) struct Ring {
    raw: Raw,
}
impl Ring {
    pub(crate) fn new() -> Self {
        Self { raw: Raw::new() }
    }

    pub(crate) fn init(&mut self) {
        Initializer::new(self).init();
    }

    pub(crate) fn enqueue(&mut self, trb: command::Allowed) -> PhysAddr {
        let a = self.raw.enqueue(trb);
        Self::notify_command_is_sent();
        a
    }

    fn phys_addr(&self) -> PhysAddr {
        self.raw.head_addr()
    }

    fn notify_command_is_sent() {
        registers::handle(|r| {
            r.doorbell.update_volatile_at(0, |r| {
                r.set_doorbell_target(0);
            });
        })
    }
}
impl Default for Ring {
    fn default() -> Self {
        Self::new()
    }
}

struct Raw {
    raw: PageBox<[[u32; 4]]>,
    enq_p: usize,
    c: CycleBit,
}
impl Raw {
    fn new() -> Self {
        Self {
            raw: PageBox::new_slice([0; 4], NUM_OF_TRBS),
            enq_p: 0,
            c: CycleBit::new(true),
        }
    }

    fn enqueue(&mut self, mut trb: command::Allowed) -> PhysAddr {
        self.set_cycle_bit(&mut trb);
        self.write_trb(trb);
        let trb_a = self.enq_addr();
        self.increment();
        trb_a
    }

    fn write_trb(&mut self, trb: command::Allowed) {
        // TODO: Write four 32-bit values. This way of writing is described in the spec, although
        // I cannot find which section has the description.
        self.raw[self.enq_p] = trb.into_raw();
    }

    fn increment(&mut self) {
        self.enq_p += 1;
        if !self.enq_p_within_ring() {
            self.enq_link();
            self.move_enq_p_to_the_beginning();
        }
    }

    fn enq_p_within_ring(&self) -> bool {
        self.enq_p < self.len() - 1
    }

    fn enq_link(&mut self) {
        // Don't call `enqueue`. It will return an `Err` value as there is no space for link TRB.
        let t = *Link::default().set_ring_segment_pointer(self.head_addr().as_u64());
        let mut t = command::Allowed::Link(t);
        self.set_cycle_bit(&mut t);
        self.raw[self.enq_p] = t.into_raw();
    }

    fn move_enq_p_to_the_beginning(&mut self) {
        self.enq_p = 0;
        self.c.toggle();
    }

    fn enq_addr(&self) -> PhysAddr {
        self.head_addr() + trb::BYTES * self.enq_p
    }

    fn head_addr(&self) -> PhysAddr {
        self.raw.phys_addr()
    }

    fn len(&self) -> usize {
        self.raw.len()
    }

    fn set_cycle_bit(&self, trb: &mut command::Allowed) {
        if self.c == CycleBit::new(true) {
            trb.set_cycle_bit();
        } else {
            trb.clear_cycle_bit();
        }
    }
}

struct Initializer<'a> {
    ring: &'a Ring,
}
impl<'a> Initializer<'a> {
    fn new(ring: &'a Ring) -> Self {
        Self { ring }
    }

    fn init(&mut self) {
        registers::handle(|r| {
            let a = self.ring.phys_addr();

            // Do not split this closure to avoid read-modify-write bug. Reading fields may return
            // 0, this will cause writing 0 to fields.
            r.operational.crcr.update_volatile(|c| {
                c.set_command_ring_pointer(a.as_u64());
                c.set_ring_cycle_state();
            });
        })
    }
}
