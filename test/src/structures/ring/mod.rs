// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) mod command;
pub(crate) mod event;
pub(crate) mod transfer;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct CycleBit(bool);
impl CycleBit {
    pub fn new(val: bool) -> Self {
        Self(val)
    }

    fn toggle(&mut self) {
        self.0 = !self.0;
    }
}
impl From<CycleBit> for bool {
    fn from(cycle_bit: CycleBit) -> Self {
        cycle_bit.0
    }
}
