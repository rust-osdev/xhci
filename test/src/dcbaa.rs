use crate::registers::Registers;
use alloc::vec;
use alloc::vec::Vec;

pub struct DeviceContextBaseAddressArray(Vec<RawDCBAA>);
impl DeviceContextBaseAddressArray {
    pub fn new(regs: &mut Registers) -> Self {
        let mut v = Self(vec![RawDCBAA::new(); number_of_slots(regs)]);

        v.init(regs);

        v
    }

    pub fn register_address(&mut self, port: u8, addr: u64) {
        self.0[port as usize].0 = addr;
    }

    fn init(&mut self, regs: &mut Registers) {
        self.register_address_with_register(regs);
    }

    fn register_address_with_register(&self, regs: &mut Registers) {
        regs.operational
            .dcbaap
            .update_volatile(|dcbaap| dcbaap.set(self.0.as_ptr() as u64));
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

fn number_of_slots(regs: &Registers) -> usize {
    regs.capability
        .hcsparams1
        .read_volatile()
        .number_of_device_slots() as usize
        + 1_usize
}
