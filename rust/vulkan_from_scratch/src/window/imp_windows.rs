#![allow(clippy::wildcard_imports, reason = "Using the Win32 API is really impractical otherwise")]
#![allow(
    clippy::undocumented_unsafe_blocks,
    reason = "Using the Win32 API is necessarily an unsafe fest. Commenting every single one would add too much noise, actually harming \
              readability. Note that we are still able to re-enable this lint in specific places if we'd like"
)]

use core::ptr::NonNull;
use alloc::sync::{Arc, Weak};
use std::{io::Result, sync::Mutex};
use std::os::windows::ffi::OsStrExt;

use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use super::{DisplayParams, WindowParams};
use crate::result_hole;

struct WindowUserdata {
    window: alloc::sync::Weak<Mutex<WindowInner>>,
}

extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CLOSE => {
            unsafe {
                // TODO: consider asking the user before exit and calling DestroyWindow() only if
                // confirmed
                result_hole::add(DestroyWindow(hwnd));
            };
            LRESULT(0)
        }
        WM_DESTROY => {
            unsafe {
                let userdata_ptr: *mut WindowUserdata = core::ptr::without_provenance_mut(GetWindowLongPtrW(hwnd, GWLP_USERDATA).cast_unsigned());
                if let Some(userdata_ptr) = NonNull::new(userdata_ptr) {
                    let userdata = Box::from_raw(userdata_ptr.as_ptr());
                    if let Some(window) = userdata.window.upgrade() {
                        if let Ok(mut window_lock) = window.lock() {
                            window_lock.hwnd_is_destroyed = true;
                        }
                    }

                    SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
                }

                // TODO: PostQuitMessage only if there are no more windows OR the app is willing to exit.
                // Several situations:
                // - Closing one of the windows is enough to close the app
                // - Closing all of the windows is enough to close the app
                // Then:
                // - Dispatch a global event on "request exit"
                PostQuitMessage(0);
            };
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

#[derive(Debug)]
pub struct Display {
    hinstance: Win32Handle<HINSTANCE>,
    typical_window_class: Mutex<Weak<WindowClass>>,
}

#[derive(Debug)]
pub struct Window(Arc<Mutex<WindowInner>>);

#[derive(Debug)]
pub struct WindowInner {
    #[expect(
        dead_code,
        reason = "The variable is not read but is really needed to manage the lifetime of the window class"
    )]
    class: Arc<WindowClass>,
    hwnd: Win32Handle<HWND>,
    hwnd_is_destroyed: bool,
}

impl Drop for WindowInner {
    fn drop(&mut self) {
        if !self.hwnd_is_destroyed {
            unsafe { result_hole::add(DestroyWindow(self.hwnd.0)) }
        }
    }
}

impl Display {
    pub fn open(_params: &DisplayParams) -> Result<Self> {
        Ok(Self {
            // Question: maybe not as simple as this? https://stackoverflow.com/a/78906765
            // Options:
            // - Consider passing None instead to APIs where possible
            // - Consider allowing the caller to override this
            hinstance: Win32Handle(unsafe { GetModuleHandleW(None) }?.into()),
            typical_window_class: Mutex::new(Weak::new()),
        })
    }
    pub fn create_window(&self, params: &WindowParams) -> Result<Window> {
        let &WindowParams { ref title, position, size } = params;
        let mut title_w: Vec<u16> = std::ffi::OsStr::new(title).encode_wide().collect();
        title_w.push(0);

        let class = {
            let mut class_lock = self.typical_window_class.lock().map_err(|_poisoned| std::io::Error::other("typical_window_class mutex poisoned"))?;
            if let Some(class) = class_lock.upgrade() {
                class
            } else {
                let class = Arc::new(self.register_window_class()?);
                *class_lock = Arc::downgrade(&class);
                class
            }
        };

        let hwnd = unsafe {
            #[expect(
                clippy::cast_possible_truncation,
                reason = "Virtual desktop coordinates are still within float integer range, so casts are lossless. This will be fine \
                          until 2084 when we have 512k resolution screens"
            )]
            CreateWindowExW(
                WINDOW_EX_STYLE::default(), // extended style flags
                windows::core::PCWSTR::from_raw(class.name.as_ptr()),
                windows::core::PCWSTR::from_raw(title_w.as_ptr()),
                WS_OVERLAPPEDWINDOW, // style flags
                position.map_or(CW_USEDEFAULT, |p| p.x.floor() as _),
                position.map_or(CW_USEDEFAULT, |p| p.y.floor() as _),
                size.map_or(CW_USEDEFAULT, |p| p.w.ceil() as _),
                size.map_or(CW_USEDEFAULT, |p| p.h.ceil() as _),
                None, // parent window
                None, // menu
                Some(self.hinstance.0),
                None, // optional payload passed to WM_CREATE
            )
        }?;

        let inner = Arc::new(Mutex::new(WindowInner { class, hwnd: Win32Handle(hwnd), hwnd_is_destroyed: false }));
        let userdata_box = Box::new(WindowUserdata { window: Arc::downgrade(&inner) });
        unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, core::ptr::from_ref(Box::leak(userdata_box)).addr().cast_signed()); };

        Ok(Window(inner))
    }

    #[expect(clippy::unused_self, reason = "On other platforms it may make sense to take self")]
    pub fn main_event_loop(&self) {
        let mut msg = MSG::default();
        unsafe {
            while GetMessageW(&raw mut msg, None, 0, 0).into() {
                let _is_translated = TranslateMessage(&raw const msg);
                DispatchMessageW(&raw const msg);
            }
        }
    }

    fn register_window_class(&self) -> Result<WindowClass> {
        let name: Vec<u16> = std::ffi::OsStr::new("VulkanExperimentWindowClass\0").encode_wide().collect();
        let wndclass = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: self.hinstance.0,
            hIcon: HICON::default(),
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW) }?,
            hbrBackground: unsafe { GetSysColorBrush(COLOR_WINDOW) },
            lpszMenuName: windows::core::PCWSTR::null(),
            lpszClassName: windows::core::PCWSTR::from_raw(name.as_ptr()),
        };

        let atom = unsafe { RegisterClassW(&raw const wndclass) };
        if atom == 0 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(WindowClass {
            name,
            hinstance: self.hinstance,
        })
    }
}

impl Window {
    pub fn show(&self) {
        let window = self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let _was_previously_visible = unsafe { ShowWindow(window.hwnd.0, SW_SHOW) };
    }
}

#[derive(Debug)]
struct WindowClass {
    name: Vec<u16>,
    hinstance: Win32Handle<HINSTANCE>,
}

trait ThreadSafeWin32Handle {}

impl ThreadSafeWin32Handle for HINSTANCE {}
impl ThreadSafeWin32Handle for HWND {}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Win32Handle<T: ThreadSafeWin32Handle>(pub T);

// TODO: I'm not certain about that. We should probably fix this another way
#[expect(clippy::non_send_fields_in_send_ty, reason = "Generated Win32 handle types use a *mut void, but we know the related APIs are thread-safe")]
unsafe impl<T: ThreadSafeWin32Handle> Send for Win32Handle<T> {}
unsafe impl<T: ThreadSafeWin32Handle> Sync for Win32Handle<T> {}

impl Drop for WindowClass {
    fn drop(&mut self) {
        unsafe {
            result_hole::add(UnregisterClassW(
                windows::core::PCWSTR::from_raw(self.name.as_ptr()),
                Some(self.hinstance.0),
            ));
        }
    }
}
