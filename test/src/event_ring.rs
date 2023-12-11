use crate::registers;
use alloc::{boxed::Box, vec, vec::Vec};
use conquer_once::spin::OnceCell;
use qemu_print::qemu_println;
use spinning_top::Spinlock;

static EVENT_RING_SEGMENT_TABLE: OnceCell<Spinlock<EventRingSegmentTable>> = OnceCell::uninit();
static EVENT_RINGS: OnceCell<Spinlock<EventRingCollection>> = OnceCell::uninit();

pub fn init() {
    allocate_event_ring_segment_table();

    allocate_event_rings();
}

fn allocate_event_ring_segment_table() {
    EVENT_RING_SEGMENT_TABLE
        .try_init_once(|| Spinlock::new(EventRingSegmentTable::new()))
        .expect("Event ring segment table already initialized");

    qemu_println!("Event ring segment table is initialized");
}

fn allocate_event_rings() {
    EVENT_RINGS
        .try_init_once(|| Spinlock::new(EventRingCollection::new()))
        .expect("Event rings already initialized");

    qemu_println!("Event rings are initialized");
}

struct EventRingSegmentTable(Vec<EventRingSegmentTableEntry>);
impl EventRingSegmentTable {
    fn new() -> Self {
        Self(vec![
            EventRingSegmentTableEntry::null();
            number_of_rings().into()
        ])
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
struct EventRingSegmentTableEntry {
    base_address: u64,
    segment_size: u64,
}
impl EventRingSegmentTableEntry {
    fn null() -> Self {
        Self {
            base_address: 0,
            segment_size: 0,
        }
    }
}

struct EventRingCollection(Vec<EventRing>);
impl EventRingCollection {
    fn new() -> Self {
        Self(vec![EventRing::new(); number_of_rings().into()])
    }
}

#[derive(Clone, Debug)]
struct EventRing(Box<[[u32; 4]; 256]>);
impl EventRing {
    fn new() -> Self {
        Self(Box::new([[0; 4]; 256]))
    }
}

fn number_of_rings() -> u16 {
    registers::handle(|r| {
        r.capability
            .hcsparams2
            .read_volatile()
            .event_ring_segment_table_max()
    })
}
