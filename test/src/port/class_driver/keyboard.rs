use crate::{
    page_box::PageBox,
    port::init::fully_operational::FullyOperational,
    structures::descriptor::{Configuration, Descriptor},
};
use alloc::{string::String, vec::Vec};
use log::info;
use spinning_top::Spinlock;
use xhci::context::EndpointType;

const LOWER_ALPHABETS: &str = "abcdefghijklmnopqrstuvwxyz";

static STR: Spinlock<String> = Spinlock::new(String::new());

pub(in crate::port) async fn task(eps: FullyOperational) {
    let mut k = Keyboard::new(eps);
    k.configure().await;

    info!("Set the Boot protocol.");
    k.set_boot_protocol().await;
    info!("Set.");

    loop {
        k.get_packet().await;
        k.store_key();
    }
}

pub(crate) struct Keyboard {
    ep: FullyOperational,
    buf: PageBox<[u8; 8]>,
}
impl Keyboard {
    pub(in crate::port) fn new(ep: FullyOperational) -> Self {
        Self {
            ep,
            buf: [0; 8].into(),
        }
    }

    async fn configure(&mut self) {
        let d = self.configuration_descriptor();
        self.ep.set_configure(d.config_val()).await;
    }

    async fn set_boot_protocol(&mut self) {
        self.ep.set_boot_protocol().await;
    }

    async fn get_packet(&mut self) {
        self.issue_normal_trb().await;
    }

    async fn issue_normal_trb(&mut self) {
        self.ep
            .issue_normal_trb(&self.buf, EndpointType::InterruptIn)
            .await
            .expect("Failed to send a Normal TRB");
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

    fn store_key(&self) {
        for c in self.buf.iter().skip(2) {
            if *c >= 4 && *c <= 0x1d {
                STR.lock()
                    .push(LOWER_ALPHABETS.chars().nth((c - 4).into()).unwrap());
            } else if *c == 0x28 {
                info!("{}", STR.lock());
                *STR.lock() = String::new();
            }
        }
    }
}
