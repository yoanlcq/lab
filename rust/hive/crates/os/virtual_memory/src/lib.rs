//! Tiny wrapper around the current platform's virtual memory system.
//! 
//! Please refer to `examples/general_functionality.rs` for an introduction to the API.

#![expect(unsafe_code, reason = "This is normal for a memory management API")]
#![cfg_attr(windows, allow(clippy::wildcard_imports, reason = "Using the windows crate without it is a pain"))]

// TODO: expose MEM_WRITE_WATCH and similar APIs?

use core::num::NonZeroUsize;
use bitflags::bitflags;

#[cfg(windows)]
use windows::Win32::System::{
    SystemInformation::*,
    Memory::*,
};

pub use std::io::Error as Error;

#[cfg(windows)]
mod imp_windows;

type Result<T> = core::result::Result<T, Error>;


// This is provided in order to be a bit nicer to read than `usize` when we are dealing with addresses, and to avoid confusing them with actual allocation sizes.
//
// This is not `NonNull<u8>`, because `NonNull` assumes that the memory be aliased, which is why isn't `Send` nor `Sync`.
// However, a virtual address does not necessarily index into committed memory. As long as it is not committed, it should be `Send` and `Sync`, hence this type exists.
//
// This is also not `NonZeroUsize` because technically you could manipulate the very first page in the range, which address is zero.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Addr(usize);

static_assertions::assert_impl_all!(Addr: Send, Sync);

impl Addr {
    #[must_use]
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }
    #[must_use]
    pub const fn get(&self) -> usize {
        self.0
    }
}

// In the APIs provided by this crate, virtual address ranges are **ALWAYS** treated as having their bounds implicitly extended in both directions until they are aligned to a page boundary.
// For instance, given a page size of 4096, a range defined by `start = 4095` and `size = 2` will be treated as if it were the `0..8192` range.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AddrRange {
    addr: Addr,
    size: usize,
}

static_assertions::assert_impl_all!(AddrRange: Send, Sync);

impl AddrRange {
    #[must_use]
    pub const fn new(addr: Addr, size: usize) -> Self {
        Self { addr, size }
    }
    #[must_use]
    pub fn covering_page_size(self, page_size: NonZeroUsize) -> Self {
        let start = (self.addr.get() / page_size) * page_size.get();
        let end = (self.addr.get() + self.size).next_multiple_of(page_size.get());
        Self::new(Addr::new(start), end - start)
    }
    #[must_use] pub const fn addr(&self) -> Addr {
        self.addr
    }
    #[must_use] pub const fn size(&self) -> usize {
        self.size
    }
    #[must_use] pub const fn to_usize_range(self) -> core::ops::Range<usize> {
        self.addr.get() .. self.addr.get() + self.size
    }
    #[must_use] pub const fn start(&self) -> usize {
        self.addr.get()
    }
    #[must_use] pub const fn end(&self) -> usize {
        self.addr.get() + self.size
    }
    #[must_use] pub const fn contains(&self, other: Self) -> bool {
        self.start() <= other.start() && other.end() <= self.end()
    }
}

// The chosen representation for a range of committed virtual memory.
// This is not a slice because there's no telling what the memory's protection flags were set to.
// Unlike `AddrRange`, we now lose the `Send` + `Sync` capabilities.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PtrRange {
    ptr: *mut u8,
    size: usize,
}

impl PtrRange {
    pub const fn new(ptr: *mut u8, size: usize) -> Self {
        Self { ptr, size }
    }
    #[must_use]
    pub fn covering_page_size(self, page_size: NonZeroUsize) -> Self {
        let addr_range = self.to_addr_range().covering_page_size(page_size);
        Self::new(self.ptr.with_addr(addr_range.addr().get()), addr_range.size())
    }
    #[must_use] pub const fn ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[must_use] pub const fn size(&self) -> usize {
        self.size
    }
    #[must_use] pub fn to_addr_range(self) -> AddrRange {
        AddrRange::new(Addr::new(self.ptr.addr()), self.size)
    }
}

/// This represents a "handle" to the OS's virtual memory API.
///
/// In practice (e.g on Windows or Unix) there is generally no such thing, however what is interesting and maybe "expensive" to get is system information such as page size.
/// So you could choose to instantiate as many `VirtualMemorySystem`s as you like if you're fine with that, or to instantiate only one. Your call, depending on what you know or prefer.
#[derive(Debug, Clone)]
pub struct VirtualMemorySystem {
    page_size: NonZeroUsize,
    allocation_granularity: NonZeroUsize,
}

static_assertions::assert_impl_all!(VirtualMemorySystem: Send, Sync);

impl Drop for VirtualMemorySystem {
    fn drop(&mut self) {
        // Nothing, just implement Drop to stay compatible with user code in case there is a need for it one day
    }
}

