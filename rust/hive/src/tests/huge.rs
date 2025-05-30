// Why this name:
// https://ell.stackexchange.com/a/250169

use std::{collections::{BTreeMap}, num::NonZeroUsize, ptr::NonNull, sync::{Arc}};

use virtual_memory::VirtualMemorySystem;

mod virtual_memory {
    use std::{num::NonZeroUsize, ptr::NonNull};

    pub use std::io::Error as Error;

    #[cfg(windows)]
    mod windows_imp {
        use super::Error;
        use std::{mem::MaybeUninit, ptr::NonNull};

        pub fn get_system_info() -> winapi::um::sysinfoapi::SYSTEM_INFO {
            let mut info = MaybeUninit::uninit();
            unsafe {
                winapi::um::sysinfoapi::GetSystemInfo(info.as_mut_ptr());
                info.assume_init()
            }
        }

        /// Windows: `starting_address_hint`:
        /// > The starting address of the region to allocate.
        /// > If the memory is being reserved, the specified address is rounded down to the nearest multiple of the allocation granularity.
        /// > If the memory is already reserved and is being committed, the address is rounded down to the next page boundary.
        /// > To determine the size of a page and the allocation granularity on the host computer, use the GetSystemInfo function.
        /// > If this parameter is NULL, the system determines where to allocate the region.
        /// `min_size` is not required to be a multiple of the page size.
        /// Also, when a page transitions from reserved to committed, its memory is automatically zeroed.
        pub fn virtual_alloc(starting_address_hint: *mut u8, size: usize, flags: u32, protection: u32) -> Result<NonNull<u8>, Error> {
            let p = unsafe { winapi::um::memoryapi::VirtualAlloc(starting_address_hint.cast(), size, flags, protection) };
            if p.is_null() {
                Err(Error::last_os_error())
            } else {
                Ok(unsafe { NonNull::new_unchecked(p) }.cast())
            }
        }

        pub fn virtual_free(va_base: NonNull<u8>, size: usize, flags: u32) -> Result<(), Error> {
            match unsafe { winapi::um::memoryapi::VirtualFree(va_base.cast().as_ptr(), size, flags) } {
                0 => Err(Error::last_os_error()),
                _ => Ok(())
            }
        }
    }

    pub struct VirtualMemorySystem {
        #[cfg(windows)]
        info: winapi::um::sysinfoapi::SYSTEM_INFO,
    }

    impl VirtualMemorySystem {
        pub fn new() -> Self {
            Self {
                #[cfg(windows)]
                info: windows_imp::get_system_info(),
            }
        }

        pub fn page_size(&self) -> NonZeroUsize {
            #[cfg(windows)]
            NonZeroUsize::new(self.info.dwPageSize as _).unwrap()
        }

        // The granularity for the starting address at which virtual memory can be allocated
        pub fn allocation_granularity(&self) -> NonZeroUsize {
            #[cfg(windows)]
            NonZeroUsize::new(self.info.dwAllocationGranularity as _).unwrap()
        }

        pub fn reserve(&self, starting_address_hint: *mut u8, min_size: NonZeroUsize) -> Result<NonNull<u8>, Error> {
            #[cfg(windows)]
            windows_imp::virtual_alloc(starting_address_hint, min_size.get(), winapi::um::winnt::MEM_RESERVE, 0)
        }

        // TODO: expose protection flags?
        pub fn commit(&self, va_base: NonNull<u8>, size: usize) -> Result<(), Error> {
            #[cfg(windows)]
            windows_imp::virtual_alloc(va_base.as_ptr(), size, winapi::um::winnt::MEM_COMMIT, winapi::um::winnt::PAGE_READWRITE).map(|_| ())
        }

        /// Windows: `va_base` and `size` are not required to be page-aligned. The function will decommit all pages that intersect the range.
        pub fn decommit(&self, va_base: NonNull<u8>, size: usize) -> Result<(), Error> {
            #[cfg(windows)]
            windows_imp::virtual_free(va_base, size, winapi::um::winnt::MEM_DECOMMIT)
        }

        /// NOTE: You MUST pass the ENTIRE range previously returned by a call to `reserve`.
        pub unsafe fn unreserve(&self, va_base: NonNull<u8>, size: NonZeroUsize) -> Result<(), Error> {
            #[cfg(windows)]
            windows_imp::virtual_free(va_base, 0 * size.get() /* Must pass 0 and prevent "unused variable `size`" warning */, winapi::um::winnt::MEM_RELEASE)
        }
    }
}

// TODO: To push this to the limit, the Allocator should be using its own allocations to back the storage for its Arc and BTreeMaps

