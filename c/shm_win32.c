#include <windows.h>
#include <stdio.h>

const char *SHM_NAME = "MySharedMemoryTest";

int main(void) {
    unsigned *shm_ptr; /* Should be void* */
    BOOL is_first = FALSE;
    HANDLE shm;
    DWORD err;
    unsigned i;
    
    shm = OpenFileMapping(FILE_MAP_ALL_ACCESS, FALSE, SHM_NAME);
    err = GetLastError();
    if(!shm) {
        if(err != ERROR_FILE_NOT_FOUND) {
            fprintf(stderr, "Could not open shared memory !\nError code : %d\n", err);
            return EXIT_FAILURE;
        }
        shm = CreateFileMapping(INVALID_HANDLE_VALUE, NULL, PAGE_READWRITE, 0, sizeof(unsigned), SHM_NAME);
        err = GetLastError();
        if(!shm) {
            fprintf(stderr, "Could not create shared memory !\nError code : %d\n", err);
            return EXIT_FAILURE;
        }
        is_first = TRUE;
    }

    shm_ptr = MapViewOfFile(shm, FILE_MAP_ALL_ACCESS, 0, 0, sizeof(unsigned));
    if(!shm_ptr) {
        fprintf(stderr, "Could not get the pointer to shared memory !\nError code : %d\n", GetLastError());
        CloseHandle(shm);
        return EXIT_FAILURE;
    }
    if(is_first) 
        *shm_ptr = 1;
    else 
        ++(*shm_ptr);

    for(i=10 ; i>0 ; --i) {
        printf("There %s currently %u process%s running this code. (%u checks remaining)\n",
                *shm_ptr==1 ? "is":"are", *shm_ptr, *shm_ptr==1 ? "":"es", i);
        Sleep(1000);
    }
    --(*shm_ptr);
    UnmapViewOfFile(shm_ptr);
    CloseHandle(shm);
    return EXIT_SUCCESS;
}
