/*
 * self_recompiling.c - A sample program that recompiles itself 
 * and replaces its own code as soon as its source file is modified, 
 * using almost only Linux system calls (inotify, clone, fork, exec).
 *
 * Compile and link with :
 *     cc self_recompiling.c -o self_recompiling
 *
 * This source file is released under the CC0 license,
 * which in short means you can do whatever you want with it.
 * (See http://creativecommons.org/publicdomain/zero/1.0/)
 *
 */

#define _GNU_SOURCE 1

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <errno.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <sys/inotify.h>
#include <limits.h>
#include <fcntl.h>
#include <sched.h>
#include <unistd.h>


int watcher(void *arg);
int program(int argc, char *argv[]);


#define STACK_SIZE 1048576 /* 1024*1024 (See EXAMPLE at clone(2))*/

int main(int argc, char *argv[])
{
    clone(&watcher, (char*)malloc(STACK_SIZE) + STACK_SIZE, 
          CLONE_FS|CLONE_FILES|CLONE_IO|CLONE_VM|CLONE_THREAD|CLONE_SIGHAND,
          argv);
    _exit(program(argc, argv));
}

#undef STACK_SIZE
#undef _GNU_SOURCE


/* Quoting inotify(7) : 
 *
 *     The behaviour when the buffer given to read(2) is too small to return
 *     information about the next event depends on the kernel version:
 *     in kernels before 2.6.21, read(2) returns 0; since kernel 2.6.21,
 *     read(2) fails with the error EINVAL. Specifying a buffer of size
 *
 *         sizeof(struct inotify_event) + NAME_MAX + 1
 *
 *     will be sufficient to read at least one event.
 * */
#define INEV_SIZE (sizeof(struct inotify_event)+NAME_MAX+1)
#define CC "/usr/bin/cc"

int watcher(void *arg)
{
    char *const *argv = arg;
    int infd, watchfd, forkid;
    struct inotify_event *inev;
    char *srcpath;

    srcpath = malloc(strlen(argv[0])+3);
    strcpy(srcpath, argv[0]);
    strcat(srcpath, ".c");

    infd = inotify_init1(IN_CLOEXEC);
    watchfd = inotify_add_watch(infd, srcpath, IN_MODIFY);

    if(watchfd == -1)
    {
        write(2, "Could not watch ", 16);
        write(2, srcpath, strlen(srcpath));
        write(2, ": ", 2);
        perror(NULL);
        _exit(errno);
    }

    inev = malloc(INEV_SIZE);
    while(read(infd, inev, INEV_SIZE) <= 0);
    free(inev);

    forkid = fork();
    if(forkid > 0)
        waitpid(forkid, NULL, 0);
    else if(forkid == 0)
        execl(CC, CC, srcpath, "-o", argv[0], NULL);

    free(srcpath);

    execv(argv[0], argv);

    /* This return statement is never actually reached, 
     * but avoids a compiler warning. */
    return EXIT_SUCCESS;
}

#undef CC
#undef INEV_SIZE


/* The "real" program goes here. 
 * Consider this as the regular main function. */

int program(int argc, char *argv[])
{
    for(;;)
    {
        write(1, "foo ", 4);
        usleep(1000000);
    }
}

