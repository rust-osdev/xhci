
use crate::transition_helper::BoxWrapper;

use super::registers;
use alloc::boxed::Box;
use x86_64::PhysAddr;
use xhci::context::{
    Device32Byte, Device64Byte, DeviceHandler, Input32Byte, Input64Byte, InputControlHandler,
    InputHandler,
};

pub(crate) struct Context {
    pub(crate) input: Input,
    pub(crate) output: BoxWrapper<Device>,
}
impl Default for Context {
    fn default() -> Self {
        Self {
            input: Input::default(),
            output: Device::default().into(),
        }
    }
}

pub(crate) enum Input {
    Byte64(BoxWrapper<Input64Byte>),
    Byte32(BoxWrapper<Input32Byte>),
}
impl Input {
    pub(crate) fn control_mut(&mut self) -> &mut dyn InputControlHandler {
        match self {
            Self::Byte32(b32) => b32.control_mut(),
            Self::Byte64(b64) => b64.control_mut(),
        }
    }

    pub(crate) fn device_mut(&mut self) -> &mut dyn DeviceHandler {
        match self {
            Self::Byte32(b32) => b32.device_mut(),
            Self::Byte64(b64) => b64.device_mut(),
        }
    }

    pub(crate) fn phys_addr(&self) -> PhysAddr {
        match self {
            Self::Byte32(b32) => b32.phys_addr(),
            Self::Byte64(b64) => b64.phys_addr(),
        }
    }
}
impl Default for Input {
    fn default() -> Self {
        if csz() {
            Self::Byte64(Input64Byte::default().into())
        } else {
            Self::Byte32(Input32Byte::default().into())
        }
    }
}

pub(crate) enum Device {
    Byte64(Box<Device64Byte>),
    Byte32(Box<Device32Byte>),
}
impl Default for Device {
    fn default() -> Self {
        if csz() {
            Self::Byte64(Device64Byte::default().into())
        } else {
            Self::Byte32(Device32Byte::default().into())
        }
    }
}

fn csz() -> bool {
    registers::handle(|r| r.capability.hccparams1.read_volatile().context_size())
}
