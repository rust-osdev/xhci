//! Debug Capability.

use super::ExtendedCapability;
use accessor::single;
use accessor::Mapper;
use bit_field::BitField;
use core::convert::TryInto;

/// The entry point to the Debug Capability.
#[derive(Debug)]
pub struct Debug<M>
where
    M: Mapper + Clone,
{
    /// Capability ID.
    pub dcid: single::ReadWrite<Id, M>,
    /// Doorbell.
    pub dcdb: single::ReadWrite<Doorbell, M>,
    /// Event Ring Segment Table Size.
    pub dcerstsz: single::ReadWrite<EventRingSegmentTableSize, M>,
    /// Event Ring Segment Table Base Address.
    pub dcerstba: single::ReadWrite<EventRingSegmentTableBaseAddress, M>,
    /// Event Ring Dequeue Pointer.
    pub dcerdp: single::ReadWrite<EventRingDequeuePointer, M>,
    /// Control.
    pub dcctrl: single::ReadWrite<Control, M>,
    /// Status.
    pub dcst: single::ReadWrite<Status, M>,
    /// Port Status and Control.
    pub dcportsc: single::ReadWrite<PortStatusAndControl, M>,
    /// Debug Capability Context Pointer.
    pub dccp: single::ReadWrite<ContextPointer, M>,
    /// Device Descriptor Info Register 1.
    pub dcddi1: single::ReadWrite<DeviceDescriptorInfo1, M>,
    /// Device Descriptor Info Register 2.
    pub dcddi2: single::ReadWrite<DeviceDescriptorInfo2, M>,
}
impl<M> Debug<M>
where
    M: Mapper + Clone,
{
    /// Creates an instance of [`struct@Debug`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the Debug Capability is accessed only through the returned
    /// accessor.
    ///
    /// # Panics
    ///
    /// This method panics if `base` is not aligned correctly.
    pub unsafe fn new(base: usize, mapper: &M) -> Self {
        macro_rules! m {
            ($offset:expr) => {
                single::ReadWrite::new(base + $offset, mapper.clone())
            };
        }

        Self {
            dcid: m!(0x00),
            dcdb: m!(0x04),
            dcerstsz: m!(0x08),
            dcerstba: m!(0x10),
            dcerdp: m!(0x18),
            dcctrl: m!(0x20),
            dcst: m!(0x24),
            dcportsc: m!(0x28),
            dccp: m!(0x30),
            dcddi1: m!(0x38),
            dcddi2: m!(0x3c),
        }
    }
}
impl<M> From<Debug<M>> for ExtendedCapability<M>
where
    M: Mapper + Clone,
{
    fn from(d: Debug<M>) -> Self {
        ExtendedCapability::Debug(d)
    }
}

/// Debug Capability ID Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Id(u32);
impl Id {
    /// Returns the value of the Debug Capability Event Ring Segment Table Max field.
    #[must_use]
    pub fn debug_capability_event_ring_segment_table_max(self) -> u8 {
        self.0.get_bits(16..=20).try_into().unwrap()
    }
}
impl_debug_from_methods! {
    Id {
        debug_capability_event_ring_segment_table_max,
    }
}

/// Debug Capability Doorbell Register.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Doorbell(u32);
impl Doorbell {
    /// Sets the value of the Doorbell Target field.
    pub fn set_doorbell_target(&mut self, target: u8) {
        self.0.set_bits(8..=15, target.into());
    }
}

/// Debug Capability Event Ring Segment Table Size Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EventRingSegmentTableSize(u32);
impl EventRingSegmentTableSize {
    /// Returns the value of the Event Ring Segment Table Size field.
    #[must_use]
    pub fn get(self) -> u16 {
        self.0.get_bits(0..=15).try_into().unwrap()
    }

    /// Sets the value of the Event Ring Segment Table Size field.
    pub fn set(&mut self, sz: u16) {
        self.0.set_bits(0..=15, sz.into());
    }
}
impl_debug_from_methods! {
    EventRingSegmentTableSize {
        get,
    }
}

