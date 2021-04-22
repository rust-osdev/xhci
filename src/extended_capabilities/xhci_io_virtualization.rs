//! xHCI I/O Virtualization (xHCI-IOV) Capability.

use bit_field::BitField;
use core::convert::TryInto;

/// VF Interrupter Range Registers.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct VfInterrupterRangeRegisters(u32);
impl VfInterrupterRangeRegisters {
    /// Returns the value of the Interrupter Offset field.
    #[must_use]
    pub fn interrupter_offset(self) -> u16 {
        self.0.get_bits(0..=9).try_into().unwrap()
    }

    /// Sets the value of the Interrupter Offset field.
    pub fn set_interrupter_offset(&mut self, offset: u16) {
        self.0.set_bits(0..=9, offset.into());
    }

    /// Returns the value of the Interrupter Count field.
    #[must_use]
    pub fn interrupter_count(self) -> u16 {
        self.0.get_bits(10..=19).try_into().unwrap()
    }

    /// Sets the value of the Interrupter Count field.
    pub fn set_interrupter_count(&mut self, count: u16) {
        self.0.set_bits(10..=19, count.into());
    }

    /// Returns the VF Run bit.
    #[must_use]
    pub fn vf_run(self) -> bool {
        self.0.get_bit(20)
    }

    /// Sets the VF Run bit.
    pub fn set_vf_run(&mut self, b: bool) {
        self.0.set_bit(20, b);
    }

    /// Returns the VF Halted bit.
    #[must_use]
    pub fn vf_halted(self) -> bool {
        self.0.get_bit(21)
    }
}
impl_debug_from_methods! {
    VfInterrupterRangeRegisters {
        interrupter_offset,
        interrupter_count,
        vf_run,
        vf_halted,
    }
}
