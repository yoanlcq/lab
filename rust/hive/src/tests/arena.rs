use std::{error::Error, marker::PhantomData, mem::MaybeUninit, num::{NonZero, NonZeroUsize}, ptr::NonNull, sync::atomic::AtomicUsize};

trait LowLevelAllocator {
    fn allocate_uninitialized_bytes(size: NonZero<usize>) -> Result<NonNull<[u8]>, impl Error>;

    /// # Safety
    /// 
    /// * `ptr` must denote a block of memory [*currently allocated*] via this allocator
    unsafe fn deallocate(p: NonNull<[u8]>) -> Result<(), impl Error>;
}

#[cfg(windows)]
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

#[cfg(not(windows))]
mod os {
    use std::{error::Error, num::NonZero, ptr::NonNull};

    use super::LowLevelAllocator;
    use super::workarounds;

    pub struct LowLevelAllocatorImpl;

    impl LowLevelAllocator for LowLevelAllocatorImpl {
        fn allocate_uninitialized_bytes(size: NonZero<usize>) -> Result<NonNull<[u8]>, impl Error> {
            let mut v = Vec::<MaybeUninit<u8>>::with_capacity(size.get());
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

#[repr(C)] // Just for a better debugging experience
#[derive(Debug)]
struct ArenaHeader {
    allocation: NonNull<[u8]>,
    strong_ref_count: AtomicUsize,
    weak_ref_count: AtomicUsize,
}

impl ArenaHeader {
    pub fn min_required_size() -> NonZero<usize> {
        NonZero::new(std::mem::align_of::<ArenaHeader>() - 1 + std::mem::size_of::<ArenaHeader>()).unwrap()
    }
    pub fn create(size: NonZero<usize>) -> ArenaStrongRef {
        assert!(size >= Self::min_required_size());
        let allocation = os::LowLevelAllocatorImpl::allocate_uninitialized_bytes(size).unwrap();
        assert!(allocation.len() >= size.get());
        // TODO: memset(0) in order to force physical pages to be allocated (consider multi-threading that?). "committing" in Windows does not do that. https://learn.microsoft.com/en-us/windows/win32/memory/reserving-and-committing-memory?redirectedfrom=MSDN
        // TODO: Add Miri and run "MIRIFLAGS='-Zmiri-strict-provenance' cargo miri test"
        let header_slice = unsafe { 
            workarounds::non_null_slice_uninit(allocation)
            .as_mut()
            .align_to_mut::<MaybeUninit<ArenaHeader>>().1
        };
        header_slice[0].write(ArenaHeader {
            allocation,
            strong_ref_count: AtomicUsize::new(1),
            weak_ref_count: AtomicUsize::new(1), // The entire set of all strong refs holds 1 weak ref
        });
        let header_ptr = NonNull::from(&header_slice[0]).cast();
        assert_ne!(header_ptr.addr(), NonZeroUsize::MAX);
        ArenaStrongRef { header_ptr, phantom: PhantomData::default() }
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
        // TODO: attempt to free inner allocations. Then shrink our allocation until only the memory for the header remains
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
    header_ptr: NonNull<ArenaHeader>,

    // I don't fully understand yet why this is needed. Something about covariance. Rc<T> does this internally, so I'm just doing the same.
    phantom: PhantomData<ArenaHeader>,
}

impl ArenaStrongRef {
    fn header(&self) -> &ArenaHeader {
        // SAFETY: As long as we exist, header_ptr points to accessible and initialized memory. We also make sure to never access it via &mut unless we are the sole owner.
        unsafe { self.header_ptr.as_ref() }
    }
    pub fn downgrade(&self) -> ArenaWeakRef {
        self.header().weak_ref_count.fetch_add(1, std::sync::atomic::Ordering::Acquire);
        ArenaWeakRef {
            header_ptr: self.header_ptr,
        }
    }
}

unsafe impl Send for ArenaStrongRef {}
unsafe impl Sync for ArenaStrongRef {}

impl Clone for ArenaStrongRef {
    fn clone(&self) -> Self {
        self.header().strong_ref_count.fetch_add(1, std::sync::atomic::Ordering::Acquire);
        Self {
            header_ptr: self.header_ptr,
            phantom: PhantomData::default(),
        }
    }
}

impl Drop for ArenaStrongRef {
    fn drop(&mut self) {
        let old_strong_ref_count = self.header().strong_ref_count.fetch_sub(1, std::sync::atomic::Ordering::Release);
        assert_ne!(old_strong_ref_count, 0);
        if old_strong_ref_count == 1 {
            self.header().strong_ref_count.load(std::sync::atomic::Ordering::Acquire); // Barrier: same as Rust's "sync.rs"'s acquire!() macro
            // SAFETY: We were the last strong owner
            unsafe { self.header().drop_suballocations() };
            drop(ArenaWeakRef { header_ptr: self.header_ptr });
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ArenaWeakRef {
    header_ptr: NonNull<ArenaHeader>,
}

impl Default for ArenaWeakRef {
    fn default() -> Self {
        Self::new()
    }
}

impl ArenaWeakRef {
    pub const fn new() -> Self {
        Self {
            header_ptr: workarounds::nonnull_without_provenance(NonZeroUsize::MAX),
        }
    }
    pub fn is_dangling(&self) -> bool {
        self.header_ptr.addr() == NonZeroUsize::MAX
    }
    fn header(&self) -> Option<&ArenaHeader> {
        if self.is_dangling() {
            None
        } else {
            // SAFETY: As long as we exist, header_ptr points to accessible and initialized memory. We also make sure to never access it via &mut unless we are the sole owner.
            Some(unsafe { self.header_ptr.as_ref() })
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

        if self.header()?.strong_ref_count.fetch_update(std::sync::atomic::Ordering::Acquire, std::sync::atomic::Ordering::Relaxed, checked_increment).is_ok() {
            Some(ArenaStrongRef {
                header_ptr: self.header_ptr,
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
        if let Some(header) = self.header() {
            header.weak_ref_count.fetch_add(1, std::sync::atomic::Ordering::Acquire);
        }
        Self {
            header_ptr: self.header_ptr,
        }
    }
}

impl Drop for ArenaWeakRef {
    fn drop(&mut self) {
        let old_weak_ref_count;
        // Don't hold the reference for long. See further below for why.
        if let Some(header) = self.header() {
            old_weak_ref_count = header.weak_ref_count.fetch_sub(1, std::sync::atomic::Ordering::Release);
        } else {
            return;
        }
        assert_ne!(old_weak_ref_count, 0);
        if old_weak_ref_count == 1 {
            // Re-get and unwrap in order not to hold the reference for long. See below.
            self.header().unwrap().weak_ref_count.load(std::sync::atomic::Ordering::Acquire); // Barrier: same as Rust's "sync.rs"'s acquire!() macro
            // SAFETY: We are the only referencer and have ensured we're not holding on to the header by reference in this function
            unsafe { self.header_ptr.drop_in_place() };
        }
    }
}

#[test]
fn test_arena() {
    let _arena = ArenaHeader::create(NonZero::new(2048).unwrap());
}