/// Debug Capability Event Ring Segment Table Base Address Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EventRingSegmentTableBaseAddress(u64);
impl EventRingSegmentTableBaseAddress {
    /// Returns the value of the Event Ring Segment Table Base Address field.
    #[must_use]
    pub fn get(self) -> u64 {
        self.0
    }

    /// Sets the value of the Event Ring Segment Table Base Address field.
    ///
    /// # Panics
    ///
    /// This method panics if the address is not 16-byte aligned.
    pub fn set(&mut self, a: u64) {
        assert!(
            a.trailing_zeros() >= 4,
            "The base address of the Event Ring Segment Table must be 16-byte aligned."
        );

        self.0 = a;
    }
}
impl_debug_from_methods! {
    EventRingSegmentTableBaseAddress {
        get,
    }
}

/// Debug Capability Event Ring Dequeue Pointer Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EventRingDequeuePointer(u64);
impl EventRingDequeuePointer {
    /// Returns the value of the Dequeue ERST Segment Index field.
    #[must_use]
    pub fn dequeue_erst_segment_index(self) -> u8 {
        self.0.get_bits(0..=2).try_into().unwrap()
    }

    /// Sets the value of the Dequeue ERST Segment Index field.
    pub fn set_dequeue_erst_segment_index(&mut self, i: u8) {
        self.0.set_bits(0..=2, i.into());
    }

    /// Returns the value of the Dequeue Pointer field.
    #[must_use]
    pub fn dequeue_pointer(self) -> u64 {
        self.0 & !0b1111
    }

    /// Sets the value of the Dequeue Pointer field.
    ///
    /// # Panics
    ///
    /// This method panics if the address is not 16-byte aligned.
    pub fn set_dequeue_pointer(&mut self, a: u64) {
        assert!(
            a.trailing_zeros() >= 4,
            "The Event Ring Dequeue Pointer must be 16-byte aligned."
        );

        self.0.set_bits(4..=63, a.get_bits(4..=63));
    }
}
impl_debug_from_methods! {
    EventRingDequeuePointer {
        dequeue_erst_segment_index,
        dequeue_pointer,
    }
}

/// Debug Capability Control Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Control(u32);
impl Control {
    ro_bit!(0, dbc_run, "DbC Run");
    rw_bit!(1, link_status_event_enable, "Link Status Event Enable");
    rw1s_bit!(2, halt_out_tr, "Halt OUT TR");
    rw1s_bit!(3, halt_in_tr, "Halt IN TR");
    rw1c_bit!(4, dbc_run_change, "DbC Run Change");

    /// Returns the value of the Debug Max Burst Size field.
    #[must_use]
    pub fn debug_max_burst_size(self) -> u8 {
        self.0.get_bits(16..=23).try_into().unwrap()
    }

    /// Returns the value of the Device Address field.
    #[must_use]
    pub fn device_address(self) -> u8 {
        self.0.get_bits(24..=30).try_into().unwrap()
    }

    rw_bit!(31, debug_capability_enable, "Debug Capability Enable");
}
impl_debug_from_methods! {
    Control {
        dbc_run,
        link_status_event_enable,
        halt_out_tr,
        halt_in_tr,
        dbc_run_change,
        debug_max_burst_size,
        device_address,
        debug_capability_enable,
    }
}

/// Debug Capability Status Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Status(u32);
impl Status {
    ro_bit!(0, event_ring_not_empty, "Event Ring Not Empty");
    ro_bit!(1, dbc_system_bus_reset, "DbC System Bus Reset");

    /// Returns the value of the Debug Port Number field.
    #[must_use]
    pub fn debug_port_number(self) -> u8 {
        self.0.get_bits(24..=31).try_into().unwrap()
    }
}
impl_debug_from_methods! {
    Status {
        event_ring_not_empty,
        dbc_system_bus_reset,
        debug_port_number,
    }
}

