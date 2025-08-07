#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::cargo_common_metadata)]
#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]

extern crate ash;
extern crate ash_window;
extern crate raw_window_handle;
extern crate windows;

pub mod gpu;
pub mod window;
pub mod discard_result;