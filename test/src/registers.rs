use crate::mapper::Mapper;
use bit_field::BitField;
use conquer_once::spin::OnceCell;
use spinning_top::Spinlock;
use xhci::Registers;

static REGISTERS: OnceCell<Spinlock<Registers<Mapper>>> = OnceCell::uninit();

pub fn init() {
    let xhc_config_space = crate::pci::iter_xhc().next().expect("xHC not found");

    // See [1] for the structure of base address registers.
    //
    // [1]: https://wiki.osdev.org/PCI#Base_Address_Registers
    let mmio_low = xhc_config_space.base_address_register(0);
    let mmio_high = xhc_config_space.base_address_register(1);

    let bar_type = mmio_low.get_bits(1..=2);

    assert_eq!(
        bar_type, 2,
        "The MMIO of xHC must be mapped to memory and 64-bit wide."
    );

    let mmio_base = (((mmio_high as u64) << 32) | (mmio_low as u64 & 0xffff_fff0)) as usize;

    REGISTERS.init_once(||
        // SAFETY: The function will be called only once.
        unsafe { Spinlock::new(xhci::Registers::new(mmio_base, Mapper) )});
}

// This function receives a closure instead of returning a lock guard to reduce
// the possibility of deadlocks.
pub fn handle<T>(f: impl FnOnce(&mut Registers<Mapper>) -> T) -> T {
    let mut regs = REGISTERS
        .try_get()
        .expect("xHC not initialized")
        .try_lock()
        .expect("xHC is already locked");

    f(&mut regs)
}