/// Debug Capability Port Status and Control Register.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PortStatusAndControl(u32);
impl PortStatusAndControl {
    ro_bit!(0, current_connect_status, "Current Connect Status");
    rw_bit!(1, port_enabled_disabled, "Port Enabled/Disabled");
    ro_bit!(4, port_reset, "Port Reset");

    /// Returns the value of the Port Link State field.
    #[must_use]
    pub fn port_link_state(self) -> u8 {
        self.0.get_bits(5..=8).try_into().unwrap()
    }

    /// Returns the value of the Port Speed field.
    #[must_use]
    pub fn port_speed(self) -> u8 {
        self.0.get_bits(10..=13).try_into().unwrap()
    }

    rw1c_bit!(17, connect_status_change, "Connect Status Change");
    rw1c_bit!(21, port_reset_change, "Port Reset Change");
    rw1c_bit!(22, port_link_status_change, "Port Link Status Change");
    rw1c_bit!(23, port_config_error_change, "Port Config Error Change");
}
impl_debug_from_methods! {
    PortStatusAndControl {
        current_connect_status,
        port_enabled_disabled,
        port_reset,
        port_link_state,
        port_speed,
        connect_status_change,
        port_reset_change,
        port_link_status_change,
        port_config_error_change,
    }
}

/// Debug Capability Context Pointer Register.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct ContextPointer(u64);
impl ContextPointer {
    /// Returns the start address of the Debug Capability Context data structure.
    #[must_use]
    pub fn get(self) -> u64 {
        self.0
    }

    /// Sets the start address of the Debug Capability Context data structure.
    ///
    /// # Panics
    ///
    /// This method panics if the address is not 16-byte aligned.
    pub fn set(&mut self, a: u64) {
        assert!(a.trailing_zeros()>=4,"The start address of the Debug Capability Context data structure must be 16-byte aligned.");

        self.0 = a;
    }
}

/// Debug Capability Device Descriptor Info Register 1
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DeviceDescriptorInfo1(u32);
impl DeviceDescriptorInfo1 {
    /// Returns the value of the DbC Protocol field.
    #[must_use]
    pub fn dbc_protocol(self) -> u8 {
        self.0.get_bits(0..=7).try_into().unwrap()
    }

    /// Sets the value of the DbC Protocol field.
    pub fn set_dbc_protocol(&mut self, protocol: u8) {
        self.0.set_bits(0..=7, protocol.into());
    }

    /// Returns the value of the Vendor ID field.
    #[must_use]
    pub fn vendor_id(self) -> u16 {
        self.0.get_bits(16..=31).try_into().unwrap()
    }

    /// Sets the value of the Vendor ID field.
    pub fn set_vendor_id(&mut self, id: u16) {
        self.0.set_bits(16..=31, id.into());
    }
}
impl_debug_from_methods! {
    DeviceDescriptorInfo1 {
        dbc_protocol,
        vendor_id,
    }
}

/// Debug Capability Device Descriptor Info Register 2.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DeviceDescriptorInfo2(u32);
impl DeviceDescriptorInfo2 {
    /// Returns the value of the Product ID field.
    #[must_use]
    pub fn product_id(self) -> u16 {
        self.0.get_bits(0..=15).try_into().unwrap()
    }

    /// Sets the value of the Product ID field.
    pub fn set_product_id(&mut self, id: u16) {
        self.0.set_bits(0..=15, id.into());
    }

    /// Returns the value of the Device Revision field.
    #[must_use]
    pub fn device_revision(self) -> u16 {
        self.0.get_bits(16..=31).try_into().unwrap()
    }

    /// Sets the value of the Device Revision field.
    pub fn set_device_revision(&mut self, revision: u16) {
        self.0.set_bits(16..=31, revision.into());
    }
}
impl_debug_from_methods! {
    DeviceDescriptorInfo2 {
        product_id,
        device_revision,
    }
}
