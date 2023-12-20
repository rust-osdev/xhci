use super::{max_packet_size_setter::MaxPacketSizeSetter, resetter::Resetter};
use crate::{
    exchanger,
    port::endpoint,
    structures::{context::Context, dcbaa, registers},
};
use alloc::sync::Arc;
use exchanger::{transfer, transfer::DoorbellWriter};
use spinning_top::Spinlock;
use xhci::context::EndpointType;

pub(super) struct SlotStructuresInitializer {
    port_number: u8,
    slot_number: u8,
    cx: Arc<Spinlock<Context>>,
    ep: endpoint::Default,
}
impl SlotStructuresInitializer {
    pub(super) async fn new(r: Resetter) -> Self {
        let slot_number = exchanger::command::enable_device_slot().await;
        let cx = Arc::new(Spinlock::new(Context::default()));
        let dbl_writer = DoorbellWriter::new(slot_number, 1);

        Self {
            port_number: r.port_number(),
            slot_number,
            cx,
            ep: endpoint::Default::new(transfer::Sender::new(dbl_writer)),
        }
    }

    pub(super) async fn init(self) -> MaxPacketSizeSetter {
        self.init_input_context();
        self.init_endpoint0_context();
        self.register_with_dcbaa();
        self.issue_address_device().await;

        MaxPacketSizeSetter::new(self)
    }

    pub(super) fn port_number(&self) -> u8 {
        self.port_number
    }

    pub(super) fn slot_number(&self) -> u8 {
        self.slot_number
    }

    pub(super) fn context(&self) -> Arc<Spinlock<Context>> {
        self.cx.clone()
    }

    pub(super) fn ep0(self) -> endpoint::Default {
        self.ep
    }

    fn init_input_context(&self) {
        InputContextInitializer::new(&mut self.cx.lock(), self.port_number).init()
    }

    fn init_endpoint0_context(&self) {
        Ep0ContextInitializer::new(&mut self.cx.lock(), self.port_number, &self.ep).init()
    }

    fn register_with_dcbaa(&self) {
        let a = self.cx.lock().output.phys_addr();
        dcbaa::register_device_context_addr(self.slot_number.into(), a);
    }

    async fn issue_address_device(&self) {
        let cx_addr = self.cx.lock().input.phys_addr();
        exchanger::command::address_device(cx_addr, self.slot_number).await;
    }
}

struct InputContextInitializer<'a> {
    context: &'a mut Context,
    port_number: u8,
}
impl<'a> InputContextInitializer<'a> {
    fn new(context: &'a mut Context, port_number: u8) -> Self {
        Self {
            context,
            port_number,
        }
    }

    fn init(&mut self) {
        self.init_input_control();
        self.init_input_slot();
    }

    fn init_input_control(&mut self) {
        let input_control = self.context.input.control_mut();
        input_control.set_add_context_flag(0);
        input_control.set_add_context_flag(1);
    }

    fn init_input_slot(&mut self) {
        let slot = self.context.input.device_mut().slot_mut();
        slot.set_context_entries(1);
        slot.set_root_hub_port_number(self.port_number);
    }
}

struct Ep0ContextInitializer<'a> {
    cx: &'a mut Context,
    port_number: u8,
    ep: &'a endpoint::Default,
}
impl<'a> Ep0ContextInitializer<'a> {
    fn new(cx: &'a mut Context, port_number: u8, ep: &'a endpoint::Default) -> Self {
        Self {
            cx,
            port_number,
            ep,
        }
    }

    fn init(self) {
        let s = self.get_max_packet_size();
        let ep_0 = self.cx.input.device_mut().endpoint_mut(1);

        ep_0.set_endpoint_type(EndpointType::Control);
        ep_0.set_max_packet_size(s);
        ep_0.set_tr_dequeue_pointer(self.ep.ring_addr().as_u64());
        ep_0.set_dequeue_cycle_state();
        ep_0.set_error_count(3);
    }

    // TODO: This function does not check the actual port speed, instead it uses the normal
    // correspondence between PSI and the port speed.
    // The actual port speed is listed on the xHCI supported protocol capability.
    // Check the capability and fetch the actual port speed. Then return the max packet size.
    fn get_max_packet_size(&self) -> u16 {
        let psi = registers::handle(|r| {
            r.port_register_set
                .read_volatile_at((self.port_number - 1).into())
                .portsc
                .port_speed()
        });

        match psi {
            1 | 3 => 64,
            2 => 8,
            4 => 512,
            _ => unimplemented!("PSI: {}", psi),
        }
    }
}
