use crate::registers;
use alloc::vec;
use alloc::vec::Vec;
use conquer_once::spin::OnceCell;
use spinning_top::Spinlock;

static DCBAA: OnceCell<Spinlock<DeviceContextBaseAddressArray>> = OnceCell::uninit();

pub fn init() {
    DCBAA.init_once(|| Spinlock::new(DeviceContextBaseAddressArray::new()));

    lock().init();
}

pub fn register_address(port: u8, addr: u64) {
    lock().register_address(port, addr);
}

fn lock() -> impl core::ops::DerefMut<Target = DeviceContextBaseAddressArray> {
    DCBAA
        .try_get()
        .expect("Device context base address array not initialized")
        .try_lock()
        .expect("Device context base address array is already locked")
}

struct DeviceContextBaseAddressArray(Vec<RawDCBAA>);
impl DeviceContextBaseAddressArray {
    fn new() -> Self {
        Self(vec![RawDCBAA::new(); number_of_slots()])
    }

    fn register_address(&mut self, port: u8, addr: u64) {
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