impl Default for VirtualMemorySystem {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualMemorySystem {
    // How "expensive" this is depends on the OS, but it's generally cheap and you typically don't need to do this many times.
    #[cfg_attr(windows, expect(clippy::missing_panics_doc, reason = "Panics are impossible in this case"))]
    #[must_use] pub fn new() -> Self {
        #[cfg(windows)]
        {
            let SYSTEM_INFO { dwPageSize, dwAllocationGranularity, .. } = imp_windows::get_system_info();
            #[expect(clippy::unwrap_used, reason = "These cannot fail")]
            Self {
                page_size: NonZeroUsize::new(dwPageSize as _).unwrap(),
                allocation_granularity: NonZeroUsize::new(dwAllocationGranularity as _).unwrap(),
            }
        }
    }

    #[must_use] pub const fn page_size(&self) -> NonZeroUsize {
        self.page_size
    }

    // The granularity for the starting address at which virtual memory can be allocated
    #[must_use] pub const fn allocation_granularity(&self) -> NonZeroUsize {
        self.allocation_granularity
    }

    /// Attempts to reserve a virtual address range.
    ///
    /// Despite the usual meaning we attribute to the `reserve` verb, This does **not** actually allocate any memory, this only ensures the virtual address range is "yours" to use:
    /// that is, if some other code (such as the global allocator) wants to reserve a virtual address range in the current process,
    /// they will have to call the same underlying API, and it will not return a range that intersects any of the currently reserved ones.
    ///
    /// `starting_address_hint` may be `None` to let the OS decide the starting address automatically.
    /// If you specify `Some(0)`, the behavior will depend on how the OS's API chooses to interpret it; but there's no reason to do that, since `Some(1)` will represent the same page but not be treated as a special case.
    /// However you may still encounter an issue because 0 is sometimes used as a failure status; for instance Windows's `VirtualAlloc()` cannot return `null` because that is how it indicates failure.
    /// 
    /// This can only reserve whole pages, therefore the range defined by the provided parameters is implicitly "extended" in both directions until its bounds are aligned to a page boundary.
    /// For instance, if `page_size() == 4096`, `starting_address_hint == 4095` and `size == 2`, then assuming the call succeeds, the reserved range will be `0 .. 8192`.
    pub fn reserve(&self, starting_address_hint: Option<Addr>, size: usize) -> Result<AddrRange> {
        #[cfg(windows)]
        {
            let non_null = imp_windows::virtual_alloc(starting_address_hint.map_or(0, |x| x.get()), size, MEM_RESERVE, PAGE_NOACCESS)?;
            Ok(AddrRange::new(Addr::new(non_null.addr().get()), size).covering_page_size(self.page_size))
        }
    }

    /// NOTE: It is unspecified whether `protection_flags` will be applied to pages that were already committed.
    /// On Windows, `protection_flags` will be applied to the entire passed range.
    /// 
    /// When a page transitions from "reserved" to "committed", its memory is zeroed. Note that this does not happen when committing an already-committed page.
    /// 
    /// This can only operate on whole pages, therefore the range defined by the provided parameters is implicitly "extended" in both directions until its bounds are aligned to a page boundary.
    /// 
    /// This returns a `PtrRange` corresponding to `addr_range.covering_page_size(page_size)`.
    pub fn commit(&self, addr_range: AddrRange, protection_flags: ProtectionFlags) -> Result<PtrRange> {
        #[cfg(windows)]
        {
            let non_null = imp_windows::virtual_alloc(addr_range.addr.get(), addr_range.size, MEM_COMMIT, PAGE_PROTECTION_FLAGS(protection_flags.to_windows().0))?;
            let aligned_addr_range = addr_range.covering_page_size(self.page_size);
            Ok(PtrRange::new(non_null.as_ptr().with_addr(aligned_addr_range.addr.get()), aligned_addr_range.size))
        }
    }

    /// This can only operate on whole pages, therefore the range defined by the provided parameters is implicitly "extended" in both directions until its bounds are aligned to a page boundary.
    /// 
    /// # Safety
    /// 
    /// You must make sure that nobody else is currently using that memory.
    pub unsafe fn decommit(&self, ptr_range: PtrRange) -> Result<()> {
        #[cfg(windows)]
        // SAFETY: We have to assume that the caller took proper precautions as described in this function's doc comment
        unsafe { imp_windows::virtual_free(ptr_range.ptr as _, ptr_range.size, MEM_DECOMMIT) }
    }

    /// This can only operate on whole pages, therefore the range defined by the provided parameters is implicitly "extended" in both directions until its bounds are aligned to a page boundary.
    /// 
    /// # Safety
    /// 
    /// - You MUST pass the ENTIRE range previously returned by a call to `reserve` (i.e: the returned pointer, and the requested size).
    /// - You must make sure that nobody else is currently using that memory.
    ///
    /// On Windows, this will implicitly de-commit the pages for you before unreserving.
    pub unsafe fn unreserve(&self, addr_range: AddrRange) -> Result<()> {
        // SAFETY: We have to assume that the caller took proper precautions as described in this function's doc comment
        #[cfg(windows)]
        unsafe { imp_windows::virtual_free(addr_range.addr.get(), 0 /* Must pass 0 */, MEM_RELEASE) }
    }

    /// If succeeds, returns the previous protection flags of the FIRST page that intersects the specified range
    /// 
    /// # Safety
    /// 
    /// You must make sure the new protection flags will not cause issues with existing allocations
    pub unsafe fn set_protection_flags(&self, addr_range: AddrRange, flags: ProtectionFlags) -> Result<OsProtectionFlags> {
        #[cfg(windows)]
        imp_windows::virtual_protect(addr_range.addr.get(), addr_range.size, PAGE_PROTECTION_FLAGS(flags.to_windows().0)).map(|x| OsProtectionFlags(x.0))
    }

    pub fn bind_to_physical_memory(&self, addr_range: AddrRange) -> Result<()> {
        #[cfg(windows)]
        imp_windows::virtual_lock(addr_range.addr.get(), addr_range.size)
    }

    pub fn unbind_from_physical_memory(&self, addr_range: AddrRange) -> Result<()> {
        #[cfg(windows)]
        imp_windows::virtual_unlock(addr_range.addr.get(), addr_range.size)
    }

    pub fn page_range_info(&self, addr: Addr) -> Result<PageRangeInfo> {
        #[cfg(windows)]
        imp_windows::virtual_query(addr.get()).map(|x| PageRangeInfo::from_windows(&x))
    }

    #[must_use] pub const fn page_range_info_iter(&self, addr: Addr) -> PageRangeInfoIterator {
        PageRangeInfoIterator { virtual_memory_system: self, addr, finished: false }
    }
}

