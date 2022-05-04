// TODO:
// - How fast can I draw into it? Idk. But I don't care either.
// - Can I put text in it? Yes. See the exemple in CreateFont()
// - Can I render OpenGL in it? Yes, see DWM

// Steps:
// - RegisterClass
// - CreateWindowEx
// - Load ARGB image from memory (with premultiplied alpha)
//   Create matching BITMAPINFO (easy)
// - Create a memory HDC, create and select a new empty bitmap with appropriate size
//   Then render the image into the bitmap via memory HDC using StretchDIBits.
//   At the last moment, we flip the height in bitmapinfo so that the image is flipped
//   on the Y-axis (otherwise, the origin is bottom-left).
//   We can't count on StretchDIBits to directly support PNG and JPEG; This is actually
//   intended for printer HDCs, not memory HDCs.
// - Center the window within the work area of whichever monitor the window was
//   created in.
// - Call UpdateLayeredWindow() which sets the position, size and content of the window.
//   The content is maintained and drawing is managed by the system, meaning we don't
//   receive WM_PAINT events anymore.
//   UpdateLayeredWindows() _also_ sets the _shape_ of the window, that is the pixels within
//   which the cursor is treated as being "within" the window. Fully transparent pixels are
//   treated as "not part of the window" and therefore truly act like holes to other underlying
//   windows.
// - Finally, show the window.
// - Then, enter the message loop.
// - When we're done, free resources as needed.

#![windows_subsystem = "windows"]

extern crate winapi;

use std::mem;
use std::ptr;
use std::os::raw::*;
use std::ffi::CStr;
use std::time::Instant;

#[allow(unused_imports)]
use winapi::{
    shared::{windef::*, minwindef::*, windowsx::*, winerror::*},
    um::{winuser::{self, *}, wingdi::*, libloaderapi::*, errhandlingapi::*, gl::gl::*, dwmapi::*,},
};

use self::splash_gl::*;


#[allow(dead_code)]
#[allow(non_upper_case_globals)]
mod splash_gl {
    use super::*;

    // NOTE: Not everything's in it. Additions welcome!

    pub const WGL_CONTEXT_DEBUG_BIT_ARB: c_int =         0x00000001;
    pub const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: c_int = 0x00000002;
    pub const WGL_CONTEXT_MAJOR_VERSION_ARB: c_int =     0x2091;
    pub const WGL_CONTEXT_MINOR_VERSION_ARB: c_int =     0x2092;
    pub const WGL_CONTEXT_LAYER_PLANE_ARB: c_int =       0x2093;
    pub const WGL_CONTEXT_FLAGS_ARB: c_int =             0x2094;
    pub const ERROR_INVALID_VERSION_ARB: c_int =         0x2095;
    pub const WGL_CONTEXT_OPENGL_NO_ERROR_ARB: c_int =   0x31B3;
    pub const WGL_CONTEXT_PROFILE_MASK_ARB: c_int =      0x9126;
    pub const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: c_int =  0x00000001;
    pub const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: c_int = 0x00000002;
    pub const ERROR_INVALID_PROFILE_ARB: c_int =         0x2096;
    pub const WGL_CONTEXT_ROBUST_ACCESS_BIT_ARB: c_int = 0x00000004;
    pub const WGL_LOSE_CONTEXT_ON_RESET_ARB: c_int =     0x8252;
    pub const WGL_CONTEXT_RESET_NOTIFICATION_STRATEGY_ARB: c_int = 0x8256;
    pub const WGL_NO_RESET_NOTIFICATION_ARB: c_int =     0x8261;
    pub const WGL_FRAMEBUFFER_SRGB_CAPABLE_ARB: c_int =  0x20A9;

    pub const WGL_SAMPLE_BUFFERS_ARB: c_int =            0x2041;
    pub const WGL_SAMPLES_ARB: c_int =                   0x2042;

