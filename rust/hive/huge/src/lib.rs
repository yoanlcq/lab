#![feature(allocator_api)]
#![expect(unsafe_code, reason = "Necessary for memory management")]

extern crate alloc;

use core::{alloc::Layout, fmt::Debug, num::NonZeroUsize, ptr::NonNull};
use alloc::{collections::BTreeMap, sync::Arc};
use virtual_memory::{Addr, AddrRange, Error, ProtectionFlags, PtrRange, VirtualMemorySystem};
use unique_ptr::Unique;

type Result<T> = core::result::Result<T, Error>;

// TODO: Provide good (and illustrated) documentation
// TODO: automated copyright notice?
// TODO: automated licenses gathering?
// TODO: automated export of non-confidential source code and commits?

// A wrapper around a function; it allows you to opt-out of dynamic allocation if you don't want it
pub enum DropResultHandler {
    Unwrap,
    Ignore,
    Fn(fn(Result<()>)),
    Box(Box<dyn FnMut(Result<()>) + Send + Sync>),
}

static_assertions::assert_impl_all!(DropResultHandler: Send, Sync);

impl Default for DropResultHandler {
    fn default() -> Self {
        Self::Unwrap
    }
}

impl Debug for DropResultHandler {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::Unwrap => write!(f, "Unwrap"),
            Self::Ignore => write!(f, "Ignore"),
            Self::Fn(func) => {
                write!(f, "Fn(")?;
                func.fmt(f)?;
                write!(f, ")")
            },
            Self::Box(_) => write!(f, "Box(<closure>)"),
        }
    }
}

impl DropResultHandler {
    /// # Panics
    /// 
    /// This will panic if `self == DropResultHandler::Unwrap` and the provided `Result` is `Err`.
    pub fn call(&mut self, r: Result<()>) {
        match *self {
            #[expect(clippy::unwrap_used, reason = "This is what we intend for this API")]
            Self::Unwrap => r.unwrap(),
            Self::Ignore => {},
            Self::Fn(f) => f(r),
            Self::Box(ref mut f) => f(r),
        }
    }
}

#[derive(Debug, Default)]
struct AllocatorState {
    num_allocations: usize,
    // Storing the pointer is not strictly necessary, it's just an optimization to avoid the iterative "index to pointer" algorithm
    // Maps indices to cached allocation address
    free_allocation_indices: BTreeMap<usize, Addr>,
    // Maps allocation index to committed size
    #[cfg(feature="track_allocation_sizes")]
    allocations_actual_committed_sizes: BTreeMap<usize, usize>,
}

#[derive(Debug)]
pub struct Allocator {
    virtual_memory_system: Arc<VirtualMemorySystem>,
    // start: Aligned to allocation granularity, which itself must be a multiple of the page size
    // size: Power of two AND multiple of page size
    reserved_addr_range: AddrRange,
    state: parking_lot::Mutex<AllocatorState>,
    drop_result_handler: DropResultHandler,
    // This is provided outside of `state` to avoid locking.
    // For the sake of correctness, this flag must only be changed while no Allocation exists.
    #[cfg(feature="track_allocation_sizes")]
    track_allocation_sizes: bool,
}

impl Drop for Allocator {
    fn drop(&mut self) {
        let r = self.destroy_impl();
        self.drop_result_handler.call(r);
    }
}

