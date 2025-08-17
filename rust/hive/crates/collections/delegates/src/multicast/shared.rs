//! "Shared" delegates, best for multi-threaded contexts or complex codebases in general.
//! 
//! Because such delegates are typically used a lot in large codebases, it is important that they offer strong guarantees about their behavior.
//! This is why the implementation is very careful about many details; the slightest modification may be a breaking change in the most advanced use cases.

/// Just like `private_reexports`, this module is only public out of necessity for macro expansion in other crates, but is not part of the public API.
#[doc(hidden)]
pub mod pending_removal {
    use std::collections::HashSet;

    use crate::listener::UntypedListenerHandle;

    /// The point is to preserve the order of removals but also have an efficient `contains()` implementation
    #[derive(Debug, Default)]
    pub struct PendingRemoval {
        ordered_listener_handles: Vec<UntypedListenerHandle>,
        set_of_uids: HashSet<u64>,
    }

    impl PendingRemoval {
        #[must_use]
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                ordered_listener_handles: Vec::with_capacity(capacity),
                set_of_uids: HashSet::with_capacity(capacity),
            }
        }
        pub fn push(&mut self, listener_handle: UntypedListenerHandle) {
            if self.set_of_uids.insert(listener_handle.uid) {
                self.ordered_listener_handles.push(listener_handle);
            }
        }
        #[must_use]
        pub fn contains_listener_uid(&self, uid: u64) -> bool {
            self.set_of_uids.contains(&uid)
        }
        #[must_use]
        pub fn into_inner_listener_handles_vec(self) -> Vec<UntypedListenerHandle> {
            self.ordered_listener_handles
        }
    }
}