    pub const WGL_NUMBER_PIXEL_FORMATS_ARB: c_int =      0x2000;
    pub const WGL_DRAW_TO_WINDOW_ARB: c_int =            0x2001;
    pub const WGL_DRAW_TO_BITMAP_ARB: c_int =            0x2002;
    pub const WGL_ACCELERATION_ARB: c_int =              0x2003;
    pub const WGL_NEED_PALETTE_ARB: c_int =              0x2004;
    pub const WGL_NEED_SYSTEM_PALETTE_ARB: c_int =       0x2005;
    pub const WGL_SWAP_LAYER_BUFFERS_ARB: c_int =        0x2006;
    pub const WGL_SWAP_METHOD_ARB: c_int =               0x2007;
    pub const WGL_NUMBER_OVERLAYS_ARB: c_int =           0x2008;
    pub const WGL_NUMBER_UNDERLAYS_ARB: c_int =          0x2009;
    pub const WGL_TRANSPARENT_ARB: c_int =               0x200A;
    pub const WGL_TRANSPARENT_RED_VALUE_ARB: c_int =     0x2037;
    pub const WGL_TRANSPARENT_GREEN_VALUE_ARB: c_int =   0x2038;
    pub const WGL_TRANSPARENT_BLUE_VALUE_ARB: c_int =    0x2039;
    pub const WGL_TRANSPARENT_ALPHA_VALUE_ARB: c_int =   0x203A;
    pub const WGL_TRANSPARENT_INDEX_VALUE_ARB: c_int =   0x203B;
    pub const WGL_SHARE_DEPTH_ARB: c_int =               0x200C;
    pub const WGL_SHARE_STENCIL_ARB: c_int =             0x200D;
    pub const WGL_SHARE_ACCUM_ARB: c_int =               0x200E;
    pub const WGL_SUPPORT_GDI_ARB: c_int =               0x200F;
    pub const WGL_SUPPORT_OPENGL_ARB: c_int =            0x2010;
    pub const WGL_DOUBLE_BUFFER_ARB: c_int =             0x2011;
    pub const WGL_STEREO_ARB: c_int =                    0x2012;
    pub const WGL_PIXEL_TYPE_ARB: c_int =                0x2013;
    pub const WGL_COLOR_BITS_ARB: c_int =                0x2014;
    pub const WGL_RED_BITS_ARB: c_int =                  0x2015;
    pub const WGL_RED_SHIFT_ARB: c_int =                 0x2016;
    pub const WGL_GREEN_BITS_ARB: c_int =                0x2017;
    pub const WGL_GREEN_SHIFT_ARB: c_int =               0x2018;
    pub const WGL_BLUE_BITS_ARB: c_int =                 0x2019;
    pub const WGL_BLUE_SHIFT_ARB: c_int =                0x201A;
    pub const WGL_ALPHA_BITS_ARB: c_int =                0x201B;
    pub const WGL_ALPHA_SHIFT_ARB: c_int =               0x201C;
    pub const WGL_ACCUM_BITS_ARB: c_int =                0x201D;
    pub const WGL_ACCUM_RED_BITS_ARB: c_int =            0x201E;
    pub const WGL_ACCUM_GREEN_BITS_ARB: c_int =          0x201F;
    pub const WGL_ACCUM_BLUE_BITS_ARB: c_int =           0x2020;
    pub const WGL_ACCUM_ALPHA_BITS_ARB: c_int =          0x2021;
    pub const WGL_DEPTH_BITS_ARB: c_int =                0x2022;
    pub const WGL_STENCIL_BITS_ARB: c_int =              0x2023;
    pub const WGL_AUX_BUFFERS_ARB: c_int =               0x2024;
    pub const WGL_NO_ACCELERATION_ARB: c_int =           0x2025;
    pub const WGL_GENERIC_ACCELERATION_ARB: c_int =      0x2026;
    pub const WGL_FULL_ACCELERATION_ARB: c_int =         0x2027;
    pub const WGL_SWAP_EXCHANGE_ARB: c_int =             0x2028;
    pub const WGL_SWAP_COPY_ARB: c_int =                 0x2029;
    pub const WGL_SWAP_UNDEFINED_ARB: c_int =            0x202A;
    pub const WGL_TYPE_RGBA_ARB: c_int =                 0x202B;
    pub const WGL_TYPE_COLORINDEX_ARB: c_int =           0x202C;

    pub const WGL_TYPE_RGBA_FLOAT_ARB: c_int =           0x21A0;

    pub const WGL_FRAMEBUFFER_SRGB_CAPABLE_EXT: c_int =   0x20A9;

    pub const WGL_DEPTH_FLOAT_EXT: c_int =                0x2040;

    pub const WGL_CONTEXT_ES_PROFILE_BIT_EXT: c_int =     0x00000004;
    pub const WGL_CONTEXT_ES2_PROFILE_BIT_EXT: c_int =    0x00000004;

    pub const WGL_COLORSPACE_EXT: c_int =                 0x3087;
    pub const WGL_COLORSPACE_SRGB_EXT: c_int =            0x3089;
    pub const WGL_COLORSPACE_LINEAR_EXT: c_int =          0x308A;

    pub const GL_TRIANGLES: GLenum = 4;
    pub const GL_DEPTH_BUFFER_BIT: GLbitfield = 256;
    pub const GL_COLOR_BUFFER_BIT: GLbitfield = 16384;

    pub static mut wglGetExtensionsStringARB: Option<extern "system" fn(HDC) -> *const c_char> = None;
    pub static mut wglCreateContextAttribsARB: Option<extern "system" fn(HDC, HGLRC, *const c_int) -> HGLRC> = None;
    pub static mut wglChoosePixelFormatARB: Option<extern "system" fn(HDC, *const c_int, *const f32, UINT, *mut c_int, *mut UINT) -> BOOL> = None;
    pub static mut wglSwapIntervalEXT: Option<extern "system" fn(c_int) -> BOOL> = None;
    pub static mut wglGetSwapIntervalEXT: Option<extern "system" fn() -> c_int> = None;

