//! Accessor for an array

use crate::{error::Error, mapper::Mapper};
use core::{convert::TryInto, fmt, marker::PhantomData, mem, ptr};

/// An accessor to read, modify, and write an array of some type on memory.
///
/// All operations are done volatilely.
pub struct Array<T, M>
where
    T: Copy,
    M: Mapper,
{
    virt: usize,
    len: usize,
    _marker: PhantomData<T>,
    mapper: M,
}

impl<T, M> Array<T, M>
where
    T: Copy,
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
        self.len
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
        if super::is_aligned::<T>(phys_base) {
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
        assert!(super::is_aligned::<T>(phys_base));

        let phys_base = phys_base + offset;
        let bytes = mem::size_of::<T>() * len;
        let virt = mapper.map(phys_base, bytes);

        Self {
            virt,
            len,
            _marker: PhantomData,
            mapper,
        }
    }

    fn addr(&self, i: usize) -> usize {
        self.virt + mem::size_of::<T>() * i
    }

    fn bytes(&self) -> usize {
        mem::size_of::<T>() * self.len
    }
}
impl<T, M> fmt::Debug for Array<T, M>
where
    T: Copy + fmt::Debug,
    M: Mapper,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}
impl<'a, T, M> IntoIterator for &'a Array<T, M>
where
    T: Copy,
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
    T: Copy,
    M: Mapper,
{
    a: &'a Array<T, M>,
    i: usize,
}
impl<'a, T, M> Iter<'a, T, M>
where
    T: Copy,
    M: Mapper,
{
    fn new(a: &'a Array<T, M>) -> Self {
        Self { a, i: 0 }
    }
}
impl<'a, T, M> Iterator for Iter<'a, T, M>
where
    T: Copy,
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

impl<T, M> Drop for Array<T, M>
where
    T: Copy,
    M: Mapper,
{
    fn drop(&mut self) {
        self.mapper.unmap(self.virt, self.bytes());
    }
}
