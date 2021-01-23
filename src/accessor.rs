//! An accessor to read, modify, and write the values of memory.

use crate::{error::Error, mapper::Mapper};
use core::fmt;
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
        if is_aligned::<T>(phys_base) {
            Ok(Self::new_aligned(phys_base, offset, mapper))
        } else {
            Err(Error::NotAligned {
                alignment: mem::align_of::<T>().try_into().unwrap(),
                address: (phys_base + offset).try_into().unwrap(),
            })
        }
    }

    /// # Safety
    ///
    /// Caller must ensure that only one accessor to the same region is created, otherwise
    /// it may cause undefined behaviors such as data race.
    unsafe fn new_aligned(phys_base: usize, offset: usize, mut mapper: M) -> Self {
        assert!(is_aligned::<T>(phys_base));

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
}
impl<T, M> fmt::Debug for Accessor<T, M>
where
    T: fmt::Debug,
    M: Mapper,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.read())
    }
}

impl<T, M> Accessor<[T], M>
where
    M: Mapper,
{
    /// Reads the `i`th element from where the accessor points.
    ///
    /// # Panics
    ///
    /// This method will panic if `i >= self.len()`
    pub fn read_at(&self, i: usize) -> T {
        assert!(i < self.len());

        // SAFETY: `Accessor::new_array` ensures that `self.addr(i)` is aligned properly.
        unsafe { ptr::read_volatile(self.addr(i) as *const _) }
    }

    /// Writes `v` to which the accessor points as the `i`th element.
    ///
    /// # Panics
    ///
    /// This method will panic if `i >= self.len()`
    pub fn write_at(&mut self, i: usize, v: T) {
        assert!(i < self.len());

        // SAFETY: `Accessor::new_array` ensures that `self.addr(i)` is aligned properly.
        unsafe { ptr::write_volatile(self.addr(i) as *mut _, v) }
    }

    /// Returns the length of the element which this accessor points.
    pub fn len(&self) -> usize {
        self.bytes / mem::size_of::<T>()
    }

    /// # Safety
    ///
    /// Caller must ensure that only one accessor to the same region is created, otherwise
    /// undefined behaviors such as data race may occur.
    pub(crate) unsafe fn new_array(
        phys_base: usize,
        offset: usize,
        len: usize,
        mapper: M,
    ) -> Result<Self, Error> {
        if is_aligned::<T>(phys_base) {
            Ok(Self::new_array_aligned(phys_base, offset, len, mapper))
        } else {
            Err(Error::NotAligned {
                alignment: (mem::align_of::<T>()).try_into().unwrap(),
                address: (phys_base + offset).try_into().unwrap(),
            })
        }
    }

    /// # Safety
    ///
    /// Caller must ensure that only one accessor to the same region is created, otherwise
    /// undefined behaviors such as data race may occur.
    unsafe fn new_array_aligned(
        phys_base: usize,
        offset: usize,
        len: usize,
        mut mapper: M,
    ) -> Self {
        assert!(is_aligned::<T>(phys_base));

        let phys_base = phys_base + offset;
        let bytes = mem::size_of::<T>() * len;
        let virt = mapper.map(phys_base, bytes);

        Self {
            virt,
            bytes,
            _marker: PhantomData,
            mapper,
        }
    }

    fn addr(&self, i: usize) -> usize {
        self.virt + mem::size_of::<T>() * i
    }
}
impl<T, M> fmt::Debug for Accessor<[T], M>
where
    T: fmt::Debug,
    M: Mapper,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}
impl<'a, T, M> IntoIterator for &'a Accessor<[T], M>
where
    M: Mapper,
{
    type Item = T;
    type IntoIter = Iter<'a, T, M>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

/// An iterator over a value of `T`.
pub struct Iter<'a, T, M>
where
    M: Mapper,
{
    a: &'a Accessor<[T], M>,
    i: usize,
}
impl<'a, T, M> Iter<'a, T, M>
where
    M: Mapper,
{
    fn new(a: &'a Accessor<[T], M>) -> Self {
        Self { a, i: 0 }
    }
}
impl<'a, T, M> Iterator for Iter<'a, T, M>
where
    M: Mapper,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.a.len() {
            let t = self.a.read_at(self.i);
            self.i += 1;
            Some(t)
        } else {
            None
        }
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

fn is_aligned<T>(phys_base: usize) -> bool {
    phys_base % mem::align_of::<T>() == 0
}