    #[link(name = "opengl32", kind = "dylib")]
    extern "system" { // Is this the correct calling convention ??
        pub fn glBegin(_: GLenum);
        pub fn glEnd();
        pub fn glClear(_: GLenum);
        pub fn glClearColor(_: f32, _: f32, _: f32, _: f32);
        pub fn glVertex2f(_: f32, _: f32);
        pub fn glVertex3f(_: f32, _: f32, _: f32);
        pub fn glVertex4f(_: f32, _: f32, _: f32, _: f32);
        pub fn glColor4f(_: f32, _: f32, _: f32, _: f32);
        pub fn glGetString(_: GLenum) -> *const c_char;
        pub fn glViewport(x: GLint, y: GLint, w: GLsizei, h: GLsizei);
        pub fn glEnable(_: GLenum);
        pub fn glDisable(_: GLenum);
        pub fn glBlendFunc(_: GLenum, _: GLenum);
    }

    pub static GL_TRUE: GLboolean = 1;
    pub static GL_FALSE: GLboolean = 0;

    pub const GL_VENDOR     : GLenum = 0x1F00;
    pub const GL_RENDERER   : GLenum = 0x1F01;
    pub const GL_VERSION    : GLenum = 0x1F02;
    pub const GL_EXTENSIONS : GLenum = 0x1F03;

    pub const GL_BLEND               : GLenum = 0x0BE2;

    pub const GL_SRC_COLOR           : GLenum = 0x0300;
    pub const GL_ONE_MINUS_SRC_COLOR : GLenum = 0x0301;
    pub const GL_SRC_ALPHA           : GLenum = 0x0302;
    pub const GL_ONE_MINUS_SRC_ALPHA : GLenum = 0x0303;
    pub const GL_DST_ALPHA           : GLenum = 0x0304;
    pub const GL_ONE_MINUS_DST_ALPHA : GLenum = 0x0305;
    pub const GL_DST_COLOR           : GLenum = 0x0306;
    pub const GL_ONE_MINUS_DST_COLOR : GLenum = 0x0307;
    pub const GL_SRC_ALPHA_SATURATE  : GLenum = 0x0308;
}

static WINDOW_CLASS_NAME: &[u16] = &['_' as _, 0];
const W: u32 = 512;
const H: u32 = 512;
static TEST_GL: bool = true;
static mut APP: Option<App> = None;

// TODO: PR winapi-rs
pub const DWM_BB_ENABLE: DWORD = 1;
pub const DWM_BB_BLURREGION: DWORD = 2;
pub const DWM_BB_TRANSITIONONMAXIMIZED: DWORD = 4;

#[link(name = "dwmapi", kind = "dylib")]
extern "system" {
    pub fn DwmIsCompositionEnabled(_: *mut BOOL) -> HRESULT;
}


