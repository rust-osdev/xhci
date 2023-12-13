use crate::mapper::Mapper;
use crate::registers::Registers;
use qemu_print::qemu_println;
use xhci::registers::Operational;

pub fn init(regs: &mut Registers) {
    qemu_println!("Initializing xHC...");

    Initializer::new(regs).init();

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

struct Initializer<'a> {
    regs: &'a mut Registers,
}
impl<'a> Initializer<'a> {
    fn new(regs: &'a mut Registers) -> Self {
        Self { regs }
    }

    fn init(&mut self) {
        self.stop_and_reset();
        self.set_num_of_enabled_slots();
    }

    fn stop_and_reset(&mut self) {
        self.stop();
        self.wait_until_halt();
        self.reset();
    }

    fn stop(&mut self) {
        self.regs.operational.usbcmd.update_volatile(|u| {
            u.clear_run_stop();
        });
    }

    fn wait_until_halt(&mut self) {
        while !self.regs.operational.usbsts.read_volatile().hc_halted() {}
    }

    fn reset(&mut self) {
        Resetter::new(&mut self.regs.operational).reset();
    }

    fn set_num_of_enabled_slots(&mut self) {
        SlotNumberSetter::new(self.regs).set();
    }
}

struct Resetter<'a> {
    op: &'a mut Operational<Mapper>,
}
impl<'a> Resetter<'a> {
    fn new(op: &'a mut Operational<Mapper>) -> Self {
        Self { op }
    }

    fn reset(&mut self) {
        self.start_resetting();
        self.wait_until_reset_completed();
        self.wait_until_ready();
    }

    fn start_resetting(&mut self) {
        self.op.usbcmd.update_volatile(|u| {
            u.set_host_controller_reset();
        });
    }

    fn wait_until_reset_completed(&self) {
        while self.op.usbcmd.read_volatile().host_controller_reset() {}
    }

    fn wait_until_ready(&self) {
        while self.op.usbsts.read_volatile().controller_not_ready() {}
    }
}

struct SlotNumberSetter<'a> {
    regs: &'a mut Registers,
}
impl<'a> SlotNumberSetter<'a> {
    fn new(regs: &'a mut Registers) -> Self {
        Self { regs }
    }

    fn set(&mut self) {
        let n = self.number_of_slots();

        self.regs.operational.config.update_volatile(|c| {
            c.set_max_device_slots_enabled(n);
        });
    }

    fn number_of_slots(&self) -> u8 {
        self.regs
            .capability
            .hcsparams1
            .read_volatile()
            .number_of_device_slots()
    }
}
