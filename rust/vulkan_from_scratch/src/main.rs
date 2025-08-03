use std::{ffi::OsStr, mem::MaybeUninit};
use std::os::windows::ffi::OsStrExt;

use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{DefWindowProcW, LoadCursorW, PostQuitMessage, RegisterClassW, COLOR_WINDOW, IDC_ARROW, WM_DESTROY, WNDCLASSW};
use winapi::um::{libloaderapi::GetModuleHandleW, winuser::{CreateWindowExW, DispatchMessageW, GetMessageW, ShowWindow, TranslateMessage, CW_USEDEFAULT, SW_SHOW, WS_OVERLAPPEDWINDOW}};

extern crate winapi;

unsafe extern "system" fn wndproc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        msg if msg == WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
                0
            }
        },
        _ => unsafe {
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
    }
}

fn main() {
    println!("Hello, world!");
    let hinstance = unsafe { GetModuleHandleW(std::ptr::null()) }; // Question: maybe not as simple as this? https://stackoverflow.com/a/78906765
    let window_class_name = OsStr::new("VulkanFromScratch\0").encode_wide().collect::<Vec<_>>();
    let window_title = OsStr::new("Window Title\0").encode_wide().collect::<Vec<_>>();
    unsafe {
        let wndclass = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: std::ptr::null_mut(),
            hCursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW),
            hbrBackground: (COLOR_WINDOW + 1) as _,
            lpszMenuName: std::ptr::null_mut(),
            lpszClassName: window_class_name.as_ptr(),
        };
        let atom = RegisterClassW(&wndclass);
        if atom == 0 {
            std::process::exit(1);
        }
    }
    let hwnd = unsafe {
        CreateWindowExW(
            0, // extended style flags
            window_class_name.as_ptr(),
            window_title.as_ptr(),
            WS_OVERLAPPEDWINDOW, // style flags
            CW_USEDEFAULT, CW_USEDEFAULT, // x, y
            CW_USEDEFAULT, CW_USEDEFAULT, // width, height
            std::ptr::null_mut(), // parent window
            std::ptr::null_mut(), // menu
            hinstance,
            std::ptr::null_mut() // optional payload passed to WM_CREATE
        )
    };
    unsafe {
        ShowWindow(hwnd, SW_SHOW);
        loop {
            let mut msg = MaybeUninit::uninit();
            let has_message = GetMessageW(msg.as_mut_ptr(), std::ptr::null_mut(), 0, 0);
            if has_message == 0 {
                break;
            }
            TranslateMessage(msg.assume_init_mut());
            DispatchMessageW(msg.assume_init_mut());
        }
    }
}
