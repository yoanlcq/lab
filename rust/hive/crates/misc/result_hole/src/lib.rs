//! Utilities for handling `Result`s where it's not obvious what to do with the `Err(...)` variant.
//! 
//! This can come up in some situations:
//! - A function call that fails 1% of the time due to factors outside of your control (such as graphics API results);
//! - When there is no good way to handle the `Err(...)` case but you don't want to panic either (example: some uses of `std::sync::Mutex::lock()`).
//! - The result occurs in a inconvenient place such as a system callback or a `Drop` implementation.
//! 
//! The primary purpose of this crate is to provide a "standardized" way of dealing with those cases.
//! Then, if it ever becomes an issue, you can easily find all places where you call this crate's API, and/or modify this crate's behavior to fit your needs.
//! 
//! The only requirement is that the `Err(...)` variant implements `core::error::Error`; this is required to have minimum guarantees that we can do something with the error.

use core::error::Error;
use std::sync::LazyLock;

use delegates::declare_shared_multicast_delegate;

#[derive(Debug, Copy, Clone)]
pub struct ErrorPayload<'a> {
    pub error: &'a dyn Error,
    pub caller_may_use_ok: bool,
    pub caller_may_use_err: bool,
}

declare_shared_multicast_delegate!{
    pub OnError, Fn<'a>(&'a ErrorPayload<'a>) + Send
}

#[derive(Debug, Default)]
pub struct ResultHole {
    pub on_error: OnError,
}

impl ResultHole {
    #[must_use]
    pub fn new() -> Self {
        Self {
            on_error: OnError::new(),
        }
    }
    #[expect(dead_code, reason = "This is a static assert")]
    const fn must_be_send_and_sync() {
        const fn f<T: Send + Sync>() {}
        f::<Self>();
    }

    pub fn consume<T, E: Error>(&self, r: Result<T, E>) {
        self.inspect_impl(&r, false, false);
        drop(r);
    }

    pub fn consume_err<T, E: Error>(&self, r: Result<T, E>) -> Option<T> {
        self.inspect_impl(&r, true, false);
        r.ok()
    }

    pub fn inspect<T, E: Error>(&self, r: Result<T, E>) -> Result<T, E> {
        self.inspect_impl(&r, true, true);
        r
    }

    pub fn inspect_ref<'a, T, E: Error>(&'_ self, r: &'a Result<T, E>) -> &'a Result<T, E> {
        self.inspect_impl(r, true, true);
        r
    }

    fn inspect_impl<T, E: Error>(&self, r: &Result<T, E>, caller_may_use_ok: bool, caller_may_use_err: bool) {
        if let Err(e) = r.as_ref() {
            eprintln!("Discarded result: {e}");
            debugger::breakpoint!();
            _ = self.on_error.broadcast(&ErrorPayload {
                error: e,
                caller_may_use_ok,
                caller_may_use_err,
            });
        }
    }
}

pub static GLOBAL: LazyLock<ResultHole> = LazyLock::new(ResultHole::new);

pub fn consume<T, E: Error>(r: Result<T, E>) {
    GLOBAL.consume(r);
}

pub fn consume_err<T, E: Error>(r: Result<T, E>) -> Option<T> {
    GLOBAL.consume_err(r)
}

pub fn inspect<T, E: Error>(r: Result<T, E>) -> Result<T, E> {
    GLOBAL.inspect(r)
}

pub fn inspect_ref<T, E: Error>(r: &Result<T, E>) -> &Result<T, E> {
    GLOBAL.inspect_ref(r)
}
