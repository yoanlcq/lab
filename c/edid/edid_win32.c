// https://ofekshilon.com/2014/06/19/reading-specific-monitor-dimensions/

#include <windows.h>
#include <setupapi.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>

#define MAX_DEVICE_ID_LEN 200 /* from cfgmgr32.h */
#ifndef EDD_GET_DEVICE_INTERFACE_NAME
#define EDD_GET_DEVICE_INTERFACE_NAME 1
#endif

const GUID GUID_CLASS_MONITOR = { 0x4d36e96e, 0xe325, 0x11ce, { 0xbf, 0xc1, 0x08, 0x00, 0x2b, 0xe1, 0x03, 0x18 }};

#define APP_MAX_MONITORS 16 // Can't have more than 16 monitors, huh ?
#define APP_MAX_DISPLAY_DEVICES APP_MAX_MONITORS

#include "edid13.h"

// XXX TODO FIXME
#define countof()

typedef struct {
	WCHAR name[32];
	WCHAR string[128];
	WCHAR id[128];
	bool is_primary_display_device;
	bool is_vga_compatible;
} app_display_device;

typedef struct {
	HMONITOR hmonitor;
	HDC hdc;
	RECT device_context_rect; // If hdc is NULL, virtual-screen coords. Otherwise, device-context coords.
	RECT virtual_screen_rect;
	RECT virtual_screen_work_area;
	bool is_primary_monitor;
	app_display_device adapter;
	app_display_device monitor;
	WCHAR device_name[CCHDEVICENAME];
	WCHAR extracted_id[128];
	SP_DEVINFO_DATA devinfo_data;
	bool has_edid;
	edid13 edid;
	uint16_t width_mm, height_mm;
	bool is_edid_header_conformant;
	bool is_edid_checksum_conformant;
	float edid_gamma;
	edid13_monitor_name edid_monitor_name;
} app_monitor;

typedef struct {
	app_monitor monitors[APP_MAX_MONITORS];
	size_t monitor_count;
} app;

// TODO use countof where needed

static void app_display_device_log(const app_display_device *d, FILE *stream) {
	fprintf(stream, "name: \"%ls\"\n", d->name);
	fprintf(stream, "string: \"%ls\"\n", d->string);
	fprintf(stream, "id: \"%ls\"\n", d->id);
	fprintf(stream, "is_primary_display_device: %s\n", d->is_primary_display_device ? "true" : "false");
	fprintf(stream, "is_vga_compatible: %s\n", d->is_vga_compatible ? "true" : "false");
}

static void RECT_log(RECT rect, FILE *stream) {
	LONG x, y, w, h;
	x = rect.left;
	y = rect.top;
	w = rect.right - rect.left;
	h = rect.bottom - rect.top;
	fprintf(stream, "{ x:%li, y:%li, w:%li, h:%li }", x, y, w, h);
}

static void app_monitor_log(const app_monitor *m, FILE *stream) {
	fprintf(stream, "device_context_rect: ");
	RECT_log(m->device_context_rect, stream);
	fprintf(stream, " (%s)\n", 	m->hdc ? "device-context coords" : "virtual-screen coords");
	fprintf(stream, "virtual_screen_rect: ");
	RECT_log(m->virtual_screen_rect, stream);
	fprintf(stream, "\nvirtual_screen_work_area: ");
	RECT_log(m->virtual_screen_work_area, stream);
	fprintf(stream, "\nis_primary_monitor: %s\n", m->is_primary_monitor ? "true" : "false");
	fprintf(stream, "--- Adapter data\n");
	app_display_device_log(&m->adapter, stream);
	fprintf(stream, "--- Monitor data\n");
	app_display_device_log(&m->monitor, stream);
	fprintf(stream, "---\n");
	fprintf(stream, "device_name: \"%ls\"\n", m->device_name);
	fprintf(stream, "extracted_id: \"%ls\"\n", m->extracted_id);
	fprintf(stream, "has_edid: %s", m->has_edid ? "true" : "false");
	if(!m->is_edid_header_conformant)
		fprintf(stream, " (invalid header)");
	if(!m->is_edid_checksum_conformant)
		fprintf(stream, " (invalid checksum)");
	fprintf(stream, "\n");
	if(m->has_edid) {
		fprintf(stream, "edid max physical size cm: (%hu, %hu)\n", m->edid.max_horizontal_image_size_cm, m->edid.max_vertical_image_size_cm);
		fprintf(stream, "edid physical size mm: (%hu, %hu)\n", m->width_mm, m->height_mm);
		fprintf(stream, "edid gamma: %f\n", m->edid_gamma);
		fprintf(stream, "edid monitor name: \"%s\"\n", m->edid_monitor_name.ascii);
	}

}

