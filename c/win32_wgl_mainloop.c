// MinGW:
//	 gcc -municode w32_mainloop.c -lgdi32 -lopengl32

#ifndef UNICODE
#error UNICODE was not defined!
#define UNICODE
#endif

#include <windows.h>
#include <windowsx.h>
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <GL/gl.h>
#include <GL/glext.h>
#include <GL/wglext.h>

#ifndef WM_MOUSEHWHEEL
	#define WM_MOUSEHWHEEL 0x020E
#endif

typedef struct {
	const char *extensions_list;
	bool has_WGL_ARB_pixel_format;
	bool has_WGL_ARB_multisample;
	bool has_WGL_ARB_make_current_read;
	bool has_WGL_ARB_create_context;
	bool has_WGL_ARB_create_context_profile;
	bool has_WGL_EXT_swap_control;
	bool has_WGL_EXT_swap_control_tear;
    bool has_WGL_EXT_create_context_es_profile;
    bool has_WGL_EXT_create_context_es2_profile;
	bool has_WGL_ARB_framebuffer_sRGB;
	PFNWGLGETEXTENSIONSSTRINGARBPROC GetExtensionsStringARB;
	PFNWGLMAKECONTEXTCURRENTARBPROC MakeContextCurrentARB;
	PFNWGLCHOOSEPIXELFORMATARBPROC ChoosePixelFormatARB;
	PFNWGLCREATECONTEXTATTRIBSARBPROC CreateContextAttribsARB;
	PFNWGLSWAPINTERVALEXTPROC SwapIntervalEXT;
	PFNWGLGETSWAPINTERVALEXTPROC GetSwapIntervalEXT;
} Wgl;

static Wgl wgl = {0};

// https://www.khronos.org/opengl/wiki/Load_OpenGL_Functions#Windows
static bool is_valid_wgl_pointer(void *p) {
	return p > (void*)3;
}

// NOTE: Not exhaustive.
static void Wgl_load(Wgl *wgl, HDC hdc) {
	*wgl = (Wgl) {0};

	wgl->GetExtensionsStringARB = (PFNWGLGETEXTENSIONSSTRINGARBPROC) wglGetProcAddress("wglGetExtensionsStringARB");
	if(!is_valid_wgl_pointer(wgl->GetExtensionsStringARB))
		return;
	wgl->extensions_list = wgl->GetExtensionsStringARB(hdc);

#define FOO(x) if(strstr(wgl->extensions_list, #x)) { wgl->has_##x = true; printf("Found \"%s\"\n", #x); } else { printf("Didn't find \"%s\"\n", #x); }
	FOO(WGL_ARB_pixel_format);
	FOO(WGL_ARB_multisample);
	FOO(WGL_ARB_make_current_read);
	FOO(WGL_ARB_create_context);
	FOO(WGL_ARB_create_context_profile);
	FOO(WGL_EXT_swap_control);
	FOO(WGL_EXT_swap_control_tear);
    FOO(WGL_EXT_create_context_es_profile);
    FOO(WGL_EXT_create_context_es2_profile);
	FOO(WGL_ARB_framebuffer_sRGB);
#undef FOO

	if(wgl->has_WGL_ARB_pixel_format)
		wgl->ChoosePixelFormatARB = (PFNWGLCHOOSEPIXELFORMATARBPROC) wglGetProcAddress("wglChoosePixelFormatARB");

	if(wgl->has_WGL_ARB_make_current_read)
		wgl->MakeContextCurrentARB = (PFNWGLMAKECONTEXTCURRENTARBPROC) wglGetProcAddress("wglMakeContextCurrentARB");

	if(wgl->has_WGL_ARB_create_context)
		wgl->CreateContextAttribsARB = (PFNWGLCREATECONTEXTATTRIBSARBPROC) wglGetProcAddress("wglCreateContextAttribsARB");

	if(wgl->has_WGL_EXT_swap_control || wgl->has_WGL_EXT_swap_control_tear) {
		wgl->SwapIntervalEXT = (PFNWGLSWAPINTERVALEXTPROC) wglGetProcAddress("wglSwapIntervalEXT");
		wgl->GetSwapIntervalEXT = (PFNWGLGETSWAPINTERVALEXTPROC) wglGetProcAddress("wglGetSwapIntervalEXT");
	}
}

