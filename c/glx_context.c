/*
 * This file is for testing GLX features for my engine.
 * I used the boilerplate from :
 * http://opensource.apple.com/source/X11server/X11server-85/mesa/Mesa-7.2/progs/xdemos/glxdemo.c
 * Compile with : -lGL -lX11 -lXxf86vm
 *
 */

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <GL/gl.h>
#include <GL/glx.h>
#include <GL/glxext.h>
#include <SFML/Window.h>
#include <X11/extensions/xf86vmode.h>


static void redraw( Display *dpy, Window w )
{
    printf("Redraw event\n");
    glClear( GL_COLOR_BUFFER_BIT );
    glColor3f( 1.0, 1.0, 0.0 );
    glRectf( -0.8, -0.8, 0.8, 0.8 );
    glXSwapBuffers( dpy, w );
}

static void resize( unsigned int width, unsigned int height )
{
    printf("Resize event\n");
    glViewport( 0, 0, width, height );
    glMatrixMode( GL_PROJECTION );
    glLoadIdentity();
    glOrtho( -1.0, 1.0, -1.0, 1.0, -1.0, 1.0 );
}

static bool str_is_in(const char *str, const char *list) {
    const char *spc;
    for(;;) {
        spc = strchr(list, ' ');
        if(!spc)
            spc = strchr(list, '\0');
        if(!strncmp(str, list, spc-list)) {
            return true;
        }
        if(*spc == '\0')
            break;
        list = spc+1;
    }
    return false;
}

static Window make_rgb_db_window( Display *dpy,
				  unsigned int width, unsigned int height )
{
    int attrib[] = { 
        GLX_RGBA,
        GLX_RED_SIZE, 8,
        GLX_GREEN_SIZE, 8,
        GLX_BLUE_SIZE, 8,
        GLX_ALPHA_SIZE, 8,
        GLX_DEPTH_SIZE, 24,
        GLX_STENCIL_SIZE, 8,
        GLX_DOUBLEBUFFER,
        None };
    int scrnum;
    XSetWindowAttributes attr;
    unsigned long mask;
    Window root;
    Window win;
    GLXContext ctx;
    XVisualInfo *visinfo;
  
    scrnum = DefaultScreen( dpy );
    root = RootWindow( dpy, scrnum );

    int glx_major, glx_minor;
    if(!glXQueryExtension(dpy, NULL, NULL)) {
        fputs("Sorry, you can't go on without glX.\n", stderr);
        exit(EXIT_FAILURE);
    }

    glXQueryVersion(dpy, &glx_major, &glx_minor);

    const char *exts, *spc;

    if(glx_major >= 1 || (glx_major==1 && glx_minor>=1))
    {
        printf("--- GLX Info ---\n"
               "    Client Vendor  : %s\n"
               "    Client Version : %s\n"
               "    Server Vendor  : %s\n"
               "    Server Version : %s\n\n",
               glXGetClientString(dpy, GLX_VENDOR),
               glXGetClientString(dpy, GLX_VERSION),
               glXQueryServerString(dpy, scrnum, GLX_VENDOR),
               glXQueryServerString(dpy, scrnum, GLX_VERSION));
        exts = glXQueryExtensionsString(dpy, scrnum);
        puts("--- GLX Extensions ---");
        for(;;) {
            spc = strchr(exts, ' ');
            if(!spc)
                spc = strchr(exts, '\0');
            fputs("    ", stdout);
            fwrite(exts, 1, spc-exts, stdout);
            putchar('\n');
            if(*spc == '\0')
                break;
            exts = spc+1;
        }
        /* There's no need for another line break */
    }

 
    visinfo = glXChooseVisual( dpy, scrnum, attrib );
    if (!visinfo) {
       printf("Error: couldn't get an RGB, Double-buffered visual\n");
       exit(1);
    }
  
    /* window attributes */
    attr.background_pixel = 0;
    attr.border_pixel = 0;
    attr.colormap = XCreateColormap( dpy, root, visinfo->visual, AllocNone);
    attr.event_mask = StructureNotifyMask | ExposureMask;
    mask = CWBackPixel | CWBorderPixel | CWColormap | CWEventMask;
  
    win = XCreateWindow( dpy, root, 0, 0, width, height,
     	        0, visinfo->depth, InputOutput,
     	        visinfo->visual, mask, &attr );

    ctx = glXCreateContext( dpy, visinfo, NULL, True );
    if (!ctx) {
       printf("Error: glXCreateContext failed\n");
       exit(1);
    }

    glXMakeCurrent( dpy, win, ctx );

    int val;
    puts("Active glX context attributes :");
#define HELPER_I(__attr__,__str__) \
        glXGetConfig(dpy, visinfo, __attr__, &val); \
        printf("    %-17s : %d\n", __str__, val);
#define HELPER_B(__attr__,__str__) \
        glXGetConfig(dpy, visinfo, __attr__, &val); \
        printf("    %-17s : %s\n", __str__, val ? "On" : "Off");
    HELPER_I(GLX_LEVEL,            "Level");
    HELPER_B(GLX_DOUBLEBUFFER,     "Double Buffering");
    HELPER_B(GLX_STEREO,           "Stereo");
    HELPER_I(GLX_AUX_BUFFERS,      "Auxiliary buffers");
    HELPER_B(GLX_RGBA,             "RGBA");
    HELPER_I(GLX_BUFFER_SIZE,      "Color buffer bits");
    HELPER_I(GLX_RED_SIZE,         "Red bits");
    HELPER_I(GLX_GREEN_SIZE,       "Blue bits");
    HELPER_I(GLX_BLUE_SIZE,        "Green bits");
    HELPER_I(GLX_ALPHA_SIZE,       "Alpha bits");
    HELPER_I(GLX_DEPTH_SIZE,       "Depth bits");
    HELPER_I(GLX_STENCIL_SIZE,     "Stencil bits");
    HELPER_I(GLX_ACCUM_RED_SIZE,   "Accum.   Red bits");
    HELPER_I(GLX_ACCUM_GREEN_SIZE, "Accum. Green bits");
    HELPER_I(GLX_ACCUM_BLUE_SIZE,  "Accum.  Blue bits");
    HELPER_I(GLX_ACCUM_ALPHA_SIZE, "Accum. Alpha bits");
#undef HELPER_I
#undef HELPER_B

    XSelectInput(dpy, win, ExposureMask|KeyPressMask|KeyReleaseMask);
    return win;
}

