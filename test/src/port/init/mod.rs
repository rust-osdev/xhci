// SPDX-License-Identifier: GPL-3.0-or-later

use fully_operational::FullyOperational;
use resetter::Resetter;

mod descriptor_fetcher;
mod endpoints_initializer;
pub(super) mod fully_operational;
mod max_packet_size_setter;
mod resetter;
mod slot_structures_initializer;

pub(super) async fn init(port_number: u8) -> FullyOperational {
    let resetter = Resetter::new(port_number);
    let slot_structures_initializer = resetter.reset().await;
    let max_packet_size_setter = slot_structures_initializer.init().await;
    let descriptor_fetcher = max_packet_size_setter.set().await;
    let endpoints_initializer = descriptor_fetcher.fetch().await;
    endpoints_initializer.init().await
}
