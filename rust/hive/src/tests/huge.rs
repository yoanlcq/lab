// Why this name:
// https://ell.stackexchange.com/a/250169

use std::{alloc::{handle_alloc_error, AllocError, Layout}, collections::BTreeMap, num::NonZeroUsize, ptr::NonNull, sync::Arc};

use virtual_memory::{ProtectionFlags, VirtualMemorySystem};

mod virtual_memory {
    use std::{num::{NonZeroUsize}, ptr::NonNull};

    pub use std::io::Error as Error;

    use bitflags::bitflags;

    #[cfg(windows)]
    mod windows_imp {
        use super::Error;
        use std::{mem::MaybeUninit, ptr::NonNull};

        #[inline(always)]
        pub fn get_system_info() -> winapi::um::sysinfoapi::SYSTEM_INFO {
            let mut info = MaybeUninit::uninit();
            unsafe {
                winapi::um::sysinfoapi::GetSystemInfo(info.as_mut_ptr());
                info.assume_init()
            }
        }

        pub fn virtual_alloc(starting_address_hint: *mut u8, size: usize, flags: u32, protection: u32) -> Result<NonNull<u8>, Error> {
            let p = unsafe { winapi::um::memoryapi::VirtualAlloc(starting_address_hint.cast(), size, flags, protection) };
            if p.is_null() {
                Err(Error::last_os_error())
            } else {
                Ok(unsafe { NonNull::new_unchecked(p) }.cast())
            }
        }

        pub unsafe fn virtual_free(ptr: *mut u8, size: usize, flags: u32) -> Result<(), Error> {
            match winapi::um::memoryapi::VirtualFree(ptr.cast(), size, flags) {
                0 => Err(Error::last_os_error()),
                _ => Ok(())
            }
        }

        pub fn virtual_query(ptr: *mut u8) -> Result<winapi::um::winnt::MEMORY_BASIC_INFORMATION, Error> {
            let mut info = MaybeUninit::uninit();
            unsafe {
                match winapi::um::memoryapi::VirtualQuery(ptr.cast(), info.as_mut_ptr(), std::mem::size_of_val(&info)) {
                    0 => Err(Error::last_os_error()),
                    _ => Ok(info.assume_init()),
                }
            }
        }

        // Returns the old protection flags of the FIRST page in the range
        pub fn virtual_protect(ptr: *mut u8, size: usize, flags: u32) -> Result<u32, Error> {
            let mut old_protect = 0;
            match unsafe { winapi::um::memoryapi::VirtualProtect(ptr.cast(), size, flags, &mut old_protect) } {
                0 => Err(Error::last_os_error()),
                _ => Ok(old_protect),
            }
        }

        pub fn virtual_lock(ptr: *mut u8, size: usize) -> Result<(), Error> {
            match unsafe { winapi::um::memoryapi::VirtualLock(ptr.cast(), size) } {
                0 => Err(Error::last_os_error()),
                _ => Ok(()),
            }
        }

        pub fn virtual_unlock(ptr: *mut u8, size: usize) -> Result<(), Error> {
            match unsafe { winapi::um::memoryapi::VirtualUnlock(ptr.cast(), size) } {
                0 => Err(Error::last_os_error()),
                _ => Ok(()),
            }
        }
    }

