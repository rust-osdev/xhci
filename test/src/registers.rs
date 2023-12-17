use bit_field::BitField;

use crate::mapper::Mapper;

pub type Registers = xhci::Registers<Mapper>;

/// # Safety
///
/// Multiple returned values must not exist in the same scope.
pub unsafe fn get_accessor() -> Registers {
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

    // SAFETY: The caller ensures only one instance is created in a scope.
    unsafe { xhci::Registers::new(mmio_base, Mapper) }
}
