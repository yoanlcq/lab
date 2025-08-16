use paste::paste;

#[macro_export]
macro_rules! declare_exclusive_multicast_delegate {
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
                use $crate::multicast::MulticastDelegateResult;
                use $crate::listener::UntypedListenerHandle;

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
                    listeners: Vec<Listener>,
                }

                impl Drop for Delegate {
                    fn drop(&mut self) {
                        // Have a well-defined drop order where listeners are dropped from last to first
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
                    #[must_use]
                    pub const fn new() -> Self {
                        Self { listeners: vec![] }
                    }
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
                    pub fn push(&mut self, func: Func) -> ListenerHandle {
                        let listener_handle = ListenerHandle::generate_new(self.listeners.len());
                        self.listeners.push(Listener { func, uid: listener_handle.untyped().uid });
                        listener_handle
                    }
                    pub fn broadcast(&mut self, $(args: $Args),*) where $($Args: Copy),* {
                        self.broadcast_cloning($({ let _: $Args; &args }),*);
                    }
                    pub fn broadcast_cloning(&mut self, $(args: &$Args),*) where $($Args: Clone),* {
                        let mut i = 0;
                        while i < self.listeners.len() {
                            match (self.listeners[i].func)($({ let _: $Args; args.clone() }),*) {
                                MulticastDelegateResult::Keep => i += 1,
                                MulticastDelegateResult::Remove => drop(self.listeners.remove(i)),
                            }
                        }
                    }
                }
            }
        }
    }
}

declare_exclusive_multicast_delegate!(pub SimpleExclusiveMulticastDelegate, FnMut());
declare_exclusive_multicast_delegate!(pub SimpleExclusiveMulticastDelegateSendAndSync, FnMut() + Send + Sync);

impl SimpleExclusiveMulticastDelegateSendAndSync {
    #[expect(dead_code, reason = "This is a static assert")]
    const fn must_be_send_and_sync() {
        const fn f<T: Send + Sync>() {}
        f::<Self>();
    }
}