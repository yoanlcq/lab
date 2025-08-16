use crate::{listener::ListenerHandle, multicast::MulticastDelegateResult};

type Func<T> = Box<dyn FnMut(T) -> MulticastDelegateResult + Send>;

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
    listeners: Vec<Listener<T>>,
}

impl<T> core::fmt::Debug for Delegate<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Delegate").field("listeners", &self.listeners).finish()
    }
}

impl<T> Default for Delegate<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Delegate<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self { listeners: vec![] }
    }
    pub fn remove(&mut self, listener_handle: &ListenerHandle<T>) -> Option<Func<T>> {
        let listener_handle = listener_handle.untyped();
        if let Some(listener) = self.listeners.get(listener_handle.initial_index_hint) {
            if listener.uid == listener_handle.uid {
                return Some(self.listeners.remove(listener_handle.initial_index_hint).func);
            }
        }
        if let Some(index) = self.listeners.iter().position(|x| x.uid == listener_handle.uid) {
            Some(self.listeners.remove(index).func)
        } else {
            None
        }
    }
    pub fn push(&mut self, func: Func<T>) -> ListenerHandle<T> {
        let listener_handle = ListenerHandle::generate_new(self.listeners.len());
        self.listeners.push(Listener { func, uid: listener_handle.untyped().uid });
        listener_handle
    }
    pub fn broadcast(&mut self, payload: T) where T: Copy {
        self.broadcast_cloning(&payload);
    }
    pub fn broadcast_cloning(&mut self, payload: &T) where T: Clone {
        let mut i = 0;
        while i < self.listeners.len() {
            match (self.listeners[i].func)(payload.clone()) {
                MulticastDelegateResult::Keep => i += 1,
                MulticastDelegateResult::Remove => drop(self.listeners.remove(i)),
            }
        }
    }
}
