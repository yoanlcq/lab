//! Contexts are the main entry point and the majestro of Nuklear and contain all required state.
//! They are used for window, memory, input, style, stack, commands and time management and need
//! to be passed into all nuklear GUI specific functions.
//!
//! # Usage
//! Create a `Context` using one of the `from_*()` associated functions.
//! Each takes in a font handle and a specific way of handling memory. Memory control
//! hereby ranges from Rust's current allocator to just specifing a fixed sized block of memory
//! which Nuklear has to manage itself from.
//!
//! ```rust,no_run
//! extern crate nuke as nk;
//! let nk = nk::Context::new(...);
//! loop {
//!     // Give input, send GUI, draw...
//!     nk.clear();
//! }
//! ```

extern crate nuke_sys;
use nuke_sys::*;

use libc::c_void;
use core::mem::*;
use core::marker::PhantomData;

use font::*;
use allocator;
use allocator::*;
use buffer::Buffer;

/// Main entry point for Nuke's API.
///
/// See the module's documentation for more.
pub struct Context<'a> {
    pub(crate) raw: nk_context,
    _booh: PhantomData<&'a mut nk_context>,
}

// TODO: Test if it's possible to misuse the api and trigger segfaults
impl<'a,'b:'a> Context<'a> {
    /// Initializes a Nuke context with a fixed-size memory region for it to manage itself in.
    /// The memory region is required to outlive the `Context`.
    pub fn from_fixed_mem(mem: &'b mut [u8], font: &'b VirtualFont) -> Self {
        let mut raw: nk_context = unsafe { uninitialized() };
        unsafe {
            // Can't return 0 in this case, see source
            nk_init_fixed(&mut raw, mem.as_mut_ptr() as *mut c_void, mem.len(), &font.to_nk());
        }
        Self {
            raw,
            _booh: PhantomData,
        }
    }
    /// Initializes a Nuke context, which is to pull memory from the given allocator.
    /// `allocator` is an exclusive reference to a mutable `Allocator` trait object, and 
    /// is required to outlive the `Context` because, internally, `nk_init()` needs an
    /// `nk_allocator` which `userdata` is a pointer which, if used by the allocator's callbacks, 
    /// must not end up dangling.
    ///
    /// # Example
    /// ```
    /// extern crate nuke as nk;
    /// # let font: UserFont = core::mem::uninitialized();
    /// let mut allocator = nk::VecAllocator::new();
    /// let mut trait_obj = &mut allocator as &mut Allocator;
    /// let nk = nk::Context::from_allocator(&mut trait_obj, &font);
    /// ```
    pub fn from_allocator(allocator: &'b mut &mut Allocator, font: &'b VirtualFont) -> Self
    {
        let mut raw: nk_context = unsafe { uninitialized() };
        let mut a = allocator::nk_allocator_from(allocator);
        unsafe {
            // Can't return 0 in this case, see source
            nk_init(&mut raw, &mut a, &font.to_nk());
        }
        Self {
            raw,
            _booh: PhantomData,
        }
    }
    
    /// Initializes a Nuke context, which is to pull memory from the given "bare" allocator.
    /// A limitation of `from_allocator` is that the `Allocator` trait object is required to
    /// outlive the Context it is given to, because internally Nuklear stores its address
    /// and we must guarantee its validity. This is not hard to achieve but slightly incovenient.
    ///
    /// However it's pointless when your allocator is a "bare" wrapper around simple
    /// allocation routines which manage their state "outside" (like `libc`'s `malloc()`),
    /// because it is assumed that said state lives as long as the program.
    /// Such allocators are named `BareAllocator`s and have no lifetime constraints.
    pub fn from_bare_allocator(allocator: BareAllocator, font: &'a VirtualFont) -> Self {
        let mut raw: nk_context = unsafe { uninitialized() };
        let mut a = allocator.to_nk();
        unsafe {
            // Can't return 0 in this case, see source
            nk_init(&mut raw, &mut a, &font.to_nk());
        }
        Self {
            raw,
            _booh: PhantomData,
        }
    }
    /// Initializes a Nuke context, which is to use `libc`'s `malloc()` and `free()` for
    /// allocating and deallocating memory.
    pub fn from_malloc_allocator(font: &'b VirtualFont) -> Self {
        Self::from_bare_allocator(MALLOC_ALLOCATOR, font)
    }
    /// Initializes a Nuke context from two fixed-or-dynamic buffers: 
    /// `cmds` is used as the buffer to store draw commands into.
    /// `pool` is used as the buffer to store GUI state into.
    pub fn from_buffers(cmds: &'b Buffer, pool: &'b Buffer, font: &'b VirtualFont) -> Self {
        let mut raw: nk_context = unsafe { uninitialized() };
        unsafe {
            // Can't return 0 in this case, see source
            nk_init_custom(&mut raw, &mut cmds.to_nk(), &mut pool.to_nk(), &font.to_nk());
        }
        Self {
            raw,
            _booh: PhantomData,
        }
    }
    /*
    // REMOVED: replaced by the `ClearStep` object.
    /// Call at the end of the frame to reset and prepare the context for the next frame
    pub fn clear(&mut self) {
        unsafe {
            nk_clear(&mut self.raw);
        }
    }
    // I removed this set of methods because they solve a problem that exists
    // only in C.
    // Might be needed for custom widgets, but I have doubts.
    // `Userdata` used to be a generic type parameter for `Context`.
    
    /// Sets the currently passed userdata passed down into each draw command
    pub fn set_user_data(&mut self, userdata: &'a mut Userdata) {
        unsafe {
            nk_set_user_data(&mut self.raw, nk_handle::from(userdata as *mut _));
        }
    }
    /// Gets the currently passed userdata passed down into each draw command
    pub fn user_data(&self) -> Option<&'a mut Userdata> {
        unsafe {
            self.raw.userdata.into().as_mut()
        }
    }
    */
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) {
        unsafe {
            nk_free(&mut self.raw)
        }
    }
}
