use std::{alloc::Layout, error::Error, marker::PhantomData, mem::MaybeUninit, num::{NonZero, NonZeroUsize}, ptr::NonNull, sync::atomic::{AtomicPtr, AtomicUsize}};

trait LowLevelAllocator {
    fn allocate_uninitialized_bytes(size: NonZero<usize>) -> Result<NonNull<[u8]>, impl Error>;

    /// # Safety
    /// 
    /// * `ptr` must denote a block of memory [*currently allocated*] via this allocator
    unsafe fn deallocate(p: NonNull<[u8]>) -> Result<(), impl Error>;
}

#[cfg(all(windows, not(miri)))]
#[allow(bad_style, dead_code)]
mod os {
    use std::{num::NonZero, ptr::NonNull};
    use std::error::Error;

    use super::{workarounds, LowLevelAllocator};

    // #[cfg(feature = "std")]
    type c_void = std::os::raw::c_void;
    // #[cfg(not(feature = "std"))]
    // enum c_void {}

    pub const PAGE_NOACCESS: u32 = 0x01;
    pub const PAGE_READONLY: u32 = 0x02;
    pub const PAGE_READWRITE: u32 = 0x04;
    pub const PAGE_WRITECOPY: u32 = 0x08;
    pub const PAGE_EXECUTE: u32 = 0x10;
    pub const PAGE_EXECUTE_READ: u32 = 0x20;
    pub const PAGE_EXECUTE_READWRITE: u32 = 0x40;
    pub const PAGE_EXECUTE_WRITECOPY: u32 = 0x80;
    pub const PAGE_GUARD: u32 = 0x100;
    pub const PAGE_NOCACHE: u32 = 0x200;
    pub const PAGE_WRITECOMBINE: u32 = 0x400;
    pub const PAGE_ENCLAVE_THREAD_CONTROL: u32 = 0x80000000;
    pub const PAGE_REVERT_TO_FILE_MAP: u32 = 0x80000000;
    pub const PAGE_TARGETS_NO_UPDATE: u32 = 0x40000000;
    pub const PAGE_TARGETS_INVALID: u32 = 0x40000000;
    pub const PAGE_ENCLAVE_UNVALIDATED: u32 = 0x20000000;
    pub const PAGE_ENCLAVE_DECOMMIT: u32 = 0x10000000;

    pub const MEM_COMMIT: u32 = 0x1000;
    pub const MEM_RESERVE: u32 = 0x2000;
    pub const MEM_DECOMMIT: u32 = 0x4000;
    pub const MEM_RELEASE: u32 = 0x8000;
    pub const MEM_FREE: u32 = 0x10000;
    pub const MEM_PRIVATE: u32 = 0x20000;
    pub const MEM_MAPPED: u32 = 0x40000;
    pub const MEM_RESET: u32 = 0x80000;
    pub const MEM_TOP_DOWN: u32 = 0x100000;
    pub const MEM_WRITE_WATCH: u32 = 0x200000;
    pub const MEM_PHYSICAL: u32 = 0x400000;
    pub const MEM_ROTATE: u32 = 0x800000;
    pub const MEM_DIFFERENT_IMAGE_BASE_OK: u32 = 0x800000;
    pub const MEM_RESET_UNDO: u32 = 0x1000000;
    pub const MEM_LARGE_PAGES: u32 = 0x20000000;
    pub const MEM_4MB_PAGES: u32 = 0x80000000;
    pub const MEM_64K_PAGES: u32 = MEM_LARGE_PAGES | MEM_PHYSICAL;

    extern "system" {
        pub fn VirtualAlloc(
            lpAddress: *mut c_void,
            dwSize: usize,
            flAllocationType: u32,
            flProtect: u32,
        ) -> *mut c_void;

        pub fn VirtualFree(
            lpAddress: *mut c_void,
            dwSize: usize,
            dwFreeType: u32,
        ) -> i32;

        // TODO: DiscardVirtualMemory(), OfferVirtualMemory() and ReclaimVirtualMemory() could be used since Windows 8.1
    }

