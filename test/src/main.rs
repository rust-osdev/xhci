#![no_std]
#![no_main]

use core::num::NonZeroUsize;
use qemu_exit::QEMUExit;
use qemu_print::qemu_println;
use uefi::table::boot::MemoryType;

mod pci;

#[derive(Clone)]
struct Mapper;
impl xhci::accessor::Mapper for Mapper {
    // UEFI sets up the identity mapping, so we don't need to do anything here.
    unsafe fn map(&mut self, physical_address: usize, _: usize) -> NonZeroUsize {
        NonZeroUsize::new(physical_address).expect("physical_address is zero")
    }

    fn unmap(&mut self, _virtual_address: usize, _size: usize) {}
}

#[uefi::entry]
fn main(image: uefi::Handle, st: uefi::table::SystemTable<uefi::table::Boot>) -> uefi::Status {
    let (_, _memory_map) = st.exit_boot_services(MemoryType::LOADER_DATA);

    let xhc_config_space = pci::iter_xhc().next().expect("xHC not found");

    let mmio_low = xhc_config_space.base_address_register(0);
    let mmio_high = xhc_config_space.base_address_register(1);

    let mmio_base = (((mmio_high as u64) << 32) | (mmio_low as u64 & 0xffff_fff0)) as usize;

    let _registers = unsafe { xhci::Registers::new(mmio_base, Mapper) };

    let handler = qemu_exit::X86::new(0xf4, 33);
    handler.exit_success();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let handler = qemu_exit::X86::new(0xf4, 33);

    qemu_println!("{}", info);

    handler.exit_failure();
}
