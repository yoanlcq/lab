#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <assert.h>

#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <X11/extensions/Xrender.h>
#include <X11/cursorfont.h>

typedef struct {
    Display* dpy;
} App;

int main() {
    Display* dpy = XOpenDisplay(NULL);
    int screen_num = DefaultScreen(dpy);
    Window root = DefaultRootWindow(dpy);
    Window win = XCreateSimpleWindow(
        dpy, RootWindow(dpy, screen_num), 0, 0, 100, 100, 0, 0, 0xff8855ff
    );
    XSelectInput(dpy, win, KeyPressMask | KeyReleaseMask | KeymapStateMask);
    XMapRaised(dpy, win);
    XSync(dpy, False);
   
    XWindowAttributes winattrs;
    XGetWindowAttributes(dpy, win, &winattrs);

    Cursor hand = XCreateFontCursor(dpy, XC_pencil);
    XDefineCursor(dpy, win, hand);

    int w = 24, h = 24, hot_x = 0, hot_y = 0;
    XColor fg = {
        .pixel = 0, .red = 0xffff, .green = 0, .blue = 0,
        .flags = DoRed | DoGreen | DoBlue
    };
    XColor bg = {
        .pixel = 0, .red = 0xffff, .green = 0, .blue = 0xffff,
        .flags = DoRed | DoGreen | DoBlue
    };
    char src_img_data[] = {
        0x00, 0x00, 0x00,
        0x00, 0x00, 0x00,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
        0x00, 0xff, 0xff,
    };
    char mask_img_data[] = {
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0xff,
        0xff, 0xff, 0x00,
    };
    Pixmap src = XCreateBitmapFromData(dpy, root, src_img_data, w, h);
    Pixmap mask = XCreateBitmapFromData(dpy, root, mask_img_data, w, h);
    Cursor sun = XCreatePixmapCursor(dpy, src, mask, &fg, &bg, hot_x, hot_y);
    XDefineCursor(dpy, win, sun);

    uint32_t* pixels = malloc(w*h*4);
    for(int y=0 ; y<h ; ++y) for(int x=0 ; x<w ; ++x) {
        pixels[w*y + x] = 0x22dd0011;
    }
    
    int event_base, error_base, major = 1, minor = 0;
    assert(XRenderQueryExtension(dpy, &event_base, &error_base));
    XRenderQueryVersion(dpy, &major, &minor);
    XRenderPictFormat* pic_format = XRenderFindStandardFormat(dpy, PictStandardARGB32);
    Pixmap pix = XCreatePixmap(dpy, root, w, h, 32);
    XImage* pix_img = XCreateImage(dpy, winattrs.visual, 32, ZPixmap, 0, (void*)pixels, w, h, 32, 4*w);
    GC pix_gc = XCreateGC(dpy, pix, 0, NULL);
    XPutImage(dpy, pix, pix_gc, pix_img, 0, 0, 0, 0, w, h);
    Picture pic = XRenderCreatePicture(dpy, pix, pic_format, 0, NULL);
    Cursor fun = XRenderCreateCursor(dpy, pic, hot_x, hot_y);
    XRenderFreePicture(dpy, pic);
    XDefineCursor(dpy, win, fun);

#define NFRAMES 3
    XAnimCursor frames[NFRAMES] = {
        { .cursor = fun,  .delay = 100 },
        { .cursor = sun,  .delay = 100 },
        { .cursor = hand, .delay = 100 },
    };
    Cursor ani = XRenderCreateAnimCursor(dpy, NFRAMES, frames);
    XDefineCursor(dpy, win, ani);

    XEvent ev;
    for(;;) {
        XNextEvent(dpy, &ev);
        switch(ev.type) {
        case KeyPress:
        case KeyRelease:
            printf("Foo!\n");
            break;
        default:
            printf("Bar!\n");
            break;
        }
    }

    XFreeCursor(dpy, hand);
    XFreeCursor(dpy, sun);
    XFreePixmap(dpy, src);
    XFreePixmap(dpy, mask);
    XDestroyWindow(dpy, win);
    XCloseDisplay(dpy);
    return EXIT_SUCCESS;
}