type OsProtectionFlagsPrimitiveUint = u32;

/// This is a wrapper type in order to explicitly not implement `Ord`, because the values vary per platform.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)] // Do NOT derive Ord, because this type's values will change depending on the OS. If users want to sort, they can write their own adapter or wrapper type.
pub struct OsProtectionFlags(pub OsProtectionFlagsPrimitiveUint);

bitflags! {
    // NOTE: If you change **any** of these values, you must update the `to_windows()` function!
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct ProtectionFlags: OsProtectionFlagsPrimitiveUint {
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
    fn to_windows_uint(self) -> OsProtectionFlagsPrimitiveUint {
        let modifiers = (self & Self::MODIFIERS_MASK).bits();
        let mut access = (self & Self::ACCESS_MASK).bits();
        if access == 0 {
            return modifiers | 1; // PAGE_NOACCESS
        }
        if access & Self::WRITE.bits() != 0 {
            access &= !Self::READ.bits();
        }
        if access & Self::EXECUTE.bits() != 0 {
            access &= !Self::EXECUTE.bits();
            access <<= 4;
        }
        modifiers | (access << 1)
    }
    #[cfg(windows)] // This isn't strictly required but I do this to remove noise for other platforms
    const fn from_windows_uint(mut flags: OsProtectionFlagsPrimitiveUint) -> Self {
        let modifiers = flags & Self::MODIFIERS_MASK.bits();

        let has_execute = flags & 0xf0 != 0;
        if has_execute {
            flags >>= 4;
        }
        flags >>= 1; // PAGE_NOACCESS
        flags &= 3;

        if flags & Self::WRITE.bits() != 0 {
            flags |= Self::READ.bits();
        }

        Self::from_bits_retain(modifiers | flags | if has_execute { Self::EXECUTE.bits() } else { 0 })
    }
    #[cfg(windows)] // This isn't strictly required but I do this to remove noise for other platforms
    #[must_use] pub fn to_windows(self) -> OsProtectionFlags {
        OsProtectionFlags(self.to_windows_uint())
    }
    #[cfg(windows)] // This isn't strictly required but I do this to remove noise for other platforms
    #[must_use] pub const fn from_windows(flags: OsProtectionFlags) -> Self {
        Self::from_windows_uint(flags.0)
    }
    #[must_use] pub const fn from_os_lossy(flags: OsProtectionFlags) -> Self {
        #[cfg(windows)]
        Self::from_windows(flags)
    }
    #[must_use] pub fn to_os(self) -> OsProtectionFlags {
        #[cfg(windows)]
        self.to_windows()
    }
}

pub struct PageRangeInfoIterator<'a> {
    virtual_memory_system: &'a VirtualMemorySystem,
    addr: Addr,
    finished: bool,
}

