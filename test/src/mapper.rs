use core::num::NonZeroUsize;

#[derive(Clone, Copy, Debug)]
pub struct Mapper;
impl xhci::accessor::Mapper for Mapper {
    // UEFI sets up the identity mapping, so we don't need to do anything here.
    unsafe fn map(&mut self, physical_address: usize, _: usize) -> NonZeroUsize {
        NonZeroUsize::new(physical_address).expect("physical_address is zero")
    }

    fn unmap(&mut self, _virtual_address: usize, _size: usize) {}
}
