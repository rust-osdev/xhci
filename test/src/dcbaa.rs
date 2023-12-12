use crate::registers::Registers;
use alloc::vec;
use alloc::vec::Vec;
use conquer_once::spin::OnceCell;
use qemu_print::qemu_println;
use spinning_top::Spinlock;

static DCBAA: OnceCell<Spinlock<DeviceContextBaseAddressArray>> = OnceCell::uninit();

pub fn init(regs: &mut Registers) {
    DCBAA
        .try_init_once(|| Spinlock::new(DeviceContextBaseAddressArray::new(regs)))
        .expect("DeviceContextBaseAddressArray::new() called more than once");

    DCBAA
        .get()
        .unwrap_or_else(|| unreachable!("Should be initialized"))
        .lock()
        .init(regs);

    qemu_println!("Device Context Base Address Array is initialized");
}

struct DeviceContextBaseAddressArray(Vec<RawDCBAA>);
impl DeviceContextBaseAddressArray {
    fn new(regs: &Registers) -> Self {
        Self(vec![RawDCBAA::new(); number_of_slots(regs)])
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
