//! Nuke - An idiomatic Nuklear binding you can't accidentally misuse.
//!
//! # Is it stable yet ?
//! No. Once the API is fully fleshed out and tested, I want it to be rewieved by more
//! experienced Rustaceans and make breaking changes.  
//! Currently, you should use Nuke only if you are not afraid of breaking changes.  
//!
//! Nuke targets the latest stable, beta and nightly Rust.
//!
//! # About Nuklear
//!
//! Nuklear is a state-of-the-art immediate-mode GUI toolkit for ANSI C 
//! developed by Micha Mettke and released under public domain ([link to the GitHub repo][1])
//!
//! [1]: https://github.com/vurtun/nuklear
//!
//! One of the main selling points of Nuklear is that is has no dependencies (not even libc).
//! That means you can use it with the input and rendering backends of your choice - all you
//! have to do is, each frame, convert user input to a format that Nuklear understands, then 
//! build your GUI, then pull back "draw commands" to feed them to your rendering backend.  
//! Once you've done this, you've unlocked a rich API for your various GUI needs.
//!
//! I's worth noting that Nuklear is powerful enough to look good even for in-game UIs, however it
//! doesn't support 3D "out-of-the-box" - but that can be achieved with some extra work on your part
//! (by writing custom widgets and establishing a mapping between your 3D world and Nuklear's internal
//!  discrete 2D world).
//! TODO need screenshots !
//!
//! # Extending Nuke
//! Nuke limits itself to the basic building blocks provided by Nuklear.
//! Anything that can be directly implemented on top of Nuke _and_ "looks pretty standard" should
//! be put in a separate crate which name starts with "nuke-".
//! Here are "official" friend crates (TODO) :
//! - nuke-styles
//! - nuke-icons
//! - nuke-file-browser
//! - nuke-sdl2
//! - Other ideas : curve editors, better text editors, better color pickers, 2D pad, 3D items...
//!
//! # Reporting issues
//! Although Nuke strives for quality, it's hard to guarantee a complete absence of bugs
//! when there's unsafe code all around within the implementation.  
//!
//! If you come up with seemingly safe and correct code which actually ends up triggering Nuklear's 
//! internal assertions, segfaulting, or making it even possible to misuse the API, then **please**
//! report an issue so we can fix it ASAP.  
//!
//! Likewise, if you are an experienced Rust dev and you find that some parts of the API
//! are not as idiomatic as they should be, are conceptually incorrect (like ownership/borrowing
//! issues), are missing `derive`s, or anything else, please report an issue
//! so we can dicuss this and possibly change the API.
//!
//! # Getting started
//!
//! TODO this section
//!
//! ```
//! extern crate nuke as nk;
//! #
//! # const FRAME_COUNT: u64 = 512;
//!
//! fn main() {
//!     // TODO: where does the font come from ?
//!     let ctx = nk::Context::from(font);
//! #   let frameno: u64 = 1;
//!     'running: loop {
//! #       if frameno > FRAME_COUNT {
//! #            break 'running;
//! #       }
//! #       frameno += 1;
//!         // Keep in mind you can choose not to `chain` the steps like I did here.
//!         ctx.input(|api| {
//!             // Feed user input to Nuklear by using `api`.
//!         }).gui(|api| {
//!             // Build your GUI using `api`
//!         }).draw(|commands| {
//!             for cmd in commands {
//!                 // Execute the command using your own backend.
//!             }
//!         }).clear(); 
//!         // You can omit that last call to `clear()` since the `ClearStep` object
//!         // automatically does it when dropped.
//!     }
//! }
//! ```
//!
//! # Why not `nuklear-rust` or `nuklear-rs` ?
//! ## Better usability
//! Nuklear was written in C, solving problems for C.  
//! There, the `userdata` pointers, `_begin()`/`_end()`-style APIs and others are the correct way
//! to solve problems. However, in Rust, these are mistakes, and IMO the existing bindings
//! repeated these without noticing.  
//! As a result, what looks life safe code might be 
//! completely incorrect, such that you might as well call into FFI directly and refer to 
//! Nuklear's doc instead.  
//! 
//! In Rust, we have many ways to enforce correctness at compile-time, and it's part of the
//! Rust experience that "if it compiles, it just works" - which is what Nuke strives to achieve.  
//! For instance, Nuklear requires that the following operations be
//! executed in this precise order in the app's main loop:
//! 1. Feed user input to Nuklear (a.k.a "Mirroring" user input);
//! 2. Build the GUI;
//! 3. Pull "draw commands" from the GUI's state (optionally feeding them to your rendering backend);
//! 4. Clear the context object (which reset Nuklear's input, gui, and draw-command states);
//! 5. End of frame - Go back to 1.
//!
//! However it's too easy to mess up the order for these steps - we can't assume that the user was
//! patient enough to read that far through the doc.  
//! Nuke solves this with specific "Step" objects that enforce the correct ordering of operations.  
//! The `begin()`-`end()` problem is solved by closures, and the `userdata` problem is solved 
//! either with trait objects or with the environnment captured by closures.
//!
//! ## Better discoverability
//! Nuke makes it easy to learn "what to do next", especially if you have 
//! fuzzy code completion. Start by creating a `Context`, and look at what you can do with it.
//! 
//! ## Better documentation
//! Nuklear itself is quite well documented, and so an idiomatic
//! binding to it should be (if not more so).
//! 
//! ## Support for `#![no_std]` 
//! Thanks to Nuklear's extreme protability, Nuke builds on `#[no_std]` when its
//! `use_std` feature is disabled (it's enabled by default).
//!
//! # Why not a complete rewrite ?
//! It's true that Nuklear :
//! - Doesn't guarantee safety: this is expected from a C library, but as a result it's tedious
//!   to make Nuke bug-free;
//! - Isn't "concurrent": for instance, it's impossible to build the GUI and read back draw 
//!   commands at the same time in a concurrent fashion (as a Producer-Consumer model).
//! - Is excessively defensive: in particular, most Nuklear functions check if any
//!   of their pointer arguments is NULL and return early if it's the case (even if there
//!   are appropriate assertions before the checks). 
//!   It's worrying that every function call is an implicit branch (think about consoles), 
//!   and even more so when we know that there's no such thing as NULL in idiomatic Rust;
//! - Has some cruft in it: There's quite a bunch of Nuklear functions that get compiled in but
//!   aren't used by Nuke. Link-Time-Optimizations might take care of this, but I'm not sure;
//! - Does not "natively" support 3D: This makes Nuklear fast and easy to use and integrate with 
//!   most rendering backends, but it's a pity that such a good API can't be used for 3D GUIs,
//!   which become more and more needed because of VR and expectations from most modern games.
//!   You might end up having to manage two different GUI APIs (one for you editor, the other for your 
//!   in-game GUI), which is not convenient.
//!   Nuklear is extensible enough to allow the user to implement that on top of it, but it's 
//!   tedious;
//! - Writing a safe Rust wrapper around Nuklear does take some time, and maybe not much more than 
//!   a rewrite.
//! - Is public domain: we could grab its code, Rewrite It In Rust™, possibly coming up with an
//!   even better API.
//!
//! For now, Nuklear is big and complex enough to dissuade me from rewriting it in Rust
//! (in particular, the font, window and layout APIs, as well as all of the widgets).
//! Plus, despite the aforementioned flaws (which do not appear to be so much of a concern to users), 
//! Nuklear is a very high-quality API as it is.
//! 
//! However, Nuke might one day be implemented as a backward-compatible, complete rewrite of 
//! Nuklear in Rust.

// TODO change this
// TODO Follow https://github.com/rust-lang/rfcs/blob/master/text/0505-api-comment-conventions.md
#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://doc.rust-lang.org/")]
#![doc(test(attr(deny(warnings))))]
#![cfg_attr(not(feature="use_std"), no_std)]
#![deny(missing_docs)]

#[cfg(feature="use_std")]
extern crate core;
// NOT feature-gated with "use_libc": needed at least for its types.
extern crate libc;
extern crate nuke_sys;

pub mod allocator;
pub use allocator::*;
pub mod context;
pub use context::*;
pub mod input;
pub use input::*;
pub mod gui;
pub use gui::*;
pub mod draw;
pub use draw::*;
pub mod clear;
pub use clear::*;
pub mod style;
pub use style::*;
pub mod color;
pub use color::*;
pub mod math;
pub use math::*;
pub mod font;
pub use font::*;
pub mod buffer;
pub use buffer::*;
pub mod string;
pub use string::*;
pub mod textedit;
pub use textedit::*;
