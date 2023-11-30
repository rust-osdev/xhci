use bit_field::BitField;
use x86_64::instructions::port::PortRead;
use x86_64::instructions::port::PortWrite;

pub fn xhci_exists() -> bool {
    for device in 0..=31 {
        for bus in 0..=255 {
            let config_address_reader = unsafe { ConfigSpaceReader::new(0, device, bus) };
            let config_space = unsafe { ConfigSpace::new(config_address_reader) };

            if config_space.vendor_id() == 0xffff {
                continue;
            }

            if config_space.base_class() == 0x0c
                && config_space.sub_class() == 0x03
                && config_space.interface() == 0x30
            {
                return true;
            }
        }
    }

    false
}

struct ConfigSpace {
    address: ConfigSpaceReader,
}
impl ConfigSpace {
    /// # Safety
    ///
    /// `address` must be a valid address.
    unsafe fn new(address: ConfigSpaceReader) -> Self {
        Self { address }
    }

    fn vendor_id(&self) -> u16 {
        let result = unsafe { self.address.read(0) };

        result.get_bits(0..=15) as u16
    }

    fn base_class(&self) -> u8 {
        let result = unsafe { self.address.read(2) };

        result.get_bits(24..=31) as u8
    }

    fn sub_class(&self) -> u8 {
        let result = unsafe { self.address.read(2) };

        result.get_bits(16..=23) as u8
    }

    fn interface(&self) -> u8 {
        let result = unsafe { self.address.read(2) };

        result.get_bits(8..=15) as u8
    }
}

struct ConfigSpaceReader {
    function: u8,
    device: u8,
    bus: u8,
}
impl ConfigSpaceReader {
    const CONFIG_ADDRESS: u16 = 0xcf8;
    const CONFIG_DATA: u16 = 0xcfc;

    /// # Safety
    ///
    /// `function`, `device`, and `bus` must be valid.
    unsafe fn new(function: u8, device: u8, bus: u8) -> Self {
        assert!(function < 8, "function must be less than 8");
        assert!(device < 32, "device must be less than 32");

        Self {
            function,
            device,
            bus,
        }
    }

    unsafe fn read(&self, offset: u8) -> u32 {
        assert!(offset < 32, "offset must be less than 32");

        unsafe { PortWrite::write_to_port(Self::CONFIG_ADDRESS, self.as_u32(offset)) };
        unsafe { PortRead::read_from_port(Self::CONFIG_DATA) }
    }

    fn as_u32(&self, offset: u8) -> u32 {
        let mut result = 0;

        result.set_bits(2..=7, offset.into());
        result.set_bits(8..=10, self.function.into());
        result.set_bits(11..=15, self.device.into());
        result.set_bits(16..=23, self.bus.into());
        result.set_bit(31, true);

        result
    }
}