impl Iterator for PageRangeInfoIterator<'_> {
    type Item = Result<PageRangeInfo>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let r = self.virtual_memory_system.page_range_info(self.addr);
        if let Ok(info) = r.as_ref() {
            self.addr = Addr::new(self.addr.get() + info.size);
        } else {
            self.finished = true;
        }
        Some(r)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PageRangeInfo {
    addr: Addr,
    size: usize,
    state: PageState,
    not_free: Option<PageRangeInfoNotFree>,
}

static_assertions::assert_impl_all!(PageRangeInfo: Send, Sync);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct PageRangeInfoNotFree {
    // Undefined if state == Reserved
    protection_flags: Option<OsProtectionFlags>,
    r#type: PageType,
    allocation_addr: Addr,
    // May be 0 if the caller does not have access
    allocation_protection_flags: OsProtectionFlags,
}

impl PageRangeInfo {
    #[cfg(windows)]
    #[expect(clippy::unwrap_used, reason = "These should never fail unless we did something stupid in their implementation")]
    fn from_windows(info: &MEMORY_BASIC_INFORMATION) -> Self {
        let &MEMORY_BASIC_INFORMATION {
            BaseAddress, AllocationBase, AllocationProtect, RegionSize, State, Protect, Type, PartitionId: _
        } = info;

        let not_free = if State == MEM_FREE {
            None
        } else {
            Some(PageRangeInfoNotFree {
                protection_flags: if State == MEM_RESERVE { None } else { Some(OsProtectionFlags(Protect.0)) },
                r#type: PageType::try_from_windows(Type.0).unwrap(),
                allocation_addr: Addr::new(AllocationBase as _),
                allocation_protection_flags: OsProtectionFlags(AllocationProtect.0),
            })
        };

        Self {
            addr: Addr::new(BaseAddress as _),
            size: RegionSize,
            state: PageState::try_from_windows(State.0).unwrap(),
            not_free,
        }
    }
    #[must_use] pub const fn addr(&self) -> Addr {
        self.addr
    }
    #[must_use] pub const fn size(&self) -> usize {
        self.size
    }
    #[must_use] pub const fn state(&self) -> PageState {
        self.state
    }
    /// Will return `None` if `state() != PageState::Committed` or if the OS does not support reporting this.
    #[must_use] pub fn os_protection_flags(&self) -> Option<OsProtectionFlags> {
        self.not_free.map(|x| x.protection_flags)?
    }
    /// Will return `None` if `state() != PageState::Committed` or if the OS does not support reporting this.
    pub fn protection_flags_lossy(&self) -> Option<ProtectionFlags> {
        self.os_protection_flags().map(ProtectionFlags::from_os_lossy)
    }
    /// Will return `None` if `state() == PageState::Free` or if the OS does not support reporting this.
    #[must_use] pub fn r#type(&self) -> Option<PageType> {
        self.not_free.map(|x| x.r#type)
    }
    /// Will return `None` if `state() == PageState::Free` or if the OS does not support reporting this.
    #[must_use] pub fn allocation_addr(&self) -> Option<Addr> {
        self.not_free.map(|x| x.allocation_addr)
    }
    /// Will return `None` if `state() == PageState::Free` or if the OS does not support reporting this.
    /// May be 0 if the caller does not have access.
    #[must_use] pub fn allocation_os_protection_flags(&self) -> Option<OsProtectionFlags> {
        self.not_free.map(|x| x.allocation_protection_flags)
    }
    /// Will return `None` if `state() == PageState::Free` or if the OS does not support reporting this.
    /// May be 0 if the caller does not have access.
    pub fn allocation_protection_flags_lossy(&self) -> Option<ProtectionFlags> {
        self.allocation_os_protection_flags().map(ProtectionFlags::from_os_lossy)
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
    Image = 0x100_0000, // MEM_IMAGE
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
    use super::*;

    #[test]
    #[expect(clippy::unwrap_used, reason = "This is a test and these should never fail")]
    fn committing_already_committed_pages() {
        let vms = VirtualMemorySystem::new();
        let page_size = vms.page_size().get();
        let r = vms.reserve(None, page_size * 2).unwrap();
        let ra = AddrRange::new(r.addr(), page_size);
        let rb = AddrRange::new(r.addr(), page_size * 2);
        _ = vms.commit(ra, ProtectionFlags::READ_WRITE).unwrap();
        let pb = vms.commit(rb, ProtectionFlags::READ_WRITE_EXECUTE).unwrap();
        let info = vms.page_range_info(ra.addr()).unwrap();
        assert_eq!(info.protection_flags_lossy().unwrap(), ProtectionFlags::READ_WRITE_EXECUTE);
        // SAFETY: We know the memory range is valid and nobody is using it
        unsafe {
            vms.decommit(pb).unwrap();
            vms.unreserve(r).unwrap();
        }
    }
}
