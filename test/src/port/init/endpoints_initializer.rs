
use super::{descriptor_fetcher::DescriptorFetcher, fully_operational::FullyOperational};
use crate::{
    exchanger,
    exchanger::transfer,
    port::endpoint,
    structures::{context::Context, descriptor, descriptor::Descriptor, registers},
};
use alloc::{sync::Arc, vec::Vec};
use bit_field::BitField;
use core::convert::TryInto;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use spinning_top::Spinlock;
use transfer::DoorbellWriter;
use x86_64::PhysAddr;
use xhci::context::{EndpointHandler, EndpointType};

pub(super) struct EndpointsInitializer {
    cx: Arc<Spinlock<Context>>,
    descriptors: Vec<Descriptor>,
    endpoints: Vec<endpoint::NonDefault>,
    ep0: endpoint::Default,
    port_number: u8,
    slot_number: u8,
}
impl EndpointsInitializer {
    pub(super) fn new(f: DescriptorFetcher, descriptors: Vec<Descriptor>) -> Self {
        let cx = f.context();
        let endpoints = descriptors_to_endpoints(&f, &descriptors);
        let port_number = f.port_number();
        let slot_number = f.slot_number();
        let ep0 = f.ep0();

        Self {
            cx,
            descriptors,
            endpoints,
            ep0,
            port_number,
            slot_number,
        }
    }

    pub(super) async fn init(mut self) -> FullyOperational {
        self.init_contexts();
        self.set_context_entries();
        self.configure_endpoint().await;
        FullyOperational::new(self)
    }

    pub(super) fn descriptors(&self) -> Vec<Descriptor> {
        self.descriptors.clone()
    }

    pub(super) fn endpoints(self) -> (endpoint::Default, Vec<endpoint::NonDefault>) {
        (self.ep0, self.endpoints)
    }

    fn init_contexts(&mut self) {
        for e in &mut self.endpoints {
            ContextInitializer::new(
                &mut self.cx.lock(),
                &e.descriptor(),
                e.transfer_ring_addr(),
                self.port_number,
            )
            .init()
        }
    }

    fn set_context_entries(&mut self) {
        let mut cx = self.cx.lock();
        cx.input.device_mut().slot_mut().set_context_entries(31);
    }

    async fn configure_endpoint(&mut self) {
        let a = self.cx.lock().input.phys_addr();
        exchanger::command::configure_endpoint(a, self.slot_number).await;
    }
}

struct ContextInitializer<'a> {
    cx: &'a mut Context,
    ep: &'a descriptor::Endpoint,
    transfer_ring_addr: PhysAddr,
    port_number: u8,
}
impl<'a> ContextInitializer<'a> {
    #[allow(clippy::too_many_arguments)] // TODO
    fn new(
        cx: &'a mut Context,
        ep: &'a descriptor::Endpoint,
        transfer_ring_addr: PhysAddr,
        port_number: u8,
    ) -> Self {
        Self {
            cx,
            ep,
            transfer_ring_addr,
            port_number,
        }
    }

    fn init(mut self) {
        self.set_aflag();
        self.init_ep_context();
    }

    fn set_aflag(&mut self) {
        let dci: usize = self.calculate_dci().into();
        let c = self.cx.input.control_mut();

        c.set_add_context_flag(0);
        c.clear_add_context_flag(1); // See xHCI dev manual 4.6.6.
        c.set_add_context_flag(dci);
    }

    fn calculate_dci(&self) -> u8 {
        let a = self.ep.endpoint_address;
        2 * a.get_bits(0..=3) + a.get_bit(7) as u8
    }

    fn init_ep_context(&mut self) {
        self.set_interval();

        let ep_ty = self.ep.ty();
        self.ep_cx().set_endpoint_type(ep_ty);

        // TODO: This initializes the context only for USB2. Branch if the version of a device is
        // USB3.
        match ep_ty {
            EndpointType::Control => self.init_for_control(),
            EndpointType::BulkOut | EndpointType::BulkIn => self.init_for_bulk(),
            EndpointType::IsochOut
            | EndpointType::IsochIn
            | EndpointType::InterruptOut
            | EndpointType::InterruptIn => self.init_for_isoch_or_interrupt(),
            EndpointType::NotValid => unreachable!("Not Valid Endpoint should not exist."),
        }
    }

