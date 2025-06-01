// Why this name:
// https://ell.stackexchange.com/a/250169

#![feature(allocator_api)]

use std::{alloc::{handle_alloc_error, AllocError, Layout}, collections::BTreeMap, num::NonZeroUsize, ptr::NonNull, sync::Arc};

use virtual_memory::{ProtectionFlags, VirtualMemorySystem};

// TODO: Fix the todo!()s in Drop
// TODO: enumerate memory ranges, just out of curiosity (https://stackoverflow.com/a/20350190)
// TODO: test that the functions work... In particular, reserve() does not set protection flags, will it work?
// TODO: check that creating Allocations from multiple threads is possible
// TODO: Provide good (and illustrated) documentation
// TODO: Be pedantic about "# Safety" in the doc and "SAFETY: " in the code? Clippy: https://rust-lang.github.io/rust-clippy/master/#undocumented_unsafe_blocks
// TODO: automated copyright notice?
// TODO: automated licenses gathering?
// TODO: automated export of non-confidential source code and commits?

#[derive(Default)]
struct AllocatorState {
    num_allocations: usize,
    // Storing the pointer is not strictly necessary, it's just an optimization to avoid the iterative "index to pointer" algorithm
    free_allocation_indices: BTreeMap<usize, NonNull<u8>>,
    #[cfg(feature="track_allocation_sizes")]
    allocations_actual_committed_sizes: BTreeMap<usize, usize>,
}

