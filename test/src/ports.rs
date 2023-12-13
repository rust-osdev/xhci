use xhci::registers::PortRegisterSet;

use crate::registers::Registers;

pub fn init_all_ports(regs: &mut Registers) {
    let num_ports = num_ports(regs);

    for port in 0..num_ports {
        if connected(regs, port) {
            init_port(regs, port);
        }
    }
}

fn connected(regs: &Registers, port: u8) -> bool {
    regs.port_register_set
        .read_volatile_at(port.into())
        .portsc
        .current_connect_status()
}

fn init_port(regs: &mut Registers, port: u8) {
    Resetter::new(regs, port).reset();
}

fn num_ports(regs: &Registers) -> u8 {
    regs.capability.hcsparams1.read_volatile().number_of_ports()
}

struct Resetter<'a> {
    regs: PortRegisterHandler<'a>,
}
impl<'a> Resetter<'a> {
    fn new(regs: &'a mut Registers, port_number: u8) -> Self {
        Self {
            regs: PortRegisterHandler::new(regs, port_number),
        }
    }

    fn reset(mut self) {
        self.start_resetting();
        self.wait_utnil_reset_completed();
    }

    fn start_resetting(&mut self) {
        self.regs.update(|r| {
            r.portsc.set_port_reset();
        });
    }

    fn wait_utnil_reset_completed(&self) {
        while !self.reset_completed() {}
    }

    fn reset_completed(&self) -> bool {
        self.regs.read().portsc.port_reset_change()
    }
}

struct PortRegisterHandler<'a> {
    regs: &'a mut Registers,
    port_number: u8,
}
impl<'a> PortRegisterHandler<'a> {
    fn new(regs: &'a mut Registers, port_number: u8) -> Self {
        Self { regs, port_number }
    }

    fn read(&self) -> PortRegisterSet {
        self.regs
            .port_register_set
            .read_volatile_at(self.port_number.into())
    }

    fn update<T>(&mut self, f: T)
    where
        T: FnOnce(&mut PortRegisterSet),
    {
        self.regs
            .port_register_set
            .update_volatile_at(self.port_number.into(), f)
    }
}
