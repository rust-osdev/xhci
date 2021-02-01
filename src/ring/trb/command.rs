//! Command TRBs.

use super::Link;
use bit_field::BitField;
use core::convert::TryInto;

/// TRBs which are allowed to be pushed to the Command Ring.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Allowed {
    /// Link TRB
    Link(Link),
    /// Enable Slot Command TRB
    EnableSlot(EnableSlot),
    /// Address Device Command TRB
    AddressDevice(AddressDevice),
    /// Configure Endpoint Command TRB
    ConfigureEndpoint(ConfigureEndpoint),
    /// No Op Command TRB
    Noop(Noop),
}
impl Allowed {
    /// Sets the value of the Cycle Bit.
    pub fn set_cycle_bit(&mut self, b: bool) -> &mut Self {
        match self {
            Self::Noop(ref mut n) => {
                n.set_cycle_bit(b);
            }
            Self::Link(ref mut l) => {
                l.set_cycle_bit(b);
            }
            Self::EnableSlot(ref mut e) => {
                e.set_cycle_bit(b);
            }
            Self::AddressDevice(ref mut a) => {
                a.set_cycle_bit(b);
            }
            Self::ConfigureEndpoint(ref mut c) => {
                c.set_cycle_bit(b);
            }
        }
        self
    }

    /// Returns the wrapped array.
    pub fn into_raw(self) -> [u32; 4] {
        match self {
            Self::Noop(n) => n.into_raw(),
            Self::Link(l) => l.into_raw(),
            Self::EnableSlot(e) => e.into_raw(),
            Self::AddressDevice(a) => a.into_raw(),
            Self::ConfigureEndpoint(c) => c.into_raw(),
        }
    }
}
impl AsRef<[u32]> for Allowed {
    fn as_ref(&self) -> &[u32] {
        match self {
            Self::Noop(n) => n.as_ref(),
            Self::Link(l) => l.as_ref(),
            Self::EnableSlot(e) => e.as_ref(),
            Self::AddressDevice(a) => a.as_ref(),
            Self::ConfigureEndpoint(c) => c.as_ref(),
        }
    }
}
impl AsMut<[u32]> for Allowed {
    fn as_mut(&mut self) -> &mut [u32] {
        match self {
            Self::Noop(ref mut n) => n.as_mut(),
            Self::Link(ref mut l) => l.as_mut(),
            Self::EnableSlot(ref mut e) => e.as_mut(),
            Self::AddressDevice(ref mut a) => a.as_mut(),
            Self::ConfigureEndpoint(ref mut c) => c.as_mut(),
        }
    }
}

add_trb_with_default!(Noop, "No Op Command TRB", Type::NoopCommand);

add_trb_with_default!(EnableSlot, "Enable Slot Command TRB", Type::EnableSlot);
impl EnableSlot {
    /// Sets the value of the Slot Type field.
    pub fn set_slot_type(&mut self, t: u8) -> &mut Self {
        self.0[3].set_bits(16..=20, t.into());
        self
    }

    /// Returns the value of the Slot Type field.
    pub fn slot_type(&self) -> u8 {
        self.0[3].get_bits(16..=20).try_into().unwrap()
    }
}

add_trb_with_default!(
    AddressDevice,
    "Address Device Command TRB",
    Type::AddressDevice
);
impl AddressDevice {
    /// Sets the value of the Input Context Pointer field.
    ///
    /// # Panics
    ///
    /// This method panics if `p` is not 16-byte aligned.
    pub fn set_input_context_pointer(&mut self, p: u64) -> &mut Self {
        assert_eq!(
            p % 16,
            0,
            "The Input Context Pointer must be 16-byte aligned."
        );

        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
        self
    }
}

add_trb_with_default!(
    ConfigureEndpoint,
    "Configure Endpoint Command TRB",
    Type::ConfigureEndpoint
);
impl ConfigureEndpoint {
    /// Sets the value of the Input Context Pointer field.
    ///
    /// # Panics
    ///
    /// This method panics if `p` is not 16-byte aligned.
    pub fn set_input_context_pointer(&mut self, p: u64) -> &mut Self {
        assert_eq!(
            p % 16,
            0,
            "The Input Context Pointer must be 16-byte aligned."
        );

        let l = p.get_bits(0..32);
        let u = p.get_bits(32..64);

        self.0[0] = l.try_into().unwrap();
        self.0[1] = u.try_into().unwrap();
        self
    }

    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }
}
