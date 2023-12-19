// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) mod bar;
mod common;
pub(crate) mod type_spec;

use self::common::Common;
use bar::Bar;
use core::{convert::TryFrom, ops::Add};
use type_spec::TypeSpec;
use x86_64::{
    structures::port::{PortRead, PortWrite},
    PhysAddr,
};

#[derive(Debug)]
pub(crate) struct Space {
    registers: Registers,
}

impl Space {
    pub(crate) fn new(bus: Bus, device: Device) -> Option<Self> {
        Some(Self {
            registers: Registers::new(bus, device)?,
        })
    }

    pub(crate) fn is_xhci(&self) -> bool {
        self.common().is_xhci()
    }

    pub(crate) fn base_address(&self, index: bar::Index) -> PhysAddr {
        self.type_spec().base_address(index)
    }

    fn type_spec(&self) -> TypeSpec<'_> {
        TypeSpec::new(&self.registers, &self.common())
    }

    fn common(&self) -> Common<'_> {
        Common::new(&self.registers)
    }
}

#[derive(Debug)]
pub(crate) struct Registers {
    bus: Bus,
    device: Device,
}
impl Registers {
    fn new(bus: Bus, device: Device) -> Option<Self> {
        if Self::valid(bus, device) {
            Some(Self { bus, device })
        } else {
            None
        }
    }

    fn valid(bus: Bus, device: Device) -> bool {
        let config_addr = ConfigAddress::new(bus, device, Function::zero(), RegisterIndex::zero());
        let id = unsafe { config_addr.read() };

        id != !0
    }

    fn get(&self, index: RegisterIndex) -> u32 {
        let accessor = ConfigAddress::new(self.bus, self.device, Function::zero(), index);
        unsafe { accessor.read() }
    }
}

struct ConfigAddress {
    bus: Bus,
    device: Device,
    function: Function,
    register: RegisterIndex,
}

impl ConfigAddress {
    const PORT_CONFIG_ADDR: u16 = 0xcf8;
    const PORT_CONFIG_DATA: u16 = 0xcfc;

    #[allow(clippy::too_many_arguments)]
    fn new(bus: Bus, device: Device, function: Function, register: RegisterIndex) -> Self {
        Self {
            bus,
            device,
            function,
            register,
        }
    }

    fn as_u32(&self) -> u32 {
        const VALID: u32 = 0x8000_0000;
        let bus = self.bus.as_u32();
        let device = self.device.as_u32();
        let function = self.function.as_u32();
        let register = u32::try_from(self.register.as_usize()).unwrap();

        VALID | bus << 16 | device << 11 | function << 8 | register << 2
    }

    /// SAFETY: `self` must contain the valid config address.
    unsafe fn read(&self) -> u32 {
        PortWrite::write_to_port(Self::PORT_CONFIG_ADDR, self.as_u32());
        PortRead::read_from_port(Self::PORT_CONFIG_DATA)
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Bus(u32);
impl Bus {
    pub(crate) const MAX: u32 = 256;
    pub(crate) fn new(bus: u32) -> Self {
        assert!(bus < Self::MAX);
        Self(bus)
    }

    fn as_u32(self) -> u32 {
        self.0
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Device(u32);
impl Device {
    pub(crate) const MAX: u32 = 32;
    pub(crate) fn new(device: u32) -> Self {
        assert!(device < Self::MAX);
        Self(device)
    }

    fn as_u32(self) -> u32 {
        self.0
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Function(u32);
impl Function {
    pub(crate) fn zero() -> Self {
        Self(0)
    }

    pub(crate) fn as_u32(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct RegisterIndex(usize);
impl RegisterIndex {
    const MAX: usize = 64;
    pub(crate) fn new(offset: usize) -> Self {
        assert!(offset < Self::MAX, "Too large register index: {}", offset);
        Self(offset)
    }

    fn zero() -> Self {
        Self(0)
    }

    fn as_usize(self) -> usize {
        self.0
    }
}

impl Add<usize> for RegisterIndex {
    type Output = RegisterIndex;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}
