// SPDX-License-Identifier: GPL-3.0-or-later

pub(super) mod command_data_block;
pub(super) mod response;

use command_data_block::CommandDataBlock;
use derive_builder::Builder;
use num_derive::FromPrimitive;

#[repr(C, packed)]
pub(super) struct CommandBlockWrapper {
    header: CommandBlockWrapperHeader,
    data: [u8; 16],
}
impl CommandBlockWrapper {
    pub(super) fn new(header: CommandBlockWrapperHeader, data: CommandDataBlock) -> Self {
        Self {
            header,
            data: data.into(),
        }
    }
}

#[repr(C, packed)]
#[derive(Builder)]
#[builder(no_std)]
pub(super) struct CommandBlockWrapperHeader {
    #[builder(default = "CommandBlockWrapperHeader::SIGNATURE")]
    signature: u32,
    #[builder(default = "0")]
    tag: u32,
    transfer_length: u32,
    flags: Flags,
    lun: u8,
    command_len: u8,
}
impl CommandBlockWrapperHeader {
    const SIGNATURE: u32 = 0x4342_5355;
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub(super) enum Flags {
    Out = 0,
    In = 0x80,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub(super) struct CommandStatusWrapper {
    signature: u32,
    tag: u32,
    data_residue: u32,
    status: u8,
}
impl CommandStatusWrapper {
    pub(super) fn check_corruption(&self) {
        const USBS: u32 = 0x5342_5355;
        let signature = self.signature;

        assert_eq!(
            signature, USBS,
            "The signature of the Command Status Wrapper is wrong."
        );
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
pub(super) enum Status {
    Good = 0,
}
impl Default for Status {
    fn default() -> Self {
        Self::Good
    }
}
