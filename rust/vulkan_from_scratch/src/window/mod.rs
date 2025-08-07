use std::io::Result;

#[cfg(windows)]
mod imp_windows;

#[derive(Debug)]
pub struct Display {
    #[cfg(windows)]
    imp: imp_windows::Display,
}

#[derive(Debug)]
pub struct DisplayParams {}

#[derive(Debug)]
pub struct Window {
    #[cfg(windows)]
    imp: imp_windows::Window,
}

#[derive(Debug)]
pub struct WindowParams {}

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
