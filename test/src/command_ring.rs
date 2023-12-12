use crate::registers;
use alloc::boxed::Box;
use conquer_once::spin::OnceCell;
use qemu_print::qemu_println;
use spinning_top::Spinlock;

static COMMAND_RING_CONTROLLER: OnceCell<Spinlock<CommandRingController>> = OnceCell::uninit();

const NUM_OF_TRBS_IN_RING: usize = 10;

pub fn init() {
    let controller = CommandRingController::new();

    COMMAND_RING_CONTROLLER
        .try_init_once(|| Spinlock::new(controller))
        .expect("CommandRingController::new() called more than once");

    COMMAND_RING_CONTROLLER
        .get()
        .unwrap_or_else(|| unreachable!("Should be initialized"))
        .lock()
        .init();

    qemu_println!("Command ring is initialized");
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

    fn init(&mut self) {
        registers::handle(|r| {
            r.operational.crcr.update_volatile(|crcr| {
                crcr.set_command_ring_pointer(self.ring.as_ref() as *const _ as u64);
                crcr.set_ring_cycle_state();
            });
        })
    }
}

#[repr(C, align(64))]
struct CommandRing([[u32; 4]; NUM_OF_TRBS_IN_RING]);
impl CommandRing {
    fn new() -> Self {
        Self([[0; 4]; NUM_OF_TRBS_IN_RING])
    }
}
