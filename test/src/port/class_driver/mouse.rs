use crate::{
    page_box::PageBox,
    port::init::fully_operational::FullyOperational,
    structures::descriptor::{Configuration, Descriptor},
};
use alloc::vec::Vec;
use log::info;
use xhci::context::EndpointType;

pub(in super::super) async fn task(eps: FullyOperational) {
    let mut m = Mouse::new(eps);

    m.configure().await;
    info!("Configuration completed.");

    m.set_boot_protocol().await;
    info!("Boot protocol is set.");

    m.set_idle().await;
    info!("Set Idle completed.");

    loop {
        m.get_packet().await;
        m.print_buf();
    }
}

pub(crate) struct Mouse {
    ep: FullyOperational,
    buf: PageBox<[i8; 4]>,
}
impl Mouse {
    pub(super) fn new(ep: FullyOperational) -> Self {
        Self {
            ep,
            buf: [0; 4].into(),
        }
    }

    async fn configure(&mut self) {
        let d = self.configuration_descriptor();
        self.ep.set_configure(d.config_val()).await;
    }

    async fn set_idle(&mut self) {
        self.ep.set_idle().await;
    }

    async fn set_boot_protocol(&mut self) {
        self.ep.set_boot_protocol().await;
    }

    fn configuration_descriptor(&self) -> Configuration {
        *self
            .ep
            .descriptors()
            .iter()
            .filter_map(|x| {
                if let Descriptor::Configuration(c) = x {
                    Some(c)
                } else {
                    None
                }
            })
            .collect::<Vec<&Configuration>>()[0]
    }

    async fn get_packet(&mut self) {
        self.issue_normal_trb().await;
    }

    async fn issue_normal_trb(&mut self) {
        self.ep
            .issue_normal_trb(&self.buf, EndpointType::InterruptIn)
            .await
            .expect("Failed to send a Normal TRB.");
    }

    fn print_buf(&self) {
        info!(
            "Button: {} {} {}, X: {}, Y: {}",
            self.buf[0] & 1 == 1,
            self.buf[0] & 2 == 2,
            self.buf[0] & 4 == 4,
            self.buf[1],
            self.buf[2]
        );
    }
}
