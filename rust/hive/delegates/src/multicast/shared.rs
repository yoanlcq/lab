use paste::paste;

mod pending_removal {
    use std::collections::HashSet;

    use crate::listener::UntypedListenerHandle;

    /// The point is to preserve the order of removals but also have an efficient `contains()` implementation
    #[derive(Debug, Default)]
    pub(super) struct PendingRemoval {
        ordered_listener_handles: Vec<UntypedListenerHandle>,
        set_of_uids: HashSet<u64>,
    }

    impl PendingRemoval {
        pub(super) fn push(&mut self, listener_handle: UntypedListenerHandle) {
            if self.set_of_uids.insert(listener_handle.uid) {
                self.ordered_listener_handles.push(listener_handle);
            }
        }
        pub(super) fn contains_listener_uid(&self, uid: u64) -> bool {
            self.set_of_uids.contains(&uid)
        }
        pub(super) fn into_vec(self) -> Vec<UntypedListenerHandle> {
            self.ordered_listener_handles
        }
    }
}

#[macro_export]
macro_rules! declare_shared_multicast_delegate {
    ($visibility:vis $Delegate:ident, $FnTrait:ident($($Args:ty),*) $(+ $ExtraTraits:tt)*) => {
        paste!{
            $visibility type $Delegate = [<$Delegate _internal>]::Delegate;

            #[allow(dead_code, reason = "This lightens the load when $visibility is empty and the delegate type is not used")]
            #[expect(clippy::allow_attributes, reason = "allow(...) is necessary if $visibility is empty")]
            $visibility type [<$Delegate ListenerHandle>] = [<$Delegate _internal>]::ListenerHandle;

            #[expect(non_snake_case, reason = "This is for convenience for the callers of this macro. That module is for encapsulation only and is never expected to be used as-is")]
            #[allow(unnameable_types, reason = "This is usable via the type aliases outside the module")]
            #[allow(unreachable_pub, reason = "May happen if $visibility is empty")]
            #[allow(dead_code, reason = "This lightens the load when $visibility is empty and the delegate type is not used")]
            #[expect(clippy::allow_attributes, reason = "allow(...) is necessary if $visibility is empty")]
            mod [<$Delegate _internal>] {
                use core::cell::RefCell;
                use parking_lot::{Mutex, ReentrantMutex};
                use $crate::{listener::UntypedListenerHandle, multicast::{shared::pending_removal::PendingRemoval, MulticastDelegateResult}};

                type Func = Box<dyn $FnTrait($($Args),*) -> MulticastDelegateResult $(+ $ExtraTraits)*>;

                struct Listener {
                    func: Func,
                    uid: u64,
                }

                impl core::fmt::Debug for Listener {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        let Self { func: _, ref uid } = *self;
                        f.debug_struct("Listener").field("uid", &uid).finish_non_exhaustive()
                    }
                }

                #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
                pub struct ListenerHandle {
                    untyped: UntypedListenerHandle,
                }

                impl ListenerHandle {
                    #[must_use]
                    pub fn generate_new(initial_index_hint: usize) -> Self {
                        Self {
                            untyped: UntypedListenerHandle::generate_new(initial_index_hint),
                        }
                    }
                    #[must_use]
                    pub const fn untyped(&self) -> UntypedListenerHandle {
                        self.untyped
                    }
                }

                pub struct Delegate {
                    listeners: ReentrantMutex<RefCell<Vec<Listener>>>,
                    pending_push: Mutex<Vec<(ListenerHandle, Func)>>,
                    pending_removal: Mutex<PendingRemoval>,
                    must_call_listeners_even_if_pending_removal: bool,
                }

                impl core::fmt::Debug for Delegate {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        let Self { ref listeners, ref pending_push, ref pending_removal, must_call_listeners_even_if_pending_removal } = *self;
                        f.debug_struct(stringify!($Delegate))
                            .field("listeners", listeners)
                            .field("pending_push", &format!("<{} boxed closures>", pending_push.lock().len()))
                            .field("pending_removal", pending_removal)
                            .field("must_call_listeners_even_if_pending_removal", &must_call_listeners_even_if_pending_removal)
                            .finish()
                    }
                }

                impl Default for Delegate {
                    fn default() -> Self {
                        Self::new()
                    }
                }

                impl Delegate {
                    #[must_use]
                    pub fn new() -> Self {
                        Self {
                            listeners: ReentrantMutex::new(RefCell::new(vec![])),
                            pending_push: Mutex::new(vec![]),
                            pending_removal: Mutex::new(PendingRemoval::default()),
                            must_call_listeners_even_if_pending_removal: false,
                        }
                    }
                    pub const fn must_call_listeners_even_if_pending_removal(&self) -> bool {
                        self.must_call_listeners_even_if_pending_removal
                    }
                    pub const fn set_must_call_listeners_even_if_pending_removal(&mut self, must_call_listeners_even_if_pending_removal: bool) {
                        self.must_call_listeners_even_if_pending_removal = must_call_listeners_even_if_pending_removal;
                    }
                    fn remove_immediate(listeners: &mut Vec<Listener>, listener_handle: &UntypedListenerHandle) -> Option<Func> {
                        if let Some(listener) = listeners.get(listener_handle.initial_index_hint) {
                            if listener.uid == listener_handle.uid {
                                return Some(listeners.remove(listener_handle.initial_index_hint).func);
                            }
                        }
                        if let Some(index) = listeners.iter().rposition(|x| x.uid == listener_handle.uid) {
                            Some(listeners.remove(index).func)
                        } else {
                            None
                        }
                    }
                    /// This returns `Some(...)` only if the listener could be removed immediately.
                    /// This means it's possible that it returns `None` even though the listener is present and will be removed eventually.
                    pub fn remove(&self, listener_handle: &ListenerHandle) -> Option<Func> {
                        if let Some(mut listeners) = self.listeners.try_lock().as_ref().and_then(|x| x.try_borrow_mut().ok()) {
                            Self::remove_immediate(&mut listeners, &listener_handle.untyped())
                        } else {
                            self.pending_removal.lock().push(listener_handle.untyped());
                            None
                        }
                    }
                    pub fn push(&self, func: Func) -> ListenerHandle {
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
                    pub fn broadcast(&self, $(args: $Args),*) where $($Args: Copy),* {
                        self.broadcast_cloning($({ let _: $Args; &args }),*);
                    }
                    pub fn broadcast_cloning(&self, $(args: &$Args),*) where $($Args: Clone),* {
                        let listeners_lock = self.listeners.lock();

                        for (i, listener) in listeners_lock.borrow().iter().enumerate() {
                            if self.must_call_listeners_even_if_pending_removal || !self.pending_removal.lock().contains_listener_uid(listener.uid) {
                                match (listener.func)($({ let _: $Args; args.clone() }),*) {
                                    MulticastDelegateResult::Keep => (),
                                    MulticastDelegateResult::Remove => self.pending_removal.lock().push(UntypedListenerHandle::from_parts(i, listener.uid)),
                                }
                            }
                        }
                        // Question: what of listeners possibly added to pending_push during the loop above?
                        // Consider adding a "policy" params to `push()` for specifying whether or not to call the pushed listener within the current call to `broadcast()`?
                        // That would add a lot of complexity and a bit of overhead. I'm probably overthinking it, since Unreal Engine doesn't bother with that (it just calls the listener in reverse order, so this ignores those added during iteration)

                        // If we are the last "owner" of the lock, flush pending operations
                        let mut removed_funcs = vec![];
                        if let Ok(mut listeners) = listeners_lock.try_borrow_mut() {
                            for (listener_handle, func) in core::mem::take(&mut *self.pending_push.lock()) {
                                listeners.push(Listener { func, uid: listener_handle.untyped().uid });
                            }

                            let pending_removal = core::mem::take(&mut *self.pending_removal.lock()).into_vec();

                            removed_funcs = Vec::with_capacity(pending_removal.len());
                            for listener_handle in pending_removal {
                                if let Some(func) = Self::remove_immediate(&mut listeners, &listener_handle) {
                                    removed_funcs.push(func);
                                }
                            }
                        }

                        // Drop the listeners lock BEFORE dropping the funcs (their Drop implementation could cause chain reactions leading to attempting to get the lock)
                        drop(listeners_lock);

                        // Drop following the order of calls to remove()
                        for func in removed_funcs {
                            drop(func);
                        }
                    }
                }
            }
        }
    }
}

declare_shared_multicast_delegate!(pub SimpleSharedMulticastDelegateNoSend, Fn());
declare_shared_multicast_delegate!(pub SimpleSharedMulticastDelegate, Fn() + Send);

impl SimpleSharedMulticastDelegate {
    #[expect(dead_code, reason = "This is a static assert")]
    const fn must_be_send_and_sync() {
        const fn f<T: Send + Sync>() {}
        f::<Self>();
    }
}