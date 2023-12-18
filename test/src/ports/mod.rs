mod context;

use self::context::Context;
use crate::{
    command_ring::CommandRingController, dcbaa::DeviceContextBaseAddressArray, event::EventHandler,
    registers::Registers, transfer_ring::TransferRingController,
};
use qemu_print::qemu_println;
use xhci::{context::EndpointType, registers::PortRegisterSet};

pub fn init_all_ports(
    regs: &mut Registers,
    event_handler: &mut EventHandler,
    cmd: &mut CommandRingController,
) {
    let num_ports = num_ports(regs);

    for port in 0..num_ports {
        if connected(regs, port) {
            init_port(regs, event_handler, cmd, port);
        }
    }
}

fn connected(regs: &Registers, port: u8) -> bool {
    regs.port_register_set
        .read_volatile_at(port.into())
        .portsc
        .current_connect_status()
}

fn init_port(regs: &mut Registers, _: &mut EventHandler, _: &mut CommandRingController, port: u8) {
    Resetter::new(regs, port).reset();
    // SlotEnabler::new(regs, event_handler, cmd).enable(move |slot| {
    //     qemu_println!("Slot {} enabled", slot);

    //     StructureCreator::new(
    //         regs,
    //         port,
    //         slot,
    //         &mut DeviceContextBaseAddressArray::new(regs),
    //         cmd,
    //         event_handler,
    //     )
    //     .create();
    // });
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

struct SlotEnabler<'a> {
    regs: &'a mut Registers,
    event_handler: &'a mut EventHandler,
    cmd: &'a mut CommandRingController,
}
impl<'a> SlotEnabler<'a> {
    fn new(
        regs: &'a mut Registers,
        event_handler: &'a mut EventHandler,
        cmd: &'a mut CommandRingController,
    ) -> Self {
        Self {
            regs,
            event_handler,
            cmd,
        }
    }

    fn enable(&mut self, on_completion: impl Fn(u8) + 'static) {
        self.cmd
            .send_enable_slot(self.event_handler, move |port_id| {
                qemu_println!("Port {} enabled", port_id);

                on_completion(port_id);
            });
    }
}

struct StructureCreator<'a> {
    regs: &'a mut Registers,
    port: u8,
    slot: u8,
    dcbaa: &'a mut DeviceContextBaseAddressArray,
    cmd: &'a mut CommandRingController,
    event_handler: &'a mut EventHandler,
    ring: TransferRingController,
    cx: Context,
}
impl<'a> StructureCreator<'a> {
    fn new(
        regs: &'a mut Registers,
        port: u8,
        slot: u8,
        dcbaa: &'a mut DeviceContextBaseAddressArray,
        cmd: &'a mut CommandRingController,
        event_handler: &'a mut EventHandler,
    ) -> Self {
        let cx = Context::new(regs);

        Self {
            regs,
            port,
            slot,
            dcbaa,
            cmd,
            event_handler,
            ring: TransferRingController::new(),
            cx,
        }
    }

    fn create(&mut self) {
        self.init_input_context();
        self.init_ep0_context();
        self.register_with_dcbaa();
        self.issue_address_device_command();
    }

    fn init_input_context(&mut self) {
        InputContextInitializer::new(&mut self.cx, self.port).init();
    }

    fn init_ep0_context(&mut self) {
        Ep0ContextInitializer::new(
            &mut self.cx,
            self.port,
            &self.ring,
            PortRegisterHandler::new(self.regs, self.port),
        )
        .init();
    }

    fn register_with_dcbaa(&mut self) {
        self.dcbaa
            .register_address(self.slot, self.cx.input.phys_addr());
    }

    fn issue_address_device_command(&mut self) {
        self.cmd
            .send_address_device(self.cx.input.phys_addr(), self.slot);
    }
}

struct InputContextInitializer<'a> {
    cx: &'a mut Context,
    port: u8,
}
impl<'a> InputContextInitializer<'a> {
    fn new(cx: &'a mut Context, port: u8) -> Self {
        Self { cx, port }
    }

    fn init(&mut self) {
        self.init_input_control();
        self.init_input_slot();
    }

    fn init_input_control(&mut self) {
        let c = self.cx.input.control_mut();

        c.set_add_context_flag(0);
        c.set_add_context_flag(1);
    }

    fn init_input_slot(&mut self) {
        let s = self.cx.input.device_mut().slot_mut();

        s.set_context_entries(1);
        s.set_root_hub_port_number(self.port);
    }
}

struct Ep0ContextInitializer<'a> {
    cx: &'a mut Context,
    port: u8,
    ring: &'a TransferRingController,
    regs: PortRegisterHandler<'a>,
}
impl<'a> Ep0ContextInitializer<'a> {
    fn new(
        cx: &'a mut Context,
        port: u8,
        ring: &'a TransferRingController,
        regs: PortRegisterHandler<'a>,
    ) -> Self {
        Self {
            cx,
            port,
            ring,
            regs,
        }
    }

    fn init(self) {
        let s = self.get_max_packet_size();
        let ep_0 = self.cx.input.device_mut().endpoint_mut(1);

        ep_0.set_endpoint_type(EndpointType::Control);
        ep_0.set_max_packet_size(s);
        ep_0.set_tr_dequeue_pointer(self.ring.head_addr());
        ep_0.set_dequeue_cycle_state();
        ep_0.set_error_count(3);
    }

    // TODO: Read the actual speed from xHCI supported protocol capability.
    fn get_max_packet_size(&self) -> u16 {
        match self.regs.read().portsc.port_speed() {
            1 | 3 => 64,
            2 => 8,
            4 => 512,
            x => todo!("PSI: {}", x),
        }
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
