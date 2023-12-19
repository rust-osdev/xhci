use crate::{exchanger::transfer, page_box::PageBox, structures::descriptor};
use x86_64::PhysAddr;
use xhci::context::EndpointType;

pub(super) struct Default {
    sender: transfer::Sender,
}
impl Default {
    pub(super) fn new(sender: transfer::Sender) -> Self {
        Self { sender }
    }

    pub(super) fn ring_addr(&self) -> PhysAddr {
        self.sender.ring_addr()
    }

    pub(super) async fn get_max_packet_size(&mut self) -> u16 {
        self.sender
            .get_max_packet_size_from_device_descriptor()
            .await
    }

    pub(super) async fn get_raw_configuration_descriptors(&mut self) -> PageBox<[u8]> {
        self.sender.get_configuration_descriptor().await
    }

    pub(super) async fn set_configuration(&mut self, config_val: u8) {
        self.sender.set_configure(config_val).await;
    }

    pub(super) async fn set_idle(&mut self) {
        self.sender.set_idle().await;
    }

    pub(super) async fn set_boot_protocol(&mut self) {
        self.sender.set_boot_protocol().await;
    }
}

pub(super) struct NonDefault {
    desc: descriptor::Endpoint,
    sender: transfer::Sender,
}
impl NonDefault {
    pub(super) fn new(desc: descriptor::Endpoint, sender: transfer::Sender) -> Self {
        Self { desc, sender }
    }

    pub(super) fn descriptor(&self) -> descriptor::Endpoint {
        self.desc
    }

    pub(super) fn transfer_ring_addr(&self) -> PhysAddr {
        self.sender.ring_addr()
    }

    pub(super) fn ty(&self) -> EndpointType {
        self.desc.ty()
    }

    pub(super) async fn issue_normal_trb<T: ?Sized>(&mut self, b: &PageBox<T>) {
        self.sender.issue_normal_trb(b).await
    }
}

#[derive(Debug)]
pub(crate) enum Error {
    NoSuchEndpoint(EndpointType),
}
