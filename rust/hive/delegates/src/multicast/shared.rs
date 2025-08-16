use core::cell::RefCell;
use parking_lot::{Mutex, ReentrantMutex};
use crate::{listener::ListenerHandle, multicast::MulticastDelegateResult};

type Func<T> = Box<dyn Fn(T) -> MulticastDelegateResult + Send>;

struct Listener<T> {
    func: Func<T>,
    uid: u64,
}

impl<T> core::fmt::Debug for Listener<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Listener").field("uid", &self.uid).finish_non_exhaustive()
    }
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

pub struct Delegate<T> {
    listeners: ReentrantMutex<RefCell<Vec<Listener<T>>>>,
    pending_push: Mutex<Vec<(ListenerHandle<T>, Func<T>)>>,
    pending_removal: Mutex<Vec<ListenerHandle<T>>>,
}

impl<T> core::fmt::Debug for Delegate<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { ref listeners, ref pending_push, ref pending_removal } = *self;
        f.debug_struct("Delegate")
            .field("listeners", listeners)
            .field("pending_push", &format!("<{} boxed closures>", pending_push.lock().len()))
            .field("pending_removal", pending_removal)
            .finish()
    }
}

impl<T> Default for Delegate<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Delegate<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            listeners: ReentrantMutex::new(RefCell::new(vec![])),
            pending_push: Mutex::new(vec![]),
            pending_removal: Mutex::new(vec![]),
        }
    }
    fn remove_immediate(listeners: &mut Vec<Listener<T>>, listener_handle: &ListenerHandle<T>) -> Option<Func<T>> {
        let listener_handle = listener_handle.untyped();
        if let Some(listener) = listeners.get(listener_handle.initial_index_hint) {
            if listener.uid == listener_handle.uid {
                return Some(listeners.remove(listener_handle.initial_index_hint).func);
            }
        }
        if let Some(index) = listeners.iter().position(|x| x.uid == listener_handle.uid) {
            Some(listeners.remove(index).func)
        } else {
            None
        }
    }
    pub fn remove(&self, listener_handle: &ListenerHandle<T>) -> Option<Func<T>> {
        if let Some(mut listeners) = self.listeners.try_lock().as_ref().and_then(|x| x.try_borrow_mut().ok()) {
            Self::remove_immediate(&mut listeners, listener_handle)
        } else {
            self.pending_removal.lock().push(*listener_handle);
            None
        }
    }
    pub fn push(&self, func: Func<T>) -> ListenerHandle<T> {
        if let Some(mut listeners) = self.listeners.try_lock().as_ref().and_then(|x| x.try_borrow_mut().ok()) {
            let listener_handle = ListenerHandle::generate_new(listeners.len());
            listeners.push(Listener { func, uid: listener_handle.untyped().uid });
            listener_handle
        } else {
            let listener_handle = ListenerHandle::generate_new(usize::MAX);
            self.pending_push.lock().push((listener_handle, func));
            listener_handle
        }
    }
    pub fn broadcast(&self, payload: T) where T: Copy {
        self.broadcast_cloning(&payload);
    }
    pub fn broadcast_cloning(&self, payload: &T) where T: Clone {
        let listeners_lock = self.listeners.lock();
        {
            let mut i = 0;
            let listeners = listeners_lock.borrow();
            while i < listeners.len() {
                // TODO: if the listener is pending removal, do not call it
                match (listeners[i].func)(payload.clone()) {
                    MulticastDelegateResult::Keep => i += 1,
                    MulticastDelegateResult::Remove => self.pending_removal.lock().push(ListenerHandle::from_parts(i, listeners[i].uid)),
                }
            }
        }

        // If we are the last "owner" of the lock, flush pending operations
        let mut removed_funcs = vec![];
        if let Ok(mut listeners) = listeners_lock.try_borrow_mut() {
            for (listener_handle, func) in self.pending_push.lock().drain(..) {
                listeners.push(Listener { func, uid: listener_handle.untyped().uid });
            }

            let mut pending_removal = self.pending_removal.lock();
            removed_funcs = Vec::with_capacity(pending_removal.len());
            for listener_handle in pending_removal.drain(..) {
                removed_funcs.push(Self::remove_immediate(&mut listeners, &listener_handle));
            }
        }

        // Drop the listeners lock BEFORE dropping the funcs (dropping them could cause chain reactions leading to attempting to get the lock)
        drop(listeners_lock);

        // Drop in a well-defined order, first to last
        for func in removed_funcs {
            drop(func);
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