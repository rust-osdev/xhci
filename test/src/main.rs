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

use qemu_exit::QEMUExit;
use qemu_print::qemu_println;
use uefi::table::boot::MemoryType;

#[uefi::entry]
fn main(image: uefi::Handle, st: uefi::table::SystemTable<uefi::table::Boot>) -> uefi::Status {
    let (_, memory_map) = st.exit_boot_services(MemoryType::LOADER_DATA);
    allocator::init(memory_map);

    registers::init();

    xhc::init();

    let nop_addr = command_ring::send_nop();

    ports::init_all_ports();

    while let Some(trb) = event::dequeue() {
        match trb {
            Ok(xhci::ring::trb::event::Allowed::CommandCompletion(x)) => {
                command_ring::process_trb(&x);
            }
            Ok(x) => panic!("Unhandled TRB: {:?}", x),
            Err(x) => panic!("Unknown TRB: {:?}", x),
        }
    }

    command_ring::assert_all_trbs_are_processed();

    let handler = qemu_exit::X86::new(0xf4, 33);
    handler.exit_success();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let handler = qemu_exit::X86::new(0xf4, 33);

    qemu_println!("{}", info);

    handler.exit_failure();
}