pub struct Allocator {
    virtual_memory_system: Arc<VirtualMemorySystem>,
    // Aligned to allocation granularity, which itself must be a multiple of the page size
    reserved_virtual_address_range_start: NonNull<u8>,
    // Power of two AND multiple of page size
    reserved_virtual_address_range_size: NonZeroUsize,
    state: parking_lot::Mutex<AllocatorState>,
    // This is provided outside of `state` to avoid locking.
    // For the sake of correctness, this flag must only be changed while no Allocation exists.
    #[cfg(feature="track_allocation_sizes")]
    track_allocation_sizes: bool,
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
    pub fn create(virtual_memory_system: &Arc<VirtualMemorySystem>, starting_size: NonZeroUsize, track_allocation_sizes: bool) -> Result<Self, virtual_memory::Error> {
        #[cfg(not(feature="track_allocation_sizes"))]
        if track_allocation_sizes {
            return Err(virtual_memory::Error::other("Feature \"track_allocation_sizes\" is disabled. Please use IS_TRACK_ALLOCATION_SIZES_FEATURE_ENABLED to detect this and make your code explicit"));
        }

        let page_size = virtual_memory_system.page_size();
        let allocation_granularity = virtual_memory_system.allocation_granularity();

        // These are not "absolutely 100%" guaranteed by the virtual_memory API (because it does't need to),
        // however this allocator in particular relies strongly on those facts which must be true in practice unless the operating system or CPU brand is insane
        assert!(page_size.get().is_power_of_two());
        assert!(allocation_granularity.get().is_multiple_of(page_size.get()));

        let mut attempt_size = ((starting_size.get() / 2) + 1).next_power_of_two().min(isize::MAX as usize);
        loop {
            match virtual_memory_system.reserve(None, unsafe { NonZeroUsize::new_unchecked(attempt_size) }) {
                Ok(reserved_virtual_address_range_start) => {
                    assert!(reserved_virtual_address_range_start.addr().get().is_multiple_of(allocation_granularity.get()));

                    let va_end = (reserved_virtual_address_range_start.addr().get() + attempt_size).next_multiple_of(page_size.get());
                    let reserved_virtual_address_range_size = NonZeroUsize::new(va_end - reserved_virtual_address_range_start.addr().get()).unwrap();
                    return Ok(Self {
                        virtual_memory_system: virtual_memory_system.clone(),
                        reserved_virtual_address_range_start,
                        reserved_virtual_address_range_size,
                        state: Default::default(),
                        #[cfg(feature="track_allocation_sizes")]
                        track_allocation_sizes: track_allocation_sizes,
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
    #[inline(always)]
    pub fn destroy(mut self) -> Result<(), virtual_memory::Error> {
        self.destroy_impl()?;
        Ok(std::mem::forget(self))
    }
    // This MUST NOT be exposed publicly!!
    #[inline(always)]
    fn destroy_impl(&mut self) -> Result<(), virtual_memory::Error> {
        unsafe { self.virtual_memory_system.unreserve(self.reserved_virtual_address_range_start, self.reserved_virtual_address_range_size) }
    }
    #[inline(always)]
    pub fn is_tracking_allocation_sizes(&self) -> bool {
        #[cfg(feature="track_allocation_sizes")]
        {
            self.track_allocation_sizes
        }
        #[cfg(not(feature="track_allocation_sizes"))]
        false
    }
    pub const IS_TRACK_ALLOCATION_SIZES_FEATURE_ENABLED: bool = cfg!(feature="track_allocation_sizes");
    #[inline(always)]
    pub fn can_track_allocations_racy(&self) -> bool {
        Self::IS_TRACK_ALLOCATION_SIZES_FEATURE_ENABLED && self.state.lock().num_allocations == 0
    }
    pub fn set_track_allocations(&mut self, track_allocation_sizes: bool) -> Result<(), virtual_memory::Error> {
        let state = self.state.lock();
        if state.num_allocations > 0 {
            return Err(virtual_memory::Error::other("Calling set_track_allocations() is forbidden while there are live allocations"))
        }
        #[cfg(feature="track_allocation_sizes")]
        {
            self.track_allocation_sizes = track_allocation_sizes;
            drop(state);
            Ok(())
        }
        #[cfg(not(feature="track_allocation_sizes"))]
        Err(virtual_memory::Error::other("Feature \"track_allocation_sizes\" is disabled"))
    }
    #[inline(always)]
    pub fn virtual_memory_system(&self) -> &Arc<VirtualMemorySystem> {
        &self.virtual_memory_system
    }
    #[inline(always)]
    pub fn page_size(&self) -> NonZeroUsize {
        self.virtual_memory_system.page_size()
    }
    #[inline(always)]
    pub fn allocation_granularity(&self) -> NonZeroUsize {
        self.virtual_memory_system.allocation_granularity()
    }
    // Note that dereferencing the memory may be unsafe if there are live allocations, because they may be using it!
    #[inline(always)]
    pub fn reserved_virtual_address_range_start(&self) -> NonNull<u8> {
        self.reserved_virtual_address_range_start
    }
    #[inline(always)]
    pub fn reserved_virtual_address_range_size(&self) -> NonZeroUsize {
        self.reserved_virtual_address_range_size
    }
    #[inline(always)]
    pub fn actual_available_size_for_any_allocation_racy(&self) -> usize {
        self.actual_available_size_for_any_allocation_given_num_allocations(self.state.lock().num_allocations)
    }
    // This is rounded DOWN to page size boundary. Example: page_size = 4096, available = 2048, in this case actual_available = 0.
    fn actual_available_size_for_any_allocation_given_num_allocations(&self, num_allocations: usize) -> usize {
        let page_size = self.virtual_memory_system.page_size().get();
        let current_max_num_allocations = num_allocations.next_power_of_two();
        let available_size = self.reserved_virtual_address_range_size.get() / current_max_num_allocations;
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
    fn allocate(&self) -> Result<AllocationStorage, virtual_memory::Error> {
        let (index, cached_start) = {
            let mut state = self.state.lock();

            // pop_first() is important as we want to keep low indices used, and high indices free, to increase the odds of being able to free by decrementing `num_allocations` which, in turn, when it crosses power-of-two boundaries, will 2x the available size for all allocations
            let (index, cached_start) = state.free_allocation_indices.pop_first().map(|x| (x.0, Some(x.1))).unwrap_or((state.num_allocations, None));

            // When increasing num_allocations in a way that crosses a power-of-two boundary, it causes the available size for all allocations to be divided by 2
            #[cfg(feature="track_allocation_sizes")]
            if self.track_allocation_sizes && index.is_power_of_two() {
                // Don't unwrap() here, the set may be empty if all allocations have a committed size of 0.
                if let Some(highest_actual_committed_size_among_allocations) = state.allocations_actual_committed_sizes.last_key_value().map(|x| *x.0) {
                    let new_available = self.actual_available_size_for_any_allocation_given_num_allocations(index + 1);
                    if new_available < highest_actual_committed_size_among_allocations {
                        return Err(virtual_memory::Error::other(format!("Cannot create a new allocation: would reduce actual_available_size_for_any_allocation to {}, which would invalidate at least one allocation because it has an actual committed size of {}", new_available, highest_actual_committed_size_among_allocations)));
                    }
                }
            }

            if index == state.num_allocations {
                state.num_allocations += 1;
            }

            (index, cached_start)
        };

        let committed_memory_start = cached_start.unwrap_or_else(|| Self::allocate_from_index(self.reserved_virtual_address_range_start, self.reserved_virtual_address_range_size.get(), index).0);

        Ok(AllocationStorage { index, committed_memory_start, committed_size: 0 })
    }
    fn deallocate(&self, storage: &AllocationStorage) {
        let mut state = self.state.lock();

        // If this was the last live allocation, we can clear our state very efficiently
        if state.free_allocation_indices.len() + 1 == state.num_allocations {
            *state = Default::default();
            return;
        }

        let &AllocationStorage { index, committed_memory_start, committed_size: _ } = storage;

        // If this was the last index, no need to add to the free list.
        // This may then trigger a chain reaction where we can keep doing that.
        if index == state.num_allocations - 1 {
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

        state.free_allocation_indices.insert(index, committed_memory_start);
    }
}

struct AllocationStorage {
    index: usize,
    // Aligned to page size boundary
    committed_memory_start: NonNull<u8>,
    // Not necessarily aligned to any boundary and is initially zero
    committed_size: usize,
}

pub struct Allocation {
    allocator: Arc<Allocator>,
    // This is None as long as committed_size == 0.
    // Making this an Option is not strictly required, but it's better for ensuring that as few resources are used as possible.
    // The cost of branching on this Option is nothing compared to the benefits it gives
    storage: Option<AllocationStorage>,
}

impl Drop for Allocation {
    fn drop(&mut self) {
        self.decommit_all().unwrap_or_else(|e| {
            Err::<(), _>(e).unwrap();
            todo!() // Provide a way to override the drop() behavior
        })
    }
}

impl Allocation {
    pub fn new(allocator: Arc<Allocator>) -> Self {
        Self { allocator, storage: None }
    }
    // The implementation _may_ avoid the Arc::clone(), hence it takes a reference
    pub fn with_committed_size(allocator: &Arc<Allocator>, committed_size: usize, protection_flags: ProtectionFlags) -> Result<Self, virtual_memory::Error> {
        let mut s = Self::new(allocator.clone());
        s.set_committed_size(committed_size, protection_flags)?;
        Ok(s)
    }
    // An alternative to `drop()` that allows you to handle the error if any
    #[inline(always)]
    pub fn destroy(mut self) -> Result<(), virtual_memory::Error> {
        self.decommit_all()
    }
    #[inline(always)]
    pub fn allocator(&self) -> &Arc<Allocator> {
        &self.allocator
    }
    #[inline(always)]
    pub fn page_size(&self) -> NonZeroUsize {
        self.allocator.virtual_memory_system.page_size()
    }
    #[inline(always)]
    pub fn committed_memory_start(&self) -> NonNull<u8> {
        self.storage.as_ref().map(|s| s.committed_memory_start).unwrap_or(NonNull::dangling())
    }
    #[inline(always)]
    pub fn committed_size(&self) -> usize {
        self.storage.as_ref().map(|s| s.committed_size).unwrap_or(0)
    }
    #[inline(always)]
    pub fn actual_committed_size(&self) -> usize {
        self.committed_size().next_multiple_of(self.page_size().get())
    }
    #[inline(always)]
    pub fn committed_memory_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.committed_memory_start().as_ptr(), self.committed_size()) }
    }
    #[inline(always)]
    pub fn actual_committed_memory_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.committed_memory_start().as_ptr(), self.actual_committed_size()) }
    }
    #[inline(always)]
    pub fn committed_memory_slice_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.committed_memory_start().as_ptr(), self.committed_size()) }
    }
    #[inline(always)]
    pub fn actual_committed_memory_slice_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.committed_memory_start().as_ptr(), self.actual_committed_size()) }
    }
    #[inline(always)]
    pub fn committed_memory_nonnull_slice(&self) -> NonNull<[u8]> {
        NonNull::slice_from_raw_parts(self.committed_memory_start(), self.committed_size())
    }
    #[inline(always)]
    pub fn actual_committed_memory_nonnull_slice(&self) -> NonNull<[u8]> {
        NonNull::slice_from_raw_parts(self.committed_memory_start(), self.actual_committed_size())
    }
    #[inline(always)]
    pub fn actual_available_size_racy(&self) -> usize {
        self.allocator.actual_available_size_for_any_allocation_racy()
    }
    // This is called `set_committed_size` because it can either grow or shrink the allocation
    pub fn set_committed_size(&mut self, new_size: usize, protection_flags: ProtectionFlags) -> Result<(), virtual_memory::Error> {
        if new_size == 0 {
            return self.decommit_all();
        }

        if new_size > isize::MAX as usize {
            return Err(virtual_memory::Error::other(format!("Cannot allocate more than isize::MAX, functions such as ptr::add() assume this")));
        }

        if self.storage.is_none() {
            self.storage = Some(self.allocator.allocate()?);
        }

        let storage = unsafe { self.storage.as_mut().unwrap_unchecked() };

        let page_size = self.allocator.virtual_memory_system.page_size().get();
        let start = storage.committed_memory_start.addr().get();
        let current_page_end = start + storage.committed_size.next_multiple_of(page_size);
        let desired_page_end = start + new_size.next_multiple_of(page_size);
        if desired_page_end != current_page_end {
            #[cfg(feature="track_allocation_sizes")]
            let (old_actual_committed_size, new_actual_committed_size, mut optional_state_lock) = (current_page_end - start, desired_page_end - start, None);

            if desired_page_end > current_page_end {
                #[cfg(feature="track_allocation_sizes")]
                if self.allocator.track_allocation_sizes {
                    let state = self.allocator.state.lock();
                    let actual_available_size = self.allocator.actual_available_size_for_any_allocation_given_num_allocations(state.num_allocations);
                    if new_actual_committed_size > actual_available_size {
                        return Err(virtual_memory::Error::other(format!("Cannot set the committed size to {} (actual: {}); available = {}", new_size, new_actual_committed_size, actual_available_size)));
                    }
                    optional_state_lock = Some(state);
                }
                self.allocator.virtual_memory_system.commit(unsafe { storage.committed_memory_start.add(current_page_end) }.as_ptr(), desired_page_end - current_page_end, protection_flags)?;
            } else {
                unsafe { self.allocator.virtual_memory_system.decommit(storage.committed_memory_start.add(desired_page_end).as_ptr(), current_page_end - desired_page_end) }?;
            }

            // Do this AFTER commit/decommit, because if they failed, we don't want to reach here.
            #[cfg(feature="track_allocation_sizes")]
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
        storage.committed_size = new_size;
        Ok(())
    }
    // Same as `set_committed_size(0)` but more efficient
    #[inline(always)]
    pub fn decommit_all(&mut self) -> Result<(), virtual_memory::Error> {
        if let Some(storage) = self.storage.as_ref() {
            unsafe { self.allocator.virtual_memory_system.decommit(storage.committed_memory_start.as_ptr(), storage.committed_size) }?;
            self.allocator.deallocate(storage);
            self.storage = None;
        }
        Ok(())
    }
    pub fn grow(&mut self, additional_size: usize, protection_flags: ProtectionFlags) -> Result<(), virtual_memory::Error> {
        self.set_committed_size(self.committed_size().saturating_add(additional_size), protection_flags)
    }
    pub fn set_layout_assuming_committed_size_is_nonzero(&mut self, layout: Layout, protection_flags: ProtectionFlags) -> Result<NonNull<[u8]>, virtual_memory::Error> {
        assert!(self.storage.is_some()); // Otherwise committed_memory_start() will give a dangling pointer
        let allocation_start = self.committed_memory_start();
        let align_offset = allocation_start.align_offset(layout.align());
        let aligned_start = unsafe { allocation_start.add(align_offset) };
        let end = unsafe { aligned_start.add(layout.size()) };
        let new_size = end.addr().get() - allocation_start.addr().get();
        self.set_committed_size(new_size, protection_flags)?;
        Ok(NonNull::slice_from_raw_parts(aligned_start, new_size))
    }
}

