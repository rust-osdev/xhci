use crate::registers::Registers;
use alloc::boxed::Box;

const NUM_OF_TRBS_IN_RING: usize = 16;

pub struct CommandRingController {
    ring: Box<CommandRing>,

    enqueue_ptr: usize,
    cycle_bit: bool,
}
impl CommandRingController {
    pub fn new(regs: &mut Registers) -> Self {
        let mut v = Self {
            ring: Box::new(CommandRing::new()),

            enqueue_ptr: 0,
            cycle_bit: true,
        };

        v.init(regs);

        v
    }

    fn init(&mut self, regs: &mut Registers) {
        regs.operational.crcr.update_volatile(|crcr| {
            crcr.set_command_ring_pointer(self.ring.as_ref() as *const _ as u64);
            crcr.set_ring_cycle_state();
        });
    }
}

#[repr(C, align(64))]
struct CommandRing([[u32; 4]; NUM_OF_TRBS_IN_RING]);
impl CommandRing {
    fn new() -> Self {
        Self([[0; 4]; NUM_OF_TRBS_IN_RING])
    }
}