/// Declare your own shared delegate types with as precise trait requirements as needed.
/// 
/// Note that you cannot use `FnMut` for the `FnTrait` parameter: `FnMut` is fundamentally incompatible with reentrancy, even if we did wrap it in a `RefCell`.
#[macro_export]
macro_rules! declare_shared_multicast_delegate {
    ($(#[$outer:meta])* $visibility:vis $Delegate:ident $(<$($delegatelifetimes:lifetime),* $($delegatetypeparams:ident),*>)?, $FnTrait:ident $(<$($argslifetimes:lifetime),*>)? ($($Args:ty),*) $(+ $ExtraTraits:path)*) => {
        $crate::private_reexports::pastey::paste!{
            $(#[$outer])* 
            $visibility type $Delegate $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)? = [<$Delegate _internal>]::Delegate $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)?;

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
                use $crate::private_reexports::parking_lot::{Mutex, ReentrantMutex, ReentrantMutexGuard};
                use $crate::{listener::{UntypedListenerHandle, ListenerReply}, multicast::shared::pending_removal::PendingRemoval};
                use $crate::multicast::BroadcastStats;
                #[allow(unused_imports, reason = "This is actually required for convenience for callers")]
                use super::*;

                type Func $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)? = Box<dyn $(for<$($argslifetimes),*>)? $FnTrait($($Args),*) -> ListenerReply $(+ $ExtraTraits)*>;

                struct Listener $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)? {
                    func: Func $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)?,
                    uid: u64,
                }

                impl $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)? core::fmt::Debug for Listener $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)? {
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

                impl From<UntypedListenerHandle> for ListenerHandle {
                    fn from(value: UntypedListenerHandle) -> Self {
                        Self { untyped: value }
                    }
                }

                impl From<ListenerHandle> for UntypedListenerHandle {
                    fn from(value: ListenerHandle) -> Self {
                        value.untyped
                    }
                }

                pub struct Delegate $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)? {
                    listeners: ReentrantMutex<RefCell<Vec<Listener $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)?>>>,
                    pending_push: Mutex<Vec<(ListenerHandle, Func $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)?)>>,
                    pending_removal: Mutex<PendingRemoval>,
                    must_call_listeners_even_if_pending_removal: bool,
                }

                /// When dropping the delegate, pending removals are applied in their natural order, then listeners are dropped in reverse order.
                impl $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)? Drop for Delegate $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)? {
                    fn drop(&mut self) {
                        let mut listeners = core::mem::take(self.listeners.get_mut().get_mut());
                        for (listener_handle, func) in core::mem::take(self.pending_push.get_mut()) {
                            // TODO: allocating within a Drop impl is a bit cursed... I did this because it's easier to write
                            listeners.push(Listener { func, uid: listener_handle.untyped().uid });
                        }
                        for listener_handle in core::mem::take(self.pending_removal.get_mut()).into_inner_listener_handles_vec() {
                            drop(Self::remove_immediate(&mut listeners, &listener_handle));
                        }
                        for listener in listeners.into_iter().rev() {
                            drop(listener);
                        }
                    }
                }

                impl $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)? core::fmt::Debug for Delegate $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)? {
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

                /// Creates a new delegate, doesn't perform any allocation.
                impl $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)? Default for Delegate $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)? {
                    fn default() -> Self {
                        Self::new()
                    }
                }

                impl $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)? Delegate $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)? {
                    /// Creates a new delegate, doesn't perform any allocation.
                    #[must_use]
                    pub fn new() -> Self {
                        Self::with_capacities(0, 0, 0)
                    }
                    #[must_use]
                    pub fn with_capacities(listeners_capacity: usize, pending_push_capacity: usize, pending_removal_capacity: usize) -> Self {
                        Self {
                            listeners: ReentrantMutex::new(RefCell::new(Vec::with_capacity(listeners_capacity))),
                            pending_push: Mutex::new(Vec::with_capacity(pending_push_capacity)),
                            pending_removal: Mutex::new(PendingRemoval::with_capacity(pending_removal_capacity)),
                            must_call_listeners_even_if_pending_removal: false,
                        }
                    }
                    /// Sets the policy for how to handle listeners that are removed during a call to `broadcast()`.
                    /// 
                    /// The default (`false`) is to not call those listeners, which incurs a lookup in the `HashSet` of "pending removal" listeners.  
                    /// The opposite is to still call those listeners.
                    /// 
                    /// It all depends on what is correct for your use case:
                    /// - `false`: "the listener was removed, to it is important that it does not get called from that point on"
                    /// - `true`: "the listener was there when `broadcast()` was called, so it should receive the event"
                    pub const fn with_must_call_listeners_even_if_pending_removal(mut self, must_call_listeners_even_if_pending_removal: bool) -> Self {
                        self.must_call_listeners_even_if_pending_removal = must_call_listeners_even_if_pending_removal;
                        self
                    }
                    /// See `with_must_call_listeners_even_if_pending_removal()`
                    pub const fn must_call_listeners_even_if_pending_removal(&self) -> bool {
                        self.must_call_listeners_even_if_pending_removal
                    }
                    /// See `with_must_call_listeners_even_if_pending_removal()`
                    pub const fn set_must_call_listeners_even_if_pending_removal(&mut self, must_call_listeners_even_if_pending_removal: bool) {
                        self.must_call_listeners_even_if_pending_removal = must_call_listeners_even_if_pending_removal;
                    }
                    /// This is suffixed "_racy" because the return value is only valid at this specific point in time; it doesn't prevent other threads from modifying this after the lock is released.
                    #[must_use]
                    pub fn len_racy(&self) -> usize {
                        self.listeners.lock().borrow().len()
                    }
                    /// This is suffixed "_racy" because the return value is only valid at this specific point in time; it doesn't prevent other threads from modifying this after the lock is released.
                    #[must_use]
                    pub fn capacity_racy(&self) -> usize {
                        self.listeners.lock().borrow().capacity()
                    }
                    /// This is suffixed "_racy" because the return value is only valid at this specific point in time; it doesn't prevent other threads from modifying this after the lock is released.
                    #[must_use]
                    pub fn is_empty_racy(&self) -> bool {
                        self.listeners.lock().borrow().is_empty()
                    }
                    fn remove_immediate(listeners: &mut Vec<Listener $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)?>, listener_handle: &UntypedListenerHandle) -> Option<Func $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)?> {
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
                    /// Removes a listener by handle, preserving ordering of the listeners list, returning `Some(...)` if it could be found and removed **immediately**.
                    ///
                    /// Note that it's possible that it returns `None` if the listener is "pending push"; it will still be removed eventually.
                    /// 
                    /// This has O(N) complexity, but has an O(1) "happy path" when removals always occur from the end of the list.
                    pub fn remove(&self, listener_handle: &ListenerHandle) -> Option<Func $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)?> {
                        if let Some(mut listeners) = self.listeners.try_lock().as_ref().and_then(|x| x.try_borrow_mut().ok()) {
                            Self::remove_immediate(&mut listeners, &listener_handle.untyped())
                        } else {
                            self.pending_removal.lock().push(listener_handle.untyped());
                            None
                        }
                    }
                    /// Tries to add a listener to the end of the list; if we are within a call to `broadcast`, then the listener will be added to a "pending push" list instead, that will be appended to the list of listeners when the last call to `broadcast()` in the call stack finishes.
                    /// 
                    /// In any case, the returned `ListenerHandle` is always a valid identifier for the listener.
                    /// 
                    /// `func` must return a `ListenerReply` indicating whether or not it should be removed from the list.
                    /// This is useful when `func` has a clear "receiver" that may expire, or if it's designed as a one-shot alarm.
                    pub fn push(&self, func: Func $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)?) -> ListenerHandle {
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
                }

                impl<$($($delegatelifetimes,)*)? $($($argslifetimes,)*)? $($($delegatetypeparams,)*)?> Delegate $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)? {
                    /// Broadcast (or "dispatch" or "invoke") this delegate, calling all listeners with the provided arguments to the function.
                    /// 
                    /// During the call, the list of listeners is "locked" to prevent modification during iteration.
                    /// 
                    /// Listeners are called following the natural order in which they appear in the list.
                    /// 
                    /// # Guarantees
                    ///
                    /// - **Ordering**: Listeners are called from first-in-the-list to last-in-the-list, as for a simple for-loop.
                    /// - **Reentrancy**: A call to `broadcast()` may cause chain reactions which may call `broadcast()` indirectly.
                    ///   This is supported and will not cause a softlock or panic.
                    /// - **Calls from another thread are blocking**: Calls to `broadcast()` will block until no other thread is still within a call to `broadcast()`.
                    /// - **Immediacy**: When `broadcast()` acquires the lock, all listeners are called immediately, even if this call was recursive (implying we were in already in the middle of broadcasting something else).
                    /// - **Added listeners will miss the train**: Any listener added **during** a call to `broadcast()` will not be called for that one. Doing so would require extra complexity and overhead.
                    /// - **The order of removals is always respected**. To guarantee this, returning `ListenerReply::Remove` will add to the list of pending removals, as if `remove()` was called.
                    pub fn broadcast(&self, $(args: $Args),*) -> BroadcastStats where $($Args: Copy),* {
                        self.broadcast_impl(self.listeners.lock(), $({ let _: $Args; &args }),*)
                    }
                    /// Same as `broadcast()` but the arguments are cloned. You should of course prefer `broadcast()` unless you have no choice.
                    pub fn broadcast_cloning(&self, $(args: &$Args),*) -> BroadcastStats where $($Args: Clone),* {
                        self.broadcast_impl(self.listeners.lock(), $({ let _: $Args; &args }),*)
                    }
                    /// Same as `broadcast()` but returns `None` if the listeners list lock could not be acquired.
                    pub fn try_broadcast(&self, $(args: $Args),*) -> Option<BroadcastStats> where $($Args: Copy),* {
                        self.listeners.try_lock().map(|listeners_lock| self.broadcast_impl(listeners_lock, $({ let _: $Args; &args }),*))
                    }
                    /// Same as `broadcast_cloning()` but returns `None` if the listeners list lock could not be acquired.
                    pub fn try_broadcast_cloning(&self, $(args: &$Args),*) -> Option<BroadcastStats> where $($Args: Clone),* {
                        self.listeners.try_lock().map(|listeners_lock| self.broadcast_impl(listeners_lock, $({ let _: $Args; &args }),*))
                    }
                    fn broadcast_impl(&self, listeners_lock: ReentrantMutexGuard<RefCell<Vec<Listener $(<$($delegatelifetimes,)* $($delegatetypeparams),*>)?>>>, $(args: &$Args),*) -> BroadcastStats where $($Args: Clone),* {
                        let mut stats = BroadcastStats::default();

                        for (i, listener) in listeners_lock.borrow().iter().enumerate() {
                            stats.had_any_listener = true;
                            if self.must_call_listeners_even_if_pending_removal || !self.pending_removal.lock().contains_listener_uid(listener.uid) {
                                stats.has_called_any_listener = true;
                                match (listener.func)($({ let _: $Args; args.clone() }),*) {
                                    ListenerReply::Keep => (),
                                    ListenerReply::Remove => self.pending_removal.lock().push(UntypedListenerHandle::from_parts(i, listener.uid)),
                                }
                            }
                        }
                        // Question: what of listeners possibly added to pending_push during the loop above?
                        // Consider adding a "policy" params to `push()` for specifying whether or not to call the pushed listener within the current call to `broadcast()`?
                        // That would add a lot of complexity and a bit of overhead. I'm probably overthinking it, since Unreal Engine doesn't bother with that (it just calls the listeners in reverse order, so this ignores those added during iteration)

                        // If we are the last "owner" of the lock, flush pending operations
                        let mut removed_funcs = vec![];
                        if let Ok(mut listeners) = listeners_lock.try_borrow_mut() {
                            for (listener_handle, func) in core::mem::take(&mut *self.pending_push.lock()) {
                                listeners.push(Listener { func, uid: listener_handle.untyped().uid });
                            }

                            let pending_removal = core::mem::take(&mut *self.pending_removal.lock()).into_inner_listener_handles_vec();

                            removed_funcs = Vec::with_capacity(pending_removal.len());
                            for listener_handle in pending_removal {
                                if let Some(func) = Self::remove_immediate(&mut listeners, &listener_handle) {
                                    removed_funcs.push(func);
                                }
                            }
                        }

                        // Drop the lock BEFORE dropping `removed_func`, since their Drop implementation may cause chain reactions that end up accessing the lock
                        drop(listeners_lock);

                        // Drop following the order of calls to remove()
                        for func in removed_funcs {
                            drop(func);
                        }

                        stats
                    }
                }
            }
        }
    }
}

declare_shared_multicast_delegate!{
    /// A shared multicast delegate that is `Send` and `Sync`.
    pub SimpleSharedMulticastDelegate, Fn() + Send
}

declare_shared_multicast_delegate!{
    /// A shared multicast delegate that isn't `Send` or `Sync`.
    /// 
    /// Compared to `SimpleSharedMulticastDelegate`, this one is given the longer name, because it is expected to be less commonly used.
    pub SimpleSharedMulticastDelegateNoSend, Fn()
}

#[expect(dead_code, unused_qualifications, reason = "This serves as a compile-time check")]
mod macro_stress_test {
    declare_shared_multicast_delegate!{
        /// A shared multicast delegate that is `Send` and `Sync`.
        MacroStressTest<T>, Fn<'a>(&'a T) + core::marker::Sync
    }
}

impl SimpleSharedMulticastDelegate {
    #[expect(dead_code, reason = "This is a static assert")]
    const fn must_be_send_and_sync() {
        const fn f<T: Send + Sync>() {}
        f::<Self>();
    }
}