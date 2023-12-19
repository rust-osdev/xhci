// SPDX-License-Identifier: GPL-3.0-or-later

use byteorder::{BigEndian, ByteOrder};
use core::fmt;

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub(crate) struct Inquiry([u8; 36]);
impl Default for Inquiry {
    fn default() -> Self {
        Self([0; 36])
    }
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub(crate) struct ReadCapacity10 {
    lba: [u8; 4],
    block_size: [u8; 4],
}
impl ReadCapacity10 {
    fn lba(self) -> u32 {
        BigEndian::read_u32(&self.lba)
    }

    fn block_size(self) -> u32 {
        BigEndian::read_u32(&self.block_size)
    }
}
impl fmt::Debug for ReadCapacity10 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReadCapacity10")
            .field("lba", &self.lba())
            .field("block_size", &self.block_size())
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub(crate) struct Read10([u8; 32768]);
impl Default for Read10 {
    fn default() -> Self {
        Self([0; 32768])
    }
}
