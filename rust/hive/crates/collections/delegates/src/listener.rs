//! Utilities related to listeners (or "subscribers")

use core::sync::atomic::AtomicU64;
use core::marker::PhantomData;

/// Indicates whether or not a listener should be removed.
/// 
/// This exists to be clearer than just a `bool`, especially since it is the expected return value for multicast delegate listeners.
#[expect(clippy::module_name_repetitions, reason = "We don't want it to be named just `Presence`")]
#[must_use = "The delegate uses this to know if the listener hasn't expired"]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ListenerReply {
    /// Keep the listener
    Keep,
    /// Remove the listener. This is useful when you know that the target object has expired and that there's no point in this listener being called again.
    Remove,
}

/// A lightweight opaque handle that is a unique identifier for a listener (or "subscriber" if you will)
/// 
/// The uniqueness of the handle is limited to the current process, so it is not meant to be used outside of it.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UntypedListenerHandle {
    /// We can't solely rely on this index to identify a listener, because that listener's position may change as elements are removed from the container.
    /// We could almost remove it to spare some space.
    /// It brings the most value when listeners are only ever removed from the end of the container: i.e "push(a) -> push(b) -> remove(a) -> remove(a)", which I expect (but cannot verify) that it is the common case.
    /// 
    /// NOTE: This is public only out of necessity for macro expansion in other crates, but is not part of the public API. Please do not use it.
    pub initial_index_hint: usize,

    /// NOTE: This is public only out of necessity for macro expansion in other crates, but is not part of the public API. Please do not use it.
    pub uid: u64,
}

impl UntypedListenerHandle {
    #[expect(dead_code, reason = "This is a static assert")]
    const fn must_be_send_and_sync() {
        const fn f<T: Send + Sync>() {}
        f::<Self>();
    }
}

static LAST_LISTENER_UID: AtomicU64 = AtomicU64::new(0);

impl UntypedListenerHandle {
    /// NOTE: This is public only out of necessity for macro expansion in other crates, but is not part of the public API. Please do not use it.
    #[must_use]
    pub fn generate_new(initial_index_hint: usize) -> Self {
        Self {
            initial_index_hint,
            uid: LAST_LISTENER_UID.fetch_add(1, core::sync::atomic::Ordering::Relaxed),
        }
    }
    /// NOTE: This is public only out of necessity for macro expansion in other crates, but is not part of the public API. Please do not use it.
    #[must_use]
    pub const fn from_parts(initial_index_hint: usize, uid: u64) -> Self {
        Self { initial_index_hint, uid }
    }
}

/// A thin wrapper around `UntypedListenerHandle` that exists simply to increase type-safety
#[expect(clippy::module_name_repetitions, reason = "We don't want it to be named just `Handle`")]
pub struct ListenerHandle<T> {
    untyped: UntypedListenerHandle,
    phantom: PhantomData<fn() -> T>,
}

impl<T> From<UntypedListenerHandle> for ListenerHandle<T> {
    fn from(value: UntypedListenerHandle) -> Self {
        Self { untyped: value, phantom: PhantomData }
    }
}

impl<T> From<ListenerHandle<T>> for UntypedListenerHandle {
    fn from(value: ListenerHandle<T>) -> Self {
        value.untyped
    }
}

impl<T> ListenerHandle<T> {
    #[must_use]
    pub const fn untyped(&self) -> UntypedListenerHandle {
        self.untyped
    }
}

impl<T> core::fmt::Debug for ListenerHandle<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ListenerHandle").field("untyped", &self.untyped).finish()
    }
}

impl<T> Clone for ListenerHandle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ListenerHandle<T> {}

impl<T> core::hash::Hash for ListenerHandle<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.untyped.hash(state);
    }
}

impl<T> PartialEq for ListenerHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.untyped.eq(&other.untyped)
    }
}

impl<T> Eq for ListenerHandle<T> {}

impl<T> PartialOrd for ListenerHandle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for ListenerHandle<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.untyped.cmp(&other.untyped)
    }
}

impl<T> ListenerHandle<T> {
    #[expect(dead_code, reason = "This is a static assert")]
    const fn must_be_send_and_sync() {
        const fn f<T: Send + Sync>() {}
        f::<Self>();
    }
}
