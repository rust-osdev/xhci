//! Accessors for 64-bit xHCI registers.

use accessor::marker::{self, AccessorTypeSpecifier, Readable, Writable};
use accessor::Mapper;
use core::fmt;
use core::marker::PhantomData;
use core::mem::{align_of, size_of};
use core::ptr;

/// The access width and ordering used for 64-bit xHCI registers.
///
/// xHCI permits 64-bit registers to be accessed as two 32-bit operations on
/// platforms that cannot issue 64-bit MMIO accesses. Some controllers require
/// a specific order for those operations.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Access64 {
    /// Access the register with one 64-bit operation.
    #[default]
    Native,
    /// Access the low 32 bits before the high 32 bits.
    LowHigh,
    /// Access the high 32 bits before the low 32 bits.
    HighLow,
}

/// A readable and writable 64-bit register accessor.
pub type ReadWrite<T, M> = Generic<T, M, marker::ReadWrite>;

/// A read-only 64-bit register accessor.
pub type ReadOnly<T, M> = Generic<T, M, marker::ReadOnly>;

/// An accessor for a 64-bit xHCI register.
pub struct Generic<T, M, A>
where
    M: Mapper,
    A: AccessorTypeSpecifier,
{
    virt: usize,
    bytes: usize,
    access: Access64,
    mapper: M,
    _register: PhantomData<T>,
    _accessor: PhantomData<A>,
}

impl<T, M, A> Generic<T, M, A>
where
    M: Mapper,
    A: AccessorTypeSpecifier,
{
    pub(crate) unsafe fn new(phys_base: usize, access: Access64, mut mapper: M) -> Self {
        assert_eq!(size_of::<T>(), size_of::<u64>());
        let alignment = match access {
            Access64::Native => align_of::<T>(),
            Access64::LowHigh | Access64::HighLow => align_of::<u32>(),
        };
        assert_eq!(phys_base % alignment, 0, "base is not aligned");

        let bytes = size_of::<T>();
        let virt = mapper.map(phys_base, bytes).get();
        Self {
            virt,
            bytes,
            access,
            mapper,
            _register: PhantomData,
            _accessor: PhantomData,
        }
    }
}

impl<T, M, A> Generic<T, M, A>
where
    T: From<u64>,
    M: Mapper,
    A: AccessorTypeSpecifier + Readable,
{
    /// Reads the register using the configured access width and ordering.
    pub fn read_volatile(&self) -> T {
        let raw = unsafe {
            match self.access {
                Access64::Native => return ptr::read_volatile(self.virt as *const T),
                Access64::LowHigh => {
                    let low = ptr::read_volatile(self.virt as *const u32);
                    let high = ptr::read_volatile((self.virt + 4) as *const u32);
                    u64::from(low) | (u64::from(high) << 32)
                }
                Access64::HighLow => {
                    let high = ptr::read_volatile((self.virt + 4) as *const u32);
                    let low = ptr::read_volatile(self.virt as *const u32);
                    u64::from(low) | (u64::from(high) << 32)
                }
            }
        };
        raw.into()
    }

    /// Alias of [`Generic::read_volatile`].
    #[deprecated(since = "0.9.3", note = "use `read_volatile`")]
    pub fn read(&self) -> T {
        self.read_volatile()
    }
}

impl<T, M, A> Generic<T, M, A>
where
    T: Into<u64>,
    M: Mapper,
    A: AccessorTypeSpecifier + Writable,
{
    /// Writes the register using the configured access width and ordering.
    pub fn write_volatile(&mut self, value: T) {
        unsafe {
            match self.access {
                Access64::Native => ptr::write_volatile(self.virt as *mut T, value),
                Access64::LowHigh => {
                    let raw = value.into();
                    let low = u32::try_from(raw & u64::from(u32::MAX)).unwrap();
                    let high = u32::try_from(raw >> 32).unwrap();
                    ptr::write_volatile(self.virt as *mut u32, low);
                    ptr::write_volatile((self.virt + 4) as *mut u32, high);
                }
                Access64::HighLow => {
                    let raw = value.into();
                    let low = u32::try_from(raw & u64::from(u32::MAX)).unwrap();
                    let high = u32::try_from(raw >> 32).unwrap();
                    ptr::write_volatile((self.virt + 4) as *mut u32, high);
                    ptr::write_volatile(self.virt as *mut u32, low);
                }
            }
        }
    }

    /// Alias of [`Generic::write_volatile`].
    #[deprecated(since = "0.9.3", note = "use `write_volatile`")]
    pub fn write(&mut self, value: T) {
        self.write_volatile(value);
    }
}

impl<T, M, A> Generic<T, M, A>
where
    T: From<u64> + Into<u64>,
    M: Mapper,
    A: AccessorTypeSpecifier + Readable + Writable,
{
    /// Updates the register using the configured access width and ordering.
    pub fn update_volatile<F>(&mut self, update: F)
    where
        F: FnOnce(&mut T),
    {
        let mut value = self.read_volatile();
        update(&mut value);
        self.write_volatile(value);
    }

    /// Alias of [`Generic::update_volatile`].
    #[deprecated(since = "0.9.3", note = "use `update_volatile`")]
    pub fn update<F>(&mut self, update: F)
    where
        F: FnOnce(&mut T),
    {
        self.update_volatile(update);
    }
}

impl<T, M, A> fmt::Debug for Generic<T, M, A>
where
    T: From<u64> + fmt::Debug,
    M: Mapper,
    A: AccessorTypeSpecifier + Readable,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.read_volatile().fmt(f)
    }
}

impl<T, M, A> Drop for Generic<T, M, A>
where
    M: Mapper,
    A: AccessorTypeSpecifier,
{
    fn drop(&mut self) {
        self.mapper.unmap(self.virt, self.bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::num::NonZeroUsize;

    #[derive(Clone, Copy)]
    struct IdentityMapper;

    impl Mapper for IdentityMapper {
        unsafe fn map(&mut self, phys_start: usize, _bytes: usize) -> NonZeroUsize {
            NonZeroUsize::new(phys_start).unwrap()
        }

        fn unmap(&mut self, _virt_start: usize, _bytes: usize) {}
    }

    #[repr(transparent)]
    #[derive(Debug, Eq, PartialEq)]
    struct TestRegister(u64);

    impl From<u64> for TestRegister {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }

    impl From<TestRegister> for u64 {
        fn from(value: TestRegister) -> Self {
            value.0
        }
    }

    #[test]
    fn access_modes_read_and_write_the_complete_register() {
        for access in [Access64::Native, Access64::LowHigh, Access64::HighLow] {
            let mut value = 0u64;
            let pointer = ptr::from_mut(&mut value);
            let mut register = unsafe {
                ReadWrite::<TestRegister, IdentityMapper>::new(
                    pointer as usize,
                    access,
                    IdentityMapper,
                )
            };

            register.write_volatile(TestRegister(0x1122_3344_5566_7788));
            assert_eq!(value, 0x1122_3344_5566_7788);
            assert_eq!(
                register.read_volatile(),
                TestRegister(0x1122_3344_5566_7788)
            );

            register.update_volatile(|value| value.0 ^= u64::MAX);
            assert_eq!(value, 0xeedd_ccbb_aa99_8877);
        }
    }
}
