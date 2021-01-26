//! xHCI registers

use accessor::Mapper;
use capability::Capability;
use operational::Operational;
use operational::PortRegisterSet;
use runtime::InterruptRegisterSet;

pub mod capability;
pub mod doorbell;
pub mod operational;
pub mod runtime;

/// The access point to xHCI registers.
///
/// If your [`accessor::Mapper`] does not implement [`Clone`], you can generate each accessor by
/// [`Capability::new`] first, then create other accessors with the accessor to the Capability
/// Register.
pub struct Registers<M>
where
    M: Mapper + Clone,
{
    /// Host Controller Capability Register
    pub capability: accessor::Single<Capability, M>,
    /// Doorbell Array
    pub doorbell: accessor::Array<doorbell::Register, M>,
    /// Host Controller Operational Register
    pub operational: accessor::Single<Operational, M>,
    /// Port Register Set Array
    pub port_register_set: accessor::Array<PortRegisterSet, M>,
    /// Interrupt Register Set Array
    pub interrupt_register_set: accessor::Array<InterruptRegisterSet, M>,
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
    /// # Errors
    ///
    /// This method may return a [`accessor::Error::NotAligned`] error if a base address of a
    /// register is not aligned properly.
    pub unsafe fn new(mmio_base: usize, mapper: M) -> Result<Self, accessor::Error> {
        let capability = Capability::new(mmio_base, mapper.clone())?;
        let doorbell = doorbell::Register::new(mmio_base, &capability.read(), mapper.clone())?;
        let operational = Operational::new(mmio_base, capability.read().caplength, mapper.clone())?;
        let port_register_set =
            PortRegisterSet::new(mmio_base, &capability.read(), mapper.clone())?;
        let interrupt_register_set =
            InterruptRegisterSet::new(mmio_base, capability.read().rtsoff, mapper)?;

        Ok(Self {
            capability,
            doorbell,
            operational,
            port_register_set,
            interrupt_register_set,
        })
    }
}