fn print_gl_stuff() {
    unsafe {
        println!("GL_VERSION: {}", CStr::from_ptr(glGetString(GL_VERSION)).to_string_lossy());
        println!("GL_RENDERER: {}", CStr::from_ptr(glGetString(GL_RENDERER)).to_string_lossy());
        println!("GL_VENDOR: {}", CStr::from_ptr(glGetString(GL_VENDOR)).to_string_lossy());
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, umsg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match umsg {
        WM_CLOSE => {
            APP.as_mut().unwrap().close_requested = true;
            0 // zero if processed
        },
        WM_ACTIVATE => {
            DefWindowProcW(hwnd, umsg, wparam, lparam) // Generates WM_MOUSEACTIVATE, IFN
        },
        WM_MOUSEACTIVATE => {
            MA_ACTIVATE as _ // Yes, we would like to be activated when we're clicked on
        },
        WM_KEYDOWN => {
            let app = APP.as_mut().unwrap();
            let vkeycode = wparam as i32;
            match vkeycode {
                VK_LEFT => {
                    app.alpha = app.alpha.saturating_sub(1);
                    println!("Alpha: {}", app.alpha);
                },
                VK_RIGHT => {
                    app.alpha = app.alpha.saturating_add(1);
                    println!("Alpha: {}", app.alpha);
                },
                VK_ESCAPE => {
                    PostMessageW(hwnd, WM_CLOSE, 0, 0);
                },
                _ => (),
            };
            0
        },
        WM_SIZING if TEST_GL => {
            // It's pointless to try to redraw here.
            1 // TRUE if processed
        },
        WM_ERASEBKGND if TEST_GL => {
            /*
            // Worth redrawing, even though I wonder if it's redundant with WM_PAINT
            if let Some(app) = APP.as_mut() {
                app.update_window(0.);
            }
            */
            1 // non-zero if processed
        },
        WM_SIZE if TEST_GL => {
            /*
            // Worth redrawing. Happens when the user temporarily stops dragging
            if let Some(app) = APP.as_mut() {
                app.update_window(0.);
            }
            */
            0 // zero if processed
        },
        WM_PAINT => {
            // println!("Handling WM_PAINT");
            if !TEST_GL {
                let mut ps = mem::zeroed();
                let _hdc = BeginPaint(hwnd, &mut ps);
                EndPaint(hwnd, &mut ps);
                0 // zero if processed
            } else {
                /*
                if let Some(app) = APP.as_mut() {
                    app.update_window(0.);
                }
                */
                0
            }
        },
        _ => DefWindowProcW(hwnd, umsg, wparam, lparam),
    }
}

pub struct App {
    // Common
    pub hinstance: HINSTANCE,
    pub class_atom: ATOM,
    pub hwnd: HWND,
    pub close_requested: bool,

    // Splash screen bitmap stuff
    pub img: Vec<u32>,
    pub bitmapinfo: BITMAPINFO,
    pub memory_hdc: HDC,
    pub screen_hdc: HDC,
    pub memory_hbitmap: HBITMAP,
    pub memory_hdc_previous_hgdiobj: HGDIOBJ,
    pub alpha: u8,

    // OpenGL stuff
    pub window_hdc: HDC,
    pub hglrc: HGLRC,

    // Timing
    pub seconds_since_start: f32
}

impl Drop for App {
    fn drop(&mut self) {
        let &mut Self {
            hinstance, class_atom, hwnd, close_requested: _,
            img: _, bitmapinfo: _,
            memory_hdc, screen_hdc, memory_hbitmap, memory_hdc_previous_hgdiobj, alpha: _,
            window_hdc, hglrc,
            seconds_since_start: _,
        } = self;

        unsafe {
            if TEST_GL {
                // NOTE: Do all of this _before_ the window is destroyed
                let is_ok = wglMakeCurrent(window_hdc, ptr::null_mut());
                assert_ne!(is_ok, FALSE);
                let is_ok = wglDeleteContext(hglrc);
                assert_ne!(is_ok, FALSE);
                // NOTE: Don't try to release the window's DC.
                // ReleaseDC() does nothing to class DCs (e.g made via CS_OWNDC) and returns FALSE in this case.
                // let is_ok = ReleaseDC(hwnd, window_hdc.unwrap());
                // assert_ne!(is_ok, FALSE);
            }
            let is_ok = DestroyWindow(hwnd);
            assert_ne!(is_ok, FALSE);
            let is_ok = UnregisterClassW(class_atom as u16 as usize as *const _, hinstance);
            assert_ne!(is_ok, FALSE);

            SelectObject(memory_hdc, memory_hdc_previous_hgdiobj);
            let is_ok = DeleteObject(memory_hbitmap as _);
            assert_ne!(is_ok, FALSE);
            let is_ok = DeleteDC(memory_hdc);
            assert_ne!(is_ok, FALSE);
            let is_ok = ReleaseDC(ptr::null_mut(), screen_hdc);
            assert_ne!(is_ok, 0);
        }
    }
}

fn lerp(a: f32, b: f32, alpha: f32) -> f32 {
    b * alpha + a * (1. - alpha)
}

impl App {
    pub fn update_window(&mut self, dt: f32) {
        self.seconds_since_start += dt;
        unsafe {
            if TEST_GL {
                let mut rect = mem::zeroed();
                let is_ok = GetClientRect(self.hwnd, &mut rect);
                assert_ne!(is_ok, FALSE);
                let w = rect.right - rect.left;
                let h = rect.bottom - rect.top;
                glViewport(0, 0, w, h);
                let a = self.alpha as f32 / 255.;
                glClearColor(a, a, 0., a);
                glClear(GL_DEPTH_BUFFER_BIT | GL_COLOR_BUFFER_BIT);

                glEnable(GL_BLEND);
                glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

                let speed = 2.;
                let scale_factor = ((speed * self.seconds_since_start).sin() + 1.) / 2.;
                let alpha_factor0 = ((self.seconds_since_start).sin() + 1.) / 2.;
                let alpha_factor1 = ((self.seconds_since_start + 8.28 * 1. / 3.).sin() + 1.) / 2.;
                let alpha_factor2 = ((self.seconds_since_start + 8.28 * 2. / 3.).sin() + 1.) / 2.;
                let s = lerp(0.50, 0.75, scale_factor);

                glBegin(GL_TRIANGLES);
                glColor4f(0., 1., 0., alpha_factor0);
                glVertex3f(-s, -s, 0.);
                glColor4f(0., 1., 0., alpha_factor1);
                glVertex3f(-s, s, 0.);
                glColor4f(0., 1., 0., alpha_factor2);
                glVertex3f(s, s, 0.);
                glEnd();

                let is_ok = SwapBuffers(self.window_hdc);
                assert_ne!(is_ok, FALSE);
            } else {
                let mut blendfunction = BLENDFUNCTION {
                    BlendOp: AC_SRC_OVER,
                    SourceConstantAlpha: self.alpha,
                    AlphaFormat: AC_SRC_ALPHA,
                    .. mem::zeroed()
                };
                let mut src_pos = POINT { x: 0, y: 0 };
                let mut dst_size = winuser::SIZE { cx: W as _, cy: H as _ };
                let is_ok = UpdateLayeredWindow(
                    self.hwnd,
                    self.screen_hdc,
                    ptr::null_mut(), // Don't change the window's position
                    &mut dst_size,
                    self.memory_hdc,
                    &mut src_pos,
                    RGB(0, 0, 0), // color key for compositing; Only used if ULW_COLORKEY is specified
                    &mut blendfunction,
                    ULW_ALPHA
                );
                assert_ne!(is_ok, FALSE, "UpdateLayeredWindow() failed!");
            }
        }
    }
}

fn main() {
    unsafe {
        // Center window, excluding taskbar!
        let (x, y) = {
            // Use the current mouse position when app starts; This is what GIMP does.
            let mut point = mem::zeroed();
            let is_ok = GetCursorPos(&mut point);
            assert_ne!(is_ok, FALSE);
            let hmonitor = MonitorFromPoint(point, MONITOR_DEFAULTTONEAREST);
            assert!(!hmonitor.is_null());
            let mut monitorinfo = MONITORINFOEXW {
                cbSize: mem::size_of::<MONITORINFOEXW>() as _,
                .. mem::zeroed()
            };
            let is_ok = GetMonitorInfoW(hmonitor, &mut monitorinfo as *mut _ as *mut MONITORINFO);
            assert_ne!(is_ok, FALSE);
            let rect = if is_ok != FALSE {
                monitorinfo.rcWork
            } else {
                let mut rect = mem::zeroed();
                let is_ok = SystemParametersInfoW(SPI_GETWORKAREA, 0, &mut rect as *mut _ as _, 0);
                assert_ne!(is_ok, FALSE);
                rect
            };

            let cx = rect.left + (rect.right - rect.left) / 2;
            let cy = rect.top + (rect.bottom - rect.top) / 2;
            let x = cx - W as i32 / 2;
            let y = cy - H as i32 / 2;
            (x, y)
        };

        let hinstance = GetModuleHandleW(ptr::null_mut());
        let style = if TEST_GL {
            CS_HREDRAW | CS_VREDRAW | CS_OWNDC  // CS_OWNDC required for GL
        } else {
            CS_HREDRAW | CS_VREDRAW | CS_NOCLOSE  // CS_OWNDC not allowed for layered windows
        };
        let wndclass = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as _,
            style,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: LoadIconW(ptr::null_mut(), IDI_QUESTION as *mut u16),
            hIconSm: LoadIconW(ptr::null_mut(), IDI_QUESTION as *mut u16),
            hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW as *mut u16),
            hbrBackground: if TEST_GL { ptr::null_mut() } else { GetStockObject(WHITE_BRUSH as _) as _ }, // NOTE: We don't actually need this!
            lpszMenuName: ptr::null_mut(),
            lpszClassName: WINDOW_CLASS_NAME.as_ptr(),
        };
        let class_atom = RegisterClassExW(&wndclass);
        assert_ne!(0, class_atom);

        // Load WGL extensions by creating a temporary GL context.
        if TEST_GL {
            let tmp_hwnd = CreateWindowExW(
                WS_EX_OVERLAPPEDWINDOW,
                WINDOW_CLASS_NAME.as_ptr(),
                ptr::null_mut(), // No title
                WS_OVERLAPPEDWINDOW, // Prevents moving by dragging the top
                CW_USEDEFAULT, // x
                CW_USEDEFAULT, // y
                CW_USEDEFAULT, // w
                CW_USEDEFAULT, // h
                ptr::null_mut(), // No parent
                ptr::null_mut(), // No menu
                hinstance,
                ptr::null_mut(), // No custom data pointer
            );
            assert!(!tmp_hwnd.is_null());

            let tmp_window_hdc = GetDC(tmp_hwnd);
            assert!(!tmp_window_hdc.is_null());

            let pfd = PIXELFORMATDESCRIPTOR {
                nSize: mem::size_of::<PIXELFORMATDESCRIPTOR>() as _,
                nVersion: 1,
                dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER, 
                iPixelType: PFD_TYPE_RGBA,
                cColorBits: 32,
                cRedBits: 0,
                cRedShift: 0,
                cGreenBits: 0,
                cGreenShift: 0,
                cBlueBits: 0,
                cBlueShift: 0,
                cAlphaBits: 0,
                cAlphaShift: 0,
                cAccumBits: 0,
                cAccumRedBits: 0,
                cAccumGreenBits: 0,
                cAccumBlueBits: 0,
                cAccumAlphaBits: 0,
                cDepthBits: 24,
                cStencilBits: 8,
                cAuxBuffers: 0,
                iLayerType: PFD_MAIN_PLANE,
                bReserved: 0,
                dwLayerMask: 0,
                dwVisibleMask: 0,
                dwDamageMask: 0,
            };
            let i_pixel_format = ChoosePixelFormat(tmp_window_hdc, &pfd);
            assert_ne!(i_pixel_format, 0);
            let is_ok = SetPixelFormat(tmp_window_hdc, i_pixel_format, &pfd);
            assert_ne!(is_ok, FALSE);
            let tmp_hglrc = wglCreateContext(tmp_window_hdc);
            assert!(!tmp_hglrc.is_null());
            let is_ok = wglMakeCurrent(tmp_window_hdc, tmp_hglrc);
            assert_ne!(is_ok, FALSE);

            println!("Dumb old context:");
            print_gl_stuff();

            unsafe fn get_fn(name: &[u8]) -> Option<&c_void> {
                assert_eq!(&0, name.last().unwrap());
                match wglGetProcAddress(name.as_ptr() as _) as usize {
                    0 => None,
                    f => Some(mem::transmute(f)),
                }
            }

            wglGetExtensionsStringARB = mem::transmute(get_fn(b"wglGetExtensionsStringARB\0"));
            wglCreateContextAttribsARB = mem::transmute(get_fn(b"wglCreateContextAttribsARB\0"));
            wglChoosePixelFormatARB = mem::transmute(get_fn(b"wglChoosePixelFormatARB\0"));
            wglSwapIntervalEXT = mem::transmute(get_fn(b"wglSwapIntervalEXT\0"));
            wglGetSwapIntervalEXT = mem::transmute(get_fn(b"wglGetSwapIntervalEXT\0"));
            assert!(wglGetExtensionsStringARB.is_some());
            assert!(wglCreateContextAttribsARB.is_some());
            assert!(wglChoosePixelFormatARB.is_some());
            assert!(wglSwapIntervalEXT.is_some());
            assert!(wglGetSwapIntervalEXT.is_some());

            // Now we've got the function pointers, get rid of the tmp window and hdc
            let is_ok = wglMakeCurrent(tmp_window_hdc, ptr::null_mut());
            assert_ne!(is_ok, FALSE);
            let is_ok = wglDeleteContext(tmp_hglrc);
            assert_ne!(is_ok, FALSE);
            // NOTE: Don't Release or Delete the HDC. Not need and will fail because of CS_OWNDC.
            let is_ok = DestroyWindow(tmp_hwnd);
            assert_ne!(is_ok, FALSE);
        }

        // WS_CLIP* recommended by MSDN (see SetPixelFormat), but breaks first call to wglMakeCurrent() ???
        let gl_window_wants_borders = false;
        let ex_style = if TEST_GL {
            if gl_window_wants_borders {
                WS_EX_OVERLAPPEDWINDOW
            } else {
                 0
            }
        } else {
            WS_EX_LAYERED
        };
        let style = if TEST_GL {
            if gl_window_wants_borders {
                WS_OVERLAPPEDWINDOW
            } else {
                WS_POPUP
            }
        } else {
            WS_POPUP
        };

        let hwnd = CreateWindowExW(
            ex_style,
            WINDOW_CLASS_NAME.as_ptr(),
            ptr::null_mut(), // No title
            style,
            x, y,
            W as _, H as _,
            ptr::null_mut(), // No parent
            ptr::null_mut(), // No menu
            hinstance,
            ptr::null_mut(), // No custom data pointer
        );
        assert!(!hwnd.is_null());

        let window_hdc = if TEST_GL {
            let hdc = GetDC(hwnd);
            assert!(!hdc.is_null());
            hdc
        } else {
            ptr::null_mut()
        };

        let img = {
            let mut img = Vec::<u32>::with_capacity((W*H) as usize);
            for y in 0..H {
                for x in 0..W {
                    let mut r = 255 * x / W;
                    let mut g = 255 * y / H;
                    let mut b = 0;
                    
                    if r > 255 {
                        r = 255;
                    }
                    if g > 255 {
                        g = 255;
                    }
                    if b > 255 {
                        b = 255;
                    }

                    let (cx, cy) = (W as i32 / 2, H as i32 / 2);
                    let (dx, dy) = (x as i32 - cx, y as i32 - cy);
                    let d_squared = (dx*dx + dy*dy) as u32;
                    let mut a = 255 * d_squared / ((W/2)*(W/2));
                    if a > 255 {
                        a = 0;
                    }

                    // Premultiply alpha, see AC_SRC_ALPHA BLENDFUNCTION docs
                    r = r * a / 255;
                    g = g * a / 255;
                    b = b * a / 255;

                    let pixel = (a << 24) | (r << 16) | (g << 8) | b;
                    img.push(pixel);
                }
            }
            img
        };

        let mut bitmapinfo = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: mem::size_of::<BITMAPINFOHEADER>() as _,
                biWidth: W as _, // FIXME: assumed
                biHeight: H as _, // FIXME: assumed
                biPlanes: 1,
                biSizeImage: img.len() as _,
                biBitCount: 32,
                .. mem::zeroed()
            },
            .. mem::zeroed()
        };

        let screen_hdc = GetDC(ptr::null_mut());
        let memory_hdc = CreateCompatibleDC(screen_hdc);
        let memory_hbitmap = CreateCompatibleBitmap(screen_hdc, W as _, H as _);
        let memory_hdc_previous_hgdiobj = SelectObject(memory_hdc, memory_hbitmap as _);
        bitmapinfo.bmiHeader.biHeight *= -1;
        let status = StretchDIBits(
            memory_hdc,
            0, 0, W as _, H as _, // dst
            0, 0, W as _, H as _, // src FIXME: assumed
            img.as_ptr() as _,
            &bitmapinfo,
            DIB_RGB_COLORS,
            SRCCOPY
        );
        bitmapinfo.bmiHeader.biHeight *= -1;
        if status == 0 {
            panic!("StretchDIBits() failed: {}", GetLastError());
        }
        assert_ne!(status, GDI_ERROR as _, "No PNG support!");

        // Try drawing some text in memory_hdc
        if false {
            let hdc = memory_hdc;
            let hfont = CreateFontA(48,0,0,0,FW_DONTCARE,FALSE as _,TRUE as _,FALSE as _,DEFAULT_CHARSET,OUT_OUTLINE_PRECIS,
                CLIP_DEFAULT_PRECIS,CLEARTYPE_QUALITY, VARIABLE_PITCH,b"Impact\0".as_ptr() as _);
            let prev_gdiobj = SelectObject(hdc, hfont as _);
            
            //Sets the coordinates for the rectangle in which the text is to be formatted.
            let mut rect = mem::zeroed();
            SetRect(&mut rect, 0, 0, W as _, H as _);
            SetTextColor(hdc, RGB(0,0,255));
            SetBkMode(hdc, TRANSPARENT as _);
            // Note that the alpha is incorrect though. See https://stackoverflow.com/a/1343551 for fixing it.
            DrawTextA(hdc, b"Drawing Text with Impact\0".as_ptr() as _, -1, &mut rect, DT_NOCLIP | DT_CENTER | DT_END_ELLIPSIS | DT_VCENTER | DT_SINGLELINE);

            SelectObject(memory_hdc, prev_gdiobj);
            DeleteObject(hfont as _);
        }

        let hglrc = if !TEST_GL {
            ptr::null_mut()
        } else {
            // TODO: Use
            // WGL_ARB_pixel_format_float: Allows for floating-point framebuffers.
            // WGL_ARB_framebuffer_sRGB: Allows for color buffers to be in sRGB format.
            // WGL_ARB_multisample: Allows for multisampled framebuffers.

            // Finding a new pixel format using extensions.
            // We can only call SetPixelFormat() once per window. To do it again,
            // we would have to recreate the window, which is a bit involved.
            let attribs_i = &[
                WGL_DRAW_TO_WINDOW_ARB, TRUE,
                WGL_SUPPORT_OPENGL_ARB, TRUE,
                WGL_DOUBLE_BUFFER_ARB, TRUE,
                WGL_PIXEL_TYPE_ARB, WGL_TYPE_RGBA_ARB,
                WGL_TRANSPARENT_ARB, TRUE,
                WGL_COLOR_BITS_ARB, 32,
                WGL_RED_BITS_ARB, 8,
                WGL_GREEN_BITS_ARB, 8,
                WGL_BLUE_BITS_ARB, 8,
                WGL_ALPHA_BITS_ARB, 8,
                WGL_DEPTH_BITS_ARB, 24,
                WGL_STENCIL_BITS_ARB, 8,
                0, // End
            ];
            assert_eq!(&0, attribs_i.last().unwrap());
            let attribs_f = &[
                0., // End
            ];
            assert_eq!(&0., attribs_f.last().unwrap());

            let mut candidate_pixel_formats = [0; 32];
            let mut num_formats = 0;
            let is_ok = (wglChoosePixelFormatARB.unwrap())(
                window_hdc,
                attribs_i.as_ptr(),
                attribs_f.as_ptr(),
                candidate_pixel_formats.len() as _,
                candidate_pixel_formats.as_mut_ptr(),
                &mut num_formats
            );
            assert_ne!(is_ok, FALSE);
            let candidate_pixel_formats = &candidate_pixel_formats[..num_formats as _];
            let i_pixel_format = candidate_pixel_formats[0];
            assert_ne!(i_pixel_format, 0);
            let pfd_kludge = {
                let mut pfd = PIXELFORMATDESCRIPTOR {
                    nSize: mem::size_of::<PIXELFORMATDESCRIPTOR>() as _,
                    .. mem::zeroed()
                };
                DescribePixelFormat(window_hdc, i_pixel_format, mem::size_of_val(&pfd) as _, &mut pfd);
                pfd
            };
            let is_ok = SetPixelFormat(window_hdc, i_pixel_format, &pfd_kludge);
            assert_ne!(is_ok, FALSE);

            let context_attribs = &[
                WGL_CONTEXT_MAJOR_VERSION_ARB, 3,
                WGL_CONTEXT_MINOR_VERSION_ARB, 2,
                WGL_CONTEXT_FLAGS_ARB, WGL_CONTEXT_DEBUG_BIT_ARB,
                WGL_CONTEXT_PROFILE_MASK_ARB, WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB,
                0, // End
            ];
            assert_eq!(&0, context_attribs.last().unwrap());
            let hglrc_share: HGLRC = ptr::null_mut();
            let hglrc = (wglCreateContextAttribsARB.unwrap())(window_hdc, hglrc_share, context_attribs.as_ptr());
            assert!(!hglrc.is_null());
            let is_ok = wglMakeCurrent(window_hdc, hglrc);
            assert_ne!(is_ok, FALSE);

            println!("Cool new context:");
            print_gl_stuff();

            wglSwapIntervalEXT.unwrap()(-1); // Adaptic VSync. Not checking for errors; too lazy.

            hglrc
        };

        let mut app = App {
            hinstance, class_atom, hwnd, close_requested: false,
            img, bitmapinfo, memory_hdc, screen_hdc, memory_hbitmap,
            memory_hdc_previous_hgdiobj, alpha: if TEST_GL { 0_u8 } else { 255_u8 },
            window_hdc, hglrc,
            seconds_since_start: 0.,
        };

        if !TEST_GL {
            app.update_window(0.);
        }

        if true {
            ShowWindow(app.hwnd, SW_SHOW);
        } else {
            let millis = 500;
            let is_ok = AnimateWindow(hwnd, millis, AW_BLEND);
            assert_ne!(is_ok, FALSE);
        }

        let is_dwm_composition_enabled = {
            let mut is_enabled = FALSE;
            let hresult = DwmIsCompositionEnabled(&mut is_enabled);
            assert_eq!(hresult, S_OK);
            hresult == S_OK && is_enabled != FALSE
        };

        if !is_dwm_composition_enabled {
            println!("DWM composition is disabled. You won't see a transparent window!");
        } else {
            let hrgn = {
                let left = 0;
                let top = 0;
                let right = 1;
                let bottom = 1;
                // If the rect is zero-size, the DWM blur effect takes all of the window :(
                CreateRectRgn(left, top, right, bottom)
            };
            assert!(!hrgn.is_null());
            let blur_behind = DWM_BLURBEHIND {
                dwFlags: DWM_BB_ENABLE | DWM_BB_BLURREGION | DWM_BB_TRANSITIONONMAXIMIZED,
                fEnable: TRUE,
                hRgnBlur: hrgn,
                fTransitionOnMaximized: FALSE, // FIXME: Play with this
            };
            let lresult = DwmEnableBlurBehindWindow(app.hwnd, &blur_behind);
            assert_eq!(lresult, S_OK);
            let is_ok = DeleteObject(hrgn as _);
            assert_ne!(is_ok, FALSE);
        }

        assert!(APP.is_none());
        APP = Some(app);

        let mut prev_frame_instant = None;
        loop {
            let now = Instant::now();
            let dt = match prev_frame_instant {
                Some(prev_frame_instant) => now.duration_since(prev_frame_instant).as_secs_f32(),
                None => 1. / 60.,
            };
            prev_frame_instant = Some(now);

            let mut msg = mem::zeroed();

            if false {
            	let has_one = PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE) != FALSE;
                if has_one {
                    if msg.message == WM_QUIT {
                        break;
                    }
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            } else {
                let ret = GetMessageW(&mut msg, ptr::null_mut(), 0, 0);
                if ret <= 0 {
                    if ret < 0 {
                        panic!("GetMessage() failed");
                    }
                    break;
                }

                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            if APP.as_ref().unwrap().close_requested {
                break;
            }
            APP.as_mut().unwrap().update_window(dt);
            // println!("DeltaTime = {}, FPS = {}", dt, 1. / dt);
        }
    }
}
