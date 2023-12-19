mod context;

use self::context::Context;
use crate::dcbaa;
use crate::{command_ring, registers, transfer_ring::TransferRingController};
use alloc::vec::Vec;
use core::ops::DerefMut;
use spinning_top::Spinlock;
use xhci::ring::trb::event::PortStatusChange;
use xhci::{context::EndpointType, registers::PortRegisterSet};

static ENABLING_SLOTS: Spinlock<Vec<u8>> = Spinlock::new(Vec::new());
static PORTS_AND_SLOTS: Spinlock<Vec<(u8, u8)>> = Spinlock::new(Vec::new());
static CONTEXTS: Spinlock<Vec<Context>> = Spinlock::new(Vec::new());

pub fn init_all_ports() {
    let num_ports = num_ports();

    for port in 0..num_ports {
        if connected(port) {
            reset_port(port);
            command_ring::send_enable_slot();

            lock_enabling_slots().push(port);
        }
    }
}

pub fn process_trb(trb: &PortStatusChange) {
    PortRegisterHandler::new(trb.port_id()).update(|r| {
        if r.portsc.current_connect_status() {
            todo!()
        }
    });
}

pub fn init_structures(slot: u8) {
    let port = lock_enabling_slots().pop().expect("No enabling slots");

    PORTS_AND_SLOTS
        .try_lock()
        .expect("Ports and slots is already locked")
        .push((port, slot));

    init_structures_for_port(port, slot);
}

pub fn init_structures_for_port(port: u8, slot: u8) {
    let mut cx = Context::new();

    StructureInitializer::new(port, slot, cx).create();
}

pub fn set_max_packet_size(slot: u8) {
    let port = PORTS_AND_SLOTS
        .try_lock()
        .expect("Ports and slots is already locked")
        .iter()
        .find(|(_, s)| *s == slot)
        .expect("No port for the slot")
        .0;
}

fn lock_enabling_slots() -> impl DerefMut<Target = Vec<u8>> {
    ENABLING_SLOTS
        .try_lock()
        .expect("Enabling slots is already locked")
}

fn connected(port: u8) -> bool {
    registers::handle(|r| {
        r.port_register_set
            .read_volatile_at(port.into())
            .portsc
            .current_connect_status()
    })
}

fn reset_port(port: u8) {
    Resetter::new(port).reset();
}

fn num_ports() -> u8 {
    registers::handle(|r| r.capability.hcsparams1.read_volatile().number_of_ports())
}

struct Resetter {
    regs: PortRegisterHandler,
}
impl Resetter {
    fn new(port_number: u8) -> Self {
        Self {
            regs: PortRegisterHandler::new(port_number),
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

struct SlotEnabler {}
impl SlotEnabler {
    fn new() -> Self {
        Self {}
    }

    fn enable(&mut self) {
        command_ring::send_enable_slot();
    }
}

struct StructureInitializer {
    port: u8,
    slot: u8,
    ring: TransferRingController,
    cx: Context,
}
impl StructureInitializer {
    fn new(port: u8, slot: u8, cx: Context) -> Self {
        Self {
            port,
            slot,
            ring: TransferRingController::new(),
            cx,
        }
    }

    fn create(mut self) {
        self.init_input_context();
        self.init_ep0_context();
        self.register_with_dcbaa();
        self.issue_address_device_command();

        CONTEXTS
            .try_lock()
            .expect("Contexts is already locked")
            .push(self.cx);
    }

    fn init_input_context(&mut self) {
        InputContextInitializer::new(&mut self.cx, self.port).init();
    }

    fn init_ep0_context(&mut self) {
        Ep0ContextInitializer::new(
            &mut self.cx,
            self.port,
            &self.ring,
            PortRegisterHandler::new(self.port),
        )
        .init();
    }

    fn register_with_dcbaa(&mut self) {
        dcbaa::register_address(self.port, self.cx.input.phys_addr());
    }

    fn issue_address_device_command(&mut self) {
        command_ring::send_address_device(self.cx.input.phys_addr(), self.slot);
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

        // Port ID starts from 1.
        s.set_root_hub_port_number(self.port + 1);
    }
}

struct Ep0ContextInitializer<'a> {
    cx: &'a mut Context,
    port: u8,
    ring: &'a TransferRingController,
    regs: PortRegisterHandler,
}
impl<'a> Ep0ContextInitializer<'a> {
    fn new(
        cx: &'a mut Context,
        port: u8,
        ring: &'a TransferRingController,
        regs: PortRegisterHandler,
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

struct PortRegisterHandler {
    port_number: u8,
}
impl PortRegisterHandler {
    fn new(port_number: u8) -> Self {
        Self { port_number }
    }

    fn read(&self) -> PortRegisterSet {
        registers::handle(|r| {
            r.port_register_set
                .read_volatile_at(self.port_number.into())
        })
    }

    fn update<T>(&mut self, f: T)
    where
        T: FnOnce(&mut PortRegisterSet),
    {
        registers::handle(|r| {
            r.port_register_set
                .update_volatile_at(self.port_number.into(), f)
        })
    }
}