/* From http://tonyobryan.com/index.php?article=9 */
struct Hints {
    unsigned long flags;
    unsigned long functions;
    unsigned long decorations;
    long          inputMode;
    unsigned long status;
};
typedef struct Hints Hints;

bool fullscreen=false;

/* TODO : It doesn't bypass the Window Manager. */
static void switch_fullscreen(Display *dpy, Window win) {
    Hints hints = {0};
    Atom property;

    if(fullscreen)
        exit(1);
    fullscreen = true;
    puts("Toggling fullscreen.");
    hints.flags = 2;
    hints.decorations = 0;
    property = XInternAtom(dpy, "_MOTIF_WM_HINTS", True);
    XChangeProperty(dpy, win, property, property, 
            32, PropModeReplace, (const unsigned char*) &hints, 5);
    int i, cnt, dflscreen = DefaultScreen(dpy);
    XF86VidModeModeInfo **modesinfo;
    XF86VidModeGetAllModeLines(dpy, dflscreen, &cnt, &modesinfo);
    for(i=0 ; i<cnt ; ++i)
        printf("%hux%hu\n", modesinfo[i]->hdisplay, modesinfo[i]->vdisplay);
    XMapRaised(dpy,win);
    XF86VidModeSetViewPort(dpy,dflscreen,0,0);
    XF86VidModeSwitchToMode(dpy,dflscreen,modesinfo[0]);
    XMoveResizeWindow(dpy,win,0,0,modesinfo[0]->hdisplay,modesinfo[0]->vdisplay);
    free(modesinfo);
    resize(modesinfo[0]->hdisplay, modesinfo[0]->vdisplay);
    XGrabPointer(dpy,win,True,0,GrabModeAsync,GrabModeAsync,win,0L,CurrentTime);
    XGrabKeyboard(dpy,win,False,GrabModeAsync,GrabModeAsync,CurrentTime);
}

static void event_loop(Display *dpy)
{
    XEvent event;

    for(;;) {
        XNextEvent(dpy, &event);
        switch (event.type) {
        case Expose:
	        redraw(dpy, event.xany.window);
	        break;
	    case ConfigureNotify:
	        resize(event.xconfigure.width, event.xconfigure.height);
	        break;
        case KeyPress: 
            switch_fullscreen(dpy, event.xkey.window);
            break;
        }
    }
}



int main( int argc, char *argv[] )
{
   Display *dpy;
   Window win;

   dpy = XOpenDisplay(NULL);

   win = make_rgb_db_window( dpy, 300, 300 );

   glShadeModel( GL_FLAT );
   glClearColor( 0.5, 0.5, 0.5, 1.0 );

   XMapWindow( dpy, win );

   event_loop( dpy );
   return 0;
}

