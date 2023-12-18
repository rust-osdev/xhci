#![no_std]
#![no_main]

extern crate alloc;

mod allocator;
mod command_ring;
mod dcbaa;
mod event;
mod mapper;
mod pci;
mod ports;
mod registers;
mod scratchpat;
mod transfer_ring;
mod xhc;

use alloc::rc::Rc;
use core::cell::RefCell;
use qemu_exit::QEMUExit;
use qemu_print::qemu_println;
use uefi::table::boot::MemoryType;
use xhci::ring::trb::event::CompletionCode;

#[uefi::entry]
fn main(image: uefi::Handle, st: uefi::table::SystemTable<uefi::table::Boot>) -> uefi::Status {
    let (_, memory_map) = st.exit_boot_services(MemoryType::LOADER_DATA);
    allocator::init(memory_map);

    // SAFETY: We are calling `get_accessor()` only once.
    let regs = unsafe { registers::get_accessor() };
    let regs = Rc::new(RefCell::new(regs));

    let (event_handler, command_ring, dcbaa) = xhc::init(&regs);

    let nop_addr = command_ring.borrow_mut().send_nop();
    event_handler.borrow_mut().register_handler(nop_addr, |c| {
        assert_eq!(
            c.completion_code(),
            Ok(CompletionCode::Success),
            "NOP failed."
        );
    });

    ports::init_all_ports(regs, event_handler.clone(), command_ring, dcbaa);

    event_handler.borrow_mut().process_trbs();
    event_handler.borrow_mut().assert_all_commands_completed();

    let handler = qemu_exit::X86::new(0xf4, 33);
    handler.exit_success();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let handler = qemu_exit::X86::new(0xf4, 33);

    qemu_println!("{}", info);

    handler.exit_failure();
}
