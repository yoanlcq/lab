#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]
#![allow(clippy::missing_docs_in_private_items, reason = "This is a personal experiment")]
#![allow(clippy::cargo_common_metadata, reason = "This isn't meant to be published")]
#![allow(clippy::missing_errors_doc, reason = "This is a personal experiment")]
#![warn(clippy::expect_used)]
#![warn(clippy::unwrap_used)]

// restriction
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::allow_attributes)]
#![warn(clippy::allow_attributes_without_reason)]

extern crate ash;
extern crate ash_window;
extern crate raw_window_handle;
extern crate windows;

pub mod gpu;
pub mod result_hole;
pub mod window;

pub fn main() -> std::io::Result<()> {
    gpu::test();

    let display = window::Display::open(&window::DisplayParams {})?;
    let window0 = display.create_window(&window::WindowParams {})?;
    let window1 = display.create_window(&window::WindowParams {})?;
    window0.show();
    window1.show();
    display.main_event_loop();

    Ok(())
}
