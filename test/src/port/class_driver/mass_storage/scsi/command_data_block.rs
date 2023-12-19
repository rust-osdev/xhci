// SPDX-License-Identifier: GPL-3.0-or-later

use byteorder::{BigEndian, ByteOrder};

#[derive(Copy, Clone)]
pub(in super::super) enum CommandDataBlock {
    Inquiry(Inquiry),
    ReadCapacity(ReadCapacity),
    Read10(Read10),
    Write10(Write10),
}
impl From<CommandDataBlock> for [u8; 16] {
    fn from(cdb: CommandDataBlock) -> Self {
        match cdb {
            CommandDataBlock::Inquiry(i) => i.0,
            CommandDataBlock::ReadCapacity(r) => r.0,
            CommandDataBlock::Read10(r) => r.0,
            CommandDataBlock::Write10(w) => w.0,
        }
    }
}

macro_rules! command {
    ($name:ident) => {
        #[derive(Copy, Clone)]
        pub(in super::super) struct $name([u8; 16]);
        impl $name {
            fn set_command(&mut self) -> &mut Self {
                self.0[0] = Command::$name.into();
                self
            }
        }
        impl Default for $name {
            fn default() -> Self {
                *Self([0; 16]).set_command()
            }
        }
        impl From<$name> for CommandDataBlock {
            fn from(c: $name) -> CommandDataBlock {
                CommandDataBlock::$name(c)
            }
        }
    };
}

command!(Inquiry);
impl Inquiry {
    pub(in super::super) fn new(length: u16) -> Self {
        *Self::default().set_length(length)
    }

    fn set_length(&mut self, l: u16) -> &mut Self {
        BigEndian::write_u16(&mut self.0[3..=4], l);
        self
    }
}

command!(ReadCapacity);

command!(Read10);
impl Read10 {
    pub(in super::super) fn new(lba: u32, num_of_blocks: u16) -> Self {
        *Self::default()
            .set_lba(lba)
            .set_num_of_blocks(num_of_blocks)
    }

    fn set_lba(&mut self, l: u32) -> &mut Self {
        BigEndian::write_u32(&mut self.0[2..6], l);
        self
    }

    fn set_num_of_blocks(&mut self, n: u16) -> &mut Self {
        BigEndian::write_u16(&mut self.0[7..=8], n);
        self
    }
}

command!(Write10);
impl Write10 {
    pub(in super::super) fn new(lba: u32, num_of_blocks: u16) -> Self {
        *Self::default()
            .set_lba(lba)
            .set_num_of_blocks(num_of_blocks)
    }

    fn set_lba(&mut self, l: u32) -> &mut Self {
        BigEndian::write_u32(&mut self.0[2..6], l);
        self
    }

    fn set_num_of_blocks(&mut self, n: u16) -> &mut Self {
        BigEndian::write_u16(&mut self.0[7..=8], n);
        self
    }
}

#[repr(u8)]
enum Command {
    Inquiry = 0x12,
    ReadCapacity = 0x25,
    Read10 = 0x28,
    Write10 = 0x2a,
}
impl From<Command> for u8 {
    fn from(c: Command) -> Self {
        c as u8
    }
}