static void app_log_all_monitors(const app *app, FILE *stream) {
	for(unsigned i=0 ; i<app->monitor_count; ++i) {
		fprintf(stream, "\n\n=== Monitor %u ===\n\n", i);
		app_monitor_log(&app->monitors[i], stream);
	}
}

static bool win32_edid_from_extracted_id(app_monitor *m) {

	// XXX Yeah this actually returns a handle to a linked list of device information,
	// which is why we have to loop over it, and match against ids.
    HDEVINFO hdevinfo = SetupDiGetClassDevsEx(
        &GUID_CLASS_MONITOR, //class GUID
        NULL, //enumerator
        NULL, //HWND
        DIGCF_PRESENT | DIGCF_PROFILE, // Flags //DIGCF_ALLCLASSES|
        NULL, // device info, create a new one.
        NULL, // machine name, local machine
        NULL // reserved
	);
	if(!hdevinfo || hdevinfo == INVALID_HANDLE_VALUE) {
		fprintf(stderr, "SetupDiGetClassDevsEx() returned %p\n", hdevinfo);
		return false;
	}

	for(unsigned i=0 ; ; ++i) {
		m->devinfo_data = (SP_DEVINFO_DATA) { .cbSize = sizeof(SP_DEVINFO_DATA) };
		BOOL success = SetupDiEnumDeviceInfo(hdevinfo, i, &m->devinfo_data);
		if(!success) {
			DWORD err = GetLastError();
			if(err != ERROR_NO_MORE_ITEMS) {
				fprintf(stderr, "GetLastError() for SetupDiEnumDeviceInfo() was not ERROR_NO_MORE_ITEMS (%lu)!\n", err);
			}
			break;
		}

		WCHAR instance_id[MAX_DEVICE_ID_LEN+1];
		success = SetupDiGetDeviceInstanceIdW(hdevinfo, &m->devinfo_data, instance_id, sizeof(instance_id)/2, NULL);
		if(!success) {
			fprintf(stderr, "SetupDiGetDeviceInstanceIdW() failed at %u (error: %lu)\n", i, GetLastError());
			continue;
		}

		if(!wcsstr(instance_id, m->extracted_id)) {
			// fprintf(stderr, "Didn't find \"%ls\" in \"%ls\"\n", m->extracted_id, instance_id);
			continue;
		}

		HKEY h_edid_regkey = SetupDiOpenDevRegKey(hdevinfo, &m->devinfo_data,
			DICS_FLAG_GLOBAL, 0, DIREG_DEV, KEY_READ);

		if (!h_edid_regkey || (h_edid_regkey == INVALID_HANDLE_VALUE)) {
			fprintf(stderr, "SetupDiOpenDevRegKey() failed at %u: %lu\n", i, GetLastError());
			continue;
		}

		DWORD sizeof_edid = sizeof m->edid;
		LONG status = RegQueryValueExW(h_edid_regkey, L"EDID", NULL, NULL, (void*)&m->edid, &sizeof_edid);
		if(status != ERROR_SUCCESS) {
			fprintf(stderr, "RegQueryValueEx() failed to query EDID key at %u (status: %li)\n", i, status);
			continue;
		}
		if(sizeof_edid != sizeof m->edid)
			printf("Actual size of EDID is %lu (expected %u), at %u\n", sizeof_edid, (unsigned) sizeof m->edid, i);

		RegCloseKey(h_edid_regkey);
    	SetupDiDestroyDeviceInfoList(hdevinfo);
		return true;
	}

    SetupDiDestroyDeviceInfoList(hdevinfo);
	return false;
}

static BOOL CALLBACK each_monitor(HMONITOR hmonitor, HDC hdc, LPRECT rect, LPARAM userdata) {
	app *a = (app*) userdata;
	a->monitors[a->monitor_count] = (app_monitor) {
		.hmonitor = hmonitor,
		.hdc = hdc,
		.device_context_rect = *rect
	};
	++(a->monitor_count);
	if(a->monitor_count >= APP_MAX_MONITORS) {
		fprintf(stderr, "Stopping enumeration early because apparently you have %u monitors.\n", (unsigned) a->monitor_count);
		return FALSE;
	}
	return TRUE;
}

