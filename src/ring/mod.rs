//! TRB Ring.

pub mod trb;

use trb::event;
use core::ops::{Index, IndexMut};

/// The Event Ring Segment Table entry.
/// This plays the same role as an array pointer, and require special care to guarantee memory safety.
///
/// For example, the entry do not implement `Drop` trait, so the user should manually free its memory.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct EventRingSegmentTableEntry([u32; 4]);

impl EventRingSegmentTableEntry {
    /// Create new segment table entry from base address `base` and entry count `len`.
    /// `len` should be the entry count, not the size in bytes.
    ///
    /// # Panics
    ///
    /// This method will panic if `len >= 4096`.
    pub unsafe fn new(base: *const event::TRB, len: usize) -> Self {
        let size_in_bytes = len * trb::BYTES;
        assert!(size_in_bytes <= u16::MAX as usize);

        let mut entry = Self([0; 4]);
        entry
            .set_ring_segment_base_address(base as usize as u64)
            .set_ring_segment_size((len * trb::BYTES) as u16);
        entry
    }

    /// Create new segment table entry from a event::TRB buffer.
    pub unsafe fn from_buf(buf: &[event::TRB]) -> Self {
        Self::new(buf.as_ptr(), buf.len())
    }

    /// Returns the entry count of the segment.
    pub fn len(&self) -> usize {
        return self.ring_segment_size() as usize / trb::BYTES;
    }

    /// Returns the slice that this entry is representing.
    pub fn as_slice(&self) -> &[event::TRB] {
        unsafe {
            let base = self.ring_segment_base_address() as usize as *const _;
            let len = self.len();

            core::slice::from_raw_parts(base, len)
        }
    }

    /// Returns the mutable slice that this entry is representing.
    pub fn as_mut_slice(&mut self) -> &mut [event::TRB] {
        unsafe {
            let base = self.ring_segment_base_address() as usize as *mut _;
            let len = self.len();

            core::slice::from_raw_parts_mut(base, len)
        }
    }
}
impl EventRingSegmentTableEntry {
    rw_double_zero_trailing!(
        pub, self,
        self.0; [0, 1]; 6~; "64-byte aligned",
        ring_segment_base_address,
        "Ring Segment Base Address",
        32, u64
    );

    rw_field!(
        pub, self,
        self.0[2]; 0..=15,
        ring_segment_size,
        "Ring Segment Size (in bytes)",
        u16
    );

    /// Returns the value of the ring segment end address.
    pub fn ring_segment_bound_address(&self) -> u64 {
        self.ring_segment_base_address() + (self.ring_segment_size() as u64)
    }
}
impl Index<usize> for EventRingSegmentTableEntry {
    type Output = event::TRB;

    fn index(&self, index: usize) -> &Self::Output {
        let slice = self.as_slice();

        &slice[index]
    }
}
impl IndexMut<usize> for EventRingSegmentTableEntry {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let slice = self.as_mut_slice();

        &mut slice[index]
    }
}
