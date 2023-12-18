use alloc::vec;
use alloc::vec::Vec;

use crate::registers;

pub struct DeviceContextBaseAddressArray(Vec<RawDCBAA>);
impl DeviceContextBaseAddressArray {
    pub fn new() -> Self {
        let mut v = Self(vec![RawDCBAA::new(); number_of_slots()]);

        v.init();

        v
    }

    pub fn register_address(&mut self, port: u8, addr: u64) {
        self.0[port as usize].0 = addr;
    }

    fn init(&mut self) {
        self.register_address_with_register();
    }

    fn register_address_with_register(&self) {
        registers::handle(|r| {
            r.operational
                .dcbaap
                .update_volatile(|dcbaap| dcbaap.set(self.0.as_ptr() as u64));
        })
    }
}

#[repr(C, align(64))]
#[derive(Clone, Copy)]
struct RawDCBAA(u64);
impl RawDCBAA {
    fn new() -> Self {
        Self(0)
    }
}

fn number_of_slots() -> usize {
    registers::handle(|r| {
        r.capability
            .hcsparams1
            .read_volatile()
            .number_of_device_slots() as usize
            + 1_usize
    })
}
