#![no_std]
#![no_main]

use qemu_exit::QEMUExit;

#[uefi::entry]
fn main(image: uefi::Handle, st: uefi::table::SystemTable<uefi::table::Boot>) -> uefi::Status {
    let handler = qemu_exit::X86::new(0xf4, 33);

    handler.exit_success();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let handler = qemu_exit::X86::new(0xf4, 33);

    handler.exit_failure();
}
