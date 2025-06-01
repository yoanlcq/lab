// TODO: expose MEM_WRITE_WATCH and similar APIs?

use std::{num::{NonZeroUsize}, ptr::NonNull};

pub use std::io::Error as Error;

use bitflags::bitflags;

#[cfg(windows)]
mod windows_imp;

/// This represents a "handle" to the OS's virtual memory API.
///
/// In practice (e.g on Windows or Unix) there is generally no such thing, however what is interesting and maybe "expensive" to get is system information such as page size.
/// So you could choose to instantiate as many `VirtualMemorySystem`s as you like if you're fine with that, or to instantiate only one. Your call, depending on what you know or prefer.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtualMemorySystem {
    page_size: NonZeroUsize,
    allocation_granularity: NonZeroUsize,
}

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

    /// NOTE: It is unspecified whether `protection_flags` will be applied to pages that were already committed.
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
        unsafe { windows_imp::virtual_free(ptr, size, winapi::um::winnt::MEM_DECOMMIT) }
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
        unsafe { windows_imp::virtual_free(ptr.as_ptr(), 0 * size.get() /* Must pass 0 and prevent "unused variable `size`" warning */, winapi::um::winnt::MEM_RELEASE) }
    }

    // If succeeds, returns the previous protection flags of the FIRST page that intersects the specified range
    #[inline(always)]
    pub fn set_protection_flags(&self, ptr: *mut u8, size: usize, flags: ProtectionFlags) -> Result<OsProtectionFlags, Error> {
        #[cfg(windows)]
        windows_imp::virtual_protect(ptr, size, flags.to_windows())
    }

    #[inline(always)]
    pub fn bind_to_physical_memory(&self, ptr: *mut u8, size: usize) -> Result<(), Error> {
        #[cfg(windows)]
        windows_imp::virtual_lock(ptr, size)
    }

    #[inline(always)]
    pub fn unbind_from_physical_memory(&self, ptr: *mut u8, size: usize) -> Result<(), Error> {
        #[cfg(windows)]
        windows_imp::virtual_unlock(ptr, size)
    }

    #[inline(always)]
    pub fn page_range_info(&self, ptr: *mut u8) -> Result<PageRangeInfo, Error> {
        #[cfg(windows)]
        windows_imp::virtual_query(ptr).map(|x| PageRangeInfo::from_windows(&x))
    }

    #[inline(always)]
    pub fn page_range_info_iter(&self, ptr: *mut u8) -> PageRangeInfoIterator {
        PageRangeInfoIterator { virtual_memory_system: self, ptr, finished: false }
    }
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
#[repr(u32)]
pub enum PageState {
    Committed = 0x1000, // MEM_COMMIT
    Free = 0x10000, // MEM_FREE
    Reserved = 0x2000, // MEM_RESERVE
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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PageType {
    Image = 0x1000000, // MEM_IMAGE
    Mapped = 0x40000, // MEM_MAPPED
    Private = 0x20000, // MEM_PRIVATE
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

#[cfg(test)]
mod tests {

    // TODO: is it fine to reserve with no protection?
    // TODO: does committing change the protection of existing pages?

    #[test]
    fn it_works() {
    }
}
