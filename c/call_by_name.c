/*
 * This source file shows how one can somewhat call "code by strings" in C.
 * It is not possible as-is, but there is a cool way to call functions by their
 * name; it might be a nice first step to making an interpreter.
 *
 * There's no Windows version of this source, though porting is trivial here.
 * Compile like so :
 * gcc -rdynamic -D_GNU_SOURCE call_by_name.c -ldl
 *
 */

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <dlfcn.h>

void print_Dl_info(const void *addr) {
#ifdef _GNU_SOURCE
    Dl_info info;
    if(dladdr(addr, &info)) {
        printf(
            "Module name : %s\n"
            "Symbol name : %s\n"
            "Module addr : %p\n"
            "Symbol addr : %p\n",
            info.dli_fname,
            info.dli_sname,
            info.dli_fbase,
            info.dli_saddr
        );
    }
#endif
}

void foo(void) {
    puts("Hello from foo!");
}
void bar(void) {
    puts("Hello from bar!");
}

#define NFUNCS 3
const char *allfuncs[NFUNCS] = {"main", "foo", "bar"};

void call(const char *name) {
    char *err;
    void (*func)(void);
    static void *handle = NULL;

    if(!name) {
        if(handle)
            dlclose(handle);
        return;
    }
    if(!handle)
        handle = dlopen(NULL, RTLD_NOW | RTLD_GLOBAL);

    dlerror();
    func = dlsym(handle, name);
    err = dlerror();
    
    if(err) {
        fprintf(stderr, "Could not resolve symbol :\n\t%s\n", err);
        return;
    }
    if(!func)
        return;
    
    print_Dl_info(func);
    func();
}

int main(void) {

    unsigned i;
    puts("Registered functions :");
    for(i=0 ; i<NFUNCS ; ++i)
        puts(allfuncs[i]);

    char buf[BUFSIZ];
    for(;;) {
        fputs("Enter the name of the function you would like to call "
              "('exit' should work too) :\n> ", stdout);
        fgets(buf, BUFSIZ, stdin);
        *strchr(buf, '\n') = '\0';
        call(buf);
    }
    call(NULL);
    return EXIT_SUCCESS;
}
