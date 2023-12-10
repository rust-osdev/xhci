use alloc::collections::LinkedList;
use core::alloc::{GlobalAlloc, Layout};
use linked_list_allocator::LockedHeap;
use uefi::table::boot;

#[global_allocator]
static mut ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init(memory_map: boot::MemoryMap) {
    let largest_chunk = find_largest_conventional_memory_chunk(memory_map);

    let heap_start = largest_chunk.phys_start;
    let heap_end = heap_start + largest_chunk.page_count * 4096;
    let heap_size = heap_end - heap_start;

    unsafe {
        ALLOCATOR
            .lock()
            .init(heap_start as *mut _, heap_size.try_into().unwrap());
    }
}

fn find_largest_conventional_memory_chunk(memory_map: boot::MemoryMap) -> boot::MemoryDescriptor {
    // Only EfiConventionalMemory is used because dealing with additional memory
    // types involves unnecessary complexity, given the presence of stack space
    // in EfiBootServicesData (confirmed by manual testing) and UEFI binaries in
    // EfiLoaderData (as specified in UEFI version 2.10, Table 7.6.)
    *memory_map
        .entries()
        .filter(|x| x.ty == boot::MemoryType::CONVENTIONAL)
        .max_by_key(|x| x.page_count)
        .expect("No conventional memory found")
}
