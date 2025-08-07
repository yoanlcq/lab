#![allow(clippy::wildcard_imports, reason = "Using the Win32 API is really impractical otherwise")]

use std::cell::RefCell;
use std::io::Result;
use std::os::windows::ffi::OsStrExt;
use std::rc::Rc;

use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::w;

use super::{DisplayParams, WindowParams};
use crate::result_hole;

extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CLOSE => {
            unsafe {
                // TODO: consider asking the user before exit and calling DestroyWindow() only if
                // confirmed
                result_hole::add(DestroyWindow(hwnd));
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            unsafe {
                // TODO: PostQuitMessage only if there are no more windows OR the app is willing to exit.
                // Several situations:
                // - Closing one of the windows is enough to close the app
                // - Closing all of the windows is enough to close the app
                // Then:
                // - Dispatch a global event on "request exit"
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

#[derive(Debug)]
pub struct Display {
    hinstance: HINSTANCE,
    typical_window_class: RefCell<std::rc::Weak<WindowClass>>,
}

#[derive(Debug)]
pub struct Window {
    #[expect(dead_code, reason = "The variable is not read but is really needed to manage the lifetime of the window class")]
    class: Rc<WindowClass>,
    hwnd: HWND,
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { result_hole::add(DestroyWindow(self.hwnd)) }
    }
}

impl Display {
    pub fn open(_params: &DisplayParams) -> Result<Self> {
        Ok(Self {
            // Question: maybe not as simple as this? https://stackoverflow.com/a/78906765
            // Options:
            // - Consider passing None instead to APIs where possible
            // - Consider allowing the caller to override this
            hinstance: unsafe { GetModuleHandleW(None) }?.into(),
            typical_window_class: RefCell::new(std::rc::Weak::new()),
        })
    }
    pub fn create_window(&self, _params: &WindowParams) -> Result<Window> {
        let window_title = w!("Window Title");

        let class = {
            let mut class_lock = self.typical_window_class.borrow_mut();
            if let Some(class) = class_lock.upgrade() {
                class
            } else {
                let class = Rc::new(self.register_window_class()?);
                *class_lock = Rc::downgrade(&class);
                class
            }
        };

        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(), // extended style flags
                windows::core::PCWSTR::from_raw(class.name.as_ptr()),
                window_title,
                WS_OVERLAPPEDWINDOW, // style flags
                CW_USEDEFAULT,
                CW_USEDEFAULT, // x, y
                CW_USEDEFAULT,
                CW_USEDEFAULT, // width, height
                None,          // parent window
                None,          // menu
                Some(self.hinstance),
                None, // optional payload passed to WM_CREATE
            )
        }?;

        Ok(Window { class, hwnd })
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
        let name: Vec<u16> = std::ffi::OsStr::new("TODO_WindowClassName\0").encode_wide().collect();
        let wndclass = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: self.hinstance,
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
        let _was_previously_visible = unsafe { ShowWindow(self.hwnd, SW_SHOW) };
    }
}

#[derive(Debug)]
struct WindowClass {
    name: Vec<u16>,
    hinstance: HINSTANCE,
}

impl Drop for WindowClass {
    fn drop(&mut self) {
        unsafe {
            result_hole::add(UnregisterClassW(
                windows::core::PCWSTR::from_raw(self.name.as_ptr()),
                Some(self.hinstance),
            ));
        }
    }
}
