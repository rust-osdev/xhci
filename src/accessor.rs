use crate::mapper::Mapper;
use core::{convert::TryInto, marker::PhantomData, mem, ptr};
use os_units::Bytes;
use x86_64::{PhysAddr, VirtAddr};

/// An accessor to a memory region.
pub(crate) struct Accessor<T, M>
where
    T: ?Sized,
    M: Mapper,
{
    virt: VirtAddr,
    bytes: Bytes,
    mapper: M,
    _marker: PhantomData<T>,
}
impl<T, M> Accessor<T, M>
where
    M: Mapper,
{
    /// Reads a value from memory.
    pub fn read(&self) -> T {
        // SAFETY: `Accessor::new` ensures that the all necessary conditions are fulfilled.
        unsafe { ptr::read_volatile(self.virt.as_ptr()) }
    }

    /// Writes `value` to memory.
    pub fn write(&mut self, value: T) {
        // SAFETY: `Accessor::new` ensures that the all necessary conditions are fulfilled.
        unsafe { ptr::write_volatile(self.virt.as_mut_ptr(), value) }
    }

    /// Update a value on memory by reading a value from the memory, modifying the value, and writing the
    /// modified value.
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
    /// Caller must ensure that:
    ///
    /// - No two `Accessor` point to the same region, otherwise this method will
    /// create multiple mutable accessor to the same address.
    /// - `phys_base + offset.as_usize()` is aligned properly.
    pub(crate) unsafe fn new(phys_base: PhysAddr, offset: Bytes, mapper: M) -> Self {
        let base = phys_base + offset.as_usize();
        let bytes = Bytes::new(mem::size_of::<T>());
        let virt = VirtAddr::new(
            mapper
                .map_pages(phys_base.as_u64().try_into().unwrap(), bytes.as_usize())
                .try_into()
                .unwrap(),
        );

        Self {
            virt,
            bytes,
            mapper,
            _marker: PhantomData,
        }
    }
}
impl<T, M> Accessor<[T], M>
where
    M: Mapper,
{
    /// Reads the `index`th value from memory.
    ///
    /// # Panics
    ///
    /// Panics if `index > self.len()`.
    pub fn read(&self, index: usize) -> T {
        assert!(
            index < self.len(),
            "index out of bounds: the length is {} but the index is {}",
            self.len(),
            index
        );

        // SAFETY: `Accessor::new_slice` ensures that the all necessary conditions are fulfilled.
        unsafe { ptr::read_volatile(self.addr(index).as_ptr()) }
    }

    /// Writes `value` on memory as the `index`th element.
    pub fn write(&self, index: usize, value: T) {
        assert!(
            index < self.len(),
            "index out of bounds: the length is {} but the index is {}",
            self.len(),
            index
        );

        // SAFETY: `Accessor::new_slice` ensures that the all necessary conditions are fulfilled.
        unsafe { ptr::write_volatile(self.addr(index).as_mut_ptr(), value) }
    }

    /// Update the `index`th value by reading the value, modifying with `f`, and writing it on memory.
    pub fn update<U>(&self, i: usize, f: U)
    where
        U: FnOnce(&mut T),
    {
        let mut v = self.read(i);
        f(&mut v);
        self.write(i, v);
    }

    /// Returns the number of elements this accessor can access.
    pub fn len(&self) -> usize {
        self.bytes.as_usize() / mem::size_of::<T>()
    }

    /// # Safety
    ///
    /// Caller must ensure that:
    ///
    /// - No two `Accessor` point to the same region, otherwise this method will
    /// create multiple mutable accessor to the same address.
    /// - `phys_base + offset.as_usize()` is aligned properly.
    pub(crate) unsafe fn new_slice(
        phys_base: PhysAddr,
        offset: Bytes,
        len: usize,
        mapper: M,
    ) -> Self {
        let base = phys_base + offset.as_usize();
        let bytes = Bytes::new(mem::size_of::<T>() * len);
        let virt = VirtAddr::new(
            mapper
                .map_pages(phys_base.as_u64().try_into().unwrap(), bytes.as_usize())
                .try_into()
                .unwrap(),
        );

        Self {
            virt,
            bytes,
            mapper,
            _marker: PhantomData,
        }
    }

    fn addr(&self, i: usize) -> VirtAddr {
        self.virt + mem::size_of::<T>() * i
    }
}
impl<T, M> Drop for Accessor<T, M>
where
    T: ?Sized,
    M: Mapper,
{
    fn drop(&mut self) {
        self.mapper.unmap_pages(
            self.virt.as_u64().try_into().unwrap(),
            self.bytes.as_usize(),
        )
    }
}
