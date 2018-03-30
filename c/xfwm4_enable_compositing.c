// See https://wiki.archlinux.org/index.php/xorg#Composite

#include <stdbool.h>
#include <stdio.h>
#include <string.h>

// Needs libxfconf-0-dev.
// Alternatively, could dynamically load it.
#define USE_LIBXFCONF 0


#if USE_LIBXFCONF
#include <xfce4/xfconf-0/xfconf/xfconf.h>
// TODO
#else
#define unknown 2 // HACK
static bool is_wm_composite_enabled(void) {
    FILE* p = popen("xfconf-query -c xfwm4 -p /general/use_compositing", "r");
    char buf[128];
    fgets(buf, sizeof buf, p);
    pclose(p);
    if(strstr(buf, "true"))
        return true;
    else if(strstr(buf, "false"))
        return false;
    return unknown;
}
// We don't need to check the previous value beforehand: if the setting
// is redundant, the screen won't blink.
bool enable_wm_composite(void) {
    FILE* p = popen("xfconf-query -c xfwm4 -p /general/use_compositing -s true", "r");
    pclose(p);
}
bool disable_wm_composite(void) {
    FILE* p = popen("xfconf-query -c xfwm4 -p /general/use_compositing -s false", "r");
    pclose(p);
}
#endif

int main(void) {
    enable_wm_composite();
}
