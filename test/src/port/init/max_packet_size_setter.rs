// SPDX-License-Identifier: GPL-3.0-or-later

use super::{
    descriptor_fetcher::DescriptorFetcher, slot_structures_initializer::SlotStructuresInitializer,
};
use crate::{exchanger, port::endpoint, structures::context::Context};
use alloc::sync::Arc;
use spinning_top::Spinlock;

pub(super) struct MaxPacketSizeSetter {
    ep: endpoint::Default,
    cx: Arc<Spinlock<Context>>,
    port_number: u8,
    slot_number: u8,
}
impl MaxPacketSizeSetter {
    pub(super) fn new(i: SlotStructuresInitializer) -> Self {
        let cx = i.context();
        let port_number = i.port_number();
        let slot_number = i.slot_number();
        let ep = i.ep0();

        Self {
            ep,
            cx,
            port_number,
            slot_number,
        }
    }

    pub(super) async fn set(mut self) -> DescriptorFetcher {
        let s = self.max_packet_size().await;
        self.set_max_packet_size(s);
        self.evaluate_context().await;

        DescriptorFetcher::new(self)
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

    async fn max_packet_size(&mut self) -> u16 {
        self.ep.get_max_packet_size().await
    }

    fn set_max_packet_size(&mut self, s: u16) {
        let mut cx = self.cx.lock();
        let ep_0 = cx.input.device_mut().endpoint_mut(1);

        ep_0.set_max_packet_size(s);
    }

    async fn evaluate_context(&self) {
        let mut cx = self.cx.lock();
        let i = &mut cx.input;

        i.control_mut().set_add_context_flag(1);

        exchanger::command::evaluate_context(i.phys_addr(), self.slot_number).await
    }
}
