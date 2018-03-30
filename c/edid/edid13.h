// Header file for the edid13 struct, which maps directly to raw EDID 1.3 data.
//
// EDID 1.3 is a 128-byte packet reported by (supposedly) each of your display monitors.
// It is a standard mandated by VESA, and allows you to get precise information related
// to your monitors and their physical characteristics.
// Accessing it usually involves a complicated dance with you OS's API. On Windows, you need
// to tap into the registry.
//
// This file does not claim completeness. The initial purpose was to have "something clean"
// to work with, have an overview of the data, and write utility functions for getting
// information related to the device's name and physical size.
//
// Go ahead and take a look at the "VESA Enhanced EDID" spec, it's only 32 pages long!
// http://read.pudn.com/downloads110/ebook/456020/E-EDID%20Standard.pdf

#pragma once

#include <stdint.h>
#include <stdbool.h>

#ifndef _MSC_VER
#define static_assert _Static_assert
#endif


// Decode by simply adding the ASCII value of 
// 'A' to each 'compressed_ascii' member.
typedef struct {
	// XXX Did I mess up with the order of bit fields ?
	uint16_t unused : 1;
	uint16_t compressed_ascii_0 : 5;
	uint16_t compressed_ascii_1 : 5;
	uint16_t compressed_ascii_2 : 5;
} eisa_3char_id;

typedef struct {
	char ascii[4]; // nul-terminated
} edid13_manufacturer_name;

typedef struct {
	char ascii[14]; // nul-terminated
} edid13_monitor_name;

typedef struct {
	uint8_t pixel_clock_div_10_000[2];

	uint8_t horizontal_active_lower;
	uint8_t horizontal_blanking_lower;
	uint8_t horizontal_active_upper: 4;
	uint8_t horizontal_blanking_upper: 4;

	uint8_t vertical_active_lower;
	uint8_t vertical_blanking_lower;
	uint8_t vertical_active_upper: 4;
	uint8_t vertical_blanking_upper: 4;

	uint8_t horizontal_sync_offset_lower;
	uint8_t horizontal_sync_pulse_width_lower;
	uint8_t vertical_sync_offset_lower: 4;
	uint8_t vertical_sync_pulse_width_lower: 4;

	uint8_t horizontal_sync_offset_upper : 2;
	uint8_t horizontal_sync_pulse_width_upper : 2;
	uint8_t vertical_sync_offset_upper: 2;
	uint8_t vertical_sync_pulse_width_upper: 2;

	uint8_t horizontal_image_size_mm_lower;
	uint8_t vertical_image_size_mm_lower;
	// XXX Had to swap the below two. I probably messed up the order of other bitfields then.
	uint8_t vertical_image_size_mm_upper : 4;
	uint8_t horizontal_image_size_mm_upper : 4;

	uint8_t horizontal_border;
	uint8_t vertical_border;

	uint8_t flags_interlace : 1;
	uint8_t flags_stereo : 2;
	// XXX Not sure at all about these fields' names
	uint8_t flags_horizontal_polarity : 2;
	uint8_t flags_vertical_polarity : 2;
	uint8_t flags_bit0 : 1;
} edid13_detailed_timing_description_block;

typedef struct {
	uint8_t zero_flag0;
	uint8_t zero_flag1;
	uint8_t zero_flag2;
	uint8_t datatype_tag;
	uint8_t zero_flag3;
	uint8_t descriptor_data[13];
} edid13_monitor_descriptor_block;

typedef union {
	edid13_detailed_timing_description_block detailed_timing_description;
	edid13_monitor_descriptor_block monitor_descriptor;
} edid13_description_block;

typedef struct {
	uint8_t data[18];
} edid13_detailed_timing_description_block_raw;

static_assert(sizeof(edid13_detailed_timing_description_block) == 18, "EDID 1.3 Detailed Timing Description is supposed to be 18 bytes long!");
static_assert(sizeof(edid13_monitor_descriptor_block) == 18, "EDID 1.3 Monitor Descripto is supposed to be 18 bytes long!");

#define SIZEOF_EDID13 128