int main() {
	app app = {0};

	EnumDisplayMonitors(NULL, NULL, each_monitor, (LPARAM) &app);

	for(unsigned i=0 ; i<app.monitor_count ; ++i) {

		app_monitor *m = &app.monitors[i];

		MONITORINFOEXW info = { .cbSize = sizeof(MONITORINFOEXW) };
		BOOL success = GetMonitorInfoW(m->hmonitor, (LPMONITORINFO)&info);
		if(!success) {
			fprintf(stderr, "GetMonitorInfoW failed at monitor %u: %lx\n", i, GetLastError());
			continue;
		}
		m->virtual_screen_rect = info.rcMonitor;
		m->virtual_screen_work_area = info.rcWork;
		m->is_primary_monitor = !!(info.dwFlags & MONITORINFOF_PRIMARY);
		memcpy(m->device_name, info.szDevice, CCHDEVICENAME * sizeof(WCHAR));

		// XXX DRY

		DISPLAY_DEVICEW dd = { .cb = sizeof(DISPLAY_DEVICEW) };
		success = EnumDisplayDevicesW(NULL, i, &dd, EDD_GET_DEVICE_INTERFACE_NAME);
		if(!success) {
			fprintf(stderr, "EnumDisplayDevicesW failed at monitor %u: %lx\n", i, GetLastError());
			continue;
		}
		memcpy(m->adapter.name,   dd.DeviceName,   sizeof(m->adapter.name));
		memcpy(m->adapter.string, dd.DeviceString, sizeof(m->adapter.string));
		memcpy(m->adapter.id,     dd.DeviceID,     sizeof(m->adapter.id));
		m->adapter.is_primary_display_device = !!(dd.StateFlags & DISPLAY_DEVICE_PRIMARY_DEVICE);
		m->adapter.is_vga_compatible = !!(dd.StateFlags & DISPLAY_DEVICE_VGA_COMPATIBLE);


		dd = (DISPLAY_DEVICEW) { .cb = sizeof(DISPLAY_DEVICEW) };
		success = EnumDisplayDevicesW(m->adapter.name, 0, &dd, 0/*EDD_GET_DEVICE_INTERFACE_NAME*/); // NOTE: don't set this flag
		if(!success) {
			fprintf(stderr, "EnumDisplayDevicesW failed at monitor %u (\"%ls\"): %lx\n", i, m->adapter.name, GetLastError());
			continue;
		}
		memcpy(m->monitor.name,   dd.DeviceName,   sizeof(m->monitor.name));
		memcpy(m->monitor.string, dd.DeviceString, sizeof(m->monitor.string));
		memcpy(m->monitor.id,     dd.DeviceID,     sizeof(m->monitor.id));
		m->monitor.is_primary_display_device = !!(dd.StateFlags & DISPLAY_DEVICE_PRIMARY_DEVICE);
		m->monitor.is_vga_compatible = !!(dd.StateFlags & DISPLAY_DEVICE_VGA_COMPATIBLE);


		WCHAR *first_slash = wcschr(dd.DeviceID, L'\\');
		WCHAR *second_slash = wcschr(first_slash+1, L'\\');
		if(second_slash-first_slash >= sizeof(m->extracted_id)/sizeof(m->extracted_id[0])) {
			fprintf(stderr, "Not enough room for Extracted ID! This should never happen.\n");
			continue;
		}
		// printf("Copying id from \"%ls\"\n", dd.DeviceID);
		memcpy(m->extracted_id, first_slash+1, (void*)second_slash-(void*)first_slash-1);
		m->extracted_id[second_slash-first_slash-1] = L'\0';
		// printf("Extracted id \"%ls\"\n", m->extracted_id);

		m->has_edid = win32_edid_from_extracted_id(m);
		m->is_edid_header_conformant = edid13_is_header_conformant(m->edid);
		m->is_edid_checksum_conformant = edid13_is_checksum_conformant(m->edid);
		m->edid_gamma = edid13_get_gamma(m->edid);
		m->edid_monitor_name = edid13_try_find_monitor_name(m->edid);
		m->width_mm = edid13_get_width_mm(m->edid);
		m->height_mm = edid13_get_height_mm(m->edid);
	}

	app_log_all_monitors(&app, stdout);

	return 0;
}