// https://www.gamedev.net/forums/topic/160535-getdc--releasedc/
// Quoting Mastaba :
//      If a window class style uses CS_OWNDC, then the DC you get for a window of that class will be private. That DC does not have to be released.

LRESULT CALLBACK WndProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
	static HGLRC new_hglrc = NULL;

	switch (uMsg) { 

	case WM_CREATE: {
		// Initialize the window. 
		PIXELFORMATDESCRIPTOR pfd = {
			sizeof(PIXELFORMATDESCRIPTOR),
			1,
			PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,	//Flags
			PFD_TYPE_RGBA,			//The kind of framebuffer. RGBA or palette.
			32,						//Colordepth of the framebuffer.
			0, 0, 0, 0, 0, 0,
			0,
			0,
			0,
			0, 0, 0, 0,
			24,						//Number of bits for the depthbuffer
			8,						//Number of bits for the stencilbuffer
			0,						//Number of Aux buffers in the framebuffer.
			PFD_MAIN_PLANE,
			0,
			0, 0, 0
		};
		HDC hdc = GetDC(hwnd);
		int iPixelFormat = ChoosePixelFormat(hdc, &pfd);
		if(!iPixelFormat) {
			fprintf(stderr, "Can't ChoosePixelFormat() (%lu)!\n", GetLastError());
			ExitProcess(1);
		}
		if(!SetPixelFormat(hdc, iPixelFormat, &pfd)) {
			fprintf(stderr, "Can't SetPixelFormat() (%lu)!\n", GetLastError());
			ExitProcess(1);
		}
		HGLRC tmp_hglrc = wglCreateContext(hdc);
		wglMakeCurrent(hdc, tmp_hglrc);

		Wgl_load(&wgl, hdc);
		printf("WGL extensions: %s\n", wgl.extensions_list);

		if(!wgl.has_WGL_ARB_create_context) {
			fprintf(stderr, "WGL_ARB_create_context is not present!\n");
			ExitProcess(1);
		}

		/*
		const int attribList[] =
		{
			WGL_DRAW_TO_WINDOW_ARB, GL_TRUE,
			WGL_SUPPORT_OPENGL_ARB, GL_TRUE,
			WGL_DOUBLE_BUFFER_ARB, GL_TRUE,
			WGL_PIXEL_TYPE_ARB, WGL_TYPE_RGBA_ARB,
			WGL_COLOR_BITS_ARB, 32,
			WGL_DEPTH_BITS_ARB, 24,
			WGL_STENCIL_BITS_ARB, 8,
			0,		//End
		};

		UINT numFormats;

		BOOL cpfarb = wgl.ChoosePixelFormatARB(hdc, attribList, NULL, 1, &iPixelFormat, &numFormats);
		if(!cfpfarb) {
			fprintf(stderr, "wglChoosePixelFormatARB() failed!\n");
			ExitProcess(1);
		}

		if(!SetPixelFormat(hdc, iPixelFormat, &pfd)) { // XXX not sure about pfd
			fprintf(stderr, "Can't SetPixelFormat() (%lu)!\n", GetLastError());
			ExitProcess(1);
		}
		*/

		const int ctxattribs[] = {
			WGL_CONTEXT_MAJOR_VERSION_ARB, 3,
			WGL_CONTEXT_MINOR_VERSION_ARB, 2,
			/* TODO
			WGL_CONTEXT_PROFILE_MASK_ARB, WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
			WGL_CONTEXT_PROFILE_MASK_ARB, WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB,
			WGL_CONTEXT_FLAGS_ARB, WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB | WGL_CONTEXT_DEBUG_BIT_ARB,
			*/
			0
		};
		// http://developer.download.nvidia.com/opengl/specs/WGL_ARB_create_context.txt
		// For seeing how the version is picked.
		HGLRC new_hglrc = wgl.CreateContextAttribsARB(hdc, NULL, ctxattribs);

		wglMakeCurrent(hdc, NULL);
		wglDeleteContext(tmp_hglrc);
		wglMakeCurrent(hdc, new_hglrc);

#define FOO(x) printf(#x ": %s\n", x);
		FOO(glGetString(GL_VERSION));
		FOO(glGetString(GL_VENDOR));
		FOO(glGetString(GL_RENDERER));
		FOO(glGetString(GL_SHADING_LANGUAGE_VERSION));
#undef FOO
	 
		return 0; 
	}

	// https://www.khronos.org/opengl/wiki/Platform_specifics:_Windows#When_do_I_destroy_the_GL_context.3F
	case WM_CLOSE: 
		wglMakeCurrent(GetDC(hwnd), NULL);
		wglDeleteContext(new_hglrc);
		// DefWindowProc just calls DestroyWindow() here.
		return DefWindowProcW(hwnd, uMsg, wParam, lParam); 

	case WM_DESTROY: 
		// Clean up window-specific data objects. 
		PostQuitMessage(0);
		return 0;
	
	// TODO follow https://www.khronos.org/opengl/wiki/Platform_specifics:_Windows#When_do_I_render_my_scene.3F
	// especially regarding Alt+Tab behaviour.

	case WM_SIZE: 
	case WM_SIZING: 
		// Set the size and position of the window. 

	case WM_PAINT: 
		// Paint the window's client area. 
		glClearColor(1,0,0,1);
		glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
		//printf("Swapping buffers\n");
		SwapBuffers(GetDC(hwnd));
		return 0; 

	// https://www.khronos.org/opengl/wiki/Platform_specifics:_Windows#It.27s_flickering.21_Why.3F
	case WM_ERASEBKGND:
		return TRUE;

	case WM_SETCURSOR:
		// Sent to a window if the mouse causes the cursor to move within a window and mouse input is not captured. 

	case WM_ACTIVATE:
		switch(LOWORD(wParam)) {
		case WA_ACTIVE: break;
		case WA_CLICKACTIVE: break;
		case WA_INACTIVE: break;
		}
		//bool isMinimized = HIWORD(wParam);
		return DefWindowProcW(hwnd, uMsg, wParam, lParam); 

	case WM_CHAR:
	case WM_DEADCHAR:
	case WM_SYSDEADCHAR:
	case WM_UNICHAR:

	case WM_APPCOMMAND: // Special keyboard buttons like play, pause, etc.
	case WM_HOTKEY: // Not super interesting. Posted when the user presses a hot key registered by the RegisterHotKey function.

	case WM_SYSKEYDOWN:
	case WM_SYSKEYUP:
	case WM_KEYDOWN:
	case WM_KEYUP: {
		DWORD vkey = wParam;
		uint32_t repeat_count = lParam & 0xf;
		uint32_t scancode = (lParam>>16) & 0x7f;
		bool is_extended = (lParam>>24) & 1;
		bool context_code = (lParam>>29) & 1;
		bool was_down = (lParam>>30) & 1;
		bool transition_state = (lParam>>31) & 1;
		return 0;
	}

	case WM_KILLFOCUS:
		// Sent to a window immediately before it loses the keyboard focus.
	case WM_SETFOCUS:
		// Sent to a window after it has gained the keyboard focus.
	case WM_CAPTURECHANGED:
		// Sent to the window that is losing the mouse capture.
		// Window should redraw itself.

	case WM_MOUSEMOVE:
	case WM_LBUTTONDBLCLK:
	case WM_LBUTTONDOWN:
	case WM_LBUTTONUP:
	case WM_MBUTTONDBLCLK:
	case WM_MBUTTONDOWN:
	case WM_MBUTTONUP:
	case WM_RBUTTONDBLCLK:
	case WM_RBUTTONDOWN:
	case WM_RBUTTONUP:
	case WM_XBUTTONDBLCLK:
	case WM_XBUTTONDOWN:
	case WM_XBUTTONUP:
	case WM_MOUSEWHEEL:
	case WM_MOUSEHWHEEL:
	{
		int xPos = GET_X_LPARAM(lParam); 
		int yPos = GET_Y_LPARAM(lParam); 
		bool is_ctrl_down  = !!(GET_KEYSTATE_WPARAM(wParam) & MK_CONTROL);
		bool is_lmb_down   = !!(GET_KEYSTATE_WPARAM(wParam) & MK_LBUTTON);
		bool is_mmb_down   = !!(GET_KEYSTATE_WPARAM(wParam) & MK_MBUTTON);
		bool is_rmb_down   = !!(GET_KEYSTATE_WPARAM(wParam) & MK_RBUTTON);
		bool is_shift_down = !!(GET_KEYSTATE_WPARAM(wParam) & MK_SHIFT);
		bool is_xmb1_down  = !!(GET_KEYSTATE_WPARAM(wParam) & MK_XBUTTON1);
		bool is_xmb2_down  = !!(GET_KEYSTATE_WPARAM(wParam) & MK_XBUTTON2);
		if(uMsg == WM_XBUTTONDBLCLK
		|| uMsg == WM_XBUTTONDOWN
		|| uMsg == WM_XBUTTONUP) {
			unsigned xbtn_number = GET_XBUTTON_WPARAM(wParam); // 1 or 2.
			return TRUE;
		}
		if(uMsg == WM_MOUSEWHEEL || uMsg == WM_MOUSEHWHEEL) {
			// Positive = forward, away from the user.
			int delta = GET_WHEEL_DELTA_WPARAM(wParam);
			int steps = delta / WHEEL_DELTA;
		}

		return 0;
	}


	case WM_MOUSEACTIVATE:
	case WM_MOUSEHOVER:
	case WM_MOUSELEAVE:

	// NOTE: NC stands for "Non-Client area".
	case WM_NCHITTEST:
	case WM_NCLBUTTONDBLCLK:
	case WM_NCLBUTTONDOWN:
	case WM_NCLBUTTONUP:
	case WM_NCMBUTTONDBLCLK:
	case WM_NCMBUTTONDOWN:
	case WM_NCMBUTTONUP:
	case WM_NCMOUSEHOVER:
	case WM_NCMOUSELEAVE:
	case WM_NCMOUSEMOVE:
	case WM_NCRBUTTONDBLCLK:
	case WM_NCRBUTTONDOWN:
	case WM_NCRBUTTONUP:
	case WM_NCXBUTTONDBLCLK:
	case WM_NCXBUTTONDOWN:
	case WM_NCXBUTTONUP:

	case WM_CLEAR:
	case WM_COPY:
	case WM_CUT:
	case WM_PASTE:
	case WM_CLIPBOARDUPDATE:
	 // See AddClipboardFormatListener
	
		return DefWindowProcW(hwnd, uMsg, wParam, lParam); 
	
	default: 
		return DefWindowProcW(hwnd, uMsg, wParam, lParam); 
	} 
	return 0; 
}

