//! This crate exists to loosen our relationship with any particular tracing service.
//! 
//! All code in this workspace should use+extend this instead of relying on any concrete tracing service.

#![allow(clippy::multiple_crate_versions, reason = "It's for regex-automata and regex-syntax. The 'loom' crate (indirectly referenced by tracy-client) depends on it, but we really never use it. Clippy complains because it conservatively assumes we have enabled cfg(loom)")]

pub struct TracingManager {
    tracy_client: tracy_client::Client,
}

impl TracingManager {
    #[must_use]
    pub fn start() -> Self {
        Self {
            tracy_client: tracy_client::Client::start(),
        }
    }
    pub fn start_first_frame(&self) {
        self.tracy_client.color_message("start_first_frame", 0xff0000, 0);
    }
    pub fn end_frame(&self) {
        self.tracy_client.frame_mark();
    }
}

pub struct Span {
    tracy_span: tracy_client::Span,
}

impl Span {
    #[must_use]
    pub const fn from_tracy(tracy_span: tracy_client::Span) -> Self {
        Self { tracy_span }
    }
    // The color is specified as RGB. It is most straightforward to specify them as hex literals such as 0xFF0000 for red, 0x00FF00 for green or 0x0000FF for blue.
    // The value 0 is reserved as meaning "no color specified". This is true for this crate, but this is also how Tracy handles it.
    #[must_use]
    pub fn with_color(mut self, color: u32) -> Self {
        self.set_color(color);
        self
    }
    /// See `with_color()`
    pub fn set_color(&mut self, color: u32) {
        self.tracy_span.emit_color(color);
    }
}

#[doc(hidden)]
pub mod private_reexports {
    pub use tracy_client;
}

#[macro_export]
macro_rules! span {
    ($name: expr) => {
        $crate::Span::from_tracy($crate::private_reexports::tracy_client::span!($name))
    };
}