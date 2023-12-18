use crate::registers::Registers;
use alloc::boxed::Box;
use alloc::{vec, vec::Vec};
use bit_field::BitField;
use xhci::ring::trb::event;
use xhci::ring::trb::{self, event::CommandCompletion};

const NUM_OF_TRBS_IN_RING: usize = 16;

pub struct EventHandler {
    segment_table: Vec<EventRingSegmentTableEntry>,
    rings: Vec<EventRing>,

    // Alas, we cannot use `HashMap` because it's not in `alloc` yet.
    // See https://github.com/rust-lang/rust/issues/27242.
    handlers: Vec<(u64, Box<dyn Fn(CommandCompletion) + 'static>)>,

    dequeue_ptr_segment: u64,
    dequeue_ptr_ring: u64,

    cycle_bit: bool,
}
impl EventHandler {
    pub fn new(regs: &mut Registers) -> Self {
        let number_of_rings = number_of_rings(regs);

        let mut v = Self {
            segment_table: vec![EventRingSegmentTableEntry::null(); number_of_rings.into()],
            rings: vec![EventRing::new(); number_of_rings.into()],
            handlers: Vec::new(),

            dequeue_ptr_segment: 0,
            dequeue_ptr_ring: 0,

            cycle_bit: true,
        };

        v.init(regs);

        v
    }

    pub fn register_handler<'a>(
        &mut self,
        trb_addr: u64,
        handler: impl Fn(CommandCompletion) + 'static,
    ) {
        self.handlers.push((trb_addr, Box::new(handler)));
    }

    pub fn process_trbs(&mut self) {
        while !self.ring_is_empty() {
            self.dequeue_and_process();
        }
    }

    pub fn assert_all_commands_completed(&self) {
        assert!(self.handlers.is_empty(), "Some commands are not completed");
    }

    fn init(&mut self, regs: &mut Registers) {
        EventHandlerInitializer::new(self, regs).init();
    }

    fn dequeue_and_process(&mut self) {
        assert!(!self.ring_is_empty());

        let t = self.rings[self.dequeue_ptr_segment as usize].0[self.dequeue_ptr_ring as usize];
        let t = event::Allowed::try_from(t);

        if let Ok(event::Allowed::CommandCompletion(t)) = t {
            let idx = self
                .handlers
                .iter()
                .position(|(trb_addr, _)| *trb_addr == t.command_trb_pointer())
                .unwrap_or_else(|| panic!("No handler for {:?}", t));

            let (_, handler) = self.handlers.remove(idx);

            handler(t);
        }

        self.increment_ptr();
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
    regs: &'a mut Registers,
}
impl<'a> EventHandlerInitializer<'a> {
    fn new(handler: &'a mut EventHandler, regs: &'a mut Registers) -> Self {
        Self { handler, regs }
    }

    fn init(&mut self) {
        self.register_dequeue_pointer();
        self.write_rings_addresses_in_table();
        self.register_table_size();
        self.enable_event_ring();
    }

    fn register_dequeue_pointer(&mut self) {
        self.regs
            .interrupter_register_set
            .interrupter_mut(0)
            .erdp
            .update_volatile(|erdp| {
                erdp.set_event_ring_dequeue_pointer(self.handler.next_trb_addr())
            })
    }

    fn write_rings_addresses_in_table(&mut self) {
        let mut segment_table = &mut self.handler.segment_table;

        for (i, ring) in self.handler.rings.iter().enumerate() {
            segment_table[i].base_addr = ring as *const _ as u64;
            segment_table[i].segment_size = NUM_OF_TRBS_IN_RING as _;
        }
    }

    fn register_table_size(&mut self) {
        self.regs
            .interrupter_register_set
            .interrupter_mut(0)
            .erstsz
            .update_volatile(|erstsz| {
                erstsz.set(self.handler.segment_table.len() as u16);
            })
    }

    fn enable_event_ring(&mut self) {
        self.regs
            .interrupter_register_set
            .interrupter_mut(0)
            .erstba
            .update_volatile(|erstba| erstba.set(self.handler.segment_table.as_ptr() as u64))
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

fn number_of_rings(regs: &Registers) -> u16 {
    regs.capability
        .hcsparams2
        .read_volatile()
        .event_ring_segment_table_max()
}
