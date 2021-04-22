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
//!                     ExtendedCapability::UsbLegacySupportCapability(u) => {}
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
use accessor::Mapper;
use accessor::Single;
use bit_field::BitField;
use core::convert::TryInto;

pub use hci_extended_power_management::HciExtendedPowerManagementCapability;
pub use usb_legacy_support_capability::UsbLegacySupportCapability;
pub use xhci_supported_protocol::XhciSupportedProtocol;

pub mod hci_extended_power_management;
pub mod usb_legacy_support_capability;
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
        let h: Header = unsafe { accessor::Single::new(current, self.m.clone()) }.read();

        self.current = if h.next() == 0 {
            None
        } else {
            Some(current + (usize::from(h.next()) << 2))
        };

        Some(match h.id() {
            // SAFETY: `List::new` ensures that the all necessary conditions are fulfilled.
            1 => Ok(ExtendedCapability::UsbLegacySupportCapability(unsafe {
                Single::new(current, self.m.clone())
            })),
            2 => Ok(ExtendedCapability::XhciSupportedProtocol(unsafe {
                XhciSupportedProtocol::new(current, self.m.clone())
            })),
            3 => Ok(ExtendedCapability::HciExtendedPowerManagementCapability(
                unsafe { Single::new(current, self.m.clone()) },
            )),
            e => Err(NotSupportedId(e)),
        })
    }
}

/// The xHCI Extended Capability.
#[non_exhaustive]
#[derive(Debug)]
pub enum ExtendedCapability<M>
where
    M: Mapper + Clone,
{
    /// USB Legacy Support Capability.
    UsbLegacySupportCapability(Single<UsbLegacySupportCapability, M>),
    /// xHCI Supported Protocol Capability.
    XhciSupportedProtocol(XhciSupportedProtocol<M>),
    /// HCI Extended Power Management Capability.
    HciExtendedPowerManagementCapability(Single<HciExtendedPowerManagementCapability, M>),
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
