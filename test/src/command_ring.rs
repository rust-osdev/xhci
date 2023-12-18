use core::cell::RefCell;

use crate::{event::EventHandler, registers::Registers};
use alloc::{boxed::Box, rc::Rc};
use xhci::ring::trb::{
    self, command,
    event::{CommandCompletion, CompletionCode},
};

const NUM_OF_TRBS_IN_RING: usize = 16;

pub struct CommandRingController {
    ring: Box<CommandRing>,

    regs: Rc<RefCell<Registers>>,

    enqueue_ptr: usize,
    cycle_bit: bool,
}
impl CommandRingController {
    pub fn new(regs: &Rc<RefCell<Registers>>) -> Self {
        let mut v = Self {
            ring: Box::new(CommandRing::new()),

            regs: Rc::clone(regs),

            enqueue_ptr: 0,
            cycle_bit: true,
        };

        v.init(&mut regs.borrow_mut());

        v
    }

    pub fn send_nop(&mut self, regs: &mut Registers, event_handler: &mut EventHandler) {
        let trb = command::Noop::new();
        let trb = command::Allowed::Noop(trb);

        let on_completion = |c: CommandCompletion| {
            assert_eq!(
                c.completion_code(),
                Ok(CompletionCode::Success),
                "No-op command failed: {:?}",
                c
            );
        };

        self.enqueue(regs, event_handler, trb, on_completion);
    }

    pub fn send_enable_slot(
        &mut self,
        regs: &mut Registers,
        event_handler: &mut EventHandler,
        after_enabling: impl Fn(u8) + 'static,
    ) {
        let trb = command::EnableSlot::new();
        let trb = command::Allowed::EnableSlot(trb);

        let on_completion = move |c: CommandCompletion| {
            assert_eq!(
                c.completion_code(),
                Ok(CompletionCode::Success),
                "Enable slot command failed: {:?}",
                c
            );

            after_enabling(c.slot_id());
        };

        self.enqueue(regs, event_handler, trb, on_completion);
    }

    pub fn send_address_device(
        &mut self,
        regs: &mut Registers,
        event_handler: &mut EventHandler,
        input_cx_addr: u64,
        slot: u8,
    ) {
        let trb = *command::AddressDevice::new()
            .set_input_context_pointer(input_cx_addr)
            .set_slot_id(slot);
        let trb = command::Allowed::AddressDevice(trb);

        let on_completion = |c: CommandCompletion| {
            assert_eq!(
                c.completion_code(),
                Ok(CompletionCode::Success),
                "Address device command failed: {:?}",
                c
            );
        };

        self.enqueue(regs, event_handler, trb, on_completion);
    }

    fn enqueue(
        &mut self,
        regs: &mut Registers,
        event_handler: &mut EventHandler,
        trb: command::Allowed,
        on_completion: impl Fn(CommandCompletion) + 'static,
    ) {
        Enqueuer::new(self, regs, event_handler).enqueue(trb, on_completion);
    }

    fn init(&mut self, regs: &mut Registers) {
        regs.operational.crcr.update_volatile(|crcr| {
            crcr.set_command_ring_pointer(self.ring.as_ref() as *const _ as u64);
            crcr.set_ring_cycle_state();
        });
    }
}

struct Enqueuer<'a> {
    controller: &'a mut CommandRingController,
    regs: &'a mut Registers,
    event_handler: &'a mut EventHandler,
}
impl<'a> Enqueuer<'a> {
    fn new(
        controller: &'a mut CommandRingController,
        regs: &'a mut Registers,
        event_handler: &'a mut EventHandler,
    ) -> Self {
        Self {
            controller,
            regs,
            event_handler,
        }
    }

    fn enqueue(
        &mut self,
        mut trb: command::Allowed,
        on_completion: impl Fn(CommandCompletion) + 'static,
    ) {
        self.modify_cycle_bit(&mut trb);
        self.write_trb(trb);
        self.register_handler(on_completion);
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

    fn register_handler(&mut self, on_completion: impl Fn(CommandCompletion) + 'static) {
        let trb_addr = self.written_trb_address();

        self.event_handler.register_handler(trb_addr, move |c| {
            on_completion(c);
        });
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
        self.regs.doorbell.update_volatile_at(0, |r| {
            r.set_doorbell_target(0);
        });
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