// TODO: check that creating Allocaions from multiple threads is possible

#[derive(Default)]
struct AllocatorState {
    num_allocations: usize,
    // Storing the pointer is not strictly necessary, it's just an optimization to avoid the iterative "index to pointer" algorithm
    free_allocation_indices: BTreeMap<usize, NonNull<u8>>,
    allocations_actual_committed_sizes: BTreeMap<usize, usize>,
}

struct Allocator {
    virtual_memory_system: Arc<VirtualMemorySystem>,
    // Aligned to allocation granularity, which itself must be a multiple of the page size
    va_base: NonNull<u8>,
    // Power of two AND multiple of page size
    va_size: NonZeroUsize,
    state: parking_lot::Mutex<AllocatorState>,
    // This is provided outside of `state` to avoid locking.
    // For the sake of correctness, this flag must only be changed while no Allocation exists.
    // TODO: Make this a compile-time feature as well
    #[cfg(feature="track_allocations")]
    is_tracking_allocations: bool,
}

impl Drop for Allocator {
    fn drop(&mut self) {
        self.destroy_impl().unwrap_or_else(|e| {
            Err::<(), _>(e).unwrap();
            todo!() // Provide a way to override the drop() behavior
        })
    }
}

impl Allocator {
    // TODO: enumerate memory ranges, just out of curiosity:
    // https://stackoverflow.com/a/20350190
    pub fn create(virtual_memory_system: &Arc<VirtualMemorySystem>, starting_size: NonZeroUsize, track_allocations: bool) -> Result<Self, virtual_memory::Error> {
        #[cfg(not(feature="track_allocations"))]
        if track_allocations {
            return Err(virtual_memory::Error::other("Feature \"track_allocations\" is disabled"));
        }

        let page_size = virtual_memory_system.page_size();
        let allocation_granularity = virtual_memory_system.allocation_granularity();

        // These are not "absolutely 100%" guaranteed by the virtual_memory API (because it does't need to),
        // however this allocator in particular relies strongly on those facts which must be true in practice unless the operating system or CPU brand is insane
        assert!(page_size.get().is_power_of_two());
        assert!(allocation_granularity.get().is_multiple_of(page_size.get()));

        let mut attempt_size = ((starting_size.get() / 2) + 1).next_power_of_two().min(isize::MAX as usize);
        loop {
            match virtual_memory_system.reserve(std::ptr::null_mut(), unsafe { NonZeroUsize::new_unchecked(attempt_size) }) {
                Ok(va_base) => {
                    assert!(va_base.addr().get().is_multiple_of(allocation_granularity.get()));

                    let va_end = (va_base.addr().get() + attempt_size).next_multiple_of(page_size.get());
                    let va_size = NonZeroUsize::new(va_end - va_base.addr().get()).unwrap();
                    return Ok(Self {
                        virtual_memory_system: virtual_memory_system.clone(),
                        va_base,
                        va_size,
                        state: Default::default(),
                        #[cfg(feature="track_allocations")]
                        is_tracking_allocations: track_allocations,
                    });
                },
                Err(e) => {
                    if attempt_size <= page_size.get() {
                        return Err(e);
                    }
                },
            }

            attempt_size /= 2;
        }
    }
    // An alternative to `drop()` that allows you to handle the error if any
    pub fn destroy(mut self) -> Result<(), virtual_memory::Error> {
        self.destroy_impl()?;
        Ok(std::mem::forget(self))
    }
    // This MUST NOT be exposed publicly!!
    fn destroy_impl(&mut self) -> Result<(), virtual_memory::Error> {
        unsafe { self.virtual_memory_system.unreserve(self.va_base, self.va_size) }
    }
    pub fn is_tracking_allocations(&self) -> bool {
        #[cfg(feature="track_allocations")]
        {
            self.is_tracking_allocations
        }
        #[cfg(not(feature="track_allocations"))]
        false
    }
    pub const fn is_track_allocations_feature_enabled() -> bool {
        cfg!(feature="track_allocations")
    }
    pub fn can_track_allocations_racy(&self) -> bool {
        Self::is_track_allocations_feature_enabled() && self.state.lock().num_allocations == 0
    }
    pub fn set_track_allocations(&mut self, track_allocations: bool) -> Result<(), virtual_memory::Error> {
        let state = self.state.lock();
        if state.num_allocations > 0 {
            return Err(virtual_memory::Error::other("Calling set_track_allocations() is forbidden while there are live allocations"))
        }
        #[cfg(feature="track_allocations")]
        {
            self.is_tracking_allocations = track_allocations;
            drop(state);
            Ok(())
        }
        #[cfg(not(feature="track_allocations"))]
        Err(virtual_memory::Error::other("Feature \"track_allocations\" is disabled"))
    }
    pub fn virtual_memory_system(&self) -> &Arc<VirtualMemorySystem> {
        &self.virtual_memory_system
    }
    // Note that dereferencing the memory may be unsafe if there are live allocations, because they may be using it!
    pub fn va_base(&self) -> NonNull<u8> {
        self.va_base
    }
    pub fn va_size(&self) -> NonZeroUsize {
        self.va_size
    }
    pub fn actual_available_size_for_any_allocation_racy(&self) -> usize {
        self.actual_available_size_for_any_allocation_given_num_allocations(self.state.lock().num_allocations)
    }
    // This is rounded DOWN to page size boundary. Example: page_size = 4096, available = 2048, in this case actual_available = 0.
    fn actual_available_size_for_any_allocation_given_num_allocations(&self, num_allocations: usize) -> usize {
        let page_size = self.virtual_memory_system.page_size().get();
        let current_max_num_allocations = num_allocations.next_power_of_two();
        let available_size = self.va_size.get() / current_max_num_allocations;
        let actual_available_size = (available_size / page_size) * page_size;
        actual_available_size
    }
    // This function is designed for working with a `size` equal to 0 or power of two.
    // It is still safe to call if the condition is not met, but the resulting pointer will not be unique for the given index.
    pub fn allocate_from_index(mut start: NonNull<u8>, mut size: usize, mut index: usize) -> (NonNull<u8>, usize) {
        loop {
            size /= 2;
            if size == 0 {
                break;
            }
            if index & 1 != 0 {
                start = unsafe { start.add(size) };
            }
            if index <= 1 {
                break;
            }
            index >>= 1;
        }
        (start, size)
    }
    fn allocate(&self) -> Result<(AllocationKey, NonNull<u8>), virtual_memory::Error> {
        let (new_index, cached_start) = {
            let mut state = self.state.lock();

            // pop_first() is important as we want to keep low indices used, and high indices free, to increase the odds of being able to free by decrementing `num_allocations` which, in turn, when it crosses power-of-two boundaries, will 2x the available size for all allocations
            let (new_index, cached_start) = state.free_allocation_indices.pop_first().map(|x| (x.0, Some(x.1))).unwrap_or((state.num_allocations, None));

            // When increasing num_allocations in a way that crosses a power-of-two boundary, it causes the available size for all allocations to be divided by 2
            #[cfg(feature="track_allocations")]
            if self.is_tracking_allocations && new_index.is_power_of_two() {
                // Don't unwrap() here, the set may be empty if all allocations have a committed size of 0.
                if let Some(highest_actual_committed_size_among_allocations) = state.allocations_actual_committed_sizes.last_key_value().map(|x| *x.0) {
                    let new_available = self.actual_available_size_for_any_allocation_given_num_allocations(new_index + 1);
                    if new_available < highest_actual_committed_size_among_allocations {
                        return Err(virtual_memory::Error::other(format!("Cannot create a new allocation: would reduce actual_available_size_for_any_allocation to {}, which would invalidate at least one allocation because it has an actual committed size of {}", new_available, highest_actual_committed_size_among_allocations)));
                    }
                }
            }

            if new_index == state.num_allocations {
                state.num_allocations += 1;
            }

            (new_index, cached_start)
        };

        let start = cached_start.unwrap_or_else(|| Self::allocate_from_index(self.va_base, self.va_size.get(), new_index).0);

        Ok(( AllocationKey { index: new_index }, start))
    }
    fn deallocate(&self, key: &AllocationKey, start: NonNull<u8>) {
        let mut state = self.state.lock();

        // If this was the last live allocation, we can clear our state very efficiently
        if state.free_allocation_indices.len() + 1 == state.num_allocations {
            *state = Default::default();
            return;
        }

        // If this was the last index, no need to add to the free list.
        // This may then trigger a chain reaction where we can keep doing that.
        if key.index == state.num_allocations - 1 {
            state.num_allocations -= 1;
            while let Some((free_index, _)) = state.free_allocation_indices.last_key_value() {
                if *free_index == state.num_allocations - 1 {
                    state.num_allocations -= 1;
                    state.free_allocation_indices.pop_last();
                } else {
                    break;
                }
            }
            return;
        }

        state.free_allocation_indices.insert(key.index, start);
    }
}

