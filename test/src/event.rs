use crate::registers;
use alloc::{vec, vec::Vec};
use bit_field::BitField;
use conquer_once::spin::OnceCell;
use core::ops::DerefMut;
use spinning_top::Spinlock;
use xhci::ring::trb::event;
use xhci::ring::trb::{self};

const NUM_OF_TRBS_IN_RING: usize = 16;

static EVENT_HANDLER: OnceCell<Spinlock<EventHandler>> = OnceCell::uninit();

pub fn init() {
    EVENT_HANDLER.init_once(|| Spinlock::new(EventHandler::new()));

    lock().init();
}

fn lock() -> impl DerefMut<Target = EventHandler> {
    EVENT_HANDLER
        .try_get()
        .expect("Event handler not initialized")
        .try_lock()
        .expect("Event handler is already locked")
}

struct EventHandler {
    segment_table: Vec<EventRingSegmentTableEntry>,
    rings: Vec<EventRing>,

    dequeue_ptr_segment: u64,
    dequeue_ptr_ring: u64,

    cycle_bit: bool,
}
impl EventHandler {
    fn new() -> Self {
        let number_of_rings = number_of_rings();

        Self {
            segment_table: vec![EventRingSegmentTableEntry::null(); number_of_rings.into()],
            rings: vec![EventRing::new(); number_of_rings.into()],

            dequeue_ptr_segment: 0,
            dequeue_ptr_ring: 0,

            cycle_bit: true,
        }
    }

    fn init(&mut self) {
        EventHandlerInitializer::new(self).init();
    }

    pub fn dequeue(&mut self) -> Option<Result<event::Allowed, [u32; 4]>> {
        if self.ring_is_empty() {
            return None;
        }

        let t = self.rings[self.dequeue_ptr_segment as usize].0[self.dequeue_ptr_ring as usize];
        let t = event::Allowed::try_from(t);

        self.increment_ptr();

        Some(t)
    }

    fn increment_ptr(&mut self) {
        self.dequeue_ptr_ring += 1;

        if self.dequeue_ptr_ring >= NUM_OF_TRBS_IN_RING as u64 {
            self.dequeue_ptr_ring = 0;
            self.dequeue_ptr_segment += 1;

            if self.dequeue_ptr_segment >= self.segment_table.len() as u64 {
                self.dequeue_ptr_segment = 0;
                self.cycle_bit = !self.cycle_bit;
            }
        }
    }

    fn ring_is_empty(&self) -> bool {
        self.cycle_bit_of_next_trb() != self.cycle_bit
    }

    fn cycle_bit_of_next_trb(&self) -> bool {
        let t = self.rings[self.dequeue_ptr_segment as usize].0[self.dequeue_ptr_ring as usize];

        t[3].get_bit(0)
    }

    fn next_trb_addr(&self) -> u64 {
        &self.rings[self.dequeue_ptr_segment as usize] as *const _ as u64
            + self.dequeue_ptr_ring as u64 * trb::BYTES as u64
    }
}

struct EventHandlerInitializer<'a> {
    handler: &'a mut EventHandler,
}
impl<'a> EventHandlerInitializer<'a> {
    fn new(handler: &'a mut EventHandler) -> Self {
        Self { handler }
    }

    fn init(&mut self) {
        self.register_dequeue_pointer();
        self.write_rings_addresses_in_table();
        self.disable_interrupts();
        self.register_table_size();
        self.enable_event_ring();
    }

    fn register_dequeue_pointer(&mut self) {
        registers::handle(|r| {
            r.interrupter_register_set
                .interrupter_mut(0)
                .erdp
                .update_volatile(|erdp| {
                    erdp.set_event_ring_dequeue_pointer(self.handler.next_trb_addr())
                })
        })
    }

    fn write_rings_addresses_in_table(&mut self) {
        let segment_table = &mut self.handler.segment_table;

        for (i, ring) in self.handler.rings.iter().enumerate() {
            segment_table[i].base_addr = ring as *const _ as u64;
            segment_table[i].segment_size = NUM_OF_TRBS_IN_RING as _;
        }
    }

    // We use polling for simplicity.
    fn disable_interrupts(&mut self) {
        registers::handle(|r| {
            r.interrupter_register_set
                .interrupter_mut(0)
                .iman
                .update_volatile(|iman| {
                    iman.clear_interrupt_enable();
                })
        })
    }

    fn register_table_size(&mut self) {
        registers::handle(|r| {
            r.interrupter_register_set
                .interrupter_mut(0)
                .erstsz
                .update_volatile(|erstsz| {
                    erstsz.set(self.handler.segment_table.len() as u16);
                })
        })
    }

    fn enable_event_ring(&mut self) {
        registers::handle(|r| {
            r.interrupter_register_set
                .interrupter_mut(0)
                .erstba
                .update_volatile(|erstba| erstba.set(self.handler.segment_table.as_ptr() as u64))
        })
    }
}

#[repr(C, align(64))]
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

fn number_of_rings() -> u16 {
    registers::handle(|r| {
        r.capability
            .hcsparams2
            .read_volatile()
            .event_ring_segment_table_max()
    })
}
