use crate::mapper::Mapper;
use conquer_once::spin::OnceCell;
use qemu_print::qemu_println;
use spinning_top::Spinlock;

pub fn init() {
    qemu_println!("Initializing registers...");

    qemu_println!("Done.");
}

/// To reduce the risk of deadlock caused by long-held register locks, the
/// approach involves minimizing the lock duration by receiving a closure
/// instead of returning the lock.
pub fn handle<T, U>(f: T) -> U
where
    T: FnOnce(&mut xhci::Registers<Mapper>) -> U,
{
    let mut r = get_accessor();

    f(&mut r)
}

fn get_accessor() -> xhci::Registers<Mapper> {
    let xhc_config_space = crate::pci::iter_xhc().next().expect("xHC not found");

    let mmio_low = xhc_config_space.base_address_register(0);
    let mmio_high = xhc_config_space.base_address_register(1);

    let mmio_base = (((mmio_high as u64) << 32) | (mmio_low as u64 & 0xffff_fff0)) as usize;

    unsafe { xhci::Registers::new(mmio_base, Mapper) }
}
