// SPDX-License-Identifier: GPL-3.0-or-later

mod non_bridge;

use super::{
    bar,
    common::{BridgeType, Common},
    Bar, RegisterIndex, Registers,
};
use x86_64::PhysAddr;

#[derive(Debug)]
pub(super) enum TypeSpec<'a> {
    NonBridge(non_bridge::TypeSpec<'a>),
}

impl<'a> TypeSpec<'a> {
    pub(super) fn new(registers: &'a Registers, common: &Common<'_>) -> Self {
        match common.bridge_type() {
            BridgeType::NonBridge => TypeSpec::NonBridge(non_bridge::TypeSpec::new(registers)),
            e => panic!("Not implemented: {:?}\ncommon:{:?}", e, common),
        }
    }

    pub(super) fn base_address(&self, index: bar::Index) -> PhysAddr {
        let TypeSpec::NonBridge(non_bridge) = self;
        non_bridge.base_addr(index)
    }
}
