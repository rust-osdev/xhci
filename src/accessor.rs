use crate::mapper::Mapper;
use core::{convert::TryInto, marker::PhantomData, mem, ptr};
use os_units::Bytes;
use x86_64::{PhysAddr, VirtAddr};

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

    pub(crate) fn read(&self) -> T {
        // SAFETY: `Accessor::new` ensures that the all necessary conditions are fulfilled.
        unsafe { ptr::read_volatile(self.virt.as_ptr()) }
    }

    pub(crate) fn write(&mut self, v: T) {
        // SAFETY: `Accessor::new` ensures that the all necessary conditions are fulfilled.
        unsafe { ptr::write_volatile(self.virt.as_mut_ptr(), v) }
    }

    pub(crate) fn update<U>(&mut self, f: U)
    where
        U: FnOnce(&mut T),
    {
        let mut v = self.read();
        f(&mut v);
        self.write(v);
    }
}
impl<T, M> Accessor<[T], M>
where
    M: Mapper,
{
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

    pub(crate) fn read(&self, i: usize) -> T {
        assert!(
            i < self.len(),
            "index out of bounds: the length is {} but the index is {}",
            self.len(),
            i
        );

        // SAFETY: `Accessor::new_slice` ensures that the all necessary conditions are fulfilled.
        unsafe { ptr::read_volatile(self.addr(i).as_ptr()) }
    }

    pub(crate) fn write(&self, i: usize, v: T) {
        assert!(
            i < self.len(),
            "index out of bounds: the length is {} but the index is {}",
            self.len(),
            i
        );

        // SAFETY: `Accessor::new_slice` ensures that the all necessary conditions are fulfilled.
        unsafe { ptr::write_volatile(self.addr(i).as_mut_ptr(), v) }
    }

    pub(crate) fn update<U>(&self, i: usize, f: U)
    where
        U: FnOnce(&mut T),
    {
        let mut v = self.read(i);
        f(&mut v);
        self.write(i, v);
    }

    fn addr(&self, i: usize) -> VirtAddr {
        self.virt + mem::size_of::<T>() * i
    }

    fn len(&self) -> usize {
        self.bytes.as_usize() / mem::size_of::<T>()
    }
}