    pub struct LowLevelAllocatorImpl;

    impl LowLevelAllocator for LowLevelAllocatorImpl {
        fn allocate_uninitialized_bytes(size: NonZero<usize>) -> Result<NonNull<[u8]>, impl Error> {
            // SAFETY: We pass valid parameters and check the result.
            let p = unsafe { VirtualAlloc(std::ptr::null_mut(), size.get(), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE) };
            if p.is_null() {
                Err(std::io::Error::last_os_error())
            } else {
                // SAFETY: The pointer is obviously non-null, we just checked.
                let p = unsafe { NonNull::new_unchecked(p) };
                Ok(NonNull::slice_from_raw_parts(p.cast(), size.get()))
            }
        }

        unsafe fn deallocate(p: NonNull<[u8]>) -> Result<(), impl Error> {
            let ok = VirtualFree(workarounds::non_null_slice_ptr(p).cast().as_ptr(), 0, MEM_RELEASE);
            if ok == 0 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            }
        }
    }
}

#[cfg(any(miri, not(windows)))]
mod os {
    use std::{error::Error, num::NonZero, ptr::NonNull};

    use super::LowLevelAllocator;
    use super::workarounds;

    pub struct LowLevelAllocatorImpl;

    impl LowLevelAllocator for LowLevelAllocatorImpl {
        fn allocate_uninitialized_bytes(size: NonZero<usize>) -> Result<NonNull<[u8]>, impl Error> {
            let mut v = Vec::<u8>::with_capacity(size.get());
            let p = v.as_mut_ptr();
            std::mem::forget(v);
            // SAFETY: The pointer of a Vec<u8> with non-zero capacity can never be null
            return Ok(NonNull::slice_from_raw_parts(unsafe { NonNull::new_unchecked(p) }, size.get()));
            #[allow(unreachable_code)]
            Err(std::io::Error::last_os_error())
        }

        unsafe fn deallocate(p: NonNull<[u8]>) -> Result<(), impl Error> {
            return Ok(drop(Vec::from_raw_parts(workarounds::non_null_slice_ptr(p).as_ptr(), 0, p.len())));
            #[allow(unreachable_code)]
            Err(std::io::Error::last_os_error())
        }
    }
}

mod workarounds {
    use std::{mem::MaybeUninit, num::NonZero, ptr::NonNull};

    // Replacement for NonNull<[T]>::as_non_null_ptr() (#[unstable(feature = "slice_ptr_get", issue = "74265")])
    pub const fn non_null_slice_ptr<T>(p: NonNull<[T]>) -> NonNull<T> {
        p.cast()
    }

    pub const fn non_null_slice_uninit<T>(p: NonNull<[T]>) -> NonNull<[MaybeUninit<T>]> {
        NonNull::slice_from_raw_parts(non_null_slice_ptr(p).cast(), p.len())
    }

    // #[unstable(feature = "nonnull_provenance", issue = "135243")]
    #[must_use]
    #[inline]
    pub const fn nonnull_without_provenance<T>(addr: NonZero<usize>) -> NonNull<T> {
        let pointer = std::ptr::without_provenance_mut(addr.get());
        // SAFETY: we know `addr` is non-zero.
        unsafe { NonNull::new_unchecked(pointer) }
    }

    #[test]
    fn non_null_slice_ptr_still_works() {
        let mut data = [0u8; 1];
        let p = NonNull::new(data.as_mut_ptr()).unwrap();
        let s = NonNull::slice_from_raw_parts(p, data.len());
        assert_eq!(non_null_slice_ptr(s), p);
    }
}

// TODO: a way to list all strong/weak refs

#[repr(C)] // I've sorted members from most accessed to least accessed, keep that order for cache efficiency.
#[derive(Debug)]
struct SuballocationHeaderInner {
    suballocation_within_arena: NonNull<[u8]>,
    strong_ref_count: usize,
    generation: usize,
    element_layout: Layout, // TODO: compress the layout's alignment?
    arena_strong_ref: Option<ArenaStrongRef>, // TODO: we could get rid of this and instead call inc_strong/dec_strong manually when a suballocation header is added/freed
}

