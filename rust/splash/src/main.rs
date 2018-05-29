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

use winapi::{
    shared::{windef::*, minwindef::*,},
    um::{winuser::{self, *}, wingdi::*, libloaderapi::*, errhandlingapi::*,},
};

static WINDOW_CLASS_NAME: &[u16] = &['_' as _, 0];
const W: u32 = 512;
const H: u32 = 512;
static mut ALPHA: u8 = 255;

unsafe extern "system" fn wndproc(hwnd: HWND, umsg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match umsg {
        WM_CLOSE => {
            DefWindowProcW(hwnd, umsg, wparam, lparam) // Destroys the window
        },
        WM_ACTIVATE => {
            DefWindowProcW(hwnd, umsg, wparam, lparam) // Generates WM_MOUSEACTIVATE, IFN
        },
        WM_MOUSEACTIVATE => {
            MA_ACTIVATE as _ // Yes, we would like to be activated when we're clicked on
        },
        WM_ACTIVATEAPP => {
            DefWindowProcW(hwnd, umsg, wparam, lparam) // ??
        },
        WM_KEYDOWN => {
            let vkeycode = wparam as i32;
            match vkeycode {
                VK_LEFT => ALPHA = ALPHA.saturating_sub(1),
                VK_RIGHT => ALPHA = ALPHA.saturating_add(1),
                VK_ESCAPE => {
                    SendMessageW(hwnd, WM_CLOSE, 0, 0);
                },
                _ => (),
            };
            0
        },
        WM_PAINT => {
            println!("Painting!");
            let mut ps = mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);
            EndPaint(hwnd, &mut ps);
            0
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
            // FIXME: Use MonitorFromWindow() (see "Taxes" in The Old New Thing)
            let owner_hwnd = GetForegroundWindow();
            assert!(!owner_hwnd.is_null());
            let hmonitor = MonitorFromWindow(owner_hwnd, MONITOR_DEFAULTTONEAREST);
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

            let cx = (rect.right - rect.left) / 2;
            let cy = (rect.bottom - rect.top) / 2;
            let x = cx - W as i32 / 2;
            let y = cy - H as i32 / 2;
            (x, y)
        };

        let hinstance = GetModuleHandleW(ptr::null_mut());
        let wndclass = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as _,
            style: CS_HREDRAW | CS_VREDRAW | CS_NOCLOSE, // CS_OWNDC not allowed for layered windows
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: LoadIconW(ptr::null_mut(), IDI_QUESTION as *mut u16),
            hIconSm: LoadIconW(ptr::null_mut(), IDI_QUESTION as *mut u16),
            hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW as *mut u16),
            hbrBackground: GetStockObject(WHITE_BRUSH as _) as _, // NOTE: We don't actually need this!
            lpszMenuName: ptr::null_mut(),
            lpszClassName: WINDOW_CLASS_NAME.as_ptr(),
        };
        let class_atom = RegisterClassExW(&wndclass);
        assert_ne!(0, class_atom);
        let hwnd = CreateWindowExW(
            WS_EX_LAYERED,
            WINDOW_CLASS_NAME.as_ptr(),
            ptr::null_mut(), // No title
            WS_POPUP, // Prevents moving by dragging the top
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
        {
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

        let update_window = || {
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

        update_window();

        ShowWindow(hwnd, SW_SHOW);

        loop {
            let mut msg = mem::uninitialized();
            let ret = GetMessageW(&mut msg, ptr::null_mut(), 0, 0);
            if ret <= 0 {
                if ret < 0 {
                    panic!("GetMessage() failed");
                }
                break;
            }

            TranslateMessage(&msg);
            DispatchMessageW(&msg);

            if IsWindow(hwnd) == FALSE {
                PostQuitMessage(0);
                continue;
            }

            update_window();
        }

        SelectObject(hdc_memory, hbmp_old);
        DeleteObject(hbmp as _);
        DeleteDC(hdc_memory);
        ReleaseDC(ptr::null_mut(), hdc_screen);
    }
}
