//! An accessor to read, modify, and write the values of memory.

use crate::{error::Error, mapper::Mapper};
use core::{convert::TryInto, mem};
use core::{marker::PhantomData, ptr};

/// An accessor to read, modify, and write the values of memory.
///
/// All operations are done volatilely.
pub struct Accessor<T, M>
where
    T: ?Sized,
    M: Mapper,
{
    virt: usize,
    bytes: usize,
    _marker: PhantomData<T>,
    mapper: M,
}
impl<T, M> Accessor<T, M>
where
    M: Mapper,
{
    /// Reads a value from where the accessor points.
    pub fn read(&self) -> T {
        // SAFETY: `Accessor::new` ensures that `self.virt` is aligned properly.
        unsafe { ptr::read_volatile(self.virt as *const _) }
    }

    /// Writes a value to where the accessor points.
    pub fn write(&mut self, v: T) {
        // SAFETY: `Accessor::new` ensures that `self.virt` is aligned properly.
        unsafe { ptr::write_volatile(self.virt as *mut _, v) }
    }

    /// Updates a value which the accessor points by reading, modifying, and writing.
    ///
    /// Note that some fields of xHCI registers (e.g. the Command Ring Pointer field of the Command
    /// Ring Control Register) may return 0 regardless of the actual value of the
    /// fields. For these registers, this operation should be called only once.
    pub fn update<U>(&mut self, f: U)
    where
        U: FnOnce(&mut T),
    {
        let mut v = self.read();
        f(&mut v);
        self.write(v);
    }

    /// # Safety
    ///
    /// Caller must ensure that only one accessor to the same region is created, otherwise
    /// it may cause undefined behaviors such as data race.
    pub(crate) unsafe fn new(phys_base: usize, offset: usize, mapper: M) -> Result<Self, Error> {
        if Self::is_aligned(phys_base) {
            Ok(Self::new_aligned(phys_base, offset, mapper))
        } else {
            Err(Error::NotAligned {
                alignment: mem::align_of::<T>().try_into().unwrap(),
                address: phys_base.try_into().unwrap(),
            })
        }
    }

    /// # Safety
    ///
    /// Caller must ensure that only one accessor to the same region is created, otherwise
    /// it may cause undefined behaviors such as data race.
    unsafe fn new_aligned(phys_base: usize, offset: usize, mut mapper: M) -> Self {
        assert!(Self::is_aligned(phys_base));

        let phys_base = phys_base + offset;
        let bytes = mem::size_of::<T>();
        let virt = mapper.map(phys_base, bytes);

        Self {
            virt,
            bytes,
            _marker: PhantomData,
            mapper,
        }
    }

    fn is_aligned(phys_base: usize) -> bool {
        phys_base % mem::align_of::<T>() == 0
    }
}

impl<T, M> Drop for Accessor<T, M>
where
    T: ?Sized,
    M: Mapper,
{
    fn drop(&mut self) {
        self.mapper.unmap(self.virt, self.bytes);
    }
}
