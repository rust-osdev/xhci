//! The xHCI Extended Capabilities
//!
//! The mutable reference of this struct implements `IntoIterator` and it iterates over the xHCI Extended Capabilities.
//!
//! # Examples
//!
//! ```no_run
//! # use core::num::NonZeroUsize;
//! # use xhci::{
//! #     accessor::Mapper, extended_capabilities, extended_capabilities::ExtendedCapability,
//! # };
//! #
//! # // The value of this constant is for showing an example. The user must get the correct base
//! # // address of the MMIO space from the PCI Configuration Space.
//! # const MMIO_BASE: usize = 0x1000;
//! #
//! # #[derive(Clone)]
//! # struct MemoryMapper;
//! # impl Mapper for MemoryMapper {
//! #     unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> NonZeroUsize {
//! #         unimplemented!()
//! #     }
//! #
//! #     fn unmap(&mut self, virt_start: usize, bytes: usize) {
//! #         unimplemented!()
//! #     }
//! # }
//! #
//! # let mapper = MemoryMapper;
//! let mut r = unsafe { xhci::Registers::new(MMIO_BASE, mapper.clone()) };
//! let mut l = unsafe {
//!     extended_capabilities::List::new(MMIO_BASE, r.capability.hccparams1.read(), mapper)
//! };
//!
//! match l {
//!     Some(mut l) => {
//!         for e in &mut l {
//!             match e {
//!                 Ok(e) => match e {
//!                     ExtendedCapability::UsbLegacySupport(u) => {}
//!                     _ => {}
//!                 },
//!                 Err(e) => {
//!                     // Currently this crate does not support this Extended Capability.
//!                 }
//!             }
//!         }
//!     }
//!     None => {
//!         // The xHC does not support the xHCI Extended Capability.
//!     }
//! }
//! ```

use super::registers::capability::CapabilityParameters1;
use accessor::single;
use accessor::Mapper;
use bit_field::BitField;
use core::convert::TryInto;
use debug::Debug;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use usb_legacy_support_capability::UsbLegacySupport;

pub use hci_extended_power_management::HciExtendedPowerManagement;
pub use xhci_extended_message_interrupt::XhciExtendedMessageInterrupt;
pub use xhci_local_memory::XhciLocalMemory;
pub use xhci_message_interrupt::XhciMessageInterrupt;
pub use xhci_supported_protocol::XhciSupportedProtocol;

pub mod debug;
pub mod hci_extended_power_management;
pub mod usb_legacy_support_capability;
pub mod xhci_extended_message_interrupt;
pub mod xhci_local_memory;
pub mod xhci_message_interrupt;
pub mod xhci_supported_protocol;

/// A struct to access xHCI Extended Capabilities.
#[derive(Debug)]
pub struct List<M>
where
    M: Mapper + Clone,
{
    base: usize,
    m: M,
}
impl<M> List<M>
where
    M: Mapper + Clone,
{
    /// Creates a new accessor to the xHCI Extended Capabilities.
    ///
    /// This method may return a [`None`] value if the xHC does not support the xHCI Extended
    /// Capabilities.
    ///
    /// # Safety
    ///
    /// The caller must ensure that each of the xHCI Extended Capabilities is accessed only through
    /// the returned accessor.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use core::num::NonZeroUsize;
    /// # use xhci::{
    /// #     accessor::Mapper, extended_capabilities, extended_capabilities::ExtendedCapability,
    /// # };
    /// #
    /// # // The value of this constant is for showing an example. The user must get the correct base
    /// # // address of the MMIO space from the PCI Configuration Space.
    /// # const MMIO_BASE: usize = 0x1000;
    /// #
    /// # #[derive(Clone)]
    /// # struct MemoryMapper;
    /// # impl Mapper for MemoryMapper {
    /// #     unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> NonZeroUsize {
    /// #         unimplemented!()
    /// #     }
    /// #
    /// #     fn unmap(&mut self, virt_start: usize, bytes: usize) {
    /// #         unimplemented!()
    /// #     }
    /// # }
    /// #
    /// # let mapper = MemoryMapper;
    /// let mut r = unsafe { xhci::Registers::new(MMIO_BASE, mapper.clone()) };
    /// let mut l = unsafe {
    ///     extended_capabilities::List::new(MMIO_BASE, r.capability.hccparams1.read(), mapper)
    /// };
    /// ```
    pub unsafe fn new(
        mmio_base: usize,
        hccparams1: CapabilityParameters1,
        mapper: M,
    ) -> Option<Self> {
        let xecp: usize = hccparams1.xhci_extended_capabilities_pointer().into();
        if xecp == 0 {
            None
        } else {
            let base = mmio_base + (xecp << 2);
            Some(Self { base, m: mapper })
        }
    }
}
impl<M> IntoIterator for &mut List<M>
where
    M: Mapper + Clone,
{
    type Item = Result<ExtendedCapability<M>, NotSupportedId>;
    type IntoIter = IterMut<M>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut::new(self)
    }
}

