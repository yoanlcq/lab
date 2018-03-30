// gcc xcomposite_detect.c -lX11 -lXcomposite

#include <stdio.h>
#include <X11/Xlib.h>
#include <X11/extensions/Xcomposite.h>

int main() {
    int error_base, event_base;
    Display *dpy = XOpenDisplay(NULL);
    Bool has_xcomposite = XCompositeQueryExtension(dpy, &event_base, &error_base);
    printf("%d", (int) has_xcomposite);
    return 0;
}
