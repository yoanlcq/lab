#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <limits.h>
#include <string.h>
#include <assert.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <errno.h>
#include <libudev.h>
#include <linux/input.h>
#include <libevdev-1.0/libevdev/libevdev.h>
#include <linux/joystick.h>

#define hope assert

// Those that interest us are:
// - DEVNAME = ... // open this with libevdev - devlinks are symlinks to it
//   // + is it /dev/input/js* (old) or /dev/input/event* (new) ?
// - ID_INPUT_JOYSTICK = 1
// - ID_MODEL = Controller
// - ID_MODEL_ID = 028e
// - ID_SERIAL
// - ID_VENDOR
// - NAME (see parent if any)

/*
void print_devlinks(struct udev_device *dev) {
    struct udev_list_entry *entries = udev_device_get_devlinks_list_entry(dev);
    struct udev_list_entry *entry;
    udev_list_entry_foreach(entry, entries) {
        printf("- %s\n", udev_list_entry_get_name(entry));
    }
}
*/
void print_properties(struct udev_device *dev) {
    struct udev_list_entry *entries = udev_device_get_properties_list_entry(dev);
    struct udev_list_entry *entry;
    udev_list_entry_foreach(entry, entries) {
        const char *key, *value;
        key = udev_list_entry_get_name(entry);
        value = udev_list_entry_get_value(entry);
        printf("- `%s` = `%s`\n", key, value);
    }
}
/*
void print_tags(struct udev_device *dev) {
    struct udev_list_entry *entries = udev_device_get_tags_list_entry(dev);
    struct udev_list_entry *entry;
    udev_list_entry_foreach(entry, entries) {
        printf("- %s\n", udev_list_entry_get_name(entry));
    }
}
*/

// On USB Xbox360 controller (name = "Microsoft X-Box 360 pad"):
// Button 0 => A
// Button 1 => B
// Button 2 => X
// Button 3 => Y
// Button 4 => LSHOULDER
// Button 5 => RSHOULDER
// Button 6 => Back
// Button 7 => Start
// Button 8 => Xbox Menu
// Axis 0 => Lstick X
// Axis 1 => Lstick Y (downwards)
// Axis 2 => LTRIGGER
// Axis 3 => Rstick X
// Axis 4 => Rstick Y (downwards)
// Axis 5 => RTRIGGER
// Axis 6 => Dpad left/right
// Axis 7 => Dpad up/down (downwards)
void try_joydev(const char *devnode) {
    int fd = open(devnode, O_RDONLY | O_NONBLOCK, 0666);
    if(fd == -1) {
        fprintf(stderr, "Failed to open `%s` (error %i)\n", devnode, errno);
        return;
    }
    char axis_count, button_count, name[512];
    int driver_version;
    ioctl(fd, JSIOCGAXES, &axis_count);
    ioctl(fd, JSIOCGBUTTONS, &button_count);
    ioctl(fd, JSIOCGVERSION, &driver_version);
    ioctl(fd, JSIOCGNAME(sizeof name), name);

    printf(
            "---- Linux Joystick\n"
            "---- axis_count: %hhi\n"
            "---- button_count: %hhi\n"
            "---- driver_version: %i\n"
            "---- name: %s\n"
            , axis_count, button_count, driver_version, name
            );

    struct js_event ev;
    for(;;) {
        ssize_t bytes_read = read(fd, &ev, sizeof ev);
        if(bytes_read <= 0) {
            if(errno == EAGAIN) {
                usleep(100000);
                continue;
            }
        }
        if(ev.type & JS_EVENT_INIT)
            continue;
        // ev.time in milliseconds
        switch(ev.type) {
        case JS_EVENT_BUTTON:
            printf("[%u] Button %i = %hi\n", ev.time, ev.number, ev.value);
            break;
        case JS_EVENT_AXIS: 
            printf("[%u] Axis %i = %hi\n", ev.time, ev.number, ev.value);
            break;
        }
    }
    close(fd);
}


