use alloc::sync::{Arc, Weak};
use delegates::multicast::SimpleSharedMulticastDelegate;
use core::fmt::Debug;
use core::sync::atomic::AtomicU32;
use std::io::Result;
use core::time::Duration;

use vek::{Extent2, Vec2};

use as_any::AsAny;

#[cfg(windows)]
mod imp_windows;

#[derive(Debug, Clone)]
pub struct DisplayArc(pub Arc<DisplayInner>);

#[derive(Debug)]
pub struct DisplayInner {
    imp: Box<dyn DisplayImpl>,
}

#[expect(
    clippy::empty_structs_with_brackets,
    reason = "This will have fields for other platforms; It just so happens that Windows doesn't need any"
)]
#[derive(Debug)]
pub struct DisplayParams {}

trait DisplayImpl: Debug + Send + Sync + AsAny {
    fn set_weak_display(&self, weak_display: Weak<DisplayInner>);

    fn create_window(&self, params: &WindowParams) -> Result<Box<dyn WindowImpl>>;

    fn pump_events(&self, params: &PumpEventParams) -> Option<PumpEventResult>;

    #[cfg(windows)]
    fn hinstance(&self) -> windows::Win32::Foundation::HINSTANCE;
}

impl DisplayArc {
    pub fn open(params: &DisplayParams) -> Result<Self> {
        let arc = Arc::new(DisplayInner {
            #[cfg(windows)]
            imp: Box::new(imp_windows::Win32Display::open(params)?),
        });
        arc.imp.set_weak_display(Arc::downgrade(&arc));
        Ok(Self(arc))
    }
    pub fn create_window(&self, params: &WindowParams) -> Result<WindowArc> {
        let arc = Arc::new(WindowInner {
            imp: self.0.imp.create_window(params)?,
            display: self.clone(),
            id: WindowID::new(),
            pre_destroy_requested: SimpleSharedMulticastDelegate::new(),
            post_destroy_confirmed: SimpleSharedMulticastDelegate::new(),
        });
        arc.imp.set_weak_window(Arc::downgrade(&arc));
        Ok(WindowArc(arc))
    }
    #[must_use]
    pub fn pump_events(&self, params: &PumpEventParams) -> Option<PumpEventResult> {
        self.0.imp.pump_events(params)
    }
    #[cfg(windows)]
    #[must_use]
    pub fn hinstance(&self) -> windows::Win32::Foundation::HINSTANCE {
        self.0.imp.hinstance()
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct PumpEventParams {
    pub timeout: Option<Duration>,
    pub max_events: Option<usize>,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct PumpEventResult {
    pub nb_received_events: usize,
    pub exit_requested: bool,
}

#[derive(Debug)]
pub struct WindowArc(pub Arc<WindowInner>);

#[derive(Debug)]
pub struct WindowInner {
    imp: Box<dyn WindowImpl>,
    display: DisplayArc,
    id: WindowID,
    pub pre_destroy_requested: SimpleSharedMulticastDelegate,
    pub post_destroy_confirmed: SimpleSharedMulticastDelegate,
}

#[derive(Debug)]
pub struct WindowParams {
    pub title: String,
    pub position: Option<Vec2<f64>>,
    pub size: Option<Extent2<f64>>,
}

trait WindowImpl: Debug + Send + Sync + AsAny {
    fn set_weak_window(&self, weak_window: Weak<WindowInner>);

    fn show(&self) -> Result<()>;

    #[cfg(windows)]
    fn hwnd(&self) -> windows::Win32::Foundation::HWND;
}

impl WindowArc {
    #[must_use]
    pub fn display(&self) -> DisplayArc {
        self.0.display.clone()
    }
    pub fn show(&self) -> Result<()> {
        self.0.imp.show()
    }
    #[must_use]
    pub fn id(&self) -> WindowID {
        self.0.id
    }
    #[must_use]
    #[cfg(windows)]
    pub fn hwnd(&self) -> windows::Win32::Foundation::HWND {
        self.0.imp.hwnd()
    }
}

impl Drop for WindowInner {
    fn drop(&mut self) {
        self.pre_destroy_requested.broadcast();
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WindowID(u32);

static LAST_WINDOW_ID: AtomicU32 = AtomicU32::new(0);

impl WindowID {
    fn new() -> Self {
        Self(LAST_WINDOW_ID.fetch_add(1, core::sync::atomic::Ordering::Relaxed))
    }
}