#[repr(C)] // Just for a better debugging experience
#[derive(Debug)]
struct SuballocationHeader {
    mutex: parking_lot::Mutex<SuballocationHeaderInner>,
}

#[repr(C)] // Just for a better debugging experience
#[derive(Debug)]
struct RelocatableVecStrongRef<T> {
    suballocation_header_ptr: NonNull<SuballocationHeader>,

    // I don't fully understand yet why this is needed. Something about covariance. Rc<T> does this internally, so I'm just doing the same.
    phantom: PhantomData<(T, SuballocationHeader)>,
}

// TODO: support zero-sized types

impl<T> RelocatableVecStrongRef<T> {
    fn suballocation_header(&self) -> &SuballocationHeader {
        // SAFETY: As long as we exist, that memory will be valid
        unsafe { self.suballocation_header_ptr.as_ref() }
    }
    fn take_arena_strong_ref(&self) -> Option<ArenaStrongRef> {
        self.suballocation_header().mutex.lock().arena_strong_ref.take()
    }
    pub fn push(&mut self, _val: T) {
        unimplemented!()
    }
}

impl<T> Drop for RelocatableVecStrongRef<T> {
    fn drop(&mut self) {
        let mut suballocation_header_inner = self.suballocation_header().mutex.lock();
        assert_ne!(suballocation_header_inner.strong_ref_count, 0);
        if suballocation_header_inner.strong_ref_count == 1 {
            let arena_strong_ref = suballocation_header_inner.arena_strong_ref.take().unwrap();
            let generation = suballocation_header_inner.generation.wrapping_add(1);
            *suballocation_header_inner = SuballocationHeaderInner {
                suballocation_within_arena: NonNull::slice_from_raw_parts(workarounds::nonnull_without_provenance(NonZeroUsize::MAX), 0),
                strong_ref_count: 0,
                element_layout: Layout::new::<u8>(),
                arena_strong_ref: None,
                generation,
            };
            if let Some(free_list) = arena_strong_ref.arena_header().suballocation_headers_free_list.as_ref() {
                free_list.lock().push(self.suballocation_header_ptr);
            }
        } else {
            suballocation_header_inner.strong_ref_count -= 1;
        }
    }
}

#[repr(C)] // Just for a better debugging experience
#[derive(Debug, Clone)]
struct RelocatableVecWeakRef<T> {
    arena_weak_ref: ArenaWeakRef,
    suballocation_header_ptr: NonNull<SuballocationHeader>,
    generation: usize,
    phantom: PhantomData<T>,
}

impl<T> Default for RelocatableVecWeakRef<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> RelocatableVecWeakRef<T> {
    pub fn new() -> Self {
        Self {
            arena_weak_ref: ArenaWeakRef::new(),
            suballocation_header_ptr: workarounds::nonnull_without_provenance(NonZeroUsize::MAX),
            generation: usize::MAX,
            phantom: Default::default(),
        }
    }
    pub fn is_dangling(&self) -> bool {
        self.arena_weak_ref.is_dangling()
    }
    pub fn upgrade(&self) -> Option<RelocatableVecStrongRef<T>> {
        let arena_strong_ref = self.arena_weak_ref.upgrade()?;

        // Assert that the arena's right_area_start_ptr never shrinks, otherwise the generation trick won't work: generation counters have to stay pinned in memory for as long as the arena is in use.
        // TODO: we COULD actually shrink right_area_start_ptr but that would require the following:
        // - Knowing when ALL weak refs to a right-side-suballocation have been dropped (i.e knowing that the suballocation is unreferenced)
        // - If we allow external users to allocate from the right side: provide a mechanism to "mark as free", which could add the pointer to a free list allocated on the left side
        // - Being able to swap SuballocationHeaders without breaking live refs. Probably needs some kind of indirection.
        // - This would allow us to "bubble up" the unreferenced headers, moving them to the left
        assert!(self.suballocation_header_ptr.addr().get() >= arena_strong_ref.arena_header().right_area_start_ptr.load(std::sync::atomic::Ordering::SeqCst).addr());

        // SAFETY: we have a strong ref to the arena and have proven that the pointer is in valid range
        let mut suballocation_header_inner = unsafe { self.suballocation_header_ptr.as_ref() }.mutex.lock();
        if suballocation_header_inner.generation == self.generation {
            suballocation_header_inner.strong_ref_count += 1;
            Some(RelocatableVecStrongRef {
                suballocation_header_ptr: self.suballocation_header_ptr,
                phantom: Default::default(),
            })
        } else {
            None
        }
    }
}

