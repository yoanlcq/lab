//! This crate exists to loosen our relationship with any particular tracing service.
//! 
//! All code in this workspace should use+extend this instead of relying on any concrete tracing service.

use core::alloc::GlobalAlloc;
use core::alloc::Layout;

TODO;
// TODO: le vrai système que je veux:
// - Un programme lancable n'importe quand
// - Il pourrait visiter tous les fichiers source du workspace et faire des vérifications
//   - copyright header
//   - loctexts
//   - header de confidentialité (vérifier si c'est safe à open-sourcer)
//   - forbidden patterns?
// - Il utilise "cargo metadata" pour lister toutes les dépendances de la crate ciblée
//   - Il génère une crate automatique qui doit être ignorée par le VCS
//     - Génère le Cargo.toml d'après un template en passant chaque dépendance
//     - Génère chaque fichier src d'après un template
//   - Il faut que la crate ciblée ajoute elle-même cette dépendance vers la crate automatique et s'en serve

#[linkme::distributed_slice(tunables::TUNABLES)]
static ENABLE_TRACY: tunables::Tunable<'static> = tunables::Tunable {
    name: "tracy",
    help: "Enables Tracy",
    default: tunables::TunableValue::Bool(true),
};

pub struct TracingManager {
    #[cfg(feature = "tracy")]
    tracy_client: tracy_client::Client,
}

#[cfg_attr(not(feature = "tracy"), expect(clippy::missing_const_for_fn, unused_variables, unused_mut, reason = "Normal"))]
impl TracingManager {
    #[must_use]
    pub fn start() -> Self {
        Self {
            #[cfg(feature = "tracy")]
            tracy_client: tracy_client::Client::start(),
        }
    }
    pub fn start_first_frame(&self) {
        #[cfg(feature = "tracy")]
        self.tracy_client.color_message("Starting first frame", 0xff0000, 0);
    }
    pub fn end_frame(&self) {
        #[cfg(feature = "tracy")]
        self.tracy_client.frame_mark();
    }
    // TODO: Consider exposing an API based on `core::fmt::Formatter` instead. Users would provide a closure in which they use `write!()`.
    pub fn log_dynamic_message<F>(&self, mut get_message: F, color: u32, callstack_depth: u16) where F: FnMut() -> String {
        #[cfg(feature = "tracy")]
        self.tracy_client.color_message(&get_message(), color, callstack_depth);
    }
    pub fn log_static_message(&self, message: &'static str, color: u32, callstack_depth: u16) {
        #[cfg(feature = "tracy")]
        self.tracy_client.color_message(message, color, callstack_depth);
    }
}

pub struct Span {
    #[cfg(feature = "tracy")]
    tracy_span: tracy_client::Span,
}

#[cfg_attr(not(feature = "tracy"), expect(clippy::missing_const_for_fn, unused_variables, unused_mut, reason = "Normal"))]
impl Span {
    #[cfg(feature = "tracy")]
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
        #[cfg(feature = "tracy")]
        self.tracy_span.emit_color(color);
    }
    #[must_use]
    pub fn with_function_call_string<F>(mut self, f: F) -> Self where F: FnMut() -> String {
        self.set_function_call_string(f);
        self
    }
    pub fn set_function_call_string<F>(&mut self, mut f: F) where F: FnMut() -> String {
        #[cfg(feature = "tracy")]
        self.tracy_span.emit_text(&f());
    }
}

#[doc(hidden)]
pub mod private_reexports {
    #[cfg(feature = "tracy")]
    pub use tracy_client;
}

#[macro_export]
#[cfg(feature = "tracy")]
macro_rules! span {
    ($name: expr) => {
        $crate::Span::from_tracy($crate::private_reexports::tracy_client::span!($name))
    };
}

#[macro_export]
#[cfg(not(feature = "tracy"))]
macro_rules! span {
    ($name: expr) => {
        $crate::Span {}
    };
}

pub struct GlobalAllocatorWrapper<A: GlobalAlloc> {
    #[cfg(feature = "tracy")]
    tracy: tracy_client::ProfiledAllocator<A>,
    #[cfg(not(feature = "tracy"))]
    inner: A,
}

impl<A: GlobalAlloc> GlobalAllocatorWrapper<A> {
    #[must_use]
    pub const fn new(inner_allocator: A, callstack_depth: u16) -> Self {
        Self {
            #[cfg(feature = "tracy")]
            tracy: tracy_client::ProfiledAllocator::new(inner_allocator, callstack_depth),
            #[cfg(not(feature = "tracy"))]
            inner: {
                _ = callstack_depth;
                inner_allocator
            },
        }
    }
}

#[expect(clippy::undocumented_unsafe_blocks, reason = "We are just a thin wrapper")]
#[expect(unsafe_code, reason = "This is to be expected for an allocator")]
unsafe impl<A: GlobalAlloc> GlobalAlloc for GlobalAllocatorWrapper<A> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        #[cfg(feature = "tracy")]
        unsafe { self.tracy.alloc(layout) }
        #[cfg(not(feature = "tracy"))]
        unsafe { self.inner.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        #[cfg(feature = "tracy")]
        unsafe { self.tracy.dealloc(ptr, layout) }
        #[cfg(not(feature = "tracy"))]
        unsafe { self.inner.dealloc(ptr, layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        #[cfg(feature = "tracy")]
        unsafe { self.tracy.alloc_zeroed(layout) }
        #[cfg(not(feature = "tracy"))]
        unsafe { self.inner.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        #[cfg(feature = "tracy")]
        unsafe { self.tracy.realloc(ptr, layout, new_size) }
        #[cfg(not(feature = "tracy"))]
        unsafe { self.inner.realloc(ptr, layout, new_size) }
    }
}