void handle_input_event(const struct input_event *ev) {
    struct timeval time = ev->time;
    uint16_t type = ev->type, code = ev->code;
    int32_t value = ev->value;
    // TODO: we're probably able to classify an input device
    // by asking if it supports the evnt codes:
    // - BTN_WHEEL,
    // - BTN_GAMEPAD
    // - BTN_JOYSTICK
    // - BTN_MISC
    switch(type) {
    case EV_SYN:      break;
    case EV_KEY:
        switch(code) {
        #define CASE(c) case BTN_##c: printf("%s key %s\n", #c, value ? "pressed": "released"); break;

        // Misc
        CASE(0)
        CASE(1)
        CASE(2)
        CASE(3)
        CASE(4)
        CASE(5)
        CASE(6)
        CASE(7)
        CASE(8)
        CASE(9)

        // Wheels
        CASE(GEAR_DOWN)
        CASE(GEAR_UP)

        // Joysticks
        CASE(TRIGGER)
        CASE(THUMB)
        CASE(THUMB2)
        CASE(TOP)
        CASE(TOP2)
        CASE(PINKIE)
        CASE(DEAD)
        CASE(BASE)
        CASE(BASE2)
        CASE(BASE3)
        CASE(BASE4)
        CASE(BASE5)
        CASE(BASE6)

        // Gamepads
        CASE(A)
        CASE(B)
        CASE(C)
        CASE(X)
        CASE(Y)
        CASE(Z)
        CASE(TL)
        CASE(TR)
        CASE(TL2)
        CASE(TR2)
        CASE(SELECT)
        CASE(START)
        CASE(MODE)
        CASE(THUMBL)
        CASE(THUMBR)
        CASE(DPAD_UP)
        CASE(DPAD_DOWN)
        CASE(DPAD_LEFT)
        CASE(DPAD_RIGHT)
        #undef CASE
        default:
            printf("Key %#hx = %i\n", code, value);
            break;
        }
        break;
    case EV_REL:
        switch(code) {
        #define CASE(c) case REL_##c: printf("REL_%s = %i\n", #c, value); break;
        CASE(X			)
        CASE(Y			)
        CASE(Z			)
        CASE(RX			)
        CASE(RY			)
        CASE(RZ			)
        CASE(MISC		)
        #undef CASE
        default:
            printf("Rel %#hx = %i\n", code, value);
            break;
        }
        break;
    case EV_ABS:
        // Xbox 360 reports dpad as HAT0X and HAT0Y (downwards).
        switch(code) {
        #define CASE(c) case ABS_##c: printf("ABS_%s = %i\n", #c, value); break;
        CASE(X			)
        CASE(Y			)
        CASE(Z			)
        CASE(RX			)
        CASE(RY			)
        CASE(RZ			)
        CASE(THROTTLE	)
        CASE(RUDDER		)
        CASE(WHEEL		)
        CASE(GAS		)
        CASE(BRAKE		)
        CASE(HAT0X		)
        CASE(HAT0Y		)
        CASE(HAT1X		)
        CASE(HAT1Y		)
        CASE(HAT2X		)
        CASE(HAT2Y		)
        CASE(HAT3X		)
        CASE(HAT3Y		)
        CASE(PRESSURE	)
        CASE(DISTANCE	)
        CASE(TILT_X		)
        CASE(TILT_Y		)
        CASE(TOOL_WIDTH	)
        CASE(VOLUME		)
        CASE(MISC		)
        #undef CASE
        default:
            printf("Abs %#hx = %i\n", code, value);
            break;
        }
        break;
    case EV_MSC:
        switch(code) {
        case MSC_TIMESTAMP: printf("Timestamp: %i\n", value); break;
        }
        break;
    case EV_SW :      break;
    case EV_LED:      break;
    case EV_SND:      break;
    case EV_REP:      break;
    case EV_FF :      break;
    case EV_PWR:      break;
    case EV_FF_STATUS: break;
    }
}

