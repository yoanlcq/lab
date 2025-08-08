
pub mod sync {
    use alloc::sync::{Arc, Weak};
    use std::sync::OnceLock;

    /// A weak reference to an Arc that owns you.
    /// 
    /// It's a pattern where you don't want to create cyclic references, but upgrading the Weak logically cannot ever fail.
    #[derive(Debug)]
    pub struct WeakSelf<T>(OnceLock<Weak<T>>);

    impl<T> Default for WeakSelf<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    #[expect(clippy::missing_panics_doc, reason = "WeakSelf is for a very specific pattern that is very hard to write incorrectly. This type exists exactly because we know the panics will not occur if the pattern is followed")]
    impl<T> WeakSelf<T> {
        #[must_use]
        pub const fn new() -> Self {
            Self(OnceLock::new())
        }
        pub fn init(&self, weak: Weak<T>) {
            self.0.set(weak).expect("init() should be called right after owner creation on the same thread... Not more than once and certainly not concurrently!");
        }
        #[must_use]
        pub fn weak(&self) -> &Weak<T> {
            self.0.get().expect("init() should have been called first")
        }
        /// Caveat: be wary of using this within `Drop`, it will most likely panic.
        #[must_use]
        pub fn upgrade_unwrap(&self) -> Arc<T> {
            self.weak().upgrade().expect("Upgrading WeakSelf cannot fail if it is already within a struct wrapped in Arc as intended")
        }
    }

    impl<T: Send + Sync> WeakSelf<T> {
        #[expect(dead_code, reason = "It's a static assert")]
        const fn must_be_send_and_sync() {
            const fn f<T: Send + Sync>() {}
            f::<Self>();
        }
    }
}

pub mod rc {
    use alloc::rc::{Rc, Weak};
    use core::cell::OnceCell;

    /// A weak reference to an Arc that owns you.
    /// 
    /// It's a pattern where you don't want to create cyclic references, but upgrading the Weak logically cannot ever fail.
    #[derive(Debug)]
    pub struct WeakSelf<T>(OnceCell<Weak<T>>);

    impl<T> Default for WeakSelf<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    #[expect(clippy::missing_panics_doc, reason = "WeakSelf is for a very specific pattern that is very hard to write incorrectly. This type exists exactly because we know the panics will not occur if the pattern is followed")]
    impl<T> WeakSelf<T> {
        #[must_use]
        pub const fn new() -> Self {
            Self(OnceCell::new())
        }
        pub fn init(&self, weak: Weak<T>) {
            self.0.set(weak).expect("init() should be called right after owner creation and not more than once");
        }
        #[must_use]
        pub fn weak(&self) -> &Weak<T> {
            self.0.get().expect("init() should have been called first")
        }
        /// Caveat: be wary of using this within `Drop`, it will most likely panic.
        #[must_use]
        pub fn upgrade_unwrap(&self) -> Rc<T> {
            self.weak().upgrade().expect("Upgrading WeakSelf cannot fail if it is already within a struct wrapped in Rc as intended")
        }
    }
}