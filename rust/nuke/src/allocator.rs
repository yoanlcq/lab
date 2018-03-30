//! Rust-y mapping to `nk_allocator` structs.

use libc::{self, c_void};
use core::ptr::*;
use nuke_sys::*;

/// A pair of alloc-dealloc callbacks, along with custom data.
pub trait Allocator {
    /// The callback for allocating a new memory region.
    /// TODO I have no idea what the `old` parameter means,
    /// but it can be safely ignored.
    fn alloc(&mut self, old: *mut u8, size: usize) -> *mut u8;
    /// The callback for deallocating a memory region.
    fn dealloc(&mut self, old: *mut u8);
}

extern fn alloc_with_traitobj(userdata: nk_handle, old: *mut c_void, size: nk_size) -> *mut c_void {
    let a: *mut &mut Allocator = userdata.into();
    unsafe {
        let a = a.as_mut().unwrap();
        a.alloc(old as *mut u8, size) as *mut c_void
    }
}
extern fn free_with_traitobj(userdata: nk_handle, old: *mut c_void) {
    let a: *mut &mut Allocator = userdata.into();
    unsafe {
        let a = a.as_mut().unwrap();
        a.dealloc(old as *mut u8)
    }
}

pub(crate) fn nk_allocator_from(a: &mut &mut Allocator) -> nk_allocator {
    nk_allocator {
        userdata: nk_handle::from(a as *mut _),
        alloc: Some(alloc_with_traitobj),
        free: Some(free_with_traitobj),
    }
}

/// Callback for allocating memory. Ignore the first parameter.
pub type BareAlloc = extern fn(userdata: nk_handle, old: *mut c_void, size: nk_size) -> *mut c_void;
/// Callback for allocating memory. Ignore the first parameter.
pub type BareDealloc = extern fn(userdata: nk_handle, old: *mut c_void);
/// An allocator that doesn't contain its own state, and therefore is allowed
/// to be outlived by the Context.
#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub struct BareAllocator {
    /// The callback for allocating memory. Ignore the first parameter.
    pub alloc: BareAlloc,
    /// The callback for deallocating memory. Ignore the first parameter.
    pub dealloc: BareDealloc,
}

impl BareAllocator {
    /// Converts this to an `nk_allocator`. This is only intended for internal use,
    /// but can't be private.
    pub(crate) fn to_nk(self) -> nk_allocator {
        nk_allocator {
            userdata: nk_handle::from(null_mut::<()>()),
            alloc: Some(self.alloc),
            free: Some(self.dealloc),
        }
    }
}

#[cfg(feature="use_libc")]
pub use malloc_allocator::*;
/// Implementation of the bare allocator based on `libc`'s `malloc()` and `free()`.
/// Not available on `no_std`.
#[cfg(feature="use_libc")]
pub mod malloc_allocator {
    use super::*;
    extern fn alloc_with_malloc(_: nk_handle, _: *mut c_void, size: nk_size) -> *mut c_void {
        unsafe { libc::malloc(size) }
    }
    extern fn free_with_free(_: nk_handle, old: *mut c_void) {
        unsafe { libc::free(old) }
    }
    /// The malloc()-based bare allocator.
    pub const MALLOC_ALLOCATOR : BareAllocator = BareAllocator {
        alloc: alloc_with_malloc,
        dealloc: free_with_free,
    };
}

#[cfg(feature="use_std")]
pub use vec_allocator::*;
/// Implementation of the Vec-based allocator.
/// Not available on `no_std`.
#[cfg(feature="use_std")]
pub mod vec_allocator {
    use super::Allocator;
    /// The Vec-based allocator for Nuke.
    #[derive(Debug, Default, Clone, Hash, PartialEq)]
    pub struct VecAllocator {
        /// The vector of allocated memory regions.
        /// TODO use a better data structure ?
        v: Vec<Vec<u8>>,
    }
    impl Allocator for VecAllocator {
        fn alloc(&mut self, _: *mut u8, size: usize) -> *mut u8 {
            let mut new = Vec::with_capacity(size);
            let ptr = new.as_mut_slice().as_mut_ptr();
            self.v.push(new);
            ptr
        }
        fn dealloc(&mut self, old: *mut u8) {
            self.v.iter()
                .position(|v| v.as_slice().as_ptr() == old)
                .map(|e| self.v.remove(e))
                .unwrap();
        }
    }
    impl VecAllocator {
        /// Creates a new `VecAllocator`.
        pub fn new() -> Self {
            Self::default()
        }
    }
}
