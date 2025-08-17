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
use source_line_info::SourceLineInfo;

#[derive(Debug, Copy, Clone)]
pub struct ErrorPayload<'a> {
    pub error: &'a dyn Error,
    pub source_line_info: &'a SourceLineInfo,
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

    pub fn consume<T, E: Error>(&self, r: Result<T, E>, source_line_info: &SourceLineInfo) {
        self.inspect_impl(&r, source_line_info, false, false);
        drop(r);
    }

    pub fn consume_err<T, E: Error>(&self, r: Result<T, E>, source_line_info: &SourceLineInfo) -> Option<T> {
        self.inspect_impl(&r, source_line_info, true, false);
        r.ok()
    }

    pub fn inspect<T, E: Error>(&self, r: Result<T, E>, source_line_info: &SourceLineInfo) -> Result<T, E> {
        self.inspect_impl(&r, source_line_info, true, true);
        r
    }

    pub fn inspect_ref<'a, T, E: Error>(&'_ self, r: &'a Result<T, E>, source_line_info: &SourceLineInfo) -> &'a Result<T, E> {
        self.inspect_impl(r, source_line_info, true, true);
        r
    }

    fn inspect_impl<T, E: Error>(&self, r: &Result<T, E>, source_line_info: &SourceLineInfo, caller_may_use_ok: bool, caller_may_use_err: bool) {
        if let Err(error) = r.as_ref() {
            eprintln!("Discarded result at ({source_line_info}): {error}");
            debugger::breakpoint!();
            _ = self.on_error.broadcast(&ErrorPayload { error, source_line_info, caller_may_use_ok, caller_may_use_err });
        }
    }
}

pub static GLOBAL: LazyLock<ResultHole> = LazyLock::new(ResultHole::new);

#[doc(hidden)]
pub mod private_reexports {
    pub use source_line_info::source_line_info;
}

#[macro_export]
macro_rules! consume {
    ($result:expr) => { $crate::GLOBAL.consume($result, &$crate::private_reexports::source_line_info!()) };
    ($result_hole:expr, $result:expr) => { $result_hole.consume($result, $crate::private_reexports::source_line_info!()) };
}

#[macro_export]
macro_rules! consume_err {
    ($result:expr) => { $crate::GLOBAL.consume_err($result, &$crate::private_reexports::source_line_info!()) };
    ($result_hole:expr, $result:expr) => { $result_hole.consume_err($result, &$crate::private_reexports::source_line_info!()) };
}

#[macro_export]
macro_rules! inspect {
    ($result:expr) => { $crate::GLOBAL.inspect($result, &$crate::private_reexports::source_line_info!()) };
    ($result_hole:expr, $result:expr) => { $result_hole.inspect($result, &$crate::private_reexports::source_line_info!()) };
}

#[macro_export]
macro_rules! inspect_ref {
    ($result:expr) => { $crate::GLOBAL.inspect_ref($result, &$crate::private_reexports::source_line_info!()) };
    ($result_hole:expr, $result:expr) => { $result_hole.inspect_ref($result, &$crate::private_reexports::source_line_info!()) };
}