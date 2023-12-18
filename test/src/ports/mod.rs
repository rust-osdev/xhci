mod context;

use self::context::Context;
use crate::{
    command_ring::CommandRingController, dcbaa::DeviceContextBaseAddressArray, event::EventHandler,
    registers::Registers, transfer_ring::TransferRingController,
};
use alloc::rc::Rc;
use core::cell::RefCell;
use qemu_print::qemu_println;
use xhci::{context::EndpointType, registers::PortRegisterSet};

pub fn init_all_ports(
    regs: Rc<RefCell<Registers>>,
    event_handler: Rc<RefCell<EventHandler>>,
    cmd: Rc<RefCell<CommandRingController>>,
    dcbaa: Rc<RefCell<DeviceContextBaseAddressArray>>,
) {
    let num_ports = num_ports(&regs.borrow());

    for port in 0..num_ports {
        if connected(&regs.borrow(), port) {
            init_port(
                regs.clone(),
                event_handler.clone(),
                cmd.clone(),
                dcbaa.clone(),
                port,
            );
        }
    }
}

fn connected(regs: &Registers, port: u8) -> bool {
    regs.port_register_set
        .read_volatile_at(port.into())
        .portsc
        .current_connect_status()
}

fn init_port(
    regs: Rc<RefCell<Registers>>,
    event_handler: Rc<RefCell<EventHandler>>,
    cmd: Rc<RefCell<CommandRingController>>,
    dcbaa: Rc<RefCell<DeviceContextBaseAddressArray>>,
    port: u8,
) {
    Resetter::new(&mut regs.borrow_mut(), port).reset();

    let addr = cmd.borrow_mut().send_enable_slot();

    event_handler
        .clone()
        .borrow_mut()
        .register_handler(addr, move |c| {
            assert_eq!(
                c.completion_code(),
                Ok(xhci::ring::trb::event::CompletionCode::Success),
                "Enable slot failed."
            );

            qemu_println!("Slot enabled.");

            StructureInitializer::new(
                regs.clone(),
                port,
                c.slot_id(),
                dcbaa,
                cmd.clone(),
                event_handler.clone(),
                Context::new(&regs.borrow()),
            )
            .create()
        });
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

struct SlotEnabler {
    regs: Rc<RefCell<Registers>>,
    event_handler: Rc<RefCell<EventHandler>>,
    cmd: Rc<RefCell<CommandRingController>>,
}
impl SlotEnabler {
    fn new(
        regs: Rc<RefCell<Registers>>,
        event_handler: Rc<RefCell<EventHandler>>,
        cmd: Rc<RefCell<CommandRingController>>,
    ) -> Self {
        Self {
            regs,
            event_handler,
            cmd,
        }
    }

    fn enable(&mut self) -> u64 {
        self.cmd.borrow_mut().send_enable_slot()
    }
}

struct StructureInitializer {
    regs: Rc<RefCell<Registers>>,
    port: u8,
    slot: u8,
    dcbaa: Rc<RefCell<DeviceContextBaseAddressArray>>,
    cmd: Rc<RefCell<CommandRingController>>,
    event_handler: Rc<RefCell<EventHandler>>,
    ring: TransferRingController,
    cx: Context,
}
impl StructureInitializer {
    fn new(
        regs: Rc<RefCell<Registers>>,
        port: u8,
        slot: u8,
        dcbaa: Rc<RefCell<DeviceContextBaseAddressArray>>,
        cmd: Rc<RefCell<CommandRingController>>,
        event_handler: Rc<RefCell<EventHandler>>,
        cx: Context,
    ) -> Self {
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
            PortRegisterHandler::new(&mut self.regs.borrow_mut(), self.port),
        )
        .init();
    }

    fn register_with_dcbaa(&mut self) {
        self.dcbaa
            .borrow_mut()
            .register_address(self.slot, self.cx.input.phys_addr());
    }

    fn issue_address_device_command(&mut self) {
        self.cmd
            .borrow_mut()
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
