use core::{
    ops::{Index, IndexMut},
    slice,
};
use qemu_print::qemu_println;
use x86_64::PhysAddr;

use crate::page_box::PageBox;

#[derive(Debug)]
pub struct SegmentTable(PageBox<[Entry]>);
impl SegmentTable {
    pub fn new(len: usize) -> Self {
        qemu_println!("SegmentTable::new({})", len);
        Self(PageBox::new_slice(Entry::null(), len))
    }

    pub fn phys_addr(&self) -> PhysAddr {
        self.0.phys_addr()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Entry> {
        self.0.iter_mut()
    }
}
impl Index<usize> for SegmentTable {
    type Output = Entry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for SegmentTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
impl<'a> IntoIterator for &'a mut SegmentTable {
    type Item = &'a mut Entry;
    type IntoIter = slice::IterMut<'a, Entry>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct Entry {
    base_address: u64,
    segment_size: u64,
}
impl Entry {
    // Although the size of segment_size is u64, bits 16:63 are reserved.
    pub fn set(&mut self, addr: PhysAddr, size: u16) {
        self.base_address = addr.as_u64();
        self.segment_size = size.into();
    }

    fn null() -> Self {
        Self {
            base_address: 0,
            segment_size: 0,
        }
    }
}
