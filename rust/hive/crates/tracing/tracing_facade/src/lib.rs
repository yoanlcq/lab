//! This crate exists to loosen our relationship with any particular tracing service.
//! 
//! All code in this workspace should use+extend this instead of relying on any concrete tracing service.

use core::alloc::GlobalAlloc;
use core::alloc::Layout;

pub struct TracingManager {
    tracy_client: tracy_client::Client,
}

impl TracingManager {
    #[must_use]
    pub fn start() -> Self {
        Self {
            tracy_client: tracy_client::Client::start(),
        }
    }
    pub fn start_first_frame(&self) {
        self.tracy_client.color_message("start_first_frame", 0xff0000, 0);
    }
    pub fn end_frame(&self) {
        self.tracy_client.frame_mark();
    }
}

pub struct Span {
    tracy_span: tracy_client::Span,
}

impl Span {
    #[must_use]
    pub const fn from_tracy(tracy_span: tracy_client::Span) -> Self {
        Self { tracy_span }
    }
    // The color is specified as RGB. It is most straightforward to specify them as hex literals such as 0xFF0000 for red, 0x00FF00 for green or 0x0000FF for blue.
    // The value 0 is reserved as meaning "no color specified". This is true for this crate, but this is also how Tracy handles it.
    #[must_use]
    pub fn with_color(mut self, color: u32) -> Self {
        self.set_color(color);
        self
    }
    /// See `with_color()`
    pub fn set_color(&mut self, color: u32) {
        self.tracy_span.emit_color(color);
    }
    #[must_use]
    pub fn with_function_call_string< F>(mut self, f: F) -> Self where F: FnMut() -> String {
        self.set_function_call_string(f);
        self
    }
    pub fn set_function_call_string<F>(&mut self, mut f: F) where F: FnMut() -> String {
        self.tracy_span.emit_text(&f());
    }
}

#[doc(hidden)]
pub mod private_reexports {
    pub use tracy_client;
}

#[macro_export]
macro_rules! span {
    ($name: expr) => {
        $crate::Span::from_tracy($crate::private_reexports::tracy_client::span!($name))
    };
}

pub struct GlobalAllocatorWrapper<A: GlobalAlloc> {
    tracy: tracy_client::ProfiledAllocator<A>,
}

impl<A: GlobalAlloc> GlobalAllocatorWrapper<A> {
    #[must_use]
    pub const fn new(inner_allocator: A, callstack_depth: u16) -> Self {
        Self {
            tracy: tracy_client::ProfiledAllocator::new(inner_allocator, callstack_depth)
        }
    }
}

#[expect(clippy::undocumented_unsafe_blocks, reason = "We are just a thin wrapper")]
#[expect(unsafe_code, reason = "This is to be expected for an allocator")]
unsafe impl<A: GlobalAlloc> GlobalAlloc for GlobalAllocatorWrapper<A> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { self.tracy.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { self.tracy.dealloc(ptr, layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { self.tracy.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe { self.tracy.realloc(ptr, layout, new_size) }
    }
}