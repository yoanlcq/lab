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

use winapi::{
    shared::{windef::*, minwindef::*, windowsx::*,},
    um::{winuser::{self, *}, wingdi::*, libloaderapi::*, errhandlingapi::*, gl::gl::*},
};


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

    #[link="opengl32.dll"]
    extern "system" { // Is this the correct calling convention ??
        pub fn glBegin(_: GLenum);
        pub fn glEnd();
        pub fn glClear(_: GLenum);
        pub fn glClearColor(_: f32, _: f32, _: f32, _: f32);
        pub fn glVertex2f(_: f32, _: f32);
        pub fn glGetString(_: GLenum) -> *const c_char;
        pub fn glViewport(x: GLint, y: GLint, w: GLsizei, h: GLsizei);
    }

    pub static GL_TRUE: GLboolean = 1;
    pub static GL_FALSE: GLboolean = 0;

    pub const GL_VENDOR     : GLenum = 0x1F00;
    pub const GL_RENDERER   : GLenum = 0x1F01;
    pub const GL_VERSION    : GLenum = 0x1F02;
    pub const GL_EXTENSIONS : GLenum = 0x1F03;
}
use self::splash_gl::*;

static WINDOW_CLASS_NAME: &[u16] = &['_' as _, 0];
const W: u32 = 512;
const H: u32 = 512;
static mut ALPHA: u8 = 255;

static TEST_GL: bool = true;

use std::ffi::CStr;

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
            PostQuitMessage(0); // We do this here because we know we only ever have one window.
            // This could break stuff if we use multiple windows.
            0 // zero if processed
        },
        WM_ACTIVATE => {
            DefWindowProcW(hwnd, umsg, wparam, lparam) // Generates WM_MOUSEACTIVATE, IFN
        },
        WM_MOUSEACTIVATE => {
            MA_ACTIVATE as _ // Yes, we would like to be activated when we're clicked on
        },
        WM_KEYDOWN => {
            let vkeycode = wparam as i32;
            match vkeycode {
                VK_LEFT => ALPHA = ALPHA.saturating_sub(1),
                VK_RIGHT => ALPHA = ALPHA.saturating_add(1),
                VK_ESCAPE => {
                    PostMessageW(hwnd, WM_CLOSE, 0, 0);
                },
                _ => (),
            };
            0
        },
        WM_SIZING if TEST_GL => {
            // https://www.gamedev.net/forums/topic/488074-win32-message-pump-and-opengl---rendering-pauses-while-draggingresizing/
            // FIXME: Call render() here! This event is emitted from a private, blocking message pump by DefWindowProc while user drags window.
            1 // TRUE if processed
        },
        /*
        WM_ERASEBKGND if TEST_GL => 1, // non-zero if processed
        WM_SIZE if TEST_GL => {
            0 // zero if processed
        },
        WM_SYSCOMMAND if TEST_GL => {
            // zero if processed
            let x = GET_X_LPARAM(lparam);
            let y = GET_Y_LPARAM(lparam);
            match wparam {
                SC_SIZE => {
                    SetWindowPos(hwnd, ptr::null_mut(), x, y, 0, 0, SWP_NOZORDER | SWP_NOSIZE);
                    0
                },
                SC_MOVE => {
                    SetWindowPos(hwnd, ptr::null_mut(), 0, 0, x, y, SWP_NOZORDER | SWP_NOMOVE);
                    0
                },
                _ => DefWindowProcW(hwnd, umsg, wparam, lparam),
            }
        },
        */
        WM_PAINT => {
            // println!("Handling WM_PAINT");
            if !TEST_GL {
                let mut ps = mem::zeroed();
                let _hdc = BeginPaint(hwnd, &mut ps);
                EndPaint(hwnd, &mut ps);
                0 // zero if processed
            } else {
                0
            }
            /*
            // Apparently calling UpdateLayeredWindow() just once was enough
            // for the window to manage its own repainting...
            DefWindowProcW(hwnd, umsg, wparam, lparam)
            */
        },
        _ => DefWindowProcW(hwnd, umsg, wparam, lparam),
    }
}

