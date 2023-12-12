use crate::mapper::Mapper;

pub type Registers = xhci::Registers<Mapper>;

pub fn get_accessor() -> Registers {
    let xhc_config_space = crate::pci::iter_xhc().next().expect("xHC not found");

    let mmio_low = xhc_config_space.base_address_register(0);
    let mmio_high = xhc_config_space.base_address_register(1);

    let mmio_base = (((mmio_high as u64) << 32) | (mmio_low as u64 & 0xffff_fff0)) as usize;

    unsafe { xhci::Registers::new(mmio_base, Mapper) }
}
