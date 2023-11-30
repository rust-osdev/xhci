#![no_std]
#![no_main]

use qemu_exit::QEMUExit;
use qemu_print::qemu_println;
use uefi::table::boot::MemoryType;

mod pci;

#[uefi::entry]
fn main(image: uefi::Handle, st: uefi::table::SystemTable<uefi::table::Boot>) -> uefi::Status {
    let (_, _memory_map) = st.exit_boot_services(MemoryType::LOADER_DATA);

    assert!(pci::xhci_exists(), "xHCI controller not found");

    let handler = qemu_exit::X86::new(0xf4, 33);
    handler.exit_success();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let handler = qemu_exit::X86::new(0xf4, 33);

    qemu_println!("{}", info);

    handler.exit_failure();
}
