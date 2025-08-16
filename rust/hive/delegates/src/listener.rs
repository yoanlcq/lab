use core::sync::atomic::AtomicU64;
use core::marker::PhantomData;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UntypedListenerHandle {
    pub(crate) initial_index_hint: usize,
    pub(crate) uid: u64,
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
    #[must_use]
    pub fn generate_new(initial_index_hint: usize) -> Self {
        Self {
            initial_index_hint,
            uid: LAST_LISTENER_UID.fetch_add(1, core::sync::atomic::Ordering::Relaxed),
        }
    }
    #[must_use]
    pub(crate) const fn from_parts(initial_index_hint: usize, uid: u64) -> Self {
        Self { initial_index_hint, uid }
    }
}

#[expect(clippy::module_name_repetitions, reason = "We don't want it to be named just `Handle`")]
pub struct ListenerHandle<T> {
    untyped: UntypedListenerHandle,
    phantom: PhantomData<T>,
}

impl<T> ListenerHandle<T> {
    #[must_use]
    pub fn generate_new(initial_index_hint: usize) -> Self {
        Self {
            untyped: UntypedListenerHandle::generate_new(initial_index_hint),
            phantom: PhantomData,
        }
    }
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
/*
FIXME
impl<T> ListenerHandle<T> {
    #[expect(dead_code, reason = "This is a static assert")]
    const fn must_be_send_and_sync() {
        const fn f<T: Send + Sync>() {}
        f::<Self>()
    }
}
    */
