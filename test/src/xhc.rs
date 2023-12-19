
use super::structures::{extended_capabilities, registers};
use xhci::extended_capabilities::ExtendedCapability;

pub(super) fn exists() -> bool {
    super::iter_xhc().next().is_some()
}

pub(crate) fn init() {
    get_ownership_from_bios();
    stop_and_reset();
    set_num_of_enabled_slots();
}

pub(crate) fn run() {
    registers::handle(|r| {
        let o = &mut r.operational;
        o.usbcmd.update_volatile(|u| {
            u.set_run_stop();
        });
        while o.usbsts.read_volatile().hc_halted() {}
    });
}

pub(crate) fn ensure_no_error_occurs() {
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

fn get_ownership_from_bios() {
    if let Some(iter) = extended_capabilities::iter() {
        for c in iter.filter_map(Result::ok) {
            if let ExtendedCapability::UsbLegacySupport(mut u) = c {
                let l = &mut u.usblegsup;
                l.update_volatile(|s| {
                    s.set_hc_os_owned_semaphore();
                });

                while l.read_volatile().hc_bios_owned_semaphore()
                    || !l.read_volatile().hc_os_owned_semaphore()
                {}
            }
        }
    }
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
    })
}

fn wait_until_halt() {
    registers::handle(|r| while !r.operational.usbsts.read_volatile().hc_halted() {})
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
        })
    })
}

fn wait_until_reset_completed() {
    registers::handle(
        |r| {
            while r.operational.usbcmd.read_volatile().host_controller_reset() {}
        },
    )
}

fn wait_until_ready() {
    registers::handle(
        |r| {
            while r.operational.usbsts.read_volatile().controller_not_ready() {}
        },
    )
}

fn set_num_of_enabled_slots() {
    let n = num_of_device_slots();
    registers::handle(|r| {
        r.operational.config.update_volatile(|c| {
            c.set_max_device_slots_enabled(n);
        });
    })
}

fn num_of_device_slots() -> u8 {
    registers::handle(|r| {
        r.capability
            .hcsparams1
            .read_volatile()
            .number_of_device_slots()
    })
}
