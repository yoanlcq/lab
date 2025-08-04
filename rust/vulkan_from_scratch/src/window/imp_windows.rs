use super::{DisplayParams, WindowParams};

use windows::{
    core::{w},
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::*,
    Win32::UI::WindowsAndMessaging::*,
};

extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        _ => unsafe {
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
    }
}

#[derive(Debug)]
pub struct Display {
    hinstance: HINSTANCE,
}

#[derive(Debug)]
pub struct Window {
    hwnd: HWND,
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { _ = CloseWindow(self.hwnd) }
    }
}

impl Display {
    pub fn open(_params: &DisplayParams) -> Result<Self, std::io::Error> {
        Ok(Self {
            // Question: maybe not as simple as this? https://stackoverflow.com/a/78906765
            // Options:
            // - Consider passing None instead to APIs where possible
            // - Consider allowing the caller to override this
            hinstance: unsafe { GetModuleHandleW(None) }?.into()
        })
    }
    pub fn create_window(&self, _params: &WindowParams) -> Result<Window, std::io::Error> {
        let window_class_name = w!("VulkanFromScratch");
        let window_title = w!("Window Title");

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
            lpszClassName: window_class_name,
        };

        let atom = unsafe { RegisterClassW(&wndclass) };
        if atom == 0 {
            return Err(std::io::Error::last_os_error());
        }

        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(), // extended style flags
                window_class_name,
                window_title,
                WS_OVERLAPPEDWINDOW, // style flags
                CW_USEDEFAULT, CW_USEDEFAULT, // x, y
                CW_USEDEFAULT, CW_USEDEFAULT, // width, height
                None, // parent window
                None, // menu
                Some(self.hinstance),
                None // optional payload passed to WM_CREATE
            )
        }?;

        Ok(Window {
            hwnd
        })
    }

    pub fn main_event_loop(&self) {
        let mut msg = MSG::default();
        unsafe {
            while GetMessageW(&mut msg, None, 0, 0).into() {
                _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}

impl Window {
    pub fn show(&self) {
        _ = unsafe { ShowWindow(self.hwnd, SW_SHOW) };
    }
}

