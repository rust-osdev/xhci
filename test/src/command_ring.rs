use crate::ports;
use crate::registers;
use alloc::{boxed::Box, vec::Vec};
use conquer_once::spin::OnceCell;
use core::ops::DerefMut;
use spinning_top::Spinlock;
use xhci::ring::trb::{
    self, command,
    event::{CommandCompletion, CompletionCode},
};

const NUM_OF_TRBS_IN_RING: usize = 16;

static COMMAND_RING_CONTROLLER: OnceCell<Spinlock<CommandRingController>> = OnceCell::uninit();

pub fn init() {
    COMMAND_RING_CONTROLLER.init_once(|| Spinlock::new(CommandRingController::new()));

    try_lock()
        .expect("Failed to lock command ring controller")
        .init();
}

pub fn assert_all_trbs_are_processed() {
    try_lock()
        .expect("Failed to lock command ring controller")
        .assert_all_trbs_are_processed()
}

pub fn process_trb(event_trb: &CommandCompletion) {
    // Just pop a TRB, not to process it within the controller to prevent a
    // deadlock.
    let trb = try_lock()
        .expect("Failed to lock command ring controller")
        .pop_corresponding_trb(event_trb);

    match trb {
        command::Allowed::Noop(_) => {}
        command::Allowed::EnableSlot(_) => {
            ports::init_structures(event_trb.slot_id());
        }
        command::Allowed::AddressDevice(_) => {
            ports::set_max_packet_size(event_trb.slot_id());
        }
        _ => panic!("Unexpected command TRB: {:?}", trb),
    }
}

pub fn send_nop() {
    try_lock()
        .expect("Failed to lock command ring controller")
        .send_nop()
}

pub fn send_enable_slot() {
    try_lock()
        .expect("Failed to lock command ring controller")
        .send_enable_slot()
}

pub fn send_address_device(input_cx_addr: u64, slot: u8) {
    try_lock()
        .expect("Failed to lock command ring controller")
        .send_address_device(input_cx_addr, slot)
}

fn try_lock() -> Option<impl DerefMut<Target = CommandRingController>> {
    COMMAND_RING_CONTROLLER
        .try_get()
        .expect("Command ring controller not initialized")
        .try_lock()
}

struct CommandRingController {
    ring: Box<CommandRing>,

    sent_trbs: Vec<(u64, command::Allowed)>,

    enqueue_ptr: usize,
    cycle_bit: bool,
}
impl CommandRingController {
    fn new() -> Self {
        Self {
            ring: Box::new(CommandRing::new()),

            sent_trbs: Vec::new(),

            enqueue_ptr: 0,
            cycle_bit: true,
        }
    }

    fn assert_all_trbs_are_processed(&self) {
        assert!(self.sent_trbs.is_empty(), "Some TRBs are not processed");
    }

    fn pop_corresponding_trb(&mut self, event_trb: &CommandCompletion) -> command::Allowed {
        let command_trb_idx = self
            .sent_trbs
            .iter()
            .position(|(addr, _)| *addr == event_trb.command_trb_pointer())
            .expect("Command TRB not found");

        let (_, command_trb) = self.sent_trbs.remove(command_trb_idx);

        assert_eq!(
            event_trb.completion_code(),
            Ok(CompletionCode::Success),
            "Event TRB in response to a command TRB is not successful: {:?}",
            command_trb
        );

        command_trb
    }

    fn send_nop(&mut self) {
        let trb = command::Noop::new();
        let trb = command::Allowed::Noop(trb);

        self.enqueue(trb)
    }

    fn send_enable_slot(&mut self) {
        let trb = command::EnableSlot::new();
        let trb = command::Allowed::EnableSlot(trb);

        self.enqueue(trb)
    }

    fn send_address_device(&mut self, input_cx_addr: u64, slot: u8) {
        let trb = *command::AddressDevice::new()
            .set_input_context_pointer(input_cx_addr)
            .set_slot_id(slot);
        let trb = command::Allowed::AddressDevice(trb);

        self.enqueue(trb)
    }

    fn enqueue(&mut self, trb: command::Allowed) {
        Enqueuer::new(self).enqueue(trb)
    }

    fn init(&mut self) {
        registers::handle(|r| {
            r.operational.crcr.update_volatile(|crcr| {
                crcr.set_command_ring_pointer(self.ring.as_ref() as *const _ as u64);
                crcr.set_ring_cycle_state();
            });
        })
    }
}

struct Enqueuer<'a> {
    controller: &'a mut CommandRingController,
}
impl<'a> Enqueuer<'a> {
    fn new(controller: &'a mut CommandRingController) -> Self {
        Self { controller }
    }

    fn enqueue(&mut self, mut trb: command::Allowed) {
        self.modify_cycle_bit(&mut trb);
        self.write_trb(trb);

        let a = self.written_trb_address();

        self.store_trb(a, trb);

        self.increment_enqueue_ptr();
        self.notify_command_is_sent();
    }

    fn enqueue_link(&mut self) {
        let link_trb = *trb::Link::new().set_ring_segment_pointer(self.ring_head_address());
        let mut link_trb = command::Allowed::Link(link_trb);

        self.modify_cycle_bit(&mut link_trb);
        self.write_trb(link_trb);
        self.move_enqueue_ptr_to_head();
    }

    fn modify_cycle_bit(&mut self, trb: &mut command::Allowed) {
        if self.controller.cycle_bit {
            trb.set_cycle_bit();
        } else {
            trb.clear_cycle_bit();
        }
    }

    fn write_trb(&mut self, trb: command::Allowed) {
        self.controller.ring.0[self.controller.enqueue_ptr] = trb.into_raw();
    }

    fn written_trb_address(&self) -> u64 {
        &self.controller.ring.0[self.controller.enqueue_ptr] as *const _ as u64
    }

    fn increment_enqueue_ptr(&mut self) {
        self.controller.enqueue_ptr += 1;

        if !self.can_enqueue_trbs() {
            self.enqueue_link();
        }
    }

    fn notify_command_is_sent(&mut self) {
        registers::handle(|r| {
            r.doorbell.update_volatile_at(0, |r| {
                r.set_doorbell_target(0);
            });
        })
    }

    fn store_trb(&mut self, addr: u64, trb: command::Allowed) {
        self.controller.sent_trbs.push((addr, trb));
    }

    fn can_enqueue_trbs(&self) -> bool {
        // -1 for the space for a link TRB.
        self.controller.enqueue_ptr < NUM_OF_TRBS_IN_RING - 1
    }

    fn move_enqueue_ptr_to_head(&mut self) {
        self.controller.enqueue_ptr = 0;
        self.controller.cycle_bit = !self.controller.cycle_bit;
    }

    fn ring_head_address(&self) -> u64 {
        self.controller.ring.as_ref() as *const _ as u64
    }
}

#[repr(C, align(64))]
struct CommandRing([[u32; 4]; NUM_OF_TRBS_IN_RING]);
impl CommandRing {
    fn new() -> Self {
        Self([[0; 4]; NUM_OF_TRBS_IN_RING])
    }
}
