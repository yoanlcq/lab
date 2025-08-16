use core::marker::PhantomData;
use core::sync::atomic::AtomicU64;
use core::cell::RefCell;
use parking_lot::{Mutex, ReentrantMutex};

#[must_use = "The delegate uses this to know if the listener hasn't expired"]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MulticastDelegateResult {
    Keep,
    Remove,
}

struct Listener<T> {
    func: Box<dyn FnMut(T) -> MulticastDelegateResult + Send>,
    salt: u64,
}

/*
FIXME
impl<T> Listener<T> {
    #[expect(dead_code, reason = "This is a static assert")]
    const fn must_be_send_and_sync() {
        const fn f<T: Send + Sync>() {}
        f::<Self>()
    }
}
 */

impl<T> core::fmt::Debug for Listener<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Listener").field("salt", &self.salt).finish_non_exhaustive()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UntypedListenerHandle {
    index: usize,
    salt: u64,
}

impl UntypedListenerHandle {
    #[expect(dead_code, reason = "This is a static assert")]
    const fn must_be_send_and_sync() {
        const fn f<T: Send + Sync>() {}
        f::<Self>();
    }
}

static LAST_LISTENER_SALT: AtomicU64 = AtomicU64::new(0);

impl UntypedListenerHandle {
    #[must_use]
    pub fn generate_new(index: usize) -> Self {
        Self {
            index,
            salt: LAST_LISTENER_SALT.fetch_add(1, core::sync::atomic::Ordering::Relaxed),
        }
    }
}

pub struct ListenerHandle<T> {
    untyped: UntypedListenerHandle,
    phantom: PhantomData<T>,
}

impl<T> ListenerHandle<T> {
    #[must_use]
    pub fn generate_new(index: usize) -> Self {
        Self {
            untyped: UntypedListenerHandle::generate_new(index),
            phantom: PhantomData,
        }
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

pub struct MulticastDelegate<T> {
    listeners: Vec<Listener<T>>,
}

impl<T> core::fmt::Debug for MulticastDelegate<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MulticastDelegate").field("listeners", &self.listeners).finish()
    }
}

impl<T> Default for MulticastDelegate<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MulticastDelegate<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self { listeners: vec![] }
    }
    pub fn remove(&mut self, listener_handle: &ListenerHandle<T>) -> Option<Box<dyn FnMut(T) -> MulticastDelegateResult>> {
        {
            let listener = self.listeners.get(listener_handle.untyped.index)?;
            if listener.salt != listener_handle.untyped.salt {
                return None;
            }
        }
        Some(self.listeners.remove(listener_handle.untyped.index).func)
    }
    pub fn push(&mut self, func: Box<dyn FnMut(T) -> MulticastDelegateResult + Send>) -> ListenerHandle<T> {
        let listener_handle = ListenerHandle::generate_new(self.listeners.len());
        self.listeners.push(Listener { func, salt: listener_handle.untyped.salt });
        listener_handle
    }
    pub fn broadcast(&mut self, payload: T)
    where
        T: Copy,
    {
        self.broadcast_cloning(&payload);
    }
    pub fn broadcast_cloning(&mut self, payload: &T)
    where
        T: Clone,
    {
        let mut i = 0;
        while i < self.listeners.len() {
            match (self.listeners[i].func)(payload.clone()) {
                MulticastDelegateResult::Keep => i += 1,
                MulticastDelegateResult::Remove => drop(self.listeners.remove(i)),
            }
        }
    }
}

pub struct SharedMulticastDelegate<T> {
    delegate: ReentrantMutex<RefCell<MulticastDelegate<T>>>,
    #[expect(clippy::type_complexity, reason = "It's fine actually")]
    pending_push: Mutex<Vec<(ListenerHandle<T>, Box<dyn FnMut(T) -> MulticastDelegateResult + Send>)>>,
    pending_removal: Mutex<Vec<ListenerHandle<T>>>,
}

