use crate::registers;
use qemu_print::qemu_println;

pub fn init() {
    qemu_println!("Initializing xHC...");

    stop_and_reset();
    set_num_of_enabled_slots();

    qemu_println!("xHC is initialized.");
}

pub fn run() {
    registers::handle(|r| {
        let o = &mut r.operational;

        o.usbcmd.update_volatile(|u| {
            u.set_run_stop();
        });

        while o.usbsts.read_volatile().hc_halted() {}
    });
}

pub fn ensure_no_error_occurs() {
    registers::handle(|r| {
        let s = r.operational.usbsts.read_volatile();

        assert!(!s.hc_halted(), "HC is halted.");
        assert!(
            !s.host_system_error(),
            "An error occured on the host system."
        );
        assert!(!s.host_controller_error(), "An error occured on the xHC.");
    });
}

fn stop_and_reset() {
    stop();
    wait_until_halt();
    reset();
}

fn stop() {
    registers::handle(|r| {
        r.operational.usbcmd.update_volatile(|u| {
            u.clear_run_stop();
        });
    });
}

fn wait_until_halt() {
    registers::handle(|r| while !r.operational.usbsts.read_volatile().hc_halted() {});
}

fn reset() {
    start_resetting();
    wait_until_reset_completed();
    wait_until_ready();
}

fn start_resetting() {
    registers::handle(|r| {
        r.operational.usbcmd.update_volatile(|u| {
            u.set_host_controller_reset();
        });
    });
}

fn wait_until_reset_completed() {
    registers::handle(
        |r| {
            while r.operational.usbcmd.read_volatile().host_controller_reset() {}
        },
    );
}

fn wait_until_ready() {
    registers::handle(
        |r| {
            while r.operational.usbsts.read_volatile().controller_not_ready() {}
        },
    );
}

fn set_num_of_enabled_slots() {
    let n = num_of_device_slots();
    registers::handle(|r| {
        r.operational.config.update_volatile(|c| {
            c.set_max_device_slots_enabled(n);
        });
    });
}

fn num_of_device_slots() -> u8 {
    registers::handle(|r| {
        r.capability
            .hcsparams1
            .read_volatile()
            .number_of_device_slots()
    })
}
