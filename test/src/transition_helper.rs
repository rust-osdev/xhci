use alloc::boxed::Box;
use alloc::vec;
use core::fmt;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::ops::Deref;
use core::ops::DerefMut;
use os_units::Bytes;
use x86_64::PhysAddr;

pub struct BoxWrapper<T: ?Sized> {
    inner: Box<T>,
    bytes: u64,
}
impl<T: ?Sized> BoxWrapper<T> {
    pub fn new(inner: Box<T>, bytes: u64) -> Self {
        Self { inner, bytes }
    }

    pub fn phys_addr(&self) -> PhysAddr {
        PhysAddr::new(self.inner.as_ref() as *const T as *const u8 as u64)
    }

    pub fn bytes(&self) -> Bytes {
        Bytes::new(self.bytes as _)
    }
}
impl<T: Clone> BoxWrapper<[T]> {
    pub fn new_slice(init: T, len: usize) -> Self {
        Self::new(
            vec![init; len].into_boxed_slice(),
            (len * core::mem::size_of::<T>()) as u64,
        )
    }
}
impl<T: ?Sized> Deref for BoxWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<T: ?Sized> DerefMut for BoxWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl<T> From<T> for BoxWrapper<T> {
    fn from(inner: T) -> Self {
        Self::new(Box::new(inner), core::mem::size_of::<T>() as u64)
    }
}
impl<T: Default> Default for BoxWrapper<T> {
    fn default() -> Self {
        Self::new(Box::new(T::default()), core::mem::size_of::<T>() as u64)
    }
}
impl<T: Debug + ?Sized> Debug for BoxWrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
