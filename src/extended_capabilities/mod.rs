//! The xHCI Extended Capabilities

use super::registers::capability::CapabilityParameters1;
use accessor::Mapper;
use bit_field::BitField;
use core::convert::TryInto;
use usb_legacy_support_capability::UsbLegacySupportCapability;

pub mod usb_legacy_support_capability;

/// A struct to access xHCI Extended Capabilities.
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
    /// this accessor.
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
impl<'a, M> IntoIterator for &'a mut List<M>
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
        let h: Header = unsafe { accessor::Single::new(current, self.m.clone()) }
            .expect("The base address of the xHCI Extended Capability must be aligned correctly.")
            .read();

        self.current = if h.next() == 0 {
            None
        } else {
            Some(current + (usize::from(h.next()) << 2))
        };

        Some(match h.id() {
            // SAFETY: `List::new` ensures that the all necessary conditions are fulfilled.
            1 => Ok(ExtendedCapability::UsbLegacySupportCapability(unsafe {
                accessor::Single::new(current, self.m.clone()).expect(
                    "The base address of the xHCI Extended Capability must be aligned correctly.",
                )
            })),
            e => Err(NotSupportedId(e)),
        })
    }
}

/// The xHCI Extended Capability.
#[non_exhaustive]
pub enum ExtendedCapability<M>
where
    M: Mapper,
{
    /// USB Legacy Support Capability.
    UsbLegacySupportCapability(accessor::Single<UsbLegacySupportCapability, M>),
}

/// A struct representing that the Extended Capability with the ID is not supported.
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