    fn init_for_control(&mut self) {
        assert_eq!(
            self.ep.ty(),
            EndpointType::Control,
            "Not the Control Endpoint."
        );

        let sz = self.ep.max_packet_size;
        let a = self.transfer_ring_addr;
        let c = self.ep_cx();

        c.set_max_packet_size(sz);
        c.set_error_count(3);
        c.set_tr_dequeue_pointer(a.as_u64());
        c.set_dequeue_cycle_state();
    }

    fn init_for_bulk(&mut self) {
        assert!(self.is_bulk(), "Not the Bulk Endpoint.");

        let sz = self.ep.max_packet_size;
        let a = self.transfer_ring_addr;
        let c = self.ep_cx();

        c.set_max_packet_size(sz);
        c.set_max_burst_size(0);
        c.set_error_count(3);
        c.set_max_primary_streams(0);
        c.set_tr_dequeue_pointer(a.as_u64());
        c.set_dequeue_cycle_state();
    }

    fn is_bulk(&self) -> bool {
        let t = self.ep.ty();

        [EndpointType::BulkOut, EndpointType::BulkIn].contains(&t)
    }

    fn init_for_isoch_or_interrupt(&mut self) {
        let t = self.ep.ty();
        assert!(
            self.is_isoch_or_interrupt(),
            "Not the Isochronous or the Interrupt Endpoint."
        );

        let sz = self.ep.max_packet_size;
        let a = self.transfer_ring_addr;
        let c = self.ep_cx();

        c.set_max_packet_size(sz & 0x7ff);
        c.set_max_burst_size(((sz & 0x1800) >> 11).try_into().unwrap());
        c.set_mult(0);

        if let EndpointType::IsochOut | EndpointType::IsochIn = t {
            c.set_error_count(0);
        } else {
            c.set_error_count(3);
        }
        c.set_tr_dequeue_pointer(a.as_u64());
        c.set_dequeue_cycle_state();
    }

    fn is_isoch_or_interrupt(&self) -> bool {
        let t = self.ep.ty();
        [
            EndpointType::IsochOut,
            EndpointType::IsochIn,
            EndpointType::InterruptOut,
            EndpointType::InterruptIn,
        ]
        .contains(&t)
    }

    // TODO: Is this calculation correct?
    fn set_interval(&mut self) {
        let s = self.port_speed();
        let t = self.ep.ty();
        let i = self.ep.interval;

        let i = if let PortSpeed::FullSpeed | PortSpeed::LowSpeed = s {
            if let EndpointType::IsochOut | EndpointType::IsochIn = t {
                i + 2
            } else {
                i + 3
            }
        } else {
            i - 1
        };

        self.ep_cx().set_interval(i);
    }

    fn port_speed(&self) -> PortSpeed {
        FromPrimitive::from_u8(registers::handle(|r| {
            r.port_register_set
                .read_volatile_at((self.port_number - 1).into())
                .portsc
                .port_speed()
        }))
        .expect("Failed to get the Port Speed.")
    }

    fn ep_cx(&mut self) -> &mut dyn EndpointHandler {
        let ep_i: usize = self.ep.endpoint_address.get_bits(0..=3).into();
        let is_input: usize = self.ep.endpoint_address.get_bit(7) as _;
        let dpi = 2 * ep_i + is_input;

        self.cx.input.device_mut().endpoint_mut(dpi)
    }
}

#[derive(Copy, Clone, FromPrimitive)]
enum PortSpeed {
    FullSpeed = 1,
    LowSpeed = 2,
    HighSpeed = 3,
    SuperSpeed = 4,
    SuperSpeedPlus = 5,
}

fn descriptors_to_endpoints(
    f: &DescriptorFetcher,
    descriptors: &[Descriptor],
) -> Vec<endpoint::NonDefault> {
    descriptors
        .iter()
        .filter_map(|desc| {
            let _ = &f;
            if let Descriptor::Endpoint(e) = desc {
                let d = DoorbellWriter::new(f.slot_number(), e.doorbell_value());
                let s = transfer::Sender::new(d);
                Some(endpoint::NonDefault::new(*e, s))
            } else {
                None
            }
        })
        .collect()
}
