#![no_std]
#![no_main]
// A workaround for the `derive_builder` crate.
#![allow(clippy::default_trait_access)]

extern crate alloc;

use alloc::sync::Arc;
use futures_intrusive::sync::{GenericMutex, GenericMutexGuard};
use multitask::{executor::Executor, task::Task};
use pci::config::bar;
use qemu_exit::QEMUExit;
use qemu_print::qemu_println;
use spinning_top::{RawSpinlock, Spinlock};
use structures::{
    dcbaa, extended_capabilities, registers,
    ring::{command, event},
    scratchpad,
};
use uefi::{
    table::{boot::MemoryType, Boot, SystemTable},
    Handle,
};
use x86_64::PhysAddr;

pub(crate) type Futurelock<T> = GenericMutex<RawSpinlock, T>;
pub(crate) type FuturelockGuard<'a, T> = GenericMutexGuard<'a, RawSpinlock, T>;

mod allocator;
mod exchanger;
mod logger;
mod mapper;
mod multitask;
mod pci;
mod port;
mod structures;
mod transition_helper;
mod xhc;

#[uefi::entry]
fn main(h: Handle, st: SystemTable<Boot>) -> uefi::Status {
    let (_, mmap) = st.exit_boot_services(MemoryType::LOADER_DATA);

    logger::init();
    allocator::init(mmap);

    assert!(xhc::exists(), "xHC does not exist.");

    init();

    let mut executor = Executor::new();
    executor.run();
}

pub(crate) fn init() {
    init_and_spawn_tasks();
}

fn init_statics() {
    let a = iter_xhc().next().expect("xHC does not exist.");

    // SAFETY: BAR 0 address is passed.
    unsafe {
        registers::init(a);
        extended_capabilities::init(a);
    }
}

fn init_and_spawn_tasks() {
    init_statics();

    // In some cases, an OS may need to get ownership of the xHC from the BIOS.
    // See 4.22.1 of xHCI spec.
    //
    // This is not necessary on QEMU, but this line is left for a reference.
    xhc::get_ownership_from_bios();

    xhc::init();

    dcbaa::init();
    scratchpad::init();
    exchanger::command::init();
    event::init();

    xhc::run();
    xhc::ensure_no_error_occurs();

    spawn_tasks();
}

fn spawn_tasks() {
    port::spawn_all_connected_port_tasks();

    multitask::add(Task::new_poll(event::task()));
}

fn iter_xhc() -> impl Iterator<Item = PhysAddr> {
    pci::iter_devices().filter_map(|device| {
        if device.is_xhci() {
            Some(device.base_address(bar::Index::new(0)))
        } else {
            None
        }
    })
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let handler = qemu_exit::X86::new(0xf4, 33);

    qemu_println!("{}", info);

    handler.exit_failure();
}
