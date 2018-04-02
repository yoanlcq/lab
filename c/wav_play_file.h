#ifndef WAV_IMPLEMENTATION
#pragma once
#endif

#include <stdbool.h>

#ifdef __linux__
extern const unsigned WAV_MAX_PLAYING_LINUX;
#endif

bool wav_play_file(const char* path);
void wav_stop_all();


#ifdef WAV_IMPLEMENTATION

#ifdef __linux__

#include <stdio.h>
#include <unistd.h>
#include <sys/types.h>
#include <signal.h>
#include <errno.h>

const unsigned WAV_MAX_PLAYING_LINUX = 8;

static pid_t s_aplay_pids[WAV_MAX_PLAYING_LINUX];
static unsigned s_aplay_pids_count = 0;

bool wav_play_file(const char* path) {
    if(s_aplay_pids_count >= WAV_MAX_PLAYING_LINUX) {
        fprintf(stderr, "%s: Cannot play more that %u simultaneously!\n", __func__, WAV_MAX_PLAYING_LINUX);
        return false;
    }
    const pid_t pid = fork();
    switch(pid) {
    case 0:
        if(-1 == execlp("aplay", "aplay", path, NULL)) {
            perror("wav_play_file(): execlp");
            _exit(EXIT_FAILURE);
        }
        break;
    case -1:
        perror("wav_play_file(): fork");
        return false;
    }
    printf("%s: Spawned child aplay process %d (playing `%s`)\n", __func__, pid, path);
    s_aplay_pids[s_aplay_pids_count++] = pid;
    return true;
}

void wav_stop_all() {
    for(; s_aplay_pids_count ; --s_aplay_pids_count) {
        const pid_t pid = s_aplay_pids[s_aplay_pids_count-1];
        printf("%s: Sending SIGKILL to child aplay process %d\n", __func__, pid);
        kill(pid, SIGKILL);
    }
}

#elif defined(_WIN32)

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <windows.h>

// Returns 0 if it fails.
static int utf8_to_ws_capacity(const char* utf8, unsigned utf8_capacity) {
    return MultiByteToWideChar(CP_UTF8, MB_ERR_INVALID_CHARS, utf8, utf8_capacity, NULL, 0);
}
// Same as above
static int utf8_to_ws(const char* utf8, unsigned utf8_capacity, WCHAR* ws, unsigned ws_capacity) {
    return MultiByteToWideChar(CP_UTF8, MB_ERR_INVALID_CHARS, utf8, utf8_capacity, ws, ws_capacity);
}

static WCHAR* strwdup(const char* utf8) {
    const unsigned utf8_capacity = strlen(utf8)+1;
    const int ws_capacity = utf8_to_ws_capacity(utf8, utf8_len);
    if(ws_capacity <= 0) {
        return NULL;
    }
    WCHAR* ws = malloc(ws_capacity * sizeof(WCHAR));
    if(!ws) {
        return NULL;
    }
    int status = utf8_to_ws(utf8, utf8_len, ws, ws_capacity);
    if(status <= 0) {
        free(ws);
        return NULL;
    }
    return ws;
}

bool wav_play_file(const char* path) {
    const DWORD flags = SND_FILENAME | SND_ASYNC | SND_NODEFAULT | SND_NOSTOP;
    const WCHAR* wpath = strwdup(path);
    if(!wpath) {
        fprintf(stderr, "%s: Could not convert `%s` as UTF-8 to a wide string\n", __func__, path);
        return false;
    }
	const BOOL success = PlaySoundW(wpath, NULL, flags);
    free(wpath);
    if(success) {
        printf("%s: Playing `%s`\n", __func__, path);
        return true;
    }
    DWORD err = GetLastError();
    WCHAR* error_text;
    FormatMessageW(
        FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_IGNORE_INSERTS,
        NULL, err, MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT),
        (WCHAR*) &error_text, 0, NULL
    );
    fprintf("%s: Could not play `%s`: PlaySoundW error %u: %ls\n", __func__, path, err, error_text);
    LocalFree(error_text);
    return false;
}
void wav_stop_all() {
    PlaySoundW(NULL, NULL, 0);
}

#else
#error Unsupported platform
#endif // platform-specific

#endif // WAV_IMPLEMENTATION
