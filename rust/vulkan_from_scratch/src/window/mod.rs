use std::io::Result;

use vek::{Extent2, Vec2};

#[cfg(windows)]
mod imp_windows;

#[derive(Debug)]
pub struct Display {
    #[cfg(windows)]
    imp: imp_windows::Display,
}

#[expect(
    clippy::empty_structs_with_brackets,
    reason = "This will have fields for other platforms; It just so happens that Windows doesn't need any"
)]
#[derive(Debug)]
pub struct DisplayParams {}

#[derive(Debug)]
pub struct Window {
    #[cfg(windows)]
    imp: imp_windows::Window,
}

#[expect(
    clippy::module_name_repetitions,
    reason = "WindowParams refers to Window as the software construct, not the module"
)]
#[derive(Debug)]
pub struct WindowParams {
    pub title: String,
    pub position: Option<Vec2<f64>>,
    pub size: Option<Extent2<f64>>,
}

impl Display {
    pub fn open(params: &DisplayParams) -> Result<Self> {
        Ok(Self {
            #[cfg(windows)]
            imp: imp_windows::Display::open(params)?,
        })
    }
    pub fn create_window(&self, params: &WindowParams) -> Result<Window> {
        Ok(Window {
            #[cfg(windows)]
            imp: self.imp.create_window(params)?,
        })
    }
    pub fn main_event_loop(&self) {
        self.imp.main_event_loop();
    }
}

impl Window {
    pub fn show(&self) {
        self.imp.show();
    }
}
