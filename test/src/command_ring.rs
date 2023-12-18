use crate::registers;
use alloc::boxed::Box;
use conquer_once::spin::OnceCell;
use core::ops::DerefMut;
use spinning_top::Spinlock;
use xhci::ring::trb::{self, command};

const NUM_OF_TRBS_IN_RING: usize = 16;

static COMMAND_RING_CONTROLLER: OnceCell<Spinlock<CommandRingController>> = OnceCell::uninit();

pub fn init() {
    COMMAND_RING_CONTROLLER.init_once(|| Spinlock::new(CommandRingController::new()));

    lock().init();
}

pub fn send_nop() -> u64 {
    lock().send_nop()
}

pub fn send_enable_slot() -> u64 {
    lock().send_enable_slot()
}

pub fn send_address_device(input_cx_addr: u64, slot: u8) -> u64 {
    lock().send_address_device(input_cx_addr, slot)
}

fn lock() -> impl DerefMut<Target = CommandRingController> {
    COMMAND_RING_CONTROLLER
        .try_get()
        .expect("Command ring controller not initialized")
        .try_lock()
        .expect("Command ring controller is already locked")
}

struct CommandRingController {
    ring: Box<CommandRing>,

    enqueue_ptr: usize,
    cycle_bit: bool,
}
impl CommandRingController {
    fn new() -> Self {
        Self {
            ring: Box::new(CommandRing::new()),

            enqueue_ptr: 0,
            cycle_bit: true,
        }
    }

    pub fn send_nop(&mut self) -> u64 {
        let trb = command::Noop::new();
        let trb = command::Allowed::Noop(trb);

        self.enqueue(trb)
    }

    pub fn send_enable_slot(&mut self) -> u64 {
        let trb = command::EnableSlot::new();
        let trb = command::Allowed::EnableSlot(trb);

        self.enqueue(trb)
    }

    pub fn send_address_device(&mut self, input_cx_addr: u64, slot: u8) -> u64 {
        let trb = *command::AddressDevice::new()
            .set_input_context_pointer(input_cx_addr)
            .set_slot_id(slot);
        let trb = command::Allowed::AddressDevice(trb);

        self.enqueue(trb)
    }

    fn enqueue<'a>(&'a mut self, trb: command::Allowed) -> u64 {
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

    fn enqueue(&mut self, mut trb: command::Allowed) -> u64 {
        let addr = self.written_trb_address();

        self.modify_cycle_bit(&mut trb);
        self.write_trb(trb);
        self.increment_enqueue_ptr();
        self.notify_command_is_sent();

        addr
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
