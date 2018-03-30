#define _GNU_SOURCE
#include <dlfcn.h>

int main() {
    for(;;) {
        void* dl = dlopen("./libtalk.so", RTLD_NOW);
        void (*talk)() = dlsym(dl, "talk");
        talk();
        dlclose(dl);
        sleep(1);
    }
    return 0;
}
