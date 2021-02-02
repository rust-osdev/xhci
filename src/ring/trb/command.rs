//! Command TRBs.

use super::Link;
use bit_field::BitField;
use core::convert::TryInto;

allowed! {
    /// TRBs which are allowed to be pushed to the Command Ring.
    enum {
        /// Link TRB
        Link,
        /// Enable Slot Command TRB
        EnableSlot,
        /// Disable Slot Command TRB
        DisableSlot,
        /// Address Device Command TRB
        AddressDevice,
        /// Configure Endpoint Command TRB
        ConfigureEndpoint,
        /// Evaluate Context Command TRB
        EvaluateContext,
        /// No Op Command TRB
        Noop
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
    #[must_use]
    pub fn slot_type(&self) -> u8 {
        self.0[3].get_bits(16..=20).try_into().unwrap()
    }
}

add_trb_with_default!(DisableSlot, "Disable Slot Command TRB", Type::DisableSlot);
impl DisableSlot {
    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
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

    /// Returns the value of the Input Context Pointer field.
    #[must_use]
    pub fn input_context_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    /// Sets the value of the Block Set Address Request field.
    pub fn set_block_set_address_request(&mut self, r: bool) -> &mut Self {
        self.0[3].set_bit(9, r);
        self
    }

    /// Returns the value of the Block Set Address Request.
    #[must_use]
    pub fn block_set_address_request(&self) -> bool {
        self.0[3].get_bit(9)
    }

    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
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

    /// Returns the value of the Input Context Pointer field.
    #[must_use]
    pub fn input_context_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    /// Sets the value of the Deconfigure field.
    pub fn set_deconfigure(&mut self, d: bool) -> &mut Self {
        self.0[3].set_bit(9, d);
        self
    }

    /// Returns the value of the Deconfigure field.
    pub fn deconfigure(&mut self) -> bool {
        self.0[3].get_bit(9)
    }

    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}

add_trb_with_default!(
    EvaluateContext,
    "Evaluate Context Command TRB",
    Type::EvaluateContext
);
impl EvaluateContext {
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

    /// Returns the value of the Input Context Pointer field.
    #[must_use]
    pub fn input_context_pointer(&self) -> u64 {
        let l: u64 = self.0[0].into();
        let u: u64 = self.0[1].into();

        (u << 32) | l
    }

    /// Sets the value of the Slot ID field.
    pub fn set_slot_id(&mut self, i: u8) -> &mut Self {
        self.0[3].set_bits(24..=31, i.into());
        self
    }

    /// Returns the value of the Slot ID field.
    #[must_use]
    pub fn slot_id(&self) -> u8 {
        self.0[3].get_bits(24..=31).try_into().unwrap()
    }
}
