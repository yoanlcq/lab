//! "Exclusive" multicast delegates, best for single-threaded contexts.
//! 
//! Unlike those in the `shared` module, an "exclusive" multicast delegate is as simple as it gets: it's a `Vec` of boxed closures.
//! 
//! - All operations require exclusive (`&mut`) access. This makes it very predictable but possibly harder to use in large codebases.
//!   You can of course wrap it in `RefCell` or `Mutex`, but beware of chain reactions that could cause a deadlock or panic.
//! - Listeners are called from first to last.
//! - Removing a listener is order-preserving.
//! - When dropping the delegate, listeners are dropped in reverse order.

use pastey::paste;

/// Declare your own exclusive delegate types with as precise trait requirements as needed.
#[macro_export]
macro_rules! declare_exclusive_multicast_delegate {
    ($(#[$outer:meta])* $visibility:vis $Delegate:ident, $FnTrait:ident($($Args:ty),*) $(+ $ExtraTraits:tt)*) => {
        paste!{
            $(#[$outer])* 
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
                use $crate::listener::ListenerReply;
                use $crate::listener::UntypedListenerHandle;
                use $crate::multicast::BroadcastStats;

                type Func = Box<dyn $FnTrait($($Args),*) -> ListenerReply $(+ $ExtraTraits)*>;

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
                    fn generate_new(initial_index_hint: usize) -> Self {
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

                pub struct Delegate {
                    listeners: Vec<Listener>,
                }

                /// Have a well-defined drop order where listeners are dropped from last to first
                impl Drop for Delegate {
                    fn drop(&mut self) {
                        for listener in core::mem::take(&mut self.listeners).into_iter().rev() {
                            drop(listener);
                        }
                    }
                }

                impl core::fmt::Debug for Delegate {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        let Self { ref listeners } = *self;
                        f.debug_struct(stringify!($Delegate))
                            .field("listeners", listeners)
                            .finish()
                    }
                }

                impl Default for Delegate {
                    fn default() -> Self {
                        Self::new()
                    }
                }

                impl Delegate {
                    /// Creates a new delegate, doesn't perform any allocation.
                    #[must_use]
                    pub const fn new() -> Self {
                        Self { listeners: vec![] }
                    }
                    #[must_use]
                    pub fn with_capacity(capacity: usize) -> Self {
                        Self { listeners: Vec::with_capacity(capacity) }
                    }
                    #[must_use]
                    pub const fn len(&self) -> usize {
                        self.listeners.len()
                    }
                    #[must_use]
                    pub const fn capacity(&self) -> usize {
                        self.listeners.capacity()
                    }
                    #[must_use]
                    pub const fn is_empty(&self) -> bool {
                        self.listeners.is_empty()
                    }
                    /// Removes a listener by handle, preserving ordering of the listeners list, returning `Some(...)` if it was found and removed.
                    /// 
                    /// This has O(N) complexity, but has an O(1) "happy path" when removals always occur from the end of the list.
                    pub fn remove(&mut self, listener_handle: &ListenerHandle) -> Option<Func> {
                        let listener_handle = listener_handle.untyped();
                        if let Some(listener) = self.listeners.get(listener_handle.initial_index_hint) {
                            if listener.uid == listener_handle.uid {
                                return Some(self.listeners.remove(listener_handle.initial_index_hint).func);
                            }
                        }
                        if let Some(index) = self.listeners.iter().rposition(|x| x.uid == listener_handle.uid) {
                            Some(self.listeners.remove(index).func)
                        } else {
                            None
                        }
                    }
                    /// Adds a listener to the end of the list.
                    /// 
                    /// `func` must return a `ListenerReply` indicating whether or not it should be removed from the list.
                    /// This is useful when `func` has a clear "receiver" that may expire, or if it's designed as a one-shot alarm.
                    pub fn push(&mut self, func: Func) -> ListenerHandle {
                        let listener_handle = ListenerHandle::generate_new(self.listeners.len());
                        self.listeners.push(Listener { func, uid: listener_handle.untyped().uid });
                        listener_handle
                    }
                    /// Broadcast (or "dispatch" or "invoke") this delegate, calling all listeners with the provided arguments to the function.
                    /// 
                    /// Listeners are called following the natural order in which they appear in the list.
                    pub fn broadcast(&mut self, $(args: $Args),*) -> BroadcastStats where $($Args: Copy),* {
                        self.broadcast_cloning($({ let _: $Args; &args }),*)
                    }
                    /// Same as `broadcast()` but the arguments are cloned. You should of course prefer `broadcast()` unless you have no choice.
                    pub fn broadcast_cloning(&mut self, $(args: &$Args),*) -> BroadcastStats where $($Args: Clone),* {
                        let mut i = 0;
                        while i < self.listeners.len() {
                            match (self.listeners[i].func)($({ let _: $Args; args.clone() }),*) {
                                ListenerReply::Keep => i += 1,
                                ListenerReply::Remove => drop(self.listeners.remove(i)),
                            }
                        }
                        BroadcastStats {
                            had_any_listener: !self.listeners.is_empty(),
                            has_called_any_listener: !self.listeners.is_empty(),
                        }
                    }
                }
            }
        }
    }
}

declare_exclusive_multicast_delegate!{
    /// An exclusive multicast delegate that takes no parameters.
    pub SimpleExclusiveMulticastDelegate, FnMut()
}

declare_exclusive_multicast_delegate!{
    /// An exclusive multicast delegate that takes no parameters, and is `Send` and `Sync`.
    /// 
    /// Compared to `SimpleExclusiveMulticastDelegate`, this one is given the longer name, because it is expected to be less commonly used.
    /// 
    /// Also consider using a "shared" delegate instead.
    pub SimpleExclusiveMulticastDelegateSendAndSync, FnMut() + Send + Sync
}

impl SimpleExclusiveMulticastDelegateSendAndSync {
    #[expect(dead_code, reason = "This is a static assert")]
    const fn must_be_send_and_sync() {
        const fn f<T: Send + Sync>() {}
        f::<Self>();
    }
}