// TODO: a way to list all strong/weak refs

#[repr(C)] // Just for a better debugging experience
#[derive(Debug)]
struct ArenaHeader {
    allocation: NonNull<[u8]>,
    strong_ref_count: AtomicUsize,
    weak_ref_count: AtomicUsize,
    left_area_end_ptr: AtomicPtr<u8>,
    right_area_start_ptr: AtomicPtr<u8>,
    suballocation_headers_free_list: Option<parking_lot::Mutex<RelocatableVecStrongRef<NonNull<SuballocationHeader>>>>,
}

impl ArenaHeader {
    pub fn min_required_size() -> NonZero<usize> {
        NonZero::new(std::mem::align_of::<ArenaHeader>() - 1 + std::mem::size_of::<ArenaHeader>()).unwrap()
    }
    pub fn create(size: NonZero<usize>) -> ArenaStrongRef {
        let allocation = os::LowLevelAllocatorImpl::allocate_uninitialized_bytes(size).unwrap();
        assert!(allocation.len() >= size.get());
        Self::with_allocation_impl(allocation)
    }
    /// # Safety
    /// 
    /// * You must ensure that the `allocation` lives at least as long as the ArenaStrongRef.
    pub unsafe fn with_allocation(allocation: NonNull<[u8]>) -> ArenaStrongRef {
        // TODO: we should take the deallocator as well, in order to know how to free the memory!
        // TODO: feature: when trying to allocate from an arena, if there is no more room, it could call into some user-provided strategy, possibly attempting to create a new arena with some capacity, and then redirect to THAT arena. This should be useful during development if we are on a good machine and memory usage momentarily exceeds the expected amount due to a poorly optimized gameplay section; would allow us to issue a warning but not interrupt whatever we're trying to do (profiling, debugging, looking for an issue, etc).
        Self::with_allocation_impl(allocation)
    }
    // This function is "safe" as long as it's not used externally.
    // The public API is with_allocation() and marked as unsafe.
    // This function is NOT marked as unsafe, to force us to highlight the unsafe places within the body.
    fn with_allocation_impl(allocation: NonNull<[u8]>) -> ArenaStrongRef {
        assert!(allocation.len() >= Self::min_required_size().get());
        assert!(allocation.len() <= isize::MAX as usize); // std's Vec panics in that case
        // TODO: memset(0) in order to force physical pages to be allocated (consider multi-threading that?). "committing" in Windows does not do that. https://learn.microsoft.com/en-us/windows/win32/memory/reserving-and-committing-memory?redirectedfrom=MSDN
        // TODO: Add Miri and run "MIRIFLAGS='-Zmiri-strict-provenance' cargo miri test"
        let arena_header_slice = unsafe { 
            workarounds::non_null_slice_uninit(allocation)
            .as_mut()
            .align_to_mut::<MaybeUninit<ArenaHeader>>().1
        };
        // SAFETY: the result pointer is still in bounds
        let left_area_end_ptr = AtomicPtr::new(unsafe { arena_header_slice.as_mut_ptr().add(1) }.cast());
        arena_header_slice[0].write(ArenaHeader {
            allocation,
            strong_ref_count: AtomicUsize::new(1),
            weak_ref_count: AtomicUsize::new(1), // The entire set of all strong refs holds 1 weak ref
            left_area_end_ptr,
            // SAFETY: the result pointer is still in bounds
            right_area_start_ptr: AtomicPtr::new(unsafe { workarounds::non_null_slice_ptr(allocation).add(allocation.len()) }.as_ptr()),
            suballocation_headers_free_list: None,
        });
        // SAFETY: we called write() just above
        let arena_header = unsafe { arena_header_slice[0].assume_init_mut() };
        arena_header.suballocation_headers_free_list = arena_header.create_relocatable_vec_internal_to_this_arena().map(parking_lot::Mutex::new);
        assert_eq!(arena_header.strong_ref_count.load(std::sync::atomic::Ordering::SeqCst), 1, "After creating internal structures, the arena's strong ref count must still be 1");
        let arena_header_ptr = NonNull::from(arena_header);
        assert_ne!(arena_header_ptr.addr(), NonZeroUsize::MAX);
        ArenaStrongRef { arena_header_ptr, phantom: PhantomData::default() }
    }
    pub fn client_area(&self) -> NonNull<[u8]> {
        let self_p = NonNull::from(self);
        // SAFETY: self_p is definitely from an allocated object, and the addition will not overflow
        let start = unsafe { self_p.add(1) }.cast();
        let end = self.allocation.addr().checked_add(self.allocation.len()).unwrap();
        debug_assert!(self_p.addr() >= self.allocation.addr() && start.addr() <= end);
        NonNull::slice_from_raw_parts(start, end.get() - start.addr().get())
    }
    pub unsafe fn drop_suballocations(&self) {
        // TODO: attempt to free inner allocations. Then shrink our allocation until only the memory for the header remains.
        // Should be a matter of calling VirtualFree(client_area().round_up_to_page_size(), MEM_DECOMMIT)
        unimplemented!()
    }
    // TODO: parallel allocations, frees, and compaction
    // TODO: pouvoir itérer sur toutes les suballocations de l'arena
    // TODO: thought experiment (juste pour le lol/tester): dans la zone droite d'une arena, faire une suballocation, et créer une arena dedans.
    pub fn create_relocatable_vec<T: Unpin>(&self) -> Option<RelocatableVecStrongRef<T>> {
        unimplemented!()
    }
    fn create_relocatable_vec_internal_to_this_arena<T: Unpin>(&self) -> Option<RelocatableVecStrongRef<T>> {
        self.create_relocatable_vec().map(|x| {
            x.take_arena_strong_ref();
            x
        })
    }
}

