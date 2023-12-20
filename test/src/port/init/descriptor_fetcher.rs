use super::{
    endpoints_initializer::EndpointsInitializer, max_packet_size_setter::MaxPacketSizeSetter,
};
use crate::{
    page_box::PageBox,
    port::endpoint,
    structures::{context::Context, descriptor, descriptor::Descriptor},
};
use alloc::{sync::Arc, vec::Vec};
use log::debug;
use spinning_top::Spinlock;

pub(super) struct DescriptorFetcher {
    port_number: u8,
    slot_number: u8,
    cx: Arc<Spinlock<Context>>,
    ep0: endpoint::Default,
}
impl DescriptorFetcher {
    pub(super) fn new(s: MaxPacketSizeSetter) -> Self {
        let port_number = s.port_number();
        let slot_number = s.slot_number();
        let cx = s.context();
        let ep0 = s.ep0();

        Self {
            port_number,
            slot_number,
            cx,
            ep0,
        }
    }

    pub(super) async fn fetch(mut self) -> EndpointsInitializer {
        let r = self.get_raw_descriptors().await;
        let ds = RawDescriptorParser::new(r).parse();
        EndpointsInitializer::new(self, ds)
    }

    pub(super) fn context(&self) -> Arc<Spinlock<Context>> {
        self.cx.clone()
    }

    pub(super) fn port_number(&self) -> u8 {
        self.port_number
    }

    pub(super) fn slot_number(&self) -> u8 {
        self.slot_number
    }

    pub(super) fn ep0(self) -> endpoint::Default {
        self.ep0
    }

    async fn get_raw_descriptors(&mut self) -> PageBox<[u8]> {
        self.ep0.get_raw_configuration_descriptors().await
    }
}

struct RawDescriptorParser {
    raw: PageBox<[u8]>,
    current: usize,
    len: usize,
}
impl RawDescriptorParser {
    fn new(raw: PageBox<[u8]>) -> Self {
        let len = raw.len();

        Self {
            raw,
            current: 0,
            len,
        }
    }

    fn parse(&mut self) -> Vec<Descriptor> {
        let mut v = Vec::new();
        while self.current < self.len && self.raw[self.current] > 0 {
            match self.parse_first_descriptor() {
                Ok(t) => v.push(t),
                Err(e) => debug!("Unrecognized USB descriptor: {:?}", e),
            }
        }
        v
    }

    fn parse_first_descriptor(&mut self) -> Result<Descriptor, descriptor::Error> {
        let raw = self.cut_raw_descriptor();
        Descriptor::from_slice(&raw)
    }

    fn cut_raw_descriptor(&mut self) -> Vec<u8> {
        let len: usize = self.raw[self.current].into();
        let v = self.raw[self.current..(self.current + len)].to_vec();
        self.current += len;
        v
    }
}
