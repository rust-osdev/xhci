// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) mod config;

use config::{Bus, Device};

pub(crate) fn iter_devices() -> impl Iterator<Item = config::Space> {
    IterPciDevices::new(0, 0)
}

struct IterPciDevices {
    bus: u32,
    device: u32,
}

impl IterPciDevices {
    fn new(bus: u32, device: u32) -> Self {
        assert!(device < 32);
        Self { bus, device }
    }
}

impl Iterator for IterPciDevices {
    type Item = config::Space;

    fn next(&mut self) -> Option<Self::Item> {
        for bus in self.bus..Bus::MAX {
            for device in self.device..Device::MAX {
                if let Some(space) = config::Space::new(Bus::new(bus), Device::new(device)) {
                    self.bus = bus;
                    self.device = device + 1;

                    return Some(space);
                }
            }

            self.device = 0;
        }

        None
    }
}
