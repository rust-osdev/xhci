// SPDX-License-Identifier: GPL-3.0-or-later

use super::RegisterIndex;
use core::{
    convert::{From, TryFrom},
    ops::Add,
};
use x86_64::PhysAddr;

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct Bar(u32);

impl Bar {
    pub(crate) fn new(bar: u32) -> Self {
        Self(bar)
    }

    pub(crate) fn base_addr(self, upper: Option<Bar>) -> Option<PhysAddr> {
        match upper {
            Some(upper) => match self.ty() {
                BarType::Bar64Bit => self.base_addr_64(upper),
                BarType::Bar32Bit => self.base_addr_32(),
            },
            None => self.base_addr_32(),
        }
    }

    fn ty(self) -> BarType {
        let ty_raw = (self.0 >> 1) & 0b11;
        if ty_raw == 0 {
            BarType::Bar32Bit
        } else if ty_raw == 0x02 {
            BarType::Bar64Bit
        } else {
            unreachable!();
        }
    }

    fn base_addr_64(self, upper: Bar) -> Option<PhysAddr> {
        match self.ty() {
            BarType::Bar32Bit => None,
            BarType::Bar64Bit => Some(PhysAddr::new(
                (u64::from(self.0 & !0xf)) | ((u64::from(upper.0)) << 32),
            )),
        }
    }

    fn base_addr_32(self) -> Option<PhysAddr> {
        match self.ty() {
            BarType::Bar32Bit => Some(PhysAddr::new(u64::from(self.0 & !0xf))),
            BarType::Bar64Bit => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub(crate) struct Index(u32);
impl Index {
    pub(crate) fn new(index: u32) -> Self {
        assert!(index < 6);
        Self(index)
    }
}
impl From<Index> for RegisterIndex {
    fn from(bar_index: Index) -> Self {
        RegisterIndex::new(usize::try_from(bar_index.0 + 4).unwrap())
    }
}
impl Add<u32> for Index {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) enum BarType {
    Bar32Bit,
    Bar64Bit,
}