impl<T> core::fmt::Debug for SharedMulticastDelegate<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { ref delegate, ref pending_push, ref pending_removal } = *self;
        f.debug_struct("SharedMulticastDelegate")
            .field("delegate", delegate)
            .field("pending_push", &format!("<{} boxed closures>", pending_push.lock().len()))
            .field("pending_removal", pending_removal)
            .finish()
    }
}

impl<T> Default for SharedMulticastDelegate<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SharedMulticastDelegate<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            delegate: ReentrantMutex::new(RefCell::new(MulticastDelegate::new())),
            pending_push: Mutex::new(vec![]),
            pending_removal: Mutex::new(vec![]),
        }
    }
    fn remove_internal(delegate: &mut MulticastDelegate<T>, listener_handle: &ListenerHandle<T>)  -> Option<Box<dyn FnMut(T) -> MulticastDelegateResult>> {
        if listener_handle.untyped.index == usize::MAX {
            if let Some(index) = delegate.listeners.iter().position(|x| x.salt == listener_handle.untyped.salt) {
                Some(delegate.listeners.remove(index).func)
            } else {
                None
            }
        } else {
            delegate.remove(listener_handle)
        }
    }
    pub fn remove(&self, listener_handle: &ListenerHandle<T>) -> Option<Box<dyn FnMut(T) -> MulticastDelegateResult>> {
        if let Some(mut delegate) = self.delegate.try_lock().as_ref().map(|x| x.try_borrow_mut().ok()).flatten() {
            Self::remove_internal(&mut delegate, listener_handle)
        } else {
            _ = self.pending_removal.lock().push(*listener_handle);
            None
        }
    }
    pub fn push(&self, func: Box<dyn FnMut(T) -> MulticastDelegateResult + Send>) -> ListenerHandle<T> {
        if let Some(mut delegate) = self.delegate.try_lock().as_ref().map(|x| x.try_borrow_mut().ok()).flatten() {
            delegate.push(func)
        } else {
            let listener_handle = ListenerHandle::generate_new(usize::MAX);
            self.pending_push.lock().push((listener_handle, func));
            listener_handle
        }
    }
    pub fn broadcast(&self, payload: T)
    where
        T: Copy,
    {
        self.broadcast_cloning(&payload);
    }
    pub fn broadcast_cloning(&self, payload: &T)
    where
        T: Clone,
    {
        let delegate_lock = self.delegate.lock();
        {
            let mut i = 0;
            let delegate = delegate_lock.borrow();
            while i < delegate.listeners.len() {
                // TODO: if the listener is pending removal, do not call it
                // TODO: to fix this error, we conceptually need a RefCell for each listener. But it would make more sense to replace FnMut by Fn.
                match (delegate.listeners[i].func)(payload.clone()) {
                    MulticastDelegateResult::Keep => i += 1,
                    MulticastDelegateResult::Remove => _ = self.pending_removal.lock().push(ListenerHandle { untyped: UntypedListenerHandle { index: i, salt: delegate.listeners[i].salt }, phantom: PhantomData }),
                }
            }
        }

        let mut removed_funcs = vec![];

        // If we are the last "owner", flush pending operations
        if let Ok(mut delegate) = delegate_lock.try_borrow_mut() {
            for (listener_handle, func) in self.pending_push.lock().drain(..) {
                delegate.listeners.push(Listener { func, salt: listener_handle.untyped.salt });
            }

            let mut pending_removal = self.pending_removal.lock();
            removed_funcs = Vec::with_capacity(pending_removal.len());
            for listener_handle in pending_removal.drain(..) {
                removed_funcs.push(Self::remove_internal(&mut delegate, &listener_handle));
            }
        }

        drop(delegate_lock);

        while !removed_funcs.is_empty() {
            drop(removed_funcs.pop());
        }
    }
/*
FIXME
    #[expect(dead_code, reason = "This is a static assert")]
    const fn must_be_send_and_sync() {
        const fn f<T: Send + Sync>() {}
        f::<Self>()
    }
    */
}