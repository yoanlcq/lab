#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <tchar.h>
#include <windows.h>
#include <Shlwapi.h>
#include <shellapi.h>
#include <strsafe.h>

#define CC "C:\\TDM-GCC-64\\bin\\gcc.exe"

BOOL WaitForSingleFile(_In_ LPCTTSTR lpPathName, _In_ DWORD dwNotifyFilter)
{
    UUIDCreate()
    StringFromCLSID
    GetTempPath(tmpdir, MAX_PATH);
    CreateDirectory(tmpdir+guid);
    CreateHardLink(tmpdir+guid+guid, srcpath, NULL);
    handle = FindFirstChangeNotification(tmpdir, FALSE, FILE_NOTIFY_CHANGE_LAST_WRITE);
    if(handle == INVALID_HANDLE_VALUE)
    {
        printf("OHH NO\n");
        ExitProcess(1);
    }
    WaitForSingleObject(handle, INFINITE);

    DeleteFile(tmpdir+guid+guid);
    DeleteDirectory(tmpdir+guid);


}

DWORD watcher(LPVOID arg) {
    HANDLE heap = GetProcessHeap(), handle;
    STARTUPINFO startupinfo;
    LPPROCESS_INFORMATION cc_procinfo;
    LPTSTR lpCmdLine = arg;
    LPTSTR argv0;
    LPTSTR srcpath;
    LPTSTR cc_cmdline;
    TCHAR tmpdir[MAX_PATH];
    TCHAR fullExecutablePath[MAX_PATH];

    PTSTR spc = StrChr(lpCmdLine, ' ');
    if(spc == NULL) {
        argv0 = HeapAlloc(heap, 0, lstrlen(lpCmdLine)+1);
        StringCbCat(argv0, lstrlen(lpCmdLine), lpCmdLine);
    } else {
        argv0 = HeapAlloc(heap, 0, spc-lpCmdLine+1);
        StringCbCat(argv0, spc-lpCmdLine, lpCmdLine);
    }

    GetModuleFileName(NULL, fullExecutablePath, MAX_PATH);
    size_t len = lstrlen(fullExecutablePath)+1;
    srcpath = HeapAlloc(heap, 0, len);
    StringCbCat(srcpath, len-4, fullExecutablePath);
    StringCchCat(srcpath, len, ".c");

    printf("srcpath is \"%s\"\n", srcpath);

    //WaitForSingleFile(srcpath, FILE_NOTIFY_LAST_WRITE);

    ZeroMemory(&startupinfo, sizeof(STARTUPINFO));
    startupinfo.cb = sizeof(STARTUPINFO);

    size_t cclen = lstrlen(CC)+1+len-1+4+lstrlen(argv0)+1;
    cc_cmdline = HeapAlloc(heap, 0, cclen);
    StringCbCat(cc_cmdline,  cclen, CC);
    StringCchCat(cc_cmdline, cclen, " ");
    StringCchCat(cc_cmdline, cclen, srcpath);
    StringCchCat(cc_cmdline, cclen, " -o ");
    StringCchCat(cc_cmdline, cclen, argv0);

    printf("cc_cmdline is \"%s\"\n", cc_cmdline);

    return 0;

    CreateProcess(NULL, cc_cmdline, NULL, NULL, TRUE, 0,
                  NULL, NULL, &startupinfo, cc_procinfo);
    HeapFree(heap, 0, argv0);
    HeapFree(heap, 0, srcpath);
    HeapFree(heap, 0, cc_cmdline);
    WaitForSingleObject(cc_procinfo->hProcess, INFINITE);

    printf("Execing...\n");

    CreateProcess(NULL, lpCmdLine, NULL, NULL, TRUE, 0,
                  NULL, NULL, &startupinfo, cc_procinfo);
    ExitProcess(0);
}

int WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, 
            LPSTR lpCmdLine, int nCmdShow) 
{
    int excode, argc;
    LPWSTR* argv;
    CreateThread(NULL, 0, &watcher, lpCmdLine, 0, NULL);
    argv = CommandLineToArgvW((LPCWSTR)lpCmdLine, &argc);
    excode = program(argc, argv);
    LocalFree(argv);
    ExitProcess(excode);
}

int program(int argc, char *argv[])
{
    for(;;)
    {
        printf("bar ");
        fflush(stdout);
        sleep(1);
    }
}
