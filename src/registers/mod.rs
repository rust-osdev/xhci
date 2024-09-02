//! xHCI registers

use accessor::array;
use accessor::Mapper;

pub use capability::Capability;
pub use doorbell::Doorbell;
pub use operational::{Operational, PortRegisterSet};
pub use runtime::InterrupterRegisterSet;
pub use runtime::Runtime;

pub mod capability;
pub mod doorbell;
pub mod operational;
pub mod runtime;

/// The access point to xHCI registers.
#[derive(Debug)]
pub struct Registers<M>
where
    M: Mapper + Clone,
{
    /// Host Controller Capability Register
    pub capability: Capability<M>,
    /// Doorbell Array
    pub doorbell: array::ReadWrite<Doorbell, M>,
    /// Host Controller Operational Register
    pub operational: Operational<M>,
    /// Port Register Set Array
    pub port_register_set: array::ReadWrite<PortRegisterSet, M>,
    /// Runtime Registers
    pub runtime: Runtime<M>,
    /// Interrupter Register Set Array
    pub interrupter_register_set: InterrupterRegisterSet<M>,
}
impl<M> Registers<M>
where
    M: Mapper + Clone,
{
    /// Creates an instance of [`Registers`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the xHCI registers are accessed only through this struct.
    ///
    /// # Panics
    ///
    /// This method panics if `mmio_base` is not aligned correctly.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use core::num::NonZeroUsize;
    /// use xhci::accessor::Mapper;
    ///
    /// // This MMIO base address is for showing an example. The user must get the correct MMIO
    /// // address from the PCI configuration space.
    /// const MMIO_BASE: usize = 0x1000;
    ///
    /// // This `Mapper` implementation is also for showing an example. The user must implement a
    /// // correct mapper.
    /// #[derive(Clone)]
    /// struct MemoryMapper;
    /// impl Mapper for MemoryMapper {
    ///     unsafe fn map(&mut self, phys_base: usize, bytes: usize) -> NonZeroUsize {
    ///         unimplemented!()
    ///     }
    ///
    ///     fn unmap(&mut self, virt_base: usize, bytes: usize) {
    ///         unimplemented!()
    ///     }
    /// }
    ///
    /// let mapper = MemoryMapper;
    /// let r = unsafe { xhci::Registers::new(MMIO_BASE, mapper) };
    /// ```
    pub unsafe fn new(mmio_base: usize, mapper: M) -> Self {
        let capability = Capability::new(mmio_base, &mapper);
        let doorbell = Doorbell::new(mmio_base, &capability, mapper.clone());
        let operational =
            Operational::new(mmio_base, capability.caplength.read_volatile(), &mapper);
        let port_register_set = PortRegisterSet::new(mmio_base, &capability, mapper.clone());
        let runtime = Runtime::new(mmio_base, capability.rtsoff.read_volatile(), mapper.clone());
        let interrupter_register_set =
            InterrupterRegisterSet::new(mmio_base, capability.rtsoff.read_volatile(), mapper);

        Self {
            capability,
            doorbell,
            operational,
            port_register_set,
            runtime,
            interrupter_register_set,
        }
    }
}
