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

use core::cell::RefCell;

use alloc::rc::Rc;
use command_ring::CommandRingController;
use dcbaa::DeviceContextBaseAddressArray;
use event::EventHandler;
use qemu_exit::QEMUExit;
use qemu_print::qemu_println;
use uefi::table::boot::MemoryType;

#[uefi::entry]
fn main(image: uefi::Handle, st: uefi::table::SystemTable<uefi::table::Boot>) -> uefi::Status {
    let (_, memory_map) = st.exit_boot_services(MemoryType::LOADER_DATA);
    allocator::init(memory_map);

    // SAFETY: We are calling `get_accessor()` only once.
    let regs = unsafe { registers::get_accessor() };
    let mut regs = Rc::new(RefCell::new(regs));

    xhc::init(&mut regs.borrow_mut());

    let mut event_handler = EventHandler::new(&mut regs.borrow_mut());
    let mut event_handler = Rc::new(RefCell::new(event_handler));
    let mut command_ring = CommandRingController::new(&mut regs, &event_handler);

    let _ = DeviceContextBaseAddressArray::new(&mut regs.borrow_mut());
    scratchpat::init(&regs.borrow());

    xhc::run(&mut regs.borrow_mut().operational);
    xhc::ensure_no_error_occurs(&regs.borrow().operational.usbsts.read_volatile());

    command_ring.send_nop();

    ports::init_all_ports(
        &mut regs.borrow_mut(),
        &mut event_handler.borrow_mut(),
        &mut command_ring,
    );

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
