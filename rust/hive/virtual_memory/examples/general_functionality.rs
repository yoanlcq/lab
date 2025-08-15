#![allow(unused_crate_dependencies, reason = "This is an example")]

use virtual_memory::{PageState, PageType, ProtectionFlags, VirtualMemorySystem};

#[expect(clippy::missing_assert_message, reason = "The conditions are self-explanatory and this is an example")]
#[expect(unsafe_code, reason = "Necessary for this example")]
#[expect(clippy::expect_used, clippy::unwrap_used, reason = "They are fine here")]
fn main() {
    // Instantiate a "virtual memory system"; it usually has no platform-specific equivalent, but does represent a platform-specific "initialization + information gathering" step.
    // You can instantiate as many as you like, they are all the same, since virtual memory is a process-wide resource.
    let virtual_memory_system = VirtualMemorySystem::new();

    // Get some information. The `.get()` is because these are `NonZeroUsize`.
    let page_size = virtual_memory_system.page_size().get();
    let allocation_granularity = virtual_memory_system.allocation_granularity().get();

    // Some stuff we promise to user code
    assert!(allocation_granularity.is_multiple_of(page_size));
    assert!(page_size.is_power_of_two());

    // Start by reserving a range of virtual addresses.
    // This does NOT "allocate" memory, it just ensures the range is yours to use.
    let reserved_range = {
        // Let the OS decide the starting address
        let starting_address_hint= None;

        // Reserving a few pages should be fine.
        // Note that the size is not required to be a multiple of the page size; in this crate, all APIs implicitly calculate the range of pages touched by user-provided ranges, because this is generally done by the platform-specific API anyway.
        let reserve_size = (page_size * 4) / 3;

        virtual_memory_system.reserve(starting_address_hint, reserve_size).expect("Should be able to reserve a few pages worth of virtual address space")
    };

    // The returned range is exactly aligned to page boundaries
    assert!(reserved_range.addr().get().is_multiple_of(allocation_granularity));
    assert!(reserved_range.size() == page_size * 2);

    // Next let's make our reserved virtual address range usable by telling the OS to allow memory accesses to some pages.
    // This still does not yet "allocate" physical memory for the virtual pages; the allocation will occur only when the page's memory is accessed for the first time, and at that moment it will be zeroed.
    // For the sake of simplicity we pass the full reserved range, but you don't have to: you could choose to only commit a subset of pages as needed according to your allocation patterns.
    let protection_flags = ProtectionFlags::READ_WRITE;
    let committed_range = virtual_memory_system.commit(reserved_range, protection_flags).expect("commit() should not fail since the range was just reserved");

    assert_eq!(committed_range.ptr().addr(), reserved_range.addr().get());
    assert_eq!(committed_range.size(), reserved_range.size());

    // We can then start using that memory!
    for i in 0..committed_range.size() {
        // SAFETY: everything is within range and we are the only ones using that memory
        let byte = unsafe { &mut *committed_range.ptr().add(i) };
        // Newly-committed pages have their memory set to zero by the OS
        assert_eq!(*byte, 0, "Newly-committed pages are supposed to have their memory zeroed by the OS");
        // We set the protection to READ_WRITE, so we can write
        *byte = 1;
    }

    // We can get information about a range of pages sharing the same properties starting at some address
    let page_range_info = virtual_memory_system.page_range_info(reserved_range.addr()).expect("Getting the page info should not fail, as it was reserved");
    assert_eq!(page_range_info.allocation_addr().unwrap(), reserved_range.addr());
    assert_eq!(page_range_info.allocation_protection_flags_lossy().unwrap(), ProtectionFlags::empty());
    assert_eq!(page_range_info.protection_flags_lossy().unwrap(), protection_flags);
    assert_eq!(page_range_info.addr().get(), committed_range.ptr().addr());
    assert_eq!(page_range_info.size(), committed_range.size());
    assert_eq!(page_range_info.state(), PageState::Committed);
    assert_eq!(page_range_info.r#type().unwrap(), PageType::Private);

    // Don't forget to clean-up.
    // Obviously, on any modern OS, this also happens automatically when the process exists.

    // SAFETY: Nobody is currently using the memory within the committed range
    unsafe {
        virtual_memory_system.decommit(committed_range).expect("decommit() should not fail, because we just committed the range");
    };

    // SAFETY: We're passing a virtual address range returned by reserve()
    unsafe {
        virtual_memory_system.unreserve(reserved_range).expect("unreserve() should not fail, because the range is exactly the one returned by reserve()");
    }
}