    /// This represents a "handle" to the OS's virtual memory API.
    ///
    /// In practice (e.g on Windows or Unix) there is generally no such thing, however what is interesting and maybe "expensive" to get is system information such as page size.
    /// So you could choose to instantiate as many `VirtualMemorySystem`s as you like if you're fine with that, or to instantiate only one. Your call, depending on what you know or prefer.
    #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct VirtualMemorySystem {
        page_size: NonZeroUsize,
        allocation_granularity: NonZeroUsize,
    }

    pub type OsProtectionFlags = u32;

    bitflags! {
        // NOTE: If you change **any** of these values, you must update the `to_windows()` function!
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct ProtectionFlags: OsProtectionFlags {
            const READ               = 0x001;
            const WRITE              = 0x002;
            const EXECUTE            = 0x004;
            const READ_WRITE         = 0x003;
            const READ_WRITE_EXECUTE = 0x007;
            const ACCESS_MASK        = 0x007;
            // Advanced, should not be used unless you now what you're doing; may cause crashes or such.
            const GUARD              = 0x100; // PAGE_GUARD on Windows
            const UNCACHED           = 0x200; // PAGE_NOCACHE on Windows
            const WRITECOMBINE       = 0x400; // PAGE_WRITECOMBINE on Windows
            const MODIFIERS_MASK     = 0x700;
        }
    }

    impl Default for ProtectionFlags {
        fn default() -> Self {
            Self::READ_WRITE
        }
    }

    impl ProtectionFlags {
        #[cfg(windows)] // This isn't strictly required but I do this to remove noise for other platforms
        pub fn to_windows(&self) -> OsProtectionFlags {
            let modifiers = (*self & Self::MODIFIERS_MASK).bits();
            let mut access = (*self & Self::ACCESS_MASK).bits();
            if access == 0 {
                return modifiers | 1; // PAGE_NOACCESS
            }
            if access & Self::EXECUTE.bits() != 0 {
                access = access & !Self::EXECUTE.bits();
                access <<= 4;
            }
            modifiers | (access << 1)
        }
        #[cfg(windows)] // This isn't strictly required but I do this to remove noise for other platforms
        pub fn from_windows(mut flags: OsProtectionFlags) -> Self {
            let modifiers = flags & Self::MODIFIERS_MASK.bits();

            let has_execute = flags & 0xf0 != 0;
            if has_execute {
                flags >>= 4;
            }
            flags >>= 1; // PAGE_NOACCESS
            flags &= 3;

            Self::from_bits_retain(modifiers | flags | if has_execute { 4 } else { 0 })
        }
        pub fn from_os(flags: OsProtectionFlags) -> Self {
            #[cfg(windows)]
            Self::from_windows(flags)
        }
    }

    // TODO: expose MEM_WRITE_WATCH and similar APIs?
    impl VirtualMemorySystem {
        // How "expensive" this is depends on the OS, but it's generally cheap and you typically don't need to do this many times.
        pub fn new() -> Self {
            #[cfg(windows)]
            {
                let winapi::um::sysinfoapi::SYSTEM_INFO { dwPageSize, dwAllocationGranularity, .. } = windows_imp::get_system_info();
                Self {
                    page_size: NonZeroUsize::new(dwPageSize as _).unwrap(),
                    allocation_granularity: NonZeroUsize::new(dwAllocationGranularity as _).unwrap(),
                }
            }
        }

        #[inline(always)]
        pub fn page_size(&self) -> NonZeroUsize {
            self.page_size
        }

        // The granularity for the starting address at which virtual memory can be allocated
        #[inline(always)]
        pub fn allocation_granularity(&self) -> NonZeroUsize {
            self.allocation_granularity
        }

        /// Attempts to reserve a virtual address range.
        ///
        /// Despite the usual meaning we attribute to the `reserve` verb, This does **not** actually allocate any memory, this only ensures the virtual address range is "yours" to use:
        /// that is, if some other code (such as the global allocator) wants to reserve a virtual address range in the current process,
        /// they will have to call the same underlying API, and it will not return a range that intersects any of the currently reserved ones.
        ///
        /// `starting_address_hint` may be `null` to let the OS decide the starting address automatically.
        /// 
        /// This can only reserve whole pages, therefore the range defined by the provided parameters is implicitly "extended" in both directions until its bounds are aligned to a page boundary.
        /// For instance, if `page_size() == 4096`, `starting_address_hint == 4095` and `size == 2`, then assuming the call succeeds, the reserved range will be `0 .. 8192`.
        #[inline(always)]
        pub fn reserve(&self, starting_address_hint: Option<NonNull<u8>>, size: NonZeroUsize) -> Result<NonNull<u8>, Error> {
            #[cfg(windows)]
            windows_imp::virtual_alloc(starting_address_hint.map(NonNull::as_ptr).unwrap_or(std::ptr::null_mut()), size.get(), winapi::um::winnt::MEM_RESERVE, winapi::um::winnt::PAGE_NOACCESS)
        }

        /// NOTE: It is unspecified whether `protection_flags` will be applied to pages that were already committed. TODO: test this with `VirtualAlloc` on Windows.
        /// 
        /// When a page transitions from "reserved" to "committed", its memory is zeroed. Note that this does not happen when committing an already-committed page.
        /// 
        /// This can only operate on whole pages, therefore the range defined by the provided parameters is implicitly "extended" in both directions until its bounds are aligned to a page boundary.
        #[inline(always)]
        pub fn commit(&self, ptr: *mut u8, size: usize, protection_flags: ProtectionFlags) -> Result<(), Error> {
            #[cfg(windows)]
            windows_imp::virtual_alloc(ptr, size, winapi::um::winnt::MEM_COMMIT, protection_flags.to_windows()).map(|_| ())
        }

        /// This can only operate on whole pages, therefore the range defined by the provided parameters is implicitly "extended" in both directions until its bounds are aligned to a page boundary.
        /// 
        /// Safety: You must make sure that nobody else is currently using that memory.
        #[inline(always)]
        pub unsafe fn decommit(&self, ptr: *mut u8, size: usize) -> Result<(), Error> {
            #[cfg(windows)]
            windows_imp::virtual_free(ptr, size, winapi::um::winnt::MEM_DECOMMIT)
        }

        /// This can only operate on whole pages, therefore the range defined by the provided parameters is implicitly "extended" in both directions until its bounds are aligned to a page boundary.
        /// 
        /// Safety:
        /// - You MUST pass the ENTIRE range previously returned by a call to `reserve` (i.e: the returned pointer, and the requested size).
        /// - You must make sure that nobody else is currently using that memory.
        ///
        /// On Windows, this will implicitly de-commit the pages for you before unreserving.
        #[inline(always)]
        pub unsafe fn unreserve(&self, ptr: NonNull<u8>, size: NonZeroUsize) -> Result<(), Error> {
            #[cfg(windows)]
            windows_imp::virtual_free(ptr.as_ptr(), 0 * size.get() /* Must pass 0 and prevent "unused variable `size`" warning */, winapi::um::winnt::MEM_RELEASE)
        }

        // If succeeds, returns the previous protection flags of the FIRST page that intersects the specified range
        pub fn set_protection_flags(&self, ptr: *mut u8, size: usize, flags: ProtectionFlags) -> Result<OsProtectionFlags, Error> {
            #[cfg(windows)]
            windows_imp::virtual_protect(ptr, size, flags.to_windows())
        }

        pub fn lock_to_physical(&self, ptr: *mut u8, size: usize) -> Result<(), Error> {
            #[cfg(windows)]
            windows_imp::virtual_lock(ptr, size)
        }

        pub fn unlock_from_physical(&self, ptr: *mut u8, size: usize) -> Result<(), Error> {
            #[cfg(windows)]
            windows_imp::virtual_unlock(ptr, size)
        }

        pub fn page_range_info(&self, ptr: *mut u8) -> Result<PageRangeInfo, Error> {
            #[cfg(windows)]
            windows_imp::virtual_query(ptr).map(|x| PageRangeInfo::from_windows(&x))
        }

        pub fn page_range_info_iter(&self, ptr: *mut u8) -> PageRangeInfoIterator {
            PageRangeInfoIterator { virtual_memory_system: self, ptr, finished: false }
        }
    }

    pub struct PageRangeInfoIterator<'a> {
        virtual_memory_system: &'a VirtualMemorySystem,
        ptr: *mut u8,
        finished: bool,
    }

    impl<'a> Iterator for PageRangeInfoIterator<'a> {
        type Item = Result<PageRangeInfo, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.finished {
                return None;
            }
            let r = self.virtual_memory_system.page_range_info(self.ptr);
            if let Ok(info) = r.as_ref() {
                self.ptr = self.ptr.wrapping_add(info.size);
            } else {
                self.finished = true;
            }
            Some(r)
        }
    }

    impl PageRangeInfo {
        #[cfg(windows)]
        fn from_windows(info: &winapi::um::winnt::MEMORY_BASIC_INFORMATION) -> Self {
            let &winapi::um::winnt::MEMORY_BASIC_INFORMATION {
                BaseAddress, AllocationBase, AllocationProtect, RegionSize, State, Protect, Type
            } = info;

            let not_free = if State == winapi::um::winnt::MEM_FREE {
                None
            } else {
                Some(PageRangeInfoNotFree {
                    protection_flags: if State == winapi::um::winnt::MEM_RESERVE { None } else { Some(Protect) },
                    type_: PageType::try_from_windows(Type).unwrap(),
                    allocation_ptr: AllocationBase.cast(),
                    allocation_protection_flags: AllocationProtect,
                })
            };

            Self {
                ptr: BaseAddress.cast(),
                size: RegionSize,
                state: PageState::try_from_windows(State).unwrap(),
                not_free,
            }
        }
        pub fn ptr(&self) -> *mut u8 {
            self.ptr
        }
        pub fn size(&self) -> usize {
            self.size
        }
        pub fn state(&self) -> PageState {
            self.state
        }
        pub fn os_protection_flags(&self) -> Option<OsProtectionFlags> {
            self.not_free.map(|x| x.protection_flags)?
        }
        pub fn type_(&self) -> Option<PageType> {
            self.not_free.map(|x| x.type_)
        }
        pub fn allocation_ptr(&self) -> Option<*mut u8> {
            self.not_free.map(|x| x.allocation_ptr)
        }
        pub fn allocation_os_protection_flags(&self) -> Option<OsProtectionFlags> {
            self.not_free.map(|x| x.allocation_protection_flags)
        }
    }

    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct PageRangeInfo {
        ptr: *mut u8,
        size: usize,
        state: PageState,
        not_free: Option<PageRangeInfoNotFree>,
    }

    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    struct PageRangeInfoNotFree {
        // Undefined if state == Reserved
        protection_flags: Option<OsProtectionFlags>,
        type_: PageType,
        allocation_ptr: *mut u8,
        // May be 0 if the caller does not have access
        allocation_protection_flags: OsProtectionFlags,
    }

    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(u32)]
    pub enum PageState {
        Committed = 0x1000, // MEM_COMMIT
        Free = 0x10000, // MEM_FREE
        Reserved = 0x2000, // MEM_RESERVE
    }

    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(u32)]
    pub enum PageType {
        Image = 0x1000000, // MEM_IMAGE
        Mapped = 0x40000, // MEM_MAPPED
        Private = 0x20000, // MEM_PRIVATE
    }

    impl PageState {
        #[cfg(windows)]
        fn try_from_windows(x: u32) -> Option<Self> {
            Some(match x {
                x if x == Self::Committed as _ => Self::Committed,
                x if x == Self::Free as _ => Self::Free,
                x if x == Self::Reserved as _ => Self::Reserved,
                _ => None?
            })
        }
    }

    impl PageType {
        #[cfg(windows)]
        fn try_from_windows(x: u32) -> Option<Self> {
            Some(match x {
                x if x == Self::Image as _ => Self::Image,
                x if x == Self::Mapped as _ => Self::Mapped,
                x if x == Self::Private as _ => Self::Private,
                _ => None?
            })
        }
    }
}

