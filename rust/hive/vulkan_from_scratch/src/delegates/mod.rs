use core::marker::PhantomData;
use core::sync::atomic::AtomicUsize;
use parking_lot::Mutex;

#[must_use = "The delegate uses this to know if the listener hasn't expired"]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MulticastDelegateResult {
    Keep,
    Remove,
}

struct Listener<T> {
    func: Box<dyn FnMut(T) -> MulticastDelegateResult + Send>,
    salt: usize,
}

impl<T> core::fmt::Debug for Listener<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Listener").field("salt", &self.salt).finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub struct ListenerHandle<T> {
    index: usize,
    salt: usize,
    phantom: PhantomData<T>,
}

static LAST_LISTENER_SALT: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct MulticastDelegate<T> {
    listeners: Vec<Listener<T>>,
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
            let listener = self.listeners.get(listener_handle.index)?;
            if listener.salt != listener_handle.salt {
                return None;
            }
        }
        Some(self.listeners.remove(listener_handle.index).func)
    }
    pub fn push(&mut self, func: Box<dyn FnMut(T) -> MulticastDelegateResult + Send>) -> ListenerHandle<T> {
        let salt = LAST_LISTENER_SALT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        let index = self.listeners.len();
        self.listeners.push(Listener { func, salt });
        ListenerHandle {
            index,
            salt,
            phantom: PhantomData,
        }
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

#[derive(Debug)]
pub struct SharedMulticastDelegate<T>(Mutex<MulticastDelegate<T>>);

impl<T> Default for SharedMulticastDelegate<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SharedMulticastDelegate<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self(Mutex::new(MulticastDelegate::new()))
    }
    pub fn remove(&self, listener_handle: &ListenerHandle<T>) -> Option<Box<dyn FnMut(T) -> MulticastDelegateResult>> {
        self.0.lock().remove(listener_handle)
    }
    pub fn push(&self, func: Box<dyn FnMut(T) -> MulticastDelegateResult + Send>) -> ListenerHandle<T> {
        self.0.lock().push(func)
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
        self.0.lock().broadcast_cloning(payload);
    }
}