void try_evdev(const char *devnode) {
    // RDWR to send force feedback effects
    int fd = open(devnode, O_RDWR | O_NONBLOCK, 0666);
    if(fd == -1) {
        fprintf(stderr, "Failed to open `%s` (error %i)\n", devnode, errno);
        return;
    }
    struct libevdev *evdev = NULL;
    int status = libevdev_new_from_fd(fd, &evdev);
    if(status < 0) {
        fprintf(stderr, "`%s` is not an evdev device (error %i)\n",
                devnode, -status);
        close(fd);
        return;
    }

    // Call libevdev_next_event() first
    for(;;) {
        struct input_event ev;
        status = libevdev_next_event(evdev, LIBEVDEV_READ_FLAG_NORMAL, &ev);
        if(status == -EAGAIN) {
            usleep(100000);
            continue;
        }
        switch(status) {
        case LIBEVDEV_READ_STATUS_SUCCESS: 
            handle_input_event(&ev);
            continue;
        case LIBEVDEV_READ_STATUS_SYNC: 
            for(;;) {
                status = libevdev_next_event(evdev, LIBEVDEV_READ_STATUS_SYNC, &ev);
                handle_input_event(&ev);
                if(status == -EAGAIN) {
                    usleep(100000);
                    continue;
                }
            }
            break;
        }
        fprintf(stderr, "Unknown error %i\n", -status);
        break;
    }

    const char *name = libevdev_get_name(evdev);
    int product_id = libevdev_get_id_product(evdev);
    int vendor_id = libevdev_get_id_vendor(evdev);
    int code = EV_KEY, type = BTN_THUMBL;
    // Those are useless cache misses for get_abs_info()
    // int abs_min = libevdev_get_abs_minimum(evdev, code);
    // int abs_max = libevdev_get_abs_maximum(evdev, code);
    // int abs_fuzz = libevdev_get_abs_fuzz(evdev, code);
    // int abs_flat = libevdev_get_abs_flat(evdev, code);
    // int abs_resolution = libevdev_get_abs_resolution(evdev, code);
    struct input_absinfo abs_info = {0};
    const struct input_absinfo *abs_info_ptr = libevdev_get_abs_info(evdev, code);
    if(abs_info_ptr)
        abs_info = *abs_info_ptr;
    int has_type = libevdev_has_event_type(evdev, type);
    int has_code = libevdev_has_event_code(evdev, type, code);

    int value = libevdev_get_event_value(evdev, type, code);
    // useless, a shortcut
    //int foo = libevdev_fetch_event_value(evdev, type, code, &value);
    int repeat_delay, repeat_period;
    status = libevdev_get_repeat(evdev, &repeat_delay, &repeat_period);

    printf(
            "---- EVDEV\n"
            "---- name: %s\n"
            "---- product_id: %i\n"
            "---- vendor_id: %i\n"
            "---- axis value: %i\n"
            "---- abs_min: %i\n"
            "---- abs_max: %i\n"
            "---- abs_fuzz: %i\n"
            "---- abs_resolution: %i\n"
            "---- repeat_delay: %i\n"
            "---- repeat_period: %i\n",
            name, product_id, vendor_id, 
            abs_info.value,
            abs_info.minimum, abs_info.maximum, abs_info.fuzz,
            abs_info.resolution, repeat_delay, repeat_period
            );


    // TODO try this:
    // #define EVIOCGVERSION		_IOR('E', 0x01, int)			/* get driver version */
    // EVIOCGKEY,
    // EVIOCGABS
    // EVIOCSFF
    // EVIORMFF
    // EVIOCGEFFECTS
    
    struct ff_effect ff = {0};
    ff.id = -1; // id must always be -1 ?? fails with EINVAL otherwise
    ff.replay.length = 1000; // milliseconds
    ff.replay.delay = 0;
    ff.type = FF_RUMBLE;
    ff.u.rumble.strong_magnitude = INT16_MAX;
    ff.u.rumble.weak_magnitude = INT16_MAX;
    //ff.constant.envelope = 

    if (ioctl(fd, EVIOCSFF, &ff) == -1) {
        perror("upload effect");
    }

    struct input_event play = {0};
    play.type = EV_FF;
    play.code = ff.id;
    play.value = 1;
retry_write:
    if(write(fd, &play, sizeof play) == -1) {
        if(errno == EAGAIN)
            goto retry_write;
        perror("playing rumble");
    }

    sleep(2);

    if (ioctl(fd, EVIOCRMFF, ff.id) == -1) {
        perror("remove effect");
    }

    close(fd);
    libevdev_free(evdev);
}

enum backend {
    NONE,
    EVDEV,
    JOYDEV,
};