struct AllocationKey {
    index: usize,
}

pub struct Allocation {
    allocator: Arc<Allocator>,
    key: AllocationKey,
    // Aligned to page size boundary
    va_start: NonNull<u8>,
    // Not necessarily aligned
    committed_size: usize,
}

impl Drop for Allocation {
    fn drop(&mut self) {
        self.destroy_impl().unwrap_or_else(|e| {
            Err::<(), _>(e).unwrap();
            todo!() // Provide a way to override the drop() behavior
        })
    }
}

impl Allocation {
    pub fn create(allocator: &Arc<Allocator>) -> Result<Self, virtual_memory::Error> {
        let (key, va_start) = allocator.allocate()?;
        Ok(Self { allocator: allocator.clone(), key, va_start, committed_size: 0 })
    }
    // An alternative to `drop()` that allows you to handle the error if any
    pub fn destroy(mut self) -> Result<(), virtual_memory::Error> {
        self.destroy_impl()?;
        Ok(std::mem::forget(self))
    }
    // This MUST NOT be exposed publicly!!
    fn destroy_impl(&mut self) -> Result<(), virtual_memory::Error> {
        self.decommit_all()?;
        self.allocator.deallocate(&self.key, self.va_start);
        Ok(())
    }
    pub fn allocator(&self) -> &Arc<Allocator> {
        &self.allocator
    }
    pub fn va_start(&self) -> NonNull<u8> {
        self.va_start
    }
    pub fn actual_available_size_racy(&self) -> usize {
        self.allocator.actual_available_size_for_any_allocation_racy()
    }
    pub fn committed_size(&self) -> usize {
        self.committed_size
    }
    pub fn actual_committed_size(&self) -> usize {
        self.committed_size.next_multiple_of(self.page_size().get())
    }
    pub fn committed_memory(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.va_start.as_ptr(), self.committed_size) }
    }
    pub fn actual_committed_memory(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.va_start.as_ptr(), self.actual_committed_size()) }
    }
    pub fn committed_memory_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.va_start.as_ptr(), self.committed_size) }
    }
    pub fn actual_committed_memory_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.va_start.as_ptr(), self.actual_committed_size()) }
    }
    // Same as `set_committed_size(0)` but faster
    pub fn decommit_all(&mut self) -> Result<(), virtual_memory::Error> {
        if self.committed_size > 0 {
            self.allocator.virtual_memory_system.decommit(self.va_start, self.committed_size)?;
            self.committed_size = 0;
        }
        Ok(())
    }
    pub fn set_committed_size(&mut self, new_size: usize) -> Result<(), virtual_memory::Error> {
        let page_size = self.page_size().get();
        let start = self.va_start.addr().get();
        let current_page_end = start + self.committed_size.next_multiple_of(page_size);
        let desired_page_end = start + new_size.next_multiple_of(page_size);
        if desired_page_end != current_page_end {
            #[cfg(feature="track_allocations")]
            let (old_actual_committed_size, new_actual_committed_size, mut optional_state_lock) = (current_page_end - start, desired_page_end - start, None);

            if desired_page_end > current_page_end {
                #[cfg(feature="track_allocations")]
                if self.allocator.is_tracking_allocations {
                    let state = self.allocator.state.lock();
                    let actual_available_size = self.allocator.actual_available_size_for_any_allocation_given_num_allocations(state.num_allocations);
                    if new_actual_committed_size > actual_available_size {
                        return Err(virtual_memory::Error::other(format!("Cannot set the committed size to {} (actual: {}); available = {}", new_size, new_actual_committed_size, actual_available_size)));
                    }
                    optional_state_lock = Some(state);
                }
                self.allocator.virtual_memory_system.commit(unsafe { self.va_start.add(current_page_end) }, desired_page_end - current_page_end)?;
            } else {
                self.allocator.virtual_memory_system.decommit(unsafe { self.va_start.add(desired_page_end) }, current_page_end - desired_page_end)?;
            }

            // Do this AFTER commit/decommit, because if they failed, we don't want to reach here.
            #[cfg(feature="track_allocations")]
            if let Some(mut state) = optional_state_lock {
                if old_actual_committed_size != 0 {
                    let refcount = state.allocations_actual_committed_sizes.get_mut(&old_actual_committed_size).unwrap();
                    if *refcount == 1 {
                        state.allocations_actual_committed_sizes.remove(&old_actual_committed_size);
                    } else {
                        *refcount -= 1;
                    }
                }
                if new_actual_committed_size != 0 {
                    *state.allocations_actual_committed_sizes.entry(new_actual_committed_size).or_default() += 1;
                }
            }
        }
        self.committed_size = new_size;
        Ok(())
    }
    fn page_size(&self) -> NonZeroUsize {
        self.allocator.virtual_memory_system.page_size()
    }
}