// TODO: enumerate memory ranges, just out of curiosity (https://stackoverflow.com/a/20350190)
// TODO: test that the functions work... In particular, reserve() does not set protection flags, will it work?
// TODO: check that creating Allocations from multiple threads is possible
// TODO: split into multiple crates
// TODO: Provide good (and illustrated) documentation

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
    reserved_virtual_address_range_start: NonNull<u8>,
    // Power of two AND multiple of page size
    reserved_virtual_address_range_size: NonZeroUsize,
    state: parking_lot::Mutex<AllocatorState>,
    // This is provided outside of `state` to avoid locking.
    // For the sake of correctness, this flag must only be changed while no Allocation exists.
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
    pub fn create(virtual_memory_system: &Arc<VirtualMemorySystem>, starting_size: NonZeroUsize, track_allocations: bool) -> Result<Self, virtual_memory::Error> {
        #[cfg(not(feature="track_allocations"))]
        if track_allocations {
            return Err(virtual_memory::Error::other("Feature \"track_allocations\" is disabled. Please use IS_TRACK_ALLOCATIONS_FEATURE_ENABLED to detect this and make your code explicit"));
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
    pub fn is_tracking_allocations(&self) -> bool {
        #[cfg(feature="track_allocations")]
        {
            self.is_tracking_allocations
        }
        #[cfg(not(feature="track_allocations"))]
        false
    }
    pub const IS_TRACK_ALLOCATIONS_FEATURE_ENABLED: bool = cfg!(feature="track_allocations");
    #[inline(always)]
    pub fn can_track_allocations_racy(&self) -> bool {
        Self::IS_TRACK_ALLOCATIONS_FEATURE_ENABLED && self.state.lock().num_allocations == 0
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
            #[cfg(feature="track_allocations")]
            if self.is_tracking_allocations && index.is_power_of_two() {
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
                self.allocator.virtual_memory_system.commit(unsafe { storage.committed_memory_start.add(current_page_end) }.as_ptr(), desired_page_end - current_page_end, protection_flags)?;
            } else {
                unsafe { self.allocator.virtual_memory_system.decommit(storage.committed_memory_start.add(desired_page_end).as_ptr(), current_page_end - desired_page_end) }?;
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
        self.allocation.lock().decommit_all().unwrap_or_else(|e| handle_alloc_error(layout))
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
        self.grow(ptr, old_layout, new_layout)
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