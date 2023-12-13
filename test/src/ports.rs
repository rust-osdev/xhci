use crate::registers::Registers;

pub fn init_all_ports(regs: &Registers) {
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

fn init_port(regs: &Registers, port: u8) {
    todo!()
}

fn num_ports(regs: &Registers) -> u8 {
    regs.capability.hcsparams1.read_volatile().number_of_ports()
}
