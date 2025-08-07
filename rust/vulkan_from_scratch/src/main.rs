// TODO: cargo rustfmt
// TODO: cargo typos
// TODO: cargo deny?

#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::cargo_common_metadata)]

use vulkan_from_scratch_lib::{gpu, window};

fn main() -> Result<(), std::io::Error> {
    gpu::test();

    let display = window::Display::open(&window::DisplayParams {})?;
    let window0 = display.create_window(&window::WindowParams {})?;
    let window1 = display.create_window(&window::WindowParams {})?;
    window0.show();
    window1.show();
    display.main_event_loop();

    Ok(())
}