impl Drop for ArenaHeader {
    fn drop(&mut self) {
        // SAFETY: we are only dropped when there are no more strong or weak references to us.
        // We make sure that our API only provides sub-allocations that refer to us by strong or weak reference.
        // Therefore there is no risk of use-after-free.
        unsafe { os::LowLevelAllocatorImpl::deallocate(self.allocation).unwrap() }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ArenaStrongRef {
    arena_header_ptr: NonNull<ArenaHeader>,

    // I don't fully understand yet why this is needed. Something about covariance. Rc<T> does this internally, so I'm just doing the same.
    phantom: PhantomData<ArenaHeader>,
}

impl ArenaStrongRef {
    fn arena_header(&self) -> &ArenaHeader {
        // SAFETY: As long as we exist, header_ptr points to accessible and initialized memory. We also make sure to never access it via &mut unless we are the sole owner.
        unsafe { self.arena_header_ptr.as_ref() }
    }
    pub fn downgrade(&self) -> ArenaWeakRef {
        self.arena_header().weak_ref_count.fetch_add(1, std::sync::atomic::Ordering::Acquire);
        ArenaWeakRef {
            arena_header_ptr: self.arena_header_ptr,
        }
    }
}

unsafe impl Send for ArenaStrongRef {}
unsafe impl Sync for ArenaStrongRef {}

impl Clone for ArenaStrongRef {
    fn clone(&self) -> Self {
        self.arena_header().strong_ref_count.fetch_add(1, std::sync::atomic::Ordering::Acquire);
        Self {
            arena_header_ptr: self.arena_header_ptr,
            phantom: PhantomData::default(),
        }
    }
}

impl Drop for ArenaStrongRef {
    fn drop(&mut self) {
        let old_strong_ref_count = self.arena_header().strong_ref_count.fetch_sub(1, std::sync::atomic::Ordering::Release);
        assert_ne!(old_strong_ref_count, 0);
        if old_strong_ref_count == 1 {
            self.arena_header().strong_ref_count.load(std::sync::atomic::Ordering::Acquire); // Barrier: same as Rust's "sync.rs"'s acquire!() macro
            // SAFETY: We were the last strong owner
            unsafe { self.arena_header().drop_suballocations() };
            drop(ArenaWeakRef { arena_header_ptr: self.arena_header_ptr });
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ArenaWeakRef {
    arena_header_ptr: NonNull<ArenaHeader>,
}

impl Default for ArenaWeakRef {
    fn default() -> Self {
        Self::new()
    }
}

impl ArenaWeakRef {
    pub const fn new() -> Self {
        Self {
            arena_header_ptr: workarounds::nonnull_without_provenance(NonZeroUsize::MAX),
        }
    }
    pub fn is_dangling(&self) -> bool {
        self.arena_header_ptr.addr() == NonZeroUsize::MAX
    }
    fn arena_header(&self) -> Option<&ArenaHeader> {
        if self.is_dangling() {
            None
        } else {
            // SAFETY: As long as we exist, header_ptr points to accessible and initialized memory. We also make sure to never access it via &mut unless we are the sole owner.
            Some(unsafe { self.arena_header_ptr.as_ref() })
        }
    }
    pub fn upgrade(&self) -> Option<ArenaStrongRef> {
        //
        // Copied from Arc::upgrade()
        //

        #[inline]
        fn checked_increment(n: usize) -> Option<usize> {
            // Any write of 0 we can observe leaves the field in permanently zero state.
            if n == 0 {
                return None;
            }
            Some(n + 1)
        }

        if self.arena_header()?.strong_ref_count.fetch_update(std::sync::atomic::Ordering::Acquire, std::sync::atomic::Ordering::Relaxed, checked_increment).is_ok() {
            Some(ArenaStrongRef {
                arena_header_ptr: self.arena_header_ptr,
                phantom: PhantomData::default(),
            })
        } else {
            None
        }
    }
}

unsafe impl Send for ArenaWeakRef {}
unsafe impl Sync for ArenaWeakRef {}

impl Clone for ArenaWeakRef {
    fn clone(&self) -> Self {
        if let Some(header) = self.arena_header() {
            header.weak_ref_count.fetch_add(1, std::sync::atomic::Ordering::Acquire);
        }
        Self {
            arena_header_ptr: self.arena_header_ptr,
        }
    }
}

impl Drop for ArenaWeakRef {
    fn drop(&mut self) {
        let old_weak_ref_count;
        // Don't hold the reference for long. See further below for why.
        if let Some(header) = self.arena_header() {
            old_weak_ref_count = header.weak_ref_count.fetch_sub(1, std::sync::atomic::Ordering::Release);
        } else {
            return;
        }
        assert_ne!(old_weak_ref_count, 0);
        if old_weak_ref_count == 1 {
            // Re-get and unwrap in order not to hold the reference for long. See below.
            self.arena_header().unwrap().weak_ref_count.load(std::sync::atomic::Ordering::Acquire); // Barrier: same as Rust's "sync.rs"'s acquire!() macro
            // SAFETY: We are the only referencer and have ensured we're not holding on to the header by reference in this function
            unsafe { self.arena_header_ptr.drop_in_place() };
        }
    }
}

#[test]
fn test_arena() {
    let _arena = ArenaHeader::create(NonZero::new(2048).unwrap());
}

