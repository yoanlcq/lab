#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <libwacom-1.0/libwacom/libwacom.h>

void handle_dev(WacomDeviceDatabase *db, WacomDevice* device) {
    printf("# Tablet\n");
    libwacom_print_device_description(1, device);
    int num_styli;
    const int * styli = libwacom_get_supported_styli(device, &num_styli);
    for(int i=0 ; i<num_styli ; ++i) {
        printf("# Stylus n°%i (ID: %x)\n", i, styli[i]);
        const WacomStylus *stylus = libwacom_stylus_get_for_id (db, styli[i]);
        libwacom_print_stylus_description (1, stylus);
    }
}

int main(int argc, char *argv[]) {
    if(argc < 2) {
        fprintf(stderr, "Usage: %s db|/dev/input/eventXXX\n", argv[0]);
        return EXIT_FAILURE;
    }

    const char* devpath = argv[1];

    WacomDeviceDatabase *db = libwacom_database_new();
    WacomError *error = libwacom_error_new();

    if(!strcmp(devpath, "db")) {
        WacomDevice** devs = libwacom_list_devices_from_database(db, error);
        for(int i=0 ; devs[i] ; ++i) {
            handle_dev(db, devs[i]);
        }
        free(devs);
    } else {
        WacomDevice *device = libwacom_new_from_path(db, devpath, WFALLBACK_NONE, error);
        if(device) {
            handle_dev(db, device);
            libwacom_destroy(device);
        } else {
            const char* msg = libwacom_error_get_message(error);
            enum WacomErrorCode code = libwacom_error_get_code(error);
            fprintf(stderr, "Error %u: %s.\n", code, msg);
        }
    }

    libwacom_error_free(&error);
    libwacom_database_destroy(db);
    return EXIT_SUCCESS;
}