// NOTE: can't use pointers as IDs
void frob_device(struct udev_device *dev) {
    const char *s_is_joystick = udev_device_get_property_value(dev, "ID_INPUT_JOYSTICK");
    if(!s_is_joystick || !strtol(s_is_joystick, NULL, 0))
       return;
    //const char *devnode = udev_device_get_property_value(dev, "DEVNAME");
    const char *devnode = udev_device_get_devnode(dev);
    if(!devnode)
        devnode=""; //return;
    enum backend backend = NONE;
    const char *backend_name = "unknown";
    if(strstr(devnode, "/event"))
        backend = EVDEV, backend_name = "evdev";
    if(strstr(devnode, "/js"))
        backend = JOYDEV, backend_name = "linux_joystick";
    if(!backend)
        return;
    const char *name = udev_device_get_property_value(dev, "NAME");
    if(!name) {
        struct udev_device *parent = udev_device_get_parent(dev);
        if(parent)
            name = udev_device_get_property_value(parent, "NAME");
    }
    const char *model = udev_device_get_property_value(dev, "ID_MODEL");
    // const char *kind = "Unknown";
    // We can't base ourselves on this.
    // For instance it was Dual_Analog_Pad on one of my gamepads.
    // if(model && !strcasecmp(model, "Controller"))
    //    kind = "Controller";
    const char *serial = udev_device_get_property_value(dev, "ID_SERIAL");
    const char *vendor = udev_device_get_property_value(dev, "ID_VENDOR");

    long long usec = udev_device_get_usec_since_initialized(dev);

    // NOTE: name is already quoted, for some reason, here
    printf("------\n"
           "devnode: \"%s\"\n"
           "backend: %s\n"
           //"kind: %s\n"
           "name: %s\n"
           "vendor: \"%s\"\n"
           "serial: \"%s\"\n"
           "usec_since_initialized: %lli\n"
           , devnode, backend_name, /*kind,*/ name, vendor, serial, usec);

    printf("properties:\n");
    print_properties(dev);

    const char *underlying_dev_id = udev_device_get_property_value(dev, "ID_FOR_SEAT");

    if(backend == EVDEV)
        try_evdev(devnode);
    if(backend == JOYDEV)
        try_joydev(devnode);

    //printf("devpath: %s\n", udev_device_get_devpath(dev));
    //printf("devlinks:\n");
    //print_devlinks(dev);
    //printf("tags:\n");
    //print_tags(dev);
}

void monitor(struct udev *udev) {
    struct udev_monitor *mon = udev_monitor_new_from_netlink(udev, "udev");
    int status = udev_monitor_enable_receiving(mon);
    for(;;) {
        struct udev_device *dev = udev_monitor_receive_device(mon);
        if(!dev) {
            // printf("Got no device\n");
            // 0.01 seconds
            usleep(10000);
        } else {
            const char *syspath = udev_device_get_syspath(dev);
            const char *action = udev_device_get_action(dev);
            if(!strcmp(action, "add")) {
                printf("Added %s\n", syspath);
            } else if(!strcmp(action, "remove")) {
                printf("Removed %s\n", syspath);
            } else if(!strcmp(action, "change")) {
                printf("Changed %s\n", syspath);
            } else if(!strcmp(action, "online")) {
                printf("Now online: %s\n", syspath);
            } else if(!strcmp(action, "offline")) {
                printf("Now offline: %s\n", syspath);
            } else {
                printf("(Unknown action: \"%s\") %s\n", action, syspath);
            }
            udev_device_unref(dev);
        }
    }
    udev_monitor_unref(mon);
}

int main() {

    struct udev *udev = udev_new();
    // monitor(udev);
    // return 0;

    struct udev_enumerate *enumerate = udev_enumerate_new(udev);
    hope(udev_enumerate_add_match_subsystem(enumerate, "input") >= 0);
    hope(udev_enumerate_scan_devices(enumerate) >= 0);
    struct udev_list_entry *entries = udev_enumerate_get_list_entry(enumerate);

    struct udev_list_entry *entry;
    udev_list_entry_foreach(entry, entries) {
        const char *syspath = udev_list_entry_get_name(entry);
        //printf("---\nsyspath: %s\n", syspath);
        struct udev_device *dev = udev_device_new_from_syspath(udev, syspath);
        frob_device(dev);
        udev_device_unref(dev);
        //printf("---\n");
    }
    udev_enumerate_unref(enumerate);
    udev_unref(udev);
    return 0;
}
