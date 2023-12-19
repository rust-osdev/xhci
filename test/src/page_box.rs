use core::alloc::Layout;
use core::fmt;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::marker::PhantomData;
use core::ops::Deref;
use core::ops::DerefMut;
use core::slice;
use os_units::Bytes;
use x86_64::PhysAddr;
use x86_64::VirtAddr;

/// A `Box`-like type that locates the inner value at a 4K bytes page boundary.
///
/// xHCI specification prohibits some structures from crossing the page
/// boundary. Here, the size of a page is determined by Page Size Register (See
/// 5.4.3 of the spec). However, the minimum size of a page is 4K bytes, meaning
/// that keeping a structure within a 4K bytes page is always safe. It is very
/// costly, but at least it works.
pub struct PageBox<T: ?Sized> {
    addr: VirtAddr,
    layout: Layout,
    _marker: PhantomData<T>,
}
impl<T: ?Sized> PageBox<T> {
    pub fn phys_addr(&self) -> PhysAddr {
        // We assume the identity mapping set up by UEFI firmware.
        PhysAddr::new(self.addr.as_u64())
    }

    pub fn bytes(&self) -> Bytes {
        Bytes::from(self.layout.size())
    }
}
impl<T: Clone> PageBox<[T]> {
    pub fn new_slice(init: T, len: usize) -> Self {
        let bytes = Bytes::from(len * core::mem::size_of::<T>());
        let align = 4096.max(core::mem::align_of::<T>());

        let layout = Layout::from_size_align(bytes.as_usize(), align);
        let layout = layout.unwrap_or_else(|_| {
            panic!(
                "Failed to create a layout for {} bytes with {} bytes alignment",
                bytes.as_usize(),
                align
            )
        });

        // SAFETY: `Layout::from_size_align` guarantees that the layout is valid.
        let addr = unsafe { alloc::alloc::alloc(layout) };

        // SAFETY: Safe as the address is well-aligned.
        unsafe {
            let mut slice = slice::from_raw_parts_mut(addr as *mut T, len);
            for i in 0..len {
                slice[i] = init.clone();
            }
        };

        Self {
            addr: VirtAddr::new(addr as u64),
            layout,
            _marker: PhantomData,
        }
    }
}
impl<T> Deref for PageBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: Safe as the address is well-aligned.
        unsafe { &*self.addr.as_ptr() }
    }
}
impl<T> Deref for PageBox<[T]> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        // SAFETY: Safe as the address is well-aligned and the memory is allocated.
        unsafe { slice::from_raw_parts(self.addr.as_ptr(), self.bytes().as_usize()) }
    }
}
impl<T> DerefMut for PageBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Safe as the address is well-aligned.
        unsafe { &mut *self.addr.as_mut_ptr() }
    }
}
impl<T> DerefMut for PageBox<[T]> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Safe as the address is well-aligned and the memory is allocated.
        unsafe { slice::from_raw_parts_mut(self.addr.as_mut_ptr(), self.bytes().as_usize()) }
    }
}
impl<T> From<T> for PageBox<T> {
    fn from(inner: T) -> Self {
        let bytes = Bytes::from(core::mem::size_of::<T>());
        let align = 4096.max(core::mem::align_of::<T>());

        let layout = Layout::from_size_align(bytes.as_usize(), align);
        let layout = layout.unwrap_or_else(|_| {
            panic!(
                "Failed to create a layout for {} bytes with {} bytes alignment",
                bytes.as_usize(),
                align
            )
        });

        // SAFETY: `Layout::from_size_align` guarantees that the layout is valid.
        let addr = unsafe { alloc::alloc::alloc(layout) };

        // SAFETY: Safe as the address is well-aligned.
        unsafe { core::ptr::write(addr as *mut T, inner) };

        Self {
            addr: VirtAddr::new(addr as u64),
            layout,
            _marker: PhantomData,
        }
    }
}
impl<T: Default> Default for PageBox<T> {
    fn default() -> Self {
        let x: T = Default::default();

        Self::from(x)
    }
}
impl<T: Debug + ?Sized> Debug for PageBox<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}
impl<T: ?Sized> Drop for PageBox<T> {
    fn drop(&mut self) {
        // SAFETY: `Layout::from_size_align` guarantees that the layout is valid.
        unsafe { alloc::alloc::dealloc(self.addr.as_mut_ptr(), self.layout) }
    }
}