/// An iterator over the xHCI Extended Capability.
#[derive(Debug)]
pub struct IterMut<M>
where
    M: Mapper + Clone,
{
    current: Option<usize>,
    m: M,
}
impl<M> IterMut<M>
where
    M: Mapper + Clone,
{
    fn new(l: &List<M>) -> Self {
        Self {
            current: Some(l.base),
            m: l.m.clone(),
        }
    }
}
impl<M> Iterator for IterMut<M>
where
    M: Mapper + Clone,
{
    type Item = Result<ExtendedCapability<M>, NotSupportedId>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;

        // SAFETY: `Iter::new` guarantees that `self.current` is the correct address.
        let h: Header = unsafe { single::ReadWrite::new(current, self.m.clone()) }.read_volatile();

        self.current = if h.next() == 0 {
            None
        } else {
            Some(current + (usize::from(h.next()) << 2))
        };

        let c = unsafe { ExtendedCapability::new(current, h, self.m.clone()) };
        Some(c.ok_or_else(|| NotSupportedId(h.id())))
    }
}

/// The xHCI Extended Capability.
///
/// # Not Supported Extended Capabilities
///
/// ## xHCI I/O Virtualization Capability
///
/// This Extended Capability requires the number of VFs.
/// However, not xHCI specification but PCIe specification defines the number.
/// It is not possible to pass an argument for a specific Extended Capability.
#[derive(Debug)]
pub enum ExtendedCapability<M>
where
    M: Mapper + Clone,
{
    /// USB Legacy Support Capability.
    UsbLegacySupport(UsbLegacySupport<M>),
    /// xHCI Supported Protocol Capability.
    XhciSupportedProtocol(XhciSupportedProtocol<M>),
    /// HCI Extended Power Management Capability.
    HciExtendedPowerManagementCapability(single::ReadWrite<HciExtendedPowerManagement, M>),
    /// xHCI Message Interrupt Capability.
    XhciMessageInterrupt(XhciMessageInterrupt<M>),
    /// xHCI Local Memory Capability.
    XhciLocalMemory(XhciLocalMemory<M>),
    /// Debug Capability.
    Debug(Debug<M>),
    /// xHCI Extended Message Interrupt.
    XhciExtendedMessageInterrupt(single::ReadWrite<XhciExtendedMessageInterrupt, M>),
}
impl<M> ExtendedCapability<M>
where
    M: Mapper + Clone,
{
    unsafe fn new(base: usize, h: Header, m: M) -> Option<Self> {
        let ty = FromPrimitive::from_u8(h.id())?;
        Self::from_ty(base, ty, m)
    }

    unsafe fn from_ty(base: usize, ty: Ty, m: M) -> Option<Self> {
        let v = match ty {
            // SAFETY: `List::new` ensures that the all necessary conditions are fulfilled.
            Ty::UsbLegacySupport => UsbLegacySupport::new(base, m).into(),
            Ty::SupportedProtocol => XhciSupportedProtocol::new(base, m).into(),
            Ty::ExtendedPowerManagement => {
                single::ReadWrite::<HciExtendedPowerManagement, M>::new(base, m).into()
            }
            Ty::MessageInterrupt => XhciMessageInterrupt::new(base, m).into(),
            Ty::LocalMemory => XhciLocalMemory::new(base, m)?.into(),
            Ty::UsbDebugCapability => Debug::new(base, &m).into(),
            Ty::ExtendedMessageInterrupt => {
                single::ReadWrite::<XhciExtendedMessageInterrupt, M>::new(base, m).into()
            }
        };

        Some(v)
    }
}

/// A struct representing that the Extended Capability with the ID is not supported by this crate.
///
/// # Examples
///
/// ```
/// // The Extended Capability with the ID 7 is not supported by this crate.
/// use xhci::extended_capabilities::NotSupportedId;
///
/// NotSupportedId(7);
/// ```
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default, Debug)]
pub struct NotSupportedId(pub u8);

#[repr(transparent)]
#[derive(Copy, Clone)]
struct Header(u32);
impl Header {
    fn id(self) -> u8 {
        self.0.get_bits(0..=7).try_into().unwrap()
    }

    fn next(self) -> u8 {
        self.0.get_bits(8..=15).try_into().unwrap()
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, FromPrimitive)]
enum Ty {
    UsbLegacySupport = 1,
    SupportedProtocol = 2,
    ExtendedPowerManagement = 3,
    MessageInterrupt = 5,
    LocalMemory = 6,
    UsbDebugCapability = 10,
    ExtendedMessageInterrupt = 17,
}
