use alloc::boxed::Box;

const NUM_OF_TRBS_IN_RING: usize = 16;

pub struct TransferRingController {
    ring: Box<TransferRing>,
}
impl TransferRingController {
    pub fn new() -> Self {
        Self {
            ring: Box::new(TransferRing::new()),
        }
    }

    pub fn head_addr(&self) -> u64 {
        self.ring.as_ref() as *const _ as u64
    }
}

#[repr(C, align(64))]
struct TransferRing([[u32; 4]; NUM_OF_TRBS_IN_RING]);
impl TransferRing {
    fn new() -> Self {
        Self([[0; 4]; NUM_OF_TRBS_IN_RING])
    }
}
