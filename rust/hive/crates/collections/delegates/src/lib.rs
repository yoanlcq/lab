//! A library for defining delegates.
//! 
//! For the purposes of this crate, a "delegate" is a container that holds one or more closures.
//! You typically "listen"/"subscribe" to a delegate by adding a closure to it; this returns an opaque handle that you can use to later unsubscribe.
//! 
//! In addition, a listener may choose to remove itself ("unsubscribe") from the delegate by returning `ListenerReply::Remove`, which is useful because listeners may be capturing a weak reference to some object that is conceptually the "receiver" and may expire.
//! 
//! This pattern (known as the "Observer" pattern) is incredibly common across many tech stacks. Examples includes:
//! - Unreal Engine's delegates (`DECLARE_DELEGATE()` macro and friends); this is the primary reference for this crate's implementations;
//! - C#'s events;
//! - JS's `addEventListener`/`removeEventListener`;
//! - etc.
//! 
//! Compared to asynchronous/deferred message passing and "event buses", delegates are strongly-typed and dispatching them always calls all listeners immediately.
//! 
//! "Dispatching immediately" is both a strength and weakness:
//! - pro: the full context of the dispatch (call stack + variables) is available when debugging the execution of any of the listeners or if they cause a panic or crash;
//! - con: one of the listeners may trigger a chain reaction that results in trying to mutate the delegate while it is being dispatched.
//!   This results in additional complexity and somewhat difficult design questions, such as: how to handle a listener that was added while the delegate is being dispatched.
//!   
//! 

pub mod multicast;
pub mod listener;

/// This module exists only so that macros from this crate work in other crates. Do not use it.
/// 
/// It is only public out of necessity.
#[doc(hidden)]
pub mod private_reexports {
    pub use pastey;
    pub use parking_lot;
}