impl Allocator {
    /// NOTE: On Windows, the highest power of two I was able to reserve that way is 2^46.
    pub fn create(virtual_memory_system: &Arc<VirtualMemorySystem>, starting_size: NonZeroUsize, drop_result_handler: DropResultHandler, track_allocation_sizes: bool) -> Result<Self> {
        #[cfg(not(feature="track_allocation_sizes"))]
        if track_allocation_sizes {
            return Err(Error::other("Feature \"track_allocation_sizes\" is disabled. Please use IS_TRACK_ALLOCATION_SIZES_FEATURE_ENABLED to detect this and make your code explicit"));
        }

        let page_size = virtual_memory_system.page_size();

        let mut attempt_size = ((starting_size.get() / 2) + 1).next_power_of_two().min(isize::MAX as usize);
        loop {
            match virtual_memory_system.reserve(None, attempt_size) {
                Ok(reserved_addr_range) => {
                    return Ok(Self {
                        virtual_memory_system: virtual_memory_system.clone(),
                        reserved_addr_range,
                        state: parking_lot::Mutex::new(AllocatorState::default()),
                        drop_result_handler,
                        #[cfg(feature="track_allocation_sizes")]
                        track_allocation_sizes,
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
    pub const fn drop_result_handler(&self) -> &DropResultHandler {
        &self.drop_result_handler
    }
    pub fn set_drop_handler(&mut self, drop_result_handler: DropResultHandler) {
        self.drop_result_handler = drop_result_handler;
    }
    /// An alternative to `drop()` that allows you to handle the error if any
    pub fn destroy(mut self) -> Result<()> {
        let out = self.destroy_impl();
        core::mem::forget(self);
        out
    }
    // This MUST NOT be exposed publicly!!
    fn destroy_impl(&mut self) -> Result<()> {
        // SAFETY: Exclusive access to `self` is proof that there are no allocations using that range, since they all use `Arc<Allocator>`.
        unsafe { self.virtual_memory_system.unreserve(self.reserved_addr_range) }
    }

    pub const IS_TRACK_ALLOCATION_SIZES_FEATURE_ENABLED: bool = cfg!(feature="track_allocation_sizes");

    pub const fn is_tracking_allocation_sizes(&self) -> bool {
        #[cfg(feature="track_allocation_sizes")]
        {
            self.track_allocation_sizes
        }
        #[cfg(not(feature="track_allocation_sizes"))]
        false
    }

    /// This is suffixed `_racy` because the returned value may change from one call to the next depending on what the background threads are doing.
    pub fn can_track_allocations_racy(&self) -> bool {
        Self::IS_TRACK_ALLOCATION_SIZES_FEATURE_ENABLED && self.state.lock().num_allocations == 0
    }
    pub fn set_track_allocations(&mut self, track_allocation_sizes: bool) -> Result<()> {
        let state = self.state.lock();
        if state.num_allocations > 0 {
            return Err(Error::other("Calling set_track_allocations() is forbidden while there are live allocations"))
        }
        #[cfg(feature="track_allocation_sizes")]
        {
            self.track_allocation_sizes = track_allocation_sizes;
            drop(state);
            Ok(())
        }
        #[cfg(not(feature="track_allocation_sizes"))]
        Err(Error::other("Feature \"track_allocation_sizes\" is disabled"))
    }
    pub const fn virtual_memory_system(&self) -> &Arc<VirtualMemorySystem> {
        &self.virtual_memory_system
    }
    pub fn page_size(&self) -> NonZeroUsize {
        self.virtual_memory_system.page_size()
    }
    pub fn allocation_granularity(&self) -> NonZeroUsize {
        self.virtual_memory_system.allocation_granularity()
    }
    pub const fn reserved_addr_range(&self) -> AddrRange {
        self.reserved_addr_range
    }
    /// This is suffixed `_racy` because the returned value may change from one call to the next depending on what the background threads are doing.
    pub fn actual_available_size_for_any_allocation_racy(&self) -> usize {
        self.actual_available_size_for_any_allocation_given_num_allocations(self.state.lock().num_allocations)
    }
    /// This is rounded DOWN to page size boundary. Example: `page_size` = 4096, available = 2048, in this case `actual_available` = 0.
    fn actual_available_size_for_any_allocation_given_num_allocations(&self, num_allocations: usize) -> usize {
        let page_size = self.virtual_memory_system.page_size().get();
        let current_max_num_allocations = num_allocations.next_power_of_two();
        let available_size = self.reserved_addr_range.size() / current_max_num_allocations;
        (available_size / page_size) * page_size
    }
    /// This function is designed for working with a `size` equal to 0 or power of two.
    /// It is still safe to call if the condition is not met, but the resulting pointer will not be unique for the given index.
    #[must_use]
    pub const fn allocate_from_index(range: AddrRange, mut index: usize) -> AddrRange {
        let (mut start, mut size) = (range.addr().get(), range.size());
        loop {
            size /= 2;
            if size == 0 {
                break;
            }
            if index & 1 != 0 {
                start += size;
            }
            if index <= 1 {
                break;
            }
            index >>= 1;
        }
        AddrRange::new(Addr::new(start), size)
    }
    fn allocate(&self) -> Result<AllocationStorage> {
        let (index, cached_start) = {
            let mut state = self.state.lock();

            // pop_first() is important as we want to keep low indices used, and high indices free, to increase the odds of being able to free by decrementing `num_allocations` which, in turn, when it crosses power-of-two boundaries, will 2x the available size for all allocations
            let (index, cached_start) = state.free_allocation_indices.pop_first().map_or((state.num_allocations, None), |x| (x.0, Some(x.1)));

            // When increasing num_allocations in a way that crosses a power-of-two boundary, it causes the available size for all allocations to be divided by 2
            #[cfg(feature="track_allocation_sizes")]
            if self.track_allocation_sizes && index.is_power_of_two() {
                // Don't unwrap() here, the set may be empty if all allocations have a committed size of 0.
                if let Some(highest_actual_committed_size_among_allocations) = state.allocations_actual_committed_sizes.last_key_value().map(|x| *x.0) {
                    let new_available = self.actual_available_size_for_any_allocation_given_num_allocations(index + 1);
                    if new_available < highest_actual_committed_size_among_allocations {
                        return Err(Error::other(format!("Cannot create a new allocation: would reduce actual_available_size_for_any_allocation to {new_available}, which would invalidate at least one allocation because it has an actual committed size of {highest_actual_committed_size_among_allocations}")));
                    }
                }
            }

            if index == state.num_allocations {
                state.num_allocations += 1;
            }

            drop(state);

            (index, cached_start)
        };

        let committed_memory_start = cached_start.unwrap_or_else(|| Self::allocate_from_index(self.reserved_addr_range, index).addr());

        Ok(AllocationStorage { index, addr: committed_memory_start, ptr: None, size: 0 })
    }
    fn deallocate(&self, storage: &AllocationStorage) {
        let mut state = self.state.lock();

        // If this was the last live allocation, we can clear our state very efficiently
        if state.free_allocation_indices.len() + 1 == state.num_allocations {
            *state = AllocatorState::default();
            return;
        }

        let &AllocationStorage { index, addr, ptr: _, size: _ } = storage;

        // If this was the last index, no need to add to the free list.
        // This may then trigger a chain reaction where we can keep doing that.
        if index == state.num_allocations - 1 {
            state.num_allocations -= 1;
            while let Some((free_index, _)) = state.free_allocation_indices.last_key_value() {
                if *free_index == state.num_allocations - 1 {
                    state.num_allocations -= 1;
                    _ = state.free_allocation_indices.pop_last();
                } else {
                    break;
                }
            }
            return;
        }

        _ = state.free_allocation_indices.insert(index, addr);
    }
}

static_assertions::assert_impl_all!(Allocator: Send, Sync);

#[derive(Debug)]
struct AllocationStorage {
    index: usize,
    addr: Addr,
    // The pointer's addr is ALWAYS equal to `addr` above,
    // but we store the pointer separately so that its provenance is properly tracked
    ptr: Option<Unique<u8>>,
    size: usize,
}

#[derive(Debug)]
pub struct Allocation {
    allocator: Arc<Allocator>,
    // This is `None` as long as `committed_size == 0`.
    // Making this an Option is not strictly required, but it's better for ensuring that as few resources are used as possible.
    // The cost of branching on this Option is nothing compared to the benefits it gives
    storage: Option<AllocationStorage>,
    drop_result_handler: DropResultHandler,
}

impl Drop for Allocation {
    fn drop(&mut self) {
        let r = self.decommit_all();
        self.drop_result_handler.call(r);
    }
}

impl Allocation {
    pub const fn new(allocator: Arc<Allocator>, drop_result_handler: DropResultHandler) -> Self {
        Self { allocator, storage: None, drop_result_handler }
    }
    /// The implementation _may_ avoid the `Arc::clone()`, hence it takes a reference
    pub fn with_committed_size(allocator: &Arc<Allocator>, drop_result_handler: DropResultHandler, committed_size: usize, protection_flags: ProtectionFlags) -> Result<Self> {
        let mut s = Self::new(allocator.clone(), drop_result_handler);
        s.set_committed_size(committed_size, protection_flags)?;
        Ok(s)
    }
    #[must_use]
    pub const fn drop_result_handler(&self) -> &DropResultHandler {
        &self.drop_result_handler
    }
    pub fn set_drop_handler(&mut self, drop_result_handler: DropResultHandler) {
        self.drop_result_handler = drop_result_handler;
    }
    /// An alternative to `drop()` that allows you to handle the error if any
    pub fn destroy(mut self) -> Result<()> {
        self.decommit_all()
    }
    #[must_use]
    pub const fn allocator(&self) -> &Arc<Allocator> {
        &self.allocator
    }
    #[must_use] pub fn page_size(&self) -> NonZeroUsize {
        self.allocator.virtual_memory_system.page_size()
    }
    #[must_use] pub fn allocation_addr(&self) -> Option<Addr> {
        self.storage.as_ref().map(|s| s.addr)
    }
    /// NOTE: The returned pointer is only guaranteed to have an addr equal to `allocation_addr()` as long as `committed_size() >= 1`.
    #[must_use] pub fn committed_memory_start(&self) -> NonNull<u8> {
        self.storage.as_ref().map_or(NonNull::dangling(), |s| s.ptr.as_ref().map_or(NonNull::dangling(), Unique::get))
    }
    #[must_use] pub fn committed_size(&self) -> usize {
        self.storage.as_ref().map_or(0, |s| s.size)
    }
    #[must_use] pub fn actual_committed_size(&self) -> usize {
        self.committed_size().next_multiple_of(self.page_size().get())
    }
    #[must_use] pub fn committed_memory_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.committed_memory_start().as_ptr(), self.committed_size()) }
    }
    #[must_use] pub fn actual_committed_memory_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.committed_memory_start().as_ptr(), self.actual_committed_size()) }
    }
    pub fn committed_memory_slice_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.committed_memory_start().as_ptr(), self.committed_size()) }
    }
    pub fn actual_committed_memory_slice_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.committed_memory_start().as_ptr(), self.actual_committed_size()) }
    }
    #[must_use] pub fn committed_memory_nonnull_slice(&self) -> NonNull<[u8]> {
        NonNull::slice_from_raw_parts(self.committed_memory_start(), self.committed_size())
    }
    #[must_use] pub fn actual_committed_memory_nonnull_slice(&self) -> NonNull<[u8]> {
        NonNull::slice_from_raw_parts(self.committed_memory_start(), self.actual_committed_size())
    }
    /// This is suffixed `_racy` because the returned value may change from one call to the next depending on what the background threads are doing.
    #[must_use] pub fn actual_available_size_racy(&self) -> usize {
        self.allocator.actual_available_size_for_any_allocation_racy()
    }
    /// This is called `set_committed_size` because it can either grow or shrink the allocation
    pub fn set_committed_size(&mut self, new_size: usize, protection_flags: ProtectionFlags) -> Result<()> {
        if new_size == 0 {
            return self.decommit_all();
        }

        if new_size > isize::MAX as usize {
            return Err(Error::other("Cannot allocate more than isize::MAX, functions such as ptr::add() assume this"));
        }

        if self.storage.is_none() {
            self.storage = Some(self.allocator.allocate()?);
        }

        let storage = unsafe { self.storage.as_mut().unwrap_unchecked() };

        let page_size = self.allocator.virtual_memory_system.page_size().get();
        let start = storage.addr.get();
        let current_page_end = start + storage.size.next_multiple_of(page_size);
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
                        return Err(Error::other(format!("Cannot set the committed size to {new_size} (actual: {new_actual_committed_size}); available = {actual_available_size}")));
                    }
                    optional_state_lock = Some(state);
                }
                let commit_result = self.allocator.virtual_memory_system.commit(AddrRange::new(Addr::new(current_page_end), desired_page_end - current_page_end), protection_flags);
                if storage.ptr.is_none() {
                    match commit_result {
                        Ok(ptr_range) => storage.ptr = Some(unsafe { Unique::new(NonNull::new(ptr_range.ptr()).unwrap()) }),
                        Err(e) => {
                            #[cfg(feature="track_allocation_sizes")]
                            drop(optional_state_lock);

                            self.allocator.deallocate(storage);
                            self.storage = None;
                            return Err(e);
                        }
                    }
                }
                commit_result?;
            } else {
                unsafe { self.allocator.virtual_memory_system.decommit(PtrRange::new(storage.ptr.unwrap().get().with_addr(NonZeroUsize::new(desired_page_end).unwrap()).as_ptr(), current_page_end - desired_page_end)) }?;
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
        storage.size = new_size;
        Ok(())
    }
    /// Same as `set_committed_size(0)` but more efficient
    pub fn decommit_all(&mut self) -> Result<()> {
        if let Some(storage) = self.storage.as_ref() {
            unsafe { self.allocator.virtual_memory_system.decommit(PtrRange::new(storage.ptr.unwrap().get().as_ptr(), storage.size)) }?;
            self.allocator.deallocate(storage);
            self.storage = None;
        }
        Ok(())
    }
    pub fn grow(&mut self, additional_size: usize, protection_flags: ProtectionFlags) -> Result<()> {
        self.set_committed_size(self.committed_size().saturating_add(additional_size), protection_flags)
    }
    pub fn set_layout_assuming_committed_size_is_nonzero(&mut self, layout: Layout, protection_flags: ProtectionFlags) -> Result<NonNull<[u8]>> {
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

static_assertions::assert_impl_all!(Allocation: Send, Sync);

#[derive(Debug)]
pub struct LinearAllocator {
    allocation: parking_lot::Mutex<Allocation>,
    protection_flags: ProtectionFlags,
}

static_assertions::assert_impl_all!(LinearAllocator: Send, Sync);

mod linear_allocator {
    use core::ptr::NonNull;
    use core::alloc::{AllocError, Layout};
    use alloc::alloc::handle_alloc_error;

    use super::LinearAllocator;

    unsafe impl core::alloc::Allocator for LinearAllocator {
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
            self.allocation.lock().decommit_all().unwrap_or_else(|_| handle_alloc_error(layout));
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
                return self.allocation.lock().decommit_all().map(|()| NonNull::slice_from_raw_parts(NonNull::dangling(), 0)).map_err(|_| AllocError);
            }
            // If we reach here, then we must be the one non-zero allocation
            self.allocation.lock().set_layout_assuming_committed_size_is_nonzero(new_layout, self.protection_flags).map_err(|_| AllocError)
        }
    }
}

#[cfg(test)]
mod tests {
    use core::num::NonZeroUsize;
    use alloc::sync::Arc;

    use virtual_memory::ProtectionFlags;

    use super::{Allocator, Allocation, DropResultHandler};

    #[test]
    fn it_works() {
        let virtual_memory_system = Arc::new(virtual_memory::VirtualMemorySystem::new());
        let allocator = Arc::new(Allocator::create(&virtual_memory_system, NonZeroUsize::new(isize::MAX as _).unwrap(), DropResultHandler::Unwrap, Allocator::IS_TRACK_ALLOCATION_SIZES_FEATURE_ENABLED).expect("Failed to create allocator"));

        let join_handle = {
            let allocator = allocator.clone();
            std::thread::spawn(move || {
                let mut allocation = Allocation::new(allocator, DropResultHandler::Unwrap);
                allocation.set_committed_size(4096, ProtectionFlags::READ_WRITE).unwrap();
            })
        };

        let mut allocation = Allocation::new(allocator, DropResultHandler::Unwrap);
        allocation.set_committed_size(4096, ProtectionFlags::READ_WRITE).unwrap();

        join_handle.join().unwrap();
    }
}

