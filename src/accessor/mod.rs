//! Accessors to access physical address

pub mod array;
pub mod single;

pub use array::Array;
pub use single::Single;

fn is_aligned<T>(phys_base: usize) -> bool {
    phys_base % core::mem::align_of::<T>() == 0
}
