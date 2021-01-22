//! Host Controller Operational Registers

use core::convert::TryInto;

use bit_field::BitField;

/// Host Controller Operational Registers
pub struct Operational {}

/// USB Command Register.
#[repr(transparent)]
pub struct UsbCommandRegister(u32);
impl UsbCommandRegister {
    /// Returns the value of the Run/Stop bit.
    pub fn run_stop(&self) -> bool {
        self.0.get_bit(0)
    }

    /// Sets the value of the Run/Stop bit.
    pub fn set_run_stop(&mut self, b: bool) {
        self.0.set_bit(0, b);
    }

    /// Returns the value of the Host Controller Reset bit.
    pub fn host_controller_reset(&self) -> bool {
        self.0.get_bit(1)
    }

    /// Sets the value of the Host Controller Reset bit.
    pub fn set_host_controller_reset(&mut self, b: bool) {
        self.0.set_bit(0, b);
    }
}

/// USB Status Register.
#[repr(transparent)]
pub struct UsbStatusRegister(u32);
impl UsbStatusRegister {
    /// Returns the value of the HCHalted bit.
    pub fn hc_halted(&self) -> bool {
        self.0.get_bit(0)
    }

    /// Returns the value of the Host System Error bit.
    pub fn host_system_error(&self) -> bool {
        self.0.get_bit(2)
    }

    /// Returns the value of the Controller Not Ready bit.
    pub fn controller_not_ready(&self) -> bool {
        self.0.get_bit(11)
    }

    /// Returns the value of the Host Controller Error bit.
    pub fn host_controller_error(&self) -> bool {
        self.0.get_bit(12)
    }
}

/// Page Size Register.
#[repr(transparent)]
pub struct PageSizeRegister(u32);
impl PageSizeRegister {
    /// Returns the value of the page size supported by xHC.
    pub fn get(&self) -> u16 {
        self.0.try_into().unwrap()
    }
}

/// Command Ring Controller Register.
#[repr(transparent)]
pub struct CommandRingControlRegister(u64);
impl CommandRingControlRegister {
    /// Sets the value of the Ring Cycle State bit.
    pub fn set_ring_cycle_state(&mut self, s: bool) {
        self.0.set_bit(0, s);
    }

    /// Returns the bit of the Command Ring Running bit.
    pub fn command_ring_running(&self) -> bool {
        self.0.get_bit(3)
    }

    /// Sets the value of the Command Ring Pointer field. It must be 64 byte aligned.
    ///
    /// # Error
    ///
    /// This method may return a `NotAligned` error if the given pointer is not 64
    /// byte aligned.
    pub fn set_command_ring_pointer(&mut self, p: u64) -> Result<(), NotAligned> {
        if p & 0b11_1111 == 0 {
            let p = p >> 6;
            self.0.set_bits(6..=63, p);
            Ok(())
        } else {
            Err(NotAligned {
                alignment: 64,
                addr: p,
            })
        }
    }
}

/// Device Context Base Address Array Pointer Register.
#[repr(transparent)]
pub struct DeviceContextBaseAddressArrayPointerRegister(u64);
impl DeviceContextBaseAddressArrayPointerRegister {
    /// Sets the value of the Device Context Base Address Array Pointer. It must be 64 byte aligned.
    ///
    /// # Error
    ///
    /// This method may return a `NotAligned` error if the given pointer is not 64 byte aligned.
    pub fn set(&mut self, p: u64) -> Result<(), NotAligned> {
        if p & 0b11_1111 == 0 {
            self.0 = p;
            Ok(())
        } else {
            Err(NotAligned {
                alignment: 64,
                addr: p,
            })
        }
    }
}

/// Configure Register.
#[repr(transparent)]
pub struct ConfigureRegister(u32);
impl ConfigureRegister {
    /// Sets the value of the MaxDevice Slots Enabled field.
    pub fn set_max_device_slots_enabled(&mut self, s: u8) {
        self.0.set_bits(0..=7, s.into());
    }
}

/// Port Status and Control Register.
#[repr(transparent)]
pub struct PortStatusAndControlRegister(u32);
impl PortStatusAndControlRegister {
    /// Returns the value of the Current Connect Status bit.
    pub fn current_connect_status(&self) -> bool {
        self.0.get_bit(0)
    }

    /// Returns the value of the Port Reset bit.
    pub fn port_reset(&self) -> bool {
        self.0.get_bit(4)
    }

    /// Sets the value of the Port Reset bit.
    pub fn set_port_reset(&mut self, b: bool) {
        self.0.set_bit(4, b);
    }

    /// Returns the value of the Port Speed field.
    pub fn port_speed(&self) -> u8 {
        self.0.get_bits(10..=13).try_into().unwrap()
    }

    /// Returns the value of the Port Reset Changed bit.
    pub fn port_reset_changed(&self) -> bool {
        self.0.get_bit(21)
    }
}

/// A struct representing that the given address is not aligned properly.
#[derive(Debug)]
pub struct NotAligned {
    /// Address must be `alignment` byte aligned.
    pub alignment: u64,
    /// Address passed as an argument.
    pub addr: u64,
}
