#![no_std]
#![no_main]

extern crate alloc;

mod allocator;
mod command_ring;
mod dcbaa;
mod event;
mod mapper;
mod pci;
mod registers;
mod scratchpat;
mod xhc;

use event::EventHandler;
use qemu_exit::QEMUExit;
use qemu_print::qemu_println;
use uefi::table::boot::MemoryType;

#[uefi::entry]
fn main(image: uefi::Handle, st: uefi::table::SystemTable<uefi::table::Boot>) -> uefi::Status {
    let (_, memory_map) = st.exit_boot_services(MemoryType::LOADER_DATA);
    allocator::init(memory_map);

    // SAFETY: We are calling `get_accessor()` only once.
    let mut regs = unsafe { registers::get_accessor() };

    xhc::init(&mut regs);

    let mut event_handler = EventHandler::new(&mut regs);
    command_ring::init(&mut regs);
    dcbaa::init(&mut regs);
    scratchpat::init(&regs);

    xhc::run(&mut regs);
    xhc::ensure_no_error_occurs(&regs);

    let handler = qemu_exit::X86::new(0xf4, 33);
    handler.exit_success();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let handler = qemu_exit::X86::new(0xf4, 33);

    qemu_println!("{}", info);

    handler.exit_failure();
}
