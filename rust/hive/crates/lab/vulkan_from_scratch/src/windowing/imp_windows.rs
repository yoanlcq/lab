#![allow(clippy::wildcard_imports, reason = "Using the Win32 API is really impractical otherwise")]
#![allow(
    clippy::undocumented_unsafe_blocks,
    reason = "Using the Win32 API is necessarily an unsafe fest. Commenting every single one would add too much noise, actually harming \
              readability. Note that we are still able to re-enable this lint in specific places if we'd like"
)]
#![expect(unsafe_code, reason = "This is normal since we're using the Vulkan API")]

use alloc::sync::{Arc, Weak};
use core::sync::atomic::AtomicBool;
use std::io::Result;
use std::os::windows::ffi::OsStrExt;
use std::sync::Mutex;

use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::System::Threading::INFINITE;
use windows::Win32::UI::WindowsAndMessaging::*;

use super::{DisplayParams, WindowParams};
use weak_self::sync::WeakSelf;
use crate::windowing::{DisplayImpl, DisplayInner, PumpEventParams, PumpEventResult, WindowImpl, WindowInner};

trait Win32HandleType {}

impl Win32HandleType for HINSTANCE {}
impl Win32HandleType for HWND {}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Win32HandleWrapper<T: Win32HandleType>(pub T);

// TODO: I'm not certain about that. We should probably fix this another way
#[expect(
    clippy::non_send_fields_in_send_ty,
    reason = "Generated Win32 handle types use a *mut void, but we know the related APIs are thread-safe"
)]
unsafe impl<T: Win32HandleType> Send for Win32HandleWrapper<T> {}
unsafe impl<T: Win32HandleType> Sync for Win32HandleWrapper<T> {}

struct Win32HwndUserdata {
    weak_window: Weak<WindowInner>,
}

impl Win32HwndUserdata {
    fn set(hwnd: HWND, this: Option<Box<Self>>) {
        unsafe {
            let _previous_value = SetWindowLongPtrW(
                hwnd,
                GWLP_USERDATA,
                this.as_ref().map_or(0, |b| { 
                    let p: *const Self = core::ptr::from_ref(b);
                    p.addr().cast_signed()
                }),
            );
        };
        core::mem::forget(this);
    }
    fn get(hwnd: HWND) -> Option<Weak<WindowInner>> {
        unsafe {
            let ptr: *const Self = core::ptr::with_exposed_provenance(GetWindowLongPtrW(hwnd, GWLP_USERDATA).cast_unsigned());
            if ptr.is_null() {
                None
            } else {
                let this = core::ptr::read(ptr);
                let weak = this.weak_window.clone();
                core::mem::forget(this);
                Some(weak)
            }
        }
    }
    fn get_box(hwnd: HWND) -> Option<Box<Self>> {
        unsafe {
            let ptr: *mut Self = core::ptr::with_exposed_provenance_mut(GetWindowLongPtrW(hwnd, GWLP_USERDATA).cast_unsigned());
            if ptr.is_null() { None } else { Some(Box::from_raw(ptr)) }
        }
    }
}

extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CLOSE => {
            // TODO: consider asking the user before exit and calling DestroyWindow() only if confirmed
            if let Some(window) = Win32HwndUserdata::get(hwnd).and_then(|x| x.upgrade()) {
                let win32_window = Win32Window::from_window_inner(&window);
                result_hole::consume(win32_window.destroy());
                LRESULT(0)
            } else {
                eprintln!("{hwnd:?} received WM_CLOSE but we couldn't get its userdata");
                // By default, DefWindowProcW calls DestroyWindow() in response to WM_CLOSE
                unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
            }
        }
        WM_DESTROY => {
            if let Some(window) = Win32HwndUserdata::get(hwnd).and_then(|x| x.upgrade()) {
                let win32_window = Win32Window::from_window_inner(&window);
                win32_window.hwnd_is_destroyed.store(true, core::sync::atomic::Ordering::Relaxed);
                _ = window.post_destroy_confirmed.broadcast();
            }
            unsafe {
                // TODO: PostQuitMessage only if there are no more windows OR the app is willing to exit.
                // Several situations:
                // - Closing one of the windows is enough to close the app
                // - Closing all of the windows is enough to close the app
                // Then:
                // - Dispatch a global event on "request exit"
                PostQuitMessage(0);
            };

            // Free the memory
            // FIXME: But what if another thread is also calling Win32HwndUserdata::get at that exact time?
            // Should we just completely rework Win32HwndUserdata to replace it with a Rust-side HashMap lookup ?
            // We could still use GWLP_USERDATA to store an ID unique for the entire session
            drop(Win32HwndUserdata::get_box(hwnd));

            Win32HwndUserdata::set(hwnd, None);
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

#[derive(Debug)]
pub(super) struct Win32Display {
    weak_display: WeakSelf<DisplayInner>,
    hinstance: Win32HandleWrapper<HINSTANCE>,
    typical_window_class: Mutex<Weak<Win32WindowClass>>,
}

impl DisplayImpl for Win32Display {
    fn set_weak_display(&self, weak_display: Weak<DisplayInner>) {
        self.weak_display.init(weak_display);
    }
    fn pump_events(&self, params: &PumpEventParams) -> Option<PumpEventResult> {
        #[expect(clippy::cast_possible_truncation, reason = "Safe because we ensure the cast is within range")]
        #[expect(clippy::cast_sign_loss, reason = "Safe because Duration is always positive")]
        // Additional tricky note: INFINITE == u32::MAX, so make sure that even a high duration is clamped to the value just below that.
        let milliseconds = params.timeout.map_or(INFINITE, |d| f64::min(d.as_secs_f64() * 1000., f64::from(u32::MAX - 1)) as u32);
        unsafe {
            // https://stackoverflow.com/a/10866466/7972165
            // https://learn.microsoft.com/fr-fr/archive/blogs/larryosterman/things-you-shouldnt-do-part-4-msgwaitformultipleobjects-is-a-very-tricky-api
            (MsgWaitForMultipleObjects(None, false, milliseconds, QS_ALLINPUT) == WAIT_OBJECT_0).then(|| {
                let mut result = PumpEventResult::default();
                let mut msg = MSG::default();
                while params.max_events.is_none_or(|max_events| result.nb_received_events < max_events) {
                    if !bool::from(PeekMessageW(&raw mut msg, None, 0, 0, PM_REMOVE)) {
                        break;
                    }

                    let _is_translated = TranslateMessage(&raw const msg);
                    let _value_returned_by_wndproc = DispatchMessageW(&raw const msg);

                    result.nb_received_events += 1;
                    if msg.message == WM_QUIT {
                        result.exit_requested = true;
                    }
                }
                result
            })
        }
    }
    fn create_window(&self, params: &WindowParams) -> Result<Box<dyn WindowImpl>> {
        let &WindowParams { ref title, position, size } = params;
        let mut title_w: Vec<u16> = std::ffi::OsStr::new(title).encode_wide().collect();
        title_w.push(0);

        let class = {
            let mut class_lock = self
                .typical_window_class
                .lock()
                .map_err(|_poisoned| std::io::Error::other("typical_window_class mutex poisoned"))?;
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

        Win32HwndUserdata::set(hwnd, None);

        let window = Win32Window {
            weak_window: WeakSelf::new(),
            class,
            hwnd: Win32HandleWrapper(hwnd),
            hwnd_is_destroyed: AtomicBool::new(false),
        };

        Ok(Box::new(window))
    }
    fn hinstance(&self) -> HINSTANCE {
        self.hinstance.0
    }
}

impl Win32Display {
    pub(super) fn open(_params: &DisplayParams) -> Result<Self> {
        Ok(Self {
            // Question: maybe not as simple as this? https://stackoverflow.com/a/78906765
            // Options:
            // - Consider passing None instead to APIs where possible
            // - Consider allowing the caller to override this
            hinstance: Win32HandleWrapper(unsafe { GetModuleHandleW(None) }?.into()),
            typical_window_class: Mutex::new(Weak::new()),
            weak_display: WeakSelf::new(),
        })
    }

    fn register_window_class(&self) -> Result<Win32WindowClass> {
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

        Ok(Win32WindowClass {
            name,
            hinstance: self.hinstance,
        })
    }
}

#[derive(Debug)]
pub(super) struct Win32Window {
    #[expect(
        dead_code,
        reason = "The variable is not read but is really needed to manage the lifetime of the window class"
    )]
    class: Arc<Win32WindowClass>,
    hwnd: Win32HandleWrapper<HWND>,
    hwnd_is_destroyed: AtomicBool,
    weak_window: WeakSelf<WindowInner>,
}

impl Drop for Win32Window {
    fn drop(&mut self) {
        result_hole::consume(self.destroy());
    }
}

impl Win32Window {
    fn destroy(&self) -> Result<bool> {
        if self.hwnd_is_destroyed.load(core::sync::atomic::Ordering::Relaxed) {
            return Ok(false);
        }

        unsafe { DestroyWindow(self.hwnd.0) }?;
        Ok(true)
    }
    fn from_window_inner(window: &WindowInner) -> &Self {
        #[expect(clippy::expect_used, reason = "See the explanation below")]
        window.imp.as_any().downcast_ref::<Self>().expect("Anyone who calls this is within this module, therefore the only possible implementation for Window in that case is Win32Window")
    }
}

impl WindowImpl for Win32Window {
    fn set_weak_window(&self, weak_window: Weak<WindowInner>) {
        self.weak_window.init(weak_window.clone());
        Win32HwndUserdata::set(self.hwnd.0, Some(Box::new(Win32HwndUserdata { weak_window })));
    }
    fn hwnd(&self) -> HWND {
        self.hwnd.0
    }
    fn show(&self) -> Result<()> {
        let _was_previously_visible = unsafe { ShowWindow(self.hwnd.0, SW_SHOW) };
        Ok(())
    }
}

#[derive(Debug)]
struct Win32WindowClass {
    name: Vec<u16>,
    hinstance: Win32HandleWrapper<HINSTANCE>,
}

impl Drop for Win32WindowClass {
    fn drop(&mut self) {
        unsafe {
            result_hole::consume(UnregisterClassW(
                windows::core::PCWSTR::from_raw(self.name.as_ptr()),
                Some(self.hinstance.0),
            ));
        }
    }
}