// Not using integers wider than uint8_t because of possible endianness issues
typedef struct {
	// Header - content assumed to be 0x00ffffffffffff00
	uint8_t header[8];

	// Vendor / Product Identification
	eisa_3char_id id_manufacturer_name;
	uint8_t id_product_code[2];
	uint8_t id_serial_number[4];
	uint8_t week_of_manufacture;
	uint8_t year_of_manufacture;

	// EDID Structure Version / Revision
	uint8_t version_number;
	uint8_t revision_number;

	// Basic Display Parameters / Features
	uint8_t video_input_definition;
	uint8_t max_horizontal_image_size_cm;
	uint8_t max_vertical_image_size_cm;
	uint8_t display_transfer_characteristic; // (Gamma)
	uint8_t feature_support;

	// Color Characteristics
	uint8_t red_green_low_bits;
	uint8_t blue_white_low_bits;
	uint8_t red_x, red_y;
	uint8_t green_x, green_y;
	uint8_t blue_x, blue_y;
	uint8_t white_x, white_y;

	// Established Timings
	uint8_t established_timings[2];
	uint8_t manufacturers_reserved_timings;

	// Standard Timing Identification
	uint8_t standard_timing_identification[8][2];

	// Detailed Timing Description
	edid13_detailed_timing_description_block detailed_timing_description;
	edid13_description_block other_description_blocks[3];

	// Extension Flag
	uint8_t extension_flag;

	// Checksum
	uint8_t checksum;
} edid13;

static_assert(sizeof(edid13) == SIZEOF_EDID13, "An unextended EDID 1.3 is supposed to be 128 bytes long!");

typedef struct {
	uint8_t data[SIZEOF_EDID13];
} edid13_raw;

typedef union {
	uint8_t raw[SIZEOF_EDID13];
	edid13 edid;
} edid13_union;

static inline bool edid13_is_header_conformant(edid13 edid) {
	return edid.header[0] == 0x00
		&& edid.header[1] == 0xff
		&& edid.header[2] == 0xff
		&& edid.header[3] == 0xff
		&& edid.header[4] == 0xff
		&& edid.header[5] == 0xff
		&& edid.header[6] == 0xff
		&& edid.header[7] == 0x00
		;
}

static inline bool edid13_is_checksum_conformant(edid13 edid) {
	edid13_union u = { .edid = edid };
	uint8_t sum = 0;
	for(int i=0 ; i<sizeof(u.raw) ; ++i)
		sum += u.raw[i];
	return !sum;
}

static inline edid13_manufacturer_name eisa_3char_id_parse(eisa_3char_id id) {
	edid13_manufacturer_name out;
	out.ascii[0] = 'A' + id.compressed_ascii_0;
	out.ascii[1] = 'A' + id.compressed_ascii_1;
	out.ascii[2] = 'A' + id.compressed_ascii_2;
	out.ascii[3] = '\0';
	return out;
}

// Value stored = (gamma x 100)-100
// vs = g*100 - 100
// vs + 100 = g*100
// (vs + 100)/100 = g;
static inline float edid13_get_gamma(edid13 edid) {
	return (edid.display_transfer_characteristic + 100)/100.f;
}

static inline uint16_t edid13_get_width_mm(edid13 edid) {
 	uint16_t lo8 = edid.detailed_timing_description.horizontal_image_size_mm_lower;
 	uint16_t hi4 = edid.detailed_timing_description.horizontal_image_size_mm_upper;
 	return lo8 + (hi4 << 8);

	// edid13_union u = { .edid = edid };
	// return u.raw[66] + ((u.raw[68] & 0xf0) << 4);
}
static inline uint16_t edid13_get_height_mm(edid13 edid) {
	uint16_t lo8 = edid.detailed_timing_description.vertical_image_size_mm_lower;
	uint16_t hi4 = edid.detailed_timing_description.vertical_image_size_mm_upper;
	return lo8 + (hi4 << 8);

	// edid13_union u = { .edid = edid };
	// return u.raw[67] + ((u.raw[68] & 0x0f) << 8);
}

static inline edid13_monitor_name edid13_try_find_monitor_name(edid13 edid) {
	edid13_monitor_name name = { .ascii = {0} };
	for(int i=0 ; i<3 ; ++i) {
		edid13_monitor_descriptor_block md = edid.other_description_blocks[i].monitor_descriptor;
		if(md.zero_flag0 || md.zero_flag1 || md.zero_flag2 || md.zero_flag3)
			continue;
		if(md.datatype_tag != 0xfc)
			continue;
		for(int j=0 ; j<13 ; ++j) {
			if(md.descriptor_data[j] == 0xa)
				break;
			name.ascii[j] = md.descriptor_data[j];
		}
		break;
	}
	return name;
}
