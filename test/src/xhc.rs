use crate::command_ring;
use crate::dcbaa;
use crate::event;
use crate::registers;
use crate::scratchpat;
use qemu_print::qemu_println;

/// Initializes the host controller according to 4.2 of xHCI spec.
///
/// Note that we do not enable interrupts as it is optional and for simplicity.
pub fn init() {
    qemu_println!("Initializing xHC...");

    wait_until_controller_is_ready();
    stop();
    reset();
    set_num_of_enabled_slots();

    event::init();
    command_ring::init();
    dcbaa::init();
    scratchpat::init();

    run();
    ensure_no_error_occurs();

    qemu_println!("xHC is initialized.");
}

fn run() {
    registers::handle(|r| {
        let op = &mut r.operational;

        op.usbcmd.update_volatile(|u| {
            u.set_run_stop();
        });

        while op.usbsts.read_volatile().hc_halted() {}
    });
}

fn ensure_no_error_occurs() {
    registers::handle(|r| {
        let s = r.operational.usbsts.read_volatile();

        assert!(!s.hc_halted(), "HC is halted.");
        assert!(
            !s.host_system_error(),
            "An error occured on the host system."
        );
        assert!(!s.host_controller_error(), "An error occured on the xHC.");
    })
}

fn wait_until_controller_is_ready() {
    registers::handle(
        |r| {
            while r.operational.usbsts.read_volatile().controller_not_ready() {}
        },
    );
}

fn stop() {
    Stopper::new().stop();
}

fn reset() {
    Resetter::new().reset();
}

fn set_num_of_enabled_slots() {
    SlotNumberSetter::new().set();
}

struct Stopper {}
impl Stopper {
    fn new() -> Self {
        Self {}
    }

    fn stop(&mut self) {
        registers::handle(|r| {
            let op = &mut r.operational;

            op.usbcmd.update_volatile(|u| {
                u.clear_run_stop();
            });

            while !op.usbsts.read_volatile().hc_halted() {}
        })
    }
}

struct Resetter {}
impl Resetter {
    fn new() -> Self {
        Self {}
    }

    fn reset(&mut self) {
        self.start_resetting();
        self.wait_until_reset_completed();
        self.wait_until_ready();
    }

    fn start_resetting(&mut self) {
        registers::handle(|r| {
            r.operational.usbcmd.update_volatile(|u| {
                u.set_host_controller_reset();
            });
        })
    }

    fn wait_until_reset_completed(&self) {
        registers::handle(
            |r| {
                while r.operational.usbcmd.read_volatile().host_controller_reset() {}
            },
        )
    }

    fn wait_until_ready(&self) {
        registers::handle(
            |r| {
                while r.operational.usbsts.read_volatile().controller_not_ready() {}
            },
        )
    }
}

struct SlotNumberSetter {}
impl SlotNumberSetter {
    fn new() -> Self {
        Self {}
    }

    fn set(&mut self) {
        let n = self.number_of_slots();

        registers::handle(|r| {
            r.operational.config.update_volatile(|c| {
                c.set_max_device_slots_enabled(n);
            });
        })
    }

    fn number_of_slots(&self) -> u8 {
        registers::handle(|r| {
            r.capability
                .hcsparams1
                .read_volatile()
                .number_of_device_slots()
        })
    }
}
