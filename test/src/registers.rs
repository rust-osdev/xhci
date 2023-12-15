use crate::mapper::Mapper;

pub type Registers = xhci::Registers<Mapper>;

/// # Safety
///
/// Multiple returned values must not exist in the same scope.
//
// See [1] or [2] to understand this process.
//
// [1]: PCI - OSDev Wiki (https://wiki.osdev.org/PCI#Configuration_Space_Access_Mechanism_.231)
// [2]: USB 3.0 ホストドライバ自作入門 第2章 (https://booth.pm/ja/items/1056355)
pub unsafe fn get_accessor() -> Registers {
    let xhc_config_space = crate::pci::iter_xhc().next().expect("xHC not found");

    let mmio_low = xhc_config_space.base_address_register(0);
    let mmio_high = xhc_config_space.base_address_register(1);

    let mmio_base = (((mmio_high as u64) << 32) | (mmio_low as u64 & 0xffff_fff0)) as usize;

    // SAFETY: The caller ensures only one instance is created in a scope.
    unsafe { xhci::Registers::new(mmio_base, Mapper) }
}
