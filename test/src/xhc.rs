use crate::registers::Registers;
use qemu_print::qemu_println;

pub fn init(regs: &mut Registers) {
    qemu_println!("Initializing xHC...");

    stop_and_reset(regs);
    set_num_of_enabled_slots(regs);

    qemu_println!("xHC is initialized.");
}

pub fn run(regs: &mut Registers) {
    let o = &mut regs.operational;

    o.usbcmd.update_volatile(|u| {
        u.set_run_stop();
    });

    while o.usbsts.read_volatile().hc_halted() {}
}

pub fn ensure_no_error_occurs(regs: &Registers) {
    let s = regs.operational.usbsts.read_volatile();

    assert!(!s.hc_halted(), "HC is halted.");
    assert!(
        !s.host_system_error(),
        "An error occured on the host system."
    );
    assert!(!s.host_controller_error(), "An error occured on the xHC.");
}

fn stop_and_reset(regs: &mut Registers) {
    stop(regs);
    wait_until_halt(regs);
    reset(regs);
}

fn stop(regs: &mut Registers) {
    regs.operational.usbcmd.update_volatile(|u| {
        u.clear_run_stop();
    });
}

fn wait_until_halt(regs: &Registers) {
    while !regs.operational.usbsts.read_volatile().hc_halted() {}
}

fn reset(regs: &mut Registers) {
    start_resetting(regs);
    wait_until_reset_completed(regs);
    wait_until_ready(regs);
}

fn start_resetting(regs: &mut Registers) {
    regs.operational.usbcmd.update_volatile(|u| {
        u.set_host_controller_reset();
    });
}

fn wait_until_reset_completed(regs: &Registers) {
    while regs
        .operational
        .usbcmd
        .read_volatile()
        .host_controller_reset()
    {}
}

fn wait_until_ready(regs: &Registers) {
    while regs
        .operational
        .usbsts
        .read_volatile()
        .controller_not_ready()
    {}
}

fn set_num_of_enabled_slots(regs: &mut Registers) {
    let n = num_of_device_slots(regs);

    regs.operational.config.update_volatile(|c| {
        c.set_max_device_slots_enabled(n);
    });
}

fn num_of_device_slots(regs: &Registers) -> u8 {
    regs.capability
        .hcsparams1
        .read_volatile()
        .number_of_device_slots()
}