fn main() {
    unsafe {
        // Center window, excluding taskbar!
        let (x, y) = {
            // Use the current mouse position when app starts; This is what GIMP does.
            let mut point = mem::uninitialized();
            let is_ok = GetCursorPos(&mut point);
            assert_ne!(is_ok, FALSE);
            let hmonitor = MonitorFromPoint(point, MONITOR_DEFAULTTONEAREST);
            assert!(!hmonitor.is_null());
            let mut monitorinfo = MONITORINFOEXW {
                cbSize: mem::size_of::<MONITORINFOEXW>() as _,
                .. mem::uninitialized()
            };
            let is_ok = GetMonitorInfoW(hmonitor, &mut monitorinfo as *mut _ as *mut MONITORINFO);
            assert_ne!(is_ok, FALSE);
            let rect = if is_ok != FALSE {
                monitorinfo.rcWork
            } else {
                let mut rect = mem::uninitialized();
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
        let hwnd = CreateWindowExW(
            // WS_CLIP* recommended by MSDN (see SetPixelFormat), but breaks first call to wglMakeCurrent() ???
            if TEST_GL { WS_EX_OVERLAPPEDWINDOW /*| WS_CLIPCHILDREN | WS_CLIPSIBLINGS*/ } else { WS_EX_LAYERED },
            WINDOW_CLASS_NAME.as_ptr(),
            ptr::null_mut(), // No title
            if TEST_GL { WS_OVERLAPPEDWINDOW } else { WS_POPUP }, // Prevents moving by dragging the top
            x, y,
            W as _, H as _,
            ptr::null_mut(), // No parent
            ptr::null_mut(), // No menu
            hinstance,
            ptr::null_mut(), // No custom data pointer
        );

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

        let hdc_screen = GetDC(ptr::null_mut());
        let hdc_memory = CreateCompatibleDC(hdc_screen);
        let hbmp = CreateCompatibleBitmap(hdc_screen, W as _, H as _);
        let hbmp_old = SelectObject(hdc_memory, hbmp as _);
        bitmapinfo.bmiHeader.biHeight *= -1;
        let status = StretchDIBits(
            hdc_memory,
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

        // Try drawing some text in hdc_memory
        if false {
            let hdc = hdc_memory;
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

            SelectObject(hdc_memory, prev_gdiobj);
            DeleteObject(hfont as _);
        }

        let hdc_window = if TEST_GL {
            let hdc = GetDC(hwnd);
            assert!(!hdc.is_null());
            Some(hdc)
        } else {
            None
        };

        let update_window = || if TEST_GL {
            let mut rect = mem::uninitialized();
            let is_ok = GetClientRect(hwnd, &mut rect);
            assert_ne!(is_ok, FALSE);
            let w = rect.right - rect.left;
            let h = rect.bottom - rect.top;
            glViewport(0, 0, w, h);
            glClearColor(1., 0., 0., 0.5);
            glClear(GL_DEPTH_BUFFER_BIT | GL_COLOR_BUFFER_BIT);

            glBegin(GL_TRIANGLES);
            glVertex2f(-1., -1.);
            glVertex2f(-1., 1.);
            glVertex2f(1., 1.);
            glEnd();

            let is_ok = SwapBuffers(hdc_window.unwrap());
            assert_ne!(is_ok, FALSE);
        } else {
            let mut blendfunction = BLENDFUNCTION {
                BlendOp: AC_SRC_OVER,
                SourceConstantAlpha: ALPHA,
                AlphaFormat: AC_SRC_ALPHA,
                .. mem::zeroed()
            };
            let mut src_pos = POINT { x: 0, y: 0 };
            let mut dst_size = winuser::SIZE { cx: W as _, cy: H as _ };
            let is_ok = UpdateLayeredWindow(
                hwnd,
                hdc_screen,
                ptr::null_mut(), // Don't change the window's position
                &mut dst_size,
                hdc_memory,
                &mut src_pos,
                RGB(0, 0, 0), // color key for compositing; Only used if ULW_COLORKEY is specified
                &mut blendfunction,
                ULW_ALPHA
            );
            assert_ne!(is_ok, FALSE, "UpdateLayeredWindow() failed!");
        };

        let hglrc = if !TEST_GL {
            None
        } else {
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
            let hdc_window = hdc_window.unwrap();
            assert!(!hdc_window.is_null());
            let pfi = ChoosePixelFormat(hdc_window, &pfd);
            assert_ne!(pfi, 0);
            let is_ok = SetPixelFormat(hdc_window, pfi, &pfd);
            assert_ne!(is_ok, FALSE);
            let hglrc = wglCreateContext(hdc_window);
            assert!(!hglrc.is_null());
            let is_ok = wglMakeCurrent(hdc_window, hglrc);
            assert_ne!(is_ok, FALSE);

            println!("Dumb old context:");
            print_gl_stuff();

            unsafe fn get_fn(name: &[u8]) -> Option<&c_void> {
                assert_eq!(&0, name.last().unwrap());
                match wglGetProcAddress(name.as_ptr() as _) as usize {
                    0 => None,
                    f => Some(mem::transmute(f)),
                }
            };
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

            // TODO: Use
            // WGL_ARB: c_int =_pixel_format_float: Allows for floating-point framebuffers.
            // WGL_ARB: c_int =_framebuffer_sRGB: Allows for color buffers to be in sRGB format.
            // WGL_ARB: c_int =_multisample: Allows for multisampled framebuffers.

            // Finding a new pixel format using extensions.
            // We can only call SetPixelFormat() once per window. To do it again,
            // we would have to recreate the window, which is a bit involved.
            if false {
                let attribs_i = &[
                    WGL_DRAW_TO_WINDOW_ARB, GL_TRUE as _,
                    WGL_SUPPORT_OPENGL_ARB, GL_TRUE as _,
                    WGL_DOUBLE_BUFFER_ARB, GL_TRUE as _,
                    WGL_PIXEL_TYPE_ARB, WGL_TYPE_RGBA_ARB,
                    WGL_COLOR_BITS_ARB, 32,
                    WGL_DEPTH_BITS_ARB, 24,
                    WGL_STENCIL_BITS_ARB, 8,
                    0, // End
                ];
                let attribs_f = &[
                    0., // End
                ];

                let mut candidate_pixel_formats = [0; 32];
                let mut num_formats = 0;
                (wglChoosePixelFormatARB.unwrap())(
                    hdc_window,
                    attribs_i.as_ptr(),
                    attribs_f.as_ptr(),
                    candidate_pixel_formats.len() as _,
                    candidate_pixel_formats.as_mut_ptr(),
                    &mut num_formats
                );
                let candidate_pixel_formats = &candidate_pixel_formats[..num_formats as _];
                let pixel_format = candidate_pixel_formats[0];
                let is_ok = SetPixelFormat(hdc_window, pixel_format, &pfd);
                assert_ne!(is_ok, FALSE);
            }


            let context_attribs = &[
                WGL_CONTEXT_MAJOR_VERSION_ARB, 3,
                WGL_CONTEXT_MINOR_VERSION_ARB, 2,
                WGL_CONTEXT_FLAGS_ARB, WGL_CONTEXT_DEBUG_BIT_ARB,
                WGL_CONTEXT_PROFILE_MASK_ARB, WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB,
                0, // End
            ];
            let hglrc_share: HGLRC = ptr::null_mut();
            let hglrc_modern = (wglCreateContextAttribsARB.unwrap())(hdc_window, hglrc_share, context_attribs.as_ptr());
            assert!(!hglrc_modern.is_null());
            let is_ok = wglMakeCurrent(hdc_window, ptr::null_mut());
            assert_ne!(is_ok, FALSE);
            let is_ok = wglDeleteContext(hglrc);
            assert_ne!(is_ok, FALSE);
            let hglrc = hglrc_modern;
            let is_ok = wglMakeCurrent(hdc_window, hglrc);
            assert_ne!(is_ok, FALSE);

            println!("Cool new context:");
            print_gl_stuff();

            Some(hglrc)
        };

        update_window();

        ShowWindow(hwnd, SW_SHOW);

        loop {
            let mut msg = mem::uninitialized();

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

            update_window();
        }

        if TEST_GL {
            // NOTE: Do all of this _before_ the window is destroyed
            let is_ok = wglMakeCurrent(hdc_window.unwrap(), ptr::null_mut());
            assert_ne!(is_ok, FALSE);
            let is_ok = wglDeleteContext(hglrc.unwrap());
            assert_ne!(is_ok, FALSE);
            // NOTE: Don't try to release the window's DC.
            // ReleaseDC() does nothing to class DCs (e.g made via CS_OWNDC) and returns FALSE in this case.
            // let is_ok = ReleaseDC(hwnd, hdc_window.unwrap());
            // assert_ne!(is_ok, FALSE);
        }
        let is_ok = DestroyWindow(hwnd);
        assert_ne!(is_ok, FALSE);

        SelectObject(hdc_memory, hbmp_old);
        let is_ok = DeleteObject(hbmp as _);
        assert_ne!(is_ok, FALSE);
        let is_ok = DeleteDC(hdc_memory);
        assert_ne!(is_ok, FALSE);
        let is_ok = ReleaseDC(ptr::null_mut(), hdc_screen);
        assert_ne!(is_ok, 0);
    }
}