pub struct LinearAllocator {
    allocation: parking_lot::Mutex<Allocation>,
    protection_flags: ProtectionFlags,
}

unsafe impl std::alloc::Allocator for LinearAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        if layout.size() == 0 {
            return Ok(NonNull::slice_from_raw_parts(NonNull::dangling(), 0));
        }
        let mut allocation = self.allocation.lock();

        // This allocator does not support multiple non-zero allocations; it supports only one, which can grow and shrink, reflecting the way it's truly implemented.
        // Supporting multiple non-zero allocations in any way that isn't plain stupid would require advanced management, such as internal allocations for storing a list of free ranges, etc.
        if allocation.committed_size() > 0 {
            return Err(AllocError);
        }
        allocation.set_committed_size(1, self.protection_flags).map_err(|_| AllocError)?;
        allocation.set_layout_assuming_committed_size_is_nonzero(layout, self.protection_flags).map_err(|_| AllocError)
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, layout: Layout) {
        // Caller may allocate() zero-size multiple times. A matching call to deallocate() for one of them must not invalidate any non-zero allocate().
        if layout.size() == 0 {
            return;
        }
        // If we reach here, then we must be the one non-zero allocation
        self.allocation.lock().decommit_all().unwrap_or_else(|_| handle_alloc_error(layout))
    }

    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // This works ONLY because we allow no more than ONE non-zero allocation.
        // If this results in new pages being committed, we know that we are the first one ever to commit them.
        // We also know that any newly-committed page is zero-initialized.
        #[cfg(windows)] // zeroed-pages-on-commit is true at least on Windows. Is it also true for other platforms?
        self.allocate(layout)
    }

    unsafe fn grow(&self, _ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // Caller may allocate() zero-size multiple times. If we are one of these zero-size allocations, make sure we do not disturb any existing non-zero allocation.
        if old_layout.size() == 0 {
            return self.allocate(new_layout);
        }
        // If we reach here, then we must be the one non-zero allocation
        self.allocation.lock().set_layout_assuming_committed_size_is_nonzero(new_layout, self.protection_flags).map_err(|_| AllocError)
    }

    unsafe fn grow_zeroed(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // See allocate_zeroed() for why this is correct
        #[cfg(windows)] // zeroed-pages-on-commit is true at least on Windows. Is it also true for other platforms?
        unsafe { self.grow(ptr, old_layout, new_layout) }
    }

    unsafe fn shrink(&self, _ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        if old_layout.size() == 0 {
            return Ok(NonNull::slice_from_raw_parts(NonNull::dangling(), 0));
        }
        if new_layout.size() == 0 {
            return self.allocation.lock().decommit_all().map(|_| NonNull::slice_from_raw_parts(NonNull::dangling(), 0)).map_err(|_| AllocError);
        }
        // If we reach here, then we must be the one non-zero allocation
        self.allocation.lock().set_layout_assuming_committed_size_is_nonzero(new_layout, self.protection_flags).map_err(|_| AllocError)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}