int APIENTRY wWinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, 
	PWSTR lpszCmdLine, int nCmdShow) 
{ 
	MSG msg;
	BOOL bRet; 
	WNDCLASSEXW wc; 
	UNREFERENCED_PARAMETER(lpszCmdLine); 
 
	// Register the window class for the main window. 
 
	static const WCHAR* class_name = L"MainWndClass";
	if (!hPrevInstance) 
	{ 
		wc.cbSize = sizeof(WNDCLASSEX);
		wc.style = CS_OWNDC | CS_HREDRAW | CS_VREDRAW; // NOTE not CS_DBLCLICK yet. see doc for WM_LBUTTONDBLCLK, it generates four messages instead of two when double-clicking.
		wc.lpfnWndProc = (WNDPROC) WndProc; 
		wc.cbClsExtra = 0; 
		wc.cbWndExtra = 0; 
		wc.hInstance = hInstance; 
		wc.hIcon = LoadIcon((HINSTANCE) NULL, 
			IDI_APPLICATION); 
		wc.hCursor = LoadCursor((HINSTANCE) NULL, 
			IDC_ARROW); 
		wc.hbrBackground = NULL; //GetStockObject(WHITE_BRUSH); 
		wc.lpszMenuName =  L"MainMenu"; 
		wc.lpszClassName = class_name;
		wc.hIconSm = NULL;
 
		if (!RegisterClassExW(&wc)) 
			return FALSE; 
	} 
 
	HINSTANCE hinst = hInstance;  // save instance handle 
 
	HWND hwnd = CreateWindowExW(0, class_name, L"Hello world!", 
		WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, CW_USEDEFAULT, 
		CW_USEDEFAULT, CW_USEDEFAULT, (HWND) NULL, 
		(HMENU) NULL, hinst, (LPVOID) NULL); 
 
	if (!hwnd) 
		return FALSE; 

	ShowWindow(hwnd, nCmdShow); 
	UpdateWindow(hwnd);
 
	while( (bRet = GetMessageW( &msg, NULL, 0, 0 )) != 0)
	{ 
		if (bRet == -1)
		{
			// TODO handle the error and possibly exit
		}
		else
		{
			TranslateMessage(&msg); 
			DispatchMessageW(&msg); 
		}
	} 
 
	// Return the exit code to the system. 
 
	return msg.wParam; 
} 

