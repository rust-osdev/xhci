use crate::registers;
use alloc::vec;
use alloc::vec::Vec;
use conquer_once::spin::OnceCell;
use qemu_print::qemu_println;
use spinning_top::Spinlock;

static DCBAA: OnceCell<Spinlock<DeviceContextBaseAddressArray>> = OnceCell::uninit();

pub fn init() {
    DCBAA
        .try_init_once(|| Spinlock::new(DeviceContextBaseAddressArray::new()))
        .expect("DeviceContextBaseAddressArray::new() called more than once");

    DCBAA
        .get()
        .unwrap_or_else(|| unreachable!("Should be initialized"))
        .lock()
        .init();

    qemu_println!("Device Context Base Address Array is initialized");
}

struct DeviceContextBaseAddressArray(Vec<u64>);
impl DeviceContextBaseAddressArray {
    fn new() -> Self {
        Self(vec![0; number_of_slots()])
    }

    fn init(&mut self) {
        self.register_address_with_register();
    }

    fn register_address_with_register(&self) {
        registers::handle(|r| {
            r.operational
                .dcbaap
                .update_volatile(|dcbaap| dcbaap.set(self.0.as_ptr() as u64));
        });
    }
}

fn number_of_slots() -> usize {
    registers::handle(|r| {
        r.capability
            .hcsparams1
            .read_volatile()
            .number_of_device_slots()
            + 1
    })
    .into()
}
