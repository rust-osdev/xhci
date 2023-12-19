use crate::registers;

use {
    alloc::boxed::Box,
    xhci::context::{
        Device32Byte, Device64Byte, DeviceHandler, Input32Byte, Input64Byte, InputControlHandler,
        InputHandler,
    },
};

pub struct Context {
    pub input: Input,
    pub output: Box<Device>,
}
impl Context {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
            output: Device::new().into(),
        }
    }
}

pub(crate) enum Input {
    Byte64(Box<Input64Byte>),
    Byte32(Box<Input32Byte>),
}
impl Input {
    pub fn control_mut(&mut self) -> &mut dyn InputControlHandler {
        match self {
            Self::Byte32(b32) => b32.control_mut(),
            Self::Byte64(b64) => b64.control_mut(),
        }
    }

    pub fn device_mut(&mut self) -> &mut dyn DeviceHandler {
        match self {
            Self::Byte32(b32) => b32.device_mut(),
            Self::Byte64(b64) => b64.device_mut(),
        }
    }

    pub fn phys_addr(&self) -> u64 {
        match self {
            Self::Byte32(b32) => b32.as_ref() as *const _ as u64,
            Self::Byte64(b64) => b64.as_ref() as *const _ as u64,
        }
    }

    fn new() -> Self {
        if csz() {
            Self::Byte64(Input64Byte::default().into())
        } else {
            Self::Byte32(Input32Byte::default().into())
        }
    }
}

pub enum Device {
    Byte64(Box<Device64Byte>),
    Byte32(Box<Device32Byte>),
}
impl Device {
    fn new() -> Self {
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
