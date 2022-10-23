//
// Pour cette démo:
// - Avoir une sky box ou sky sphere
// - Avoir un modèle qui UV avec ses normales, dans la skybox
// - Surface d'eau via spline
//
// Things I'd like to try:
// - VFPU benchmark
// - Number-of-Polygons benchmark
// - Media Engine
// - Transient resources (e.g depth buffer can be recycled after main 3D rendering is done)
// - Reflective 3D models (new)
//   - Draw the scene into a "cubemap" (so 6 times)
//     The front face needs highest resolution, the back face needs least resolution.
//     Keep a 2px border around the texture where alpha is zero.
//     This will be used later to prevent undesired pixels from being rendered.
//     The border has to be 2px so that sampling at UV (0,0) with GU_LINEAR will give a color with zero alpha.
//   - Draw objects into fb0 ("N-buffer") with:
//     - fb0 format = RGBA_8888
//     - sceGuTexProjMapMode(GU_NORMAL) (or normalized; offer both options)
//     - texture matrix = modelview matrix (so normals are in view-space)
//     - Input texture generated as follows:
//       - Designed to be sampled directly by view-space normal vectors, so, hemispherical
//       - Pixel format = RGBA_8888
//       - Each texel is encoded as follows (1 character = 1 bit):
//         IIUUUUUUUUUUUUUUJJVVVVVVVVVVVVVV
//         - IIJJ form a 4-bit value encoding the cubemap face index
//         - U and V are texture coordinates into the cubemap face
//   - For each cubemap face index, from 0 to 5
//     - For each column_parity in "even", "odd": (details below)
//       - Clear fb1 to zero; the idea is that, interpreted as UV, this would sample the cubemap face's border, which would be rejected by an alpha test.
//       - Draw fb0 => fb1, with color test enabled to rejecting all pixels for which the IIJJ pattern is not the current cubemap face index
//       - "Convert" fb1 to a vertex buffer fb1_vb, with vertex format UV16+XYZ8.
//         The idea is that fb1 and fb1_vb point to the same memory.
//         In order for vertex positions to be valid, we render a "positions mask" onto fb1 (using a 512*272 viewport and scissor, to make sure the entire 512-480 region is written to, and not reinterpreted as garbage vertices)
//         Notice that fb1's content is: UUVV UUVV UUVV UUVV ...
//         And fb1_vb's content is:      UUVV XYZ_ UUVV XYZ_ ... (the "_" is padding)
//         fb1_vb starting 4 bytes after:     UUVV XYZ_ ...
//         TODO: Need to confirm possibility to use 4-byte aligned vertex buffer (everyone aligns to 16 bytes)
//         So fb1_vb is one vertex for each pixel in the columns which index is even.
//         To account for all pixels, we'll have to draw the "mesh" twice.
//         So if we create the "positions mask" as:  XYZ0 XYZ1 XYZ0 XYZ1
//         We can render the even columns with an alpha test that only accepts 0, and the odd columns with an alpha test that only accepts 1.
//         That alpha component will not affect the vertices' positions.
//       - Call sceGuTexOffset() such that, during texture sampling,
//         IIUUUUUUUUUUUUUUJJVVVVVVVVVVVVVV
//         becomes
//         00UUUUUUUUUUUUUU00VVVVVVVVVVVVVV
//         i.e make sure the encoded cubemap face index doesn't interfere with actual texture coordinates
//       - Draw fb1_vb:
//         - as GU_POINTS
//         - using the cubemap face as input texture
//         - with an alpha test that rejects zeroed alpha
//         - with a model matrix that transforms the 8-bits positions into their respective pixel positions (8-bits = 256 values, not enough to cover the 480 width, but enough to cover half of it (interleaved columns)).
// - Reflective 3D models (archive)
//   - Draw normals into fb0 (sceGuTexProjMapMode(GU_NORMAL), use a texture generated at program initialization)
//     OR the texture could map to direct reflected UV16 values.
//   - Problem: we want view-space normals, but PSP only provides model-space or world-space
//     But then again, we have the texture matrix for that
//   - Draw fb0 => fb0 with LUT f(n) = reflect(forward, n)
//     The LUT also stores a "cubemap face index" in the alpha component.
//   - For cubemap_face_index in 0..6
//     - Clear fb1 with a value such that, when interpreted as a vector, will end up sampling magenta from the cubemap face texture (i.e the texture's border)
//     - Draw fb0 => fb1 with alpha test = i, so fb1 contains only the vectors pointing towards the current cubemap face
//     - For as many times as needed to cover all pixels:
//       - Convert fb1 to vertex buffer fb1_vb (TODO)
//         Do something about the 512-480 region (either skip via vertex indices, or clear it such that the vertices have a 3D position outside the screen)
//       - Draw fb1_vb => fb2 with:
//         sceGuTexProjMapMode(GU_NORMAL);
//         sceGuTexMapMode(GU_TEXTURE_MATRIX);
//         texture matrix = some rotation * matrix used to render the cubemap face
//         color test = reject magenta color
// - Refractive 3D models; same tech
//   Except the cubemap is rendered in two passes: 1 usual, + 1 with distortions applied (camera is inside the model)
//   Assumes that the model is convex.
// - Normal mapping??
// - Post-processing effects
//   - Blur
//     - Render to lower resolution texture
//     - Draw shifted horizontally
//     - Draw shifted vertically
//   - Depth of field
//     - Render far objects into render target and blur it
//   - Bloom
//     - Clear fb1 with threshold color
//     - Draw fb0 => fb1 with blending GU_MAX and color test = reject threshold color
//     - Blur fb1
//     - Draw fb1 additive => fb0
//   - Distortion
//     - Option 1: Fullscreen mesh using fb0 as texture; update the mesh every frame (move the vertices or the UVs); or use morph weights
//       In addition, a slight blur may do the trick
//     - Option 2: Render into normals buffer fb1
//       Draw into offsets buffer fb2 (i.e each rgba pixel = uv16 sampling position for the fb0 texture)
//       Combine fb2 and fb1 (i.e "add" the normals to the undistorted UV coords)
//       Reinterpret fb1 as fb1_vb: UV16 + XYZ8
//       Draw fb1_vb with fb0 as input texture
//   - Tone mapping
//     - For each 2x2 pixel quad:
//       - Draw a mesh that samples top-left pixel, into half-res rendertarget, with GU_MAX blending
//       - Repeat for top-right, botom-left, bottom-right
//     - Go back to the loop, this time using the half-res RT as source
//     - Do this until the RT is 1x1, this gives the max.
//     - Then fill a LUT according to min and max
//     - Then render through that LUT
//   - LUTs for gamma correction
//
//
// TexMap_Unknown
// TexMap_TextureCoords
//
// TexMap_Texture_Matrix:
// - POSITION: model-space
// - UV: {u,v,0}
// - [NORMALIZED_]NORMAL: model-space normal
// La TEXTURE_MATRIX transforme cette coordonée; cela donne UVW
//
// TexMap_Environment_Map:
// - Pour 2 lights: prend la position, normalize, puis:
//   uv[0] = (1.0f + Dot(lightpos0, worldnormal))/2.0f;
//   uv[1] = (1.0f + Dot(lightpos1, worldnormal))/2.0f;
//   uv[2] = 1.0f;
//
// 1. rgba rgba rgba rgba rgba rgba rgba
// 2. 123x yz12 3xyz 123x
// 3. rgba 123x yz   rgba 123x yz   rgba
//
// 1. Format of the normals buffer: rgb = xyz, a = cubemap face index
// 2. Vertex_Tf32_Pf32 format GU_NORMAL_8BIT | GU_VERTEX_8BIT
// 3. Vertex_Tf32_Pf32 format GU_COLOR_8888 | GU_NORMAL_8BIT | GU_VERTEX_8BIT
//

#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <inttypes.h>

#include <pspctrl.h>
#include <pspdebug.h>
#include <pspdisplay.h>
#include <pspfpu.h>
#include <pspgu.h>
#include <pspgum.h>
#include <pspkernel.h>
#include <psprtc.h>

//
//
// Foundation
//
//

typedef int8_t i8;
typedef int16_t i16;
typedef int32_t i32;
typedef int64_t i64;
typedef float f32;
typedef double f64;

typedef f32 __attribute__((vector_size(16))) v4;
typedef struct { v4 cols[4]; } m4;

#define ALIGN_N(x) __attribute__((aligned(x)))
#define ALIGN16 ALIGN_N(16)

#define countof(x) (sizeof((x)) / sizeof((x)[0]))

static inline void* psp_uncached_ptr_non_null(const void* p) {
	assert(p); // If you're passing NULL, you'll get an uncached NULL ptr but it won't evaluate to NULL, so that may trick conditionals and do bad stuff.
	return (void*) (((uintptr_t) p) | 0x40000000ul);
}

static inline void* psp_uncached_ptr_or_null(const void* p) {
	return p ? psp_uncached_ptr_non_null(p) : NULL;
}

static inline bool size_is_power_of_two_nonzero(size_t x) {
	return x != 0 && !(x & (x - 1));
}

static inline bool size_is_power_of_two_or_zero(size_t x) {
	return x == 0 || !(x & (x - 1));
}

static inline void* ptr_align(const void* p, uintptr_t a) {
	assert(size_is_power_of_two_nonzero(a));
	return (void*) ((((uintptr_t) p) + a - 1) & ~(a - 1));
}

static inline bool ptr_is_aligned(const void* p, uintptr_t a) {
	assert(size_is_power_of_two_nonzero(a));
	return (((uintptr_t) p) & (a - 1)) == 0;
}


//
//
// Spin locks
//
//

typedef struct SpinLock { bool value; } SpinLock;

bool spin_try_lock(SpinLock* m) {
	return !__atomic_test_and_set(&m->value, __ATOMIC_ACQUIRE);
}

void spin_lock(SpinLock* m) {
	while (!spin_try_lock(m)) {}
}

void spin_unlock(SpinLock* m) {
	__atomic_clear(&m->value, __ATOMIC_RELEASE);
}

//
//
// Timing
//
//

int psp_rtc_get_current_tick_checked(u64* dst) {
	const int result = sceRtcGetCurrentTick(dst);
	if (result != 0)
		fprintf(stderr, "sceRtcGetCurrentTick() failed: %08x\n", result);

	return result;
}

int psp_rtc_get_current_tick_sync(u64* dst) {
	static SpinLock s_lock = {0};
	spin_lock(&s_lock);
	const int result = psp_rtc_get_current_tick_checked(dst);
	spin_unlock(&s_lock);
	return result;
}

typedef struct {
	u64 start, end;
} TickRange;

static float tick_range_get_duration(TickRange m) {
	return (m.end - m.start) / (float) sceRtcGetTickResolution();
}

//
//
// VFPU
//
//

void vfpu_m4_mul(m4* result, const m4* a, const m4* b) {
	assert(ptr_is_aligned(result, 64));
	assert(ptr_is_aligned(a, 64));
	assert(ptr_is_aligned(b, 64));
	__asm__ volatile
	(
		"lv.q C000,  0 + %1\n"
		"lv.q C010, 16 + %1\n"
		"lv.q C020, 32 + %1\n"
		"lv.q C030, 48 + %1\n"

		"lv.q C100,  0 + %2\n"
		"lv.q C110, 16 + %2\n"
		"lv.q C120, 32 + %2\n"
		"lv.q C130, 48 + %2\n"

		"vmmul.q M200, M000, M100\n"

		"sv.q C200,  0 + %0\n"
		"sv.q C210, 16 + %0\n"
		"sv.q C220, 32 + %0\n"
		"sv.q C230, 48 + %0\n"
	: "=m"(*result) : "m"(*a), "m"(*b) : "memory");
}

//
//
// Graphics
//
//

#define PSP_SCREEN_STRIDE 512
#define PSP_SCREEN_WIDTH  480
#define PSP_SCREEN_HEIGHT 272

size_t gu_psm_get_bits_per_pixel(int psm) {
	switch (psm) {
	case GU_PSM_5650: return 16;
	case GU_PSM_5551: return 16;
	case GU_PSM_4444: return 16;
	case GU_PSM_8888: return 32;
	case GU_PSM_T4: return 4;
	case GU_PSM_T8: return 8;
	case GU_PSM_T16: return 16;
	case GU_PSM_T32: return 32;
	case GU_PSM_DXT1: assert(0 && "Attempted to get bits per pixel for DXT; this doesn't make sense, must count of a per 4x4 block basis instead"); return 0;
	case GU_PSM_DXT3: assert(0 && "Attempted to get bits per pixel for DXT; this doesn't make sense, must count of a per 4x4 block basis instead"); return 0;
	case GU_PSM_DXT5: assert(0 && "Attempted to get bits per pixel for DXT; this doesn't make sense, must count of a per 4x4 block basis instead"); return 0;
	default: assert(0 && "Unknown PSM"); return 0;
	}
}

size_t gu_psm_get_bytes_per_pixel(int psm) {
	return gu_psm_get_bits_per_pixel(psm) / 8;
}

typedef struct {
	u64 nb_elements;
	u64 nb_vertices;
	u64 nb_faces;
} FrameMeshesStats;

typedef struct {
	TickRange cpu, gpu;
	FrameMeshesStats meshes;
} FrameStats;

FrameStats g_frame_stats = {0};

typedef enum {
	GU_SIGNAL_ID__INVALID = 0,
	GU_SIGNAL_ID__CLOCK_START,
	GU_SIGNAL_ID__CLOCK_END,
	GU_SIGNAL_ID__COUNT // Keep last
} GuSignalID;

void gu_on_signal(int id) {
	switch (id) {
	case GU_SIGNAL_ID__CLOCK_START: 
		psp_rtc_get_current_tick_sync(&g_frame_stats.gpu.start);
		break;
	case GU_SIGNAL_ID__CLOCK_END: 
		psp_rtc_get_current_tick_sync(&g_frame_stats.gpu.end);
		break;
	default:
		break;
	}
}

void gu_insert_clock_start_marker() {
	sceGuSignal(GU_BEHAVIOR_CONTINUE, GU_SIGNAL_ID__CLOCK_START);
}

void gu_insert_clock_end_marker() {
	sceGuSignal(GU_BEHAVIOR_CONTINUE, GU_SIGNAL_ID__CLOCK_END);
}

typedef struct { f32 uv[2]; f32 position[3]; } Vertex_Tf32_Pf32;
typedef struct { i8 normal[3]; i8 position[3]; } Vertex_Ni8_Pi8;
typedef struct { i8 normal[3]; i16 position[3]; } Vertex_Ni8_Pi16;
typedef struct { f32 normal[3]; f32 position[3]; } Vertex_Nf32_Pf32;

#define Vertex_Tf32_Pf32_FORMAT (GU_TEXTURE_32BITF | GU_VERTEX_32BITF)
#define Vertex_Ni8_Pi8_FORMAT (GU_NORMAL_8BIT | GU_VERTEX_8BIT)
#define Vertex_Ni8_Pi16_FORMAT (GU_NORMAL_8BIT | GU_VERTEX_16BIT)
#define Vertex_Nf32_Pf32_FORMAT (GU_NORMAL_32BITF | GU_VERTEX_32BITF)

void gu_draw_fullscreen_quad(f32 uv0, f32 uv1) {
	Vertex_Tf32_Pf32* v = sceGuGetMemory(2 * sizeof(Vertex_Tf32_Pf32));
	v[0] = (Vertex_Tf32_Pf32) {
		.uv = { 1, 1 }, // UV is 1 rather than 0; avoids slight wraparound in the top-left corner; GU_CLAMP doesn't seem to fix it
		.position = { 0, 0, 0 },
	};
	v[1] = (Vertex_Tf32_Pf32) {
		.uv = { uv0, uv1 },
		.position = { PSP_SCREEN_WIDTH, PSP_SCREEN_HEIGHT, 0 },
	};
	sceGuDrawArray(GU_SPRITES, Vertex_Tf32_Pf32_FORMAT | GU_TRANSFORM_2D, 2, 0, v);
}

typedef struct {
	void* data;
	u16 size_px[2];
	u16 stride_px;
	u8 psm : 4;
	u8 nb_mipmap_levels : 4; // Must not be 0. Values range from 1 to 9
	u8 is_swizzled : 1;
} Texture;

void texture_check_common(const Texture* m) {
	assert(m->nb_mipmap_levels >= 1);
	if (m->size_px[0] && m->size_px[1]) {
		assert(size_is_power_of_two_nonzero(m->stride_px));
		assert(m->data);
	} else {
		assert(size_is_power_of_two_or_zero(m->stride_px));
	}
}

void texture_check_as_input(const Texture* m) {
	texture_check_common(m);
	assert(size_is_power_of_two_or_zero(m->size_px[0]));
	assert(size_is_power_of_two_or_zero(m->size_px[1]));
}

void texture_check_as_rendertarget(const Texture* m) {
	texture_check_common(m);
}

void texture_allocate_buffers(Texture* m) {
	m->data = malloc(m->stride_px * m->size_px[1] * gu_psm_get_bytes_per_pixel(m->psm));
}

void texture_destroy(Texture* m) {
	free(m->data);
	*m = (Texture) {0};
}

// From http://hitmen.c02.at/files/yapspd/psp_doc/chap27.html :
//
// Internally, the GE processes textures as 16 bytes by 8 rows blocks (independent of actual pixelformat, so a 32*32 32-bit texture is a 128*32 texture from the swizzlings point of view). When you are not swizzling, this means it will have to do scattered reads from the texture as it moves the block into its texture-cache, which has a big impact on performance. To improve on this, you can re-order your textures into these blocks so that it can fetch one entire block by reading sequentially.
//
// 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 0G 0H 0I 0J 0K 0L 0M 0N 0O 0P 0Q 0R 0S 0T 0U 0V
// 10 11 12 13 14 15 16 17 18 19 1A 1B 1C 1D 1E 1F 1G 1H 1I 1J 1K 1L 1M 1N 1O 1P 1Q 1R 1S 1T 1U 1V
// 20 21 22 23 24 25 26 27 28 29 2A 2B 2C 2D 2E 2F 2G 2H 2I 2J 2K 2L 2M 2N 2O 2P 2Q 2R 2S 2T 2U 2V
// 30 31 32 33 34 35 36 37 38 39 3A 3B 3C 3D 3E 3F 3G 3H 3I 3J 3K 3L 3M 3N 3O 3P 3Q 3R 3S 3T 3U 3V
// 40 41 42 43 44 45 46 47 48 49 4A 4B 4C 4D 4E 4F 4G 4H 4I 4J 4K 4L 4M 4N 4O 4P 4Q 4R 4S 4T 4U 4V
// 50 51 52 53 54 55 56 57 58 59 5A 5B 5C 5D 5E 5F 5G 5H 5I 5J 5K 5L 5M 5N 5O 5P 5Q 5R 5S 5T 5U 5V
// 60 61 62 63 64 65 66 67 68 69 6A 6B 6C 6D 6E 6F 6G 6H 6I 6J 6K 6L 6M 6N 6O 6P 6Q 6R 6S 6T 6U 6V
// 70 71 72 73 74 75 76 77 78 79 7A 7B 7C 7D 7E 7F 7G 7H 7I 7J 7K 7L 7M 7N 7O 7P 7Q 7R 7S 7T 7U 7V
//
// The block above is a 32 bytes by 8 lines texture block (so it could be a 8*8 32-bit block, or a 16*8 16-bit block). Each pixel is represented here by a vertical index (first value) of 0-7. The second index is the horizontal index, ranging from 0-U. When reorganizing this for swizzling, we will order the data so that when the GE needs to read something in the first 16×8 block, if can just fetch that entire block, instead of offsetting into the texture for each line it has to read. The resulting swizzled portion looks like this:
//
// 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 10 11 12 13 14 15 16 17 18 19 1A 1B 1C 1D 1E 1F
// 20 21 22 23 24 25 26 27 28 29 2A 2B 2C 2D 2E 2F 30 31 32 33 34 35 36 37 38 39 3A 3B 3C 3D 3E 3F
// 40 41 42 43 44 45 46 47 48 49 4A 4B 4C 4D 4E 4F 50 51 52 53 54 55 56 57 58 59 5A 5B 5C 5D 5E 5F
// 60 61 62 63 64 65 66 67 68 69 6A 6B 6C 6D 6E 6F 70 71 72 73 74 75 76 77 78 79 7A 7B 7C 7D 7E 7F
// 0G 0H 0I 0J 0K 0L 0M 0N 0O 0P 0Q 0R 0S 0T 0U 0V 1G 1H 1I 1J 1K 1L 1M 1N 1O 1P 1Q 1R 1S 1T 1U 1V
// 2G 2H 2I 2J 2K 2L 2M 2N 2O 2P 2Q 2R 2S 2T 2U 2V 3G 3H 3I 3J 3K 3L 3M 3N 3O 3P 3Q 3R 3S 3T 3U 3V
// 4G 4H 4I 4J 4K 4L 4M 4N 4O 4P 4Q 4R 4S 4T 4U 4V 5G 5H 5I 5J 5K 5L 5M 5N 5O 5P 5Q 5R 5S 5T 5U 5V
// 6G 6H 6I 6J 6K 6L 6M 6N 6O 6P 6Q 6R 6S 6T 6U 6V 7G 7H 7I 7J 7K 7L 7M 7N 7O 7P 7Q 7R 7S 7T 7U 7V
//
//
// Notice how the rectangular 16*8 blocks have ended up as sequential data, ready for direct reading by the GE.

// NOTE: Courtesy of samples/gu/blit/blit.c
void swizzle_fast(void* out, const void* in, uint32_t width_in_bytes, uint32_t height) {
	const u32 width_blocks = (width_in_bytes / 16);
	const u32 height_blocks = (height / 8);
	const u32 src_pitch = (width_in_bytes - 16) / 4;
	const u32 src_row = width_in_bytes * 8;

	assert(ptr_is_aligned(in, 4));
	assert(ptr_is_aligned(out, 4));
	const u8* ysrc = in;
	u32* dst = (u32*) out;

	for (u32 blocky = 0; blocky < height_blocks; ++blocky) {
		const u8* xsrc = ysrc;
		for (u32 blockx = 0; blockx < width_blocks; ++blockx) {
			const u32* src = (u32*) (void*) xsrc;
			for (u32 j = 0; j < 8; ++j) {
				*(dst++) = *(src++);
				*(dst++) = *(src++);
				*(dst++) = *(src++);
				*(dst++) = *(src++);
				src += src_pitch;
			}
			xsrc += 16;
		}
		ysrc += src_row;
	}
}

void gu_set_offset(u32 w, u32 h) {
	sceGuOffset(2048 - (w / 2), 2048 - (h / 2));
}

void gu_set_viewport(u32 w, u32 h) {
	sceGuViewport(2048, 2048, w, h);
}

void gu_set_scissor(u32 w, u32 h) {
	sceGuScissor(0, 0, w, h);
}

void gu_set_offset_and_viewport_and_scissor(u32 w, u32 h) {
	gu_set_offset(w, h);
	gu_set_viewport(w, h);
	gu_set_scissor(w, h);
}

void gu_set_rendertarget(const Texture* m) {
	texture_check_as_rendertarget(m);
	sceGuDrawBufferList(m->psm, m->data, m->stride_px);
	gu_set_offset_and_viewport_and_scissor(m->size_px[0], m->size_px[1]);
}

void gu_set_texture(const Texture* m) {
	texture_check_as_input(m);
	assert(m->nb_mipmap_levels == 1); // TODO: m->data should support multiple levels; needs to handle offset calculation
	sceGuTexMode(m->psm, m->nb_mipmap_levels - 1, 0, m->is_swizzled);
	for (size_t level = 0; level < m->nb_mipmap_levels; ++level)
		sceGuTexImage(level, m->size_px[0] >> level, m->size_px[1] >> level, m->stride_px >> level, m->data);
}

typedef struct {
	u8 gu_topology;
	u32 gu_vertex_format;
	size_t sizeof_vertex;
	void* vertices;
	size_t nb_vertices;
	u16* indices;
	size_t nb_indices;
} Mesh;

void mesh_allocate_buffers(Mesh* m) {
	m->vertices = malloc(m->nb_vertices * m->sizeof_vertex);
	m->indices = malloc(m->nb_indices * sizeof m->indices[0]);
	assert(m->nb_vertices * m->sizeof_vertex == 0 || m->vertices);
	assert(m->nb_indices == 0 || m->indices);
	assert(ptr_is_aligned(m->vertices, 16));
	assert(ptr_is_aligned(m->indices, 16));
}

void mesh_destroy(Mesh* m) {
	free(m->vertices);
	free(m->indices);
	*m = (Mesh) {0};
}

void mesh_draw_impl(const Mesh* m, bool b2d) {
	u32 count = m->nb_vertices;
	u32 vtype = m->gu_vertex_format | (b2d ? GU_TRANSFORM_2D : GU_TRANSFORM_3D);
	if (m->nb_indices) {
		vtype |= GU_INDEX_16BIT;
		count = m->nb_indices;
	}
	sceGuDrawArray(m->gu_topology, vtype, count, m->indices, m->vertices);

	// Stats
	g_frame_stats.meshes.nb_elements += count;
	g_frame_stats.meshes.nb_vertices += m->nb_vertices;
	if (m->gu_topology == GU_TRIANGLES) {
		g_frame_stats.meshes.nb_faces += count / 3;
	} else {
		assert(0 && "Calculating face number from this topology is not implemented yet");
	}
}

void mesh_draw_3d(const Mesh* m) {
	mesh_draw_impl(m, false);
}

void mesh_draw_2d(const Mesh* m) {
	mesh_draw_impl(m, true);
}

// Stolen from shadowprojection sample
void mesh_generate_grid(Mesh* m, size_t rows, size_t columns) {
	const f32 columns_minus_one_inv = 1.f / (columns - 1.f);
	const f32 rows_minus_one_inv = 1.f / (rows - 1.f);

	m->gu_topology = GU_TRIANGLES;
	m->gu_vertex_format = Vertex_Ni8_Pi16_FORMAT;
	Vertex_Ni8_Pi16* vertices = m->vertices;
	m->sizeof_vertex = sizeof vertices[0];

	m->nb_vertices = rows * columns;
	if (vertices) {
		for (size_t j = 0; j < rows; ++j) {
			for (size_t i = 0; i < columns; ++i) {
				vertices[j * columns + i] = (Vertex_Ni8_Pi16) {
					.normal = { 0, INT8_MAX, 0 },
					.position = {
						(i * rows_minus_one_inv * 2.f - 1.f) * INT16_MAX,
						0,
						(j * columns_minus_one_inv * 2.f - 1.f) * INT16_MAX,
					},
				};
			}
		}
		sceKernelDcacheWritebackRange(vertices, m->nb_vertices * sizeof vertices[0]);
	}

	m->nb_indices = (rows - 1) * (columns - 1) * 6;
	if (m->indices) {
		for (size_t j = 0; j < rows - 1; ++j) {
			for (size_t i = 0; i < columns - 1; ++i) {
				u16* curr = &m->indices[(i + (j * (columns - 1))) * 6];

				*curr++ = i + j * columns;
				*curr++ = (i+1) + j * columns;
				*curr++ = i + (j+1) * columns;

				*curr++ = (i+1) + j * columns;
				*curr++ = (i+1) + (j+1) * columns;
				*curr++ = i + (j + 1) * columns;
			}
		}
		sceKernelDcacheWritebackRange(m->indices, m->nb_indices * sizeof m->indices[0]);
	}
}

// Stolen from shadowprojection sample
void mesh_generate_torus(Mesh* m, size_t slices, size_t rows, f32 radius, f32 thickness) {
	// We're going to fit positions in a normalized integer format
	assert(radius + thickness <= 1.f);

	const f32 slices_inv = 1.f / slices;
	const f32 rows_inv = 1.f / rows;

	m->gu_topology = GU_TRIANGLES;
	m->gu_vertex_format = Vertex_Ni8_Pi16_FORMAT;
	Vertex_Ni8_Pi16* vertices = /*psp_uncached_ptr_or_null*/(m->vertices);
	m->sizeof_vertex = sizeof vertices[0];

	m->nb_vertices = slices * rows;
	if (vertices) {
		for (size_t j = 0; j < slices; ++j) {
			for (size_t i = 0; i < rows; ++i) {
				const f32 s = i + 0.5f;
				const f32 t = j;

				const f32 cs = cosf(s * 2 * GU_PI * slices_inv);
				const f32 ss = sinf(s * 2 * GU_PI * slices_inv);
				const f32 ct = cosf(t * 2 * GU_PI * rows_inv);
				const f32 st = sinf(t * 2 * GU_PI * rows_inv);

				f32 n[3] = { cs * ct, cs * st, ss };
				f32 p[3] = {
					(radius + thickness * cs) * ct,
					(radius + thickness * cs) * st,
					thickness * ss,
				};

				for (size_t d = 0; d < 3; ++d) {
					n[d] *= INT8_MAX;
					p[d] *= INT16_MAX;
				}

				vertices[j * rows + i] = (Vertex_Ni8_Pi16) {
					.normal = { n[0], n[1], n[2] },
					.position = { p[0], p[1], p[2] },
				};
			}
		}
		sceKernelDcacheWritebackRange(vertices, m->nb_vertices * sizeof vertices[0]);
	}

	m->nb_indices = slices * rows * 6;
	if (m->indices) {
		for (size_t j = 0; j < slices; ++j) {
			for (size_t i = 0; i < rows; ++i) {
				u16* curr = &m->indices[(i + (j * rows)) * 6];
				const size_t i1 = (i + 1) % rows;
				const size_t j1 = (j + 1) % slices;

				*curr++ = i + j * rows;
				*curr++ = i1 + j * rows;
				*curr++ = i + j1 * rows;

				*curr++ = i1 + j * rows;
				*curr++ = i1 + j1 * rows;
				*curr++ = i + j1 * rows;
			}
		}
		sceKernelDcacheWritebackRange(m->indices, m->nb_indices * sizeof m->indices[0]);
	}
}



u32 ALIGN16 g_gu_main_list[256 * 1024] = {0}; // Zeroing should not be necessary, but samples declare it as static, which zeroes it, so...

//
//
// Assets
//
//

void chdir_to_assets_directory(const char* argv0) {
	char pathbuf[256];

	// ----- Compute res path
	snprintf(pathbuf, sizeof pathbuf, "%s", argv0);
	if (!strncmp(argv0, "host0:/", 7)) {
		// PSPLINK
		// expected: host0:/.../bin/psp-xxx/foo.prx
		// final   : host0:/.../assets
	} else if (!strncmp(argv0, "ms0:/", 5)) {
		// Memory stick
		// expected: ms0:/PSP/GAME/foo/EBOOT.PBP
		// final   : ms0:/PSP/GAME/foo/assets
	}

	memcpy(strrchr(pathbuf, '/') + 1, "assets", strlen("assets") + 1);

	assert(pathbuf[0] != '\0');
	printf("Assets path: `%s`\n", pathbuf);

	strcat(pathbuf, "/..");
	printf("Setting current directory: `%s`\n", pathbuf);

	// ----- Set res path as current dir
	const int chdir_status = sceIoChdir(pathbuf);
	assert(chdir_status >= 0);
}

typedef SceUID Fd;

bool fd_is_valid(Fd fd) {
	return fd >= 0;
}

Fd fd_open_readonly(const char* path, bool should_assert) {
	const Fd fd = sceIoOpen(path, PSP_O_RDONLY, 0777);
	if (!fd_is_valid(fd)) {
		fprintf(stderr, "Could not open `%s`\n", path);
		assert(!should_assert && "Failed to open file");
	}
	return fd;
}

void fd_close(Fd fd) {
	assert(fd_is_valid(fd));
	sceIoClose(fd);
}

ssize_t fd_read(Fd fd, void* data, ssize_t size) {
	assert(fd_is_valid(fd));
	return sceIoRead(fd, data, size);
}

//
//
// TGA
//
//

// For writing a BGRA (32 bits per pixel) TGA file:
//	.image_type = 2, // Uncompressed true color
//	.image_width = vp.w,
//	.image_height = vp.h,
//	.bits_per_pixel = 32,
//	.image_descriptor = 8, // TGA 32
// Then write the header, then write the pixel data.
typedef struct TgaHeader {
	// 0–255 The number of bytes that the image ID field consists of. The image ID field can contain any information, but it is common for it to contain the date and time the image was created or a serial number.
	// As of version 2.0 of the TGA spec, the date and time the image was created is catered for in the extension area.
	u8 id_length;
	// 0 if image file contains no color map
	// 1 if present
	// 2–127 reserved by Truevision
	// 128–255 available for developer use
	u8 color_map_type;
	// Is enumerated in the lower three bits, with the fourth bit as a flag for RLE. Some possible values are:
	//
	// 0  -  No image data included.
	// 1  -  Uncompressed, color-mapped images.
	// 2  -  Uncompressed, RGB images.
	// 3  -  Uncompressed, black and white images.
	// 9  -  Runlength encoded color-mapped images.
	// 10  -  Runlength encoded RGB images.
	// 11  -  Compressed, black and white images.
	// 32  -  Compressed color-mapped data, using Huffman, Delta, and
	//         runlength encoding.
	// 33  -  Compressed color-mapped data, using Huffman, Delta, and
	//         runlength encoding.  4-pass quadtree-type process.
	//
	// Image type 1 and 9: Depending on the Pixel Depth value, image data representation is an 8, 15, or 16 bit index into a color map that defines the color of the pixel. Image type 2 and 10: The image data is a direct representation of the pixel color. For a Pixel Depth of 15 and 16 bit, each pixel is stored with 5 bits per color. If the pixel depth is 16 bits, the topmost bit is reserved for transparency. For a pixel depth of 24 bits, each pixel is stored with 8 bits per color. A 32-bit pixel depth defines an additional 8-bit alpha channel. Image type 3 and 11: The image data is a direct representation of grayscale data. The pixel depth is 8 bits for images of this type.
	u8 image_type; // 2 for uncompressed true color
				   // Has three subfields:
				   // - First entry index (2 bytes): index of first color map entry that is included in the file
				   // - Color map length (2 bytes): number of entries of the color map that are included in the file
				   // - Color map entry size (1 byte): number of bits per color map entry
				   // In case that not the entire color map is actually used by the image, a non-zero first entry index allows to store only a required part of the color map in the file.
	u8 color_map[5];
	u16 x_origin; // absolute coordinate of lower-left corner for displays where origin is at the lower left
	u16 y_origin; // as for X-origin
	u16 image_width;
	u16 image_height;
	u8 bits_per_pixel;
	// Bits 3-0 give the alpha channel depth, bits 5-4 give pixel ordering
	// Bit 4 of the image descriptor byte indicates right-to-left pixel ordering if set.
	// Bit 5 indicates an ordering of top-to-bottom.
	// Otherwise, pixels are stored in bottom-to-top, left-to-right order.
	//
	// For plain RGB data (no color map):
	// Bits 3-0 - number of attribute bits associated with each  |
	//            pixel.  For the Targa 16, this would be 0 or   |
	//            1.  For the Targa 24, it should be 0.  For     |
	//            Targa 32, it should be 8.                      |
	// Bit 4    - reserved.  Must be set to 0.                   |
	// Bit 5    - screen origin bit.                             |
	//            0 = Origin in lower left-hand corner.          |
	//            1 = Origin in upper left-hand corner.          |
	//            Must be 0 for Truevision images.               |
	// Bits 7-6 - Data storage interleaving flag.                |
	//            00 = non-interleaved.                          |
	//            01 = two-way (even/odd) interleaving.          |
	//            10 = four way interleaving.                    |
	//            11 = reserved.
	//
	u8 image_descriptor;

	// And after these, the following must appear in memory:
	// Image ID (length given from id_length)
	// Color map data (from color_map_type)
	// Image data (pixel colors)
	// Developer area (optional)
	// Extension area (optional)
	// File footer (optional)
} TgaHeader;

typedef struct TgaLoadedData {
	TgaHeader tga_header;
	void* pixel_data;
} TgaLoadedData;

TgaLoadedData tga_load(Fd fd) {
	TgaLoadedData out = { 0 };
	TgaHeader* hdr = &out.tga_header;

	ssize_t nread = fd_read(fd, hdr, sizeof *hdr);
	if (nread != sizeof *hdr)
		return (TgaLoadedData) { 0 }; // Don't risk returning a corrupted header.

	const size_t w = hdr->image_width;
	const size_t h = hdr->image_height;

	if (hdr->color_map_type
    || (hdr->image_type != 2) // Uncompressed RGB
	|| (hdr->bits_per_pixel != 32)
    || (hdr->image_descriptor & 0x1f) != 8 // Targa 32, include testing that the reserved bit is zero
	|| (hdr->image_descriptor >> 6) // We want non-interleaved
	) {
        assert(0 && "TGA Format isn't TGA32; make sure you saved it with an alpha channel"); // Format doesn't match
		return out;
    }

	if (hdr->id_length) {
		u8 image_id[255];
		nread = fd_read(fd, image_id, hdr->id_length);
		if (nread != hdr->id_length) {
			assert(0); // Invalid image ID
			return out;
		}
	}

	const ssize_t data_size = w * h * (hdr->bits_per_pixel / 8);
	out.pixel_data = malloc(data_size);

	nread = fd_read(fd, out.pixel_data, data_size);

	if (nread != data_size) {
		assert(0); // Not enough data for pixels?
		free(out.pixel_data);
		return out;
	}

	const bool is_origin_in_upper_left = !!(hdr->image_descriptor & (1 << 5));
	assert(is_origin_in_upper_left);

	// Data is BGRA in memory, need to swap to RGBA
	u8* pixel_data_u8 = out.pixel_data;
	for (size_t i = 0; i < data_size / 4; ++i) {
		const u8 tmp = pixel_data_u8[i * 4 + 0];
		pixel_data_u8[i * 4 + 0] = pixel_data_u8[i * 4 + 2];
		pixel_data_u8[i * 4 + 2] = tmp;
	}

	return out;
}

void tga_destroy(TgaLoadedData* m) {
	free(m->pixel_data);
	*m = (TgaLoadedData) {0};
}

typedef struct {
	bool should_swizzle;
} TextureLoadParams;

Texture texture_load_from_tga_fd(const TextureLoadParams* p, Fd fd) {
	TgaLoadedData tga = tga_load(fd);
	if (!tga.pixel_data) {
		tga_destroy(&tga);
		return (Texture) {0};
	}

	Texture out = {
		.psm = GU_PSM_8888,
		.size_px = { tga.tga_header.image_width, tga.tga_header.image_height },
		.stride_px = tga.tga_header.image_width,
		.nb_mipmap_levels = 1,
	};

	texture_allocate_buffers(&out);

	if (p->should_swizzle) {
		swizzle_fast(out.data, tga.pixel_data, tga.tga_header.image_width * 4, tga.tga_header.image_height);
		out.is_swizzled = true;
	} else {
		memcpy(out.data, tga.pixel_data, tga.tga_header.image_width * tga.tga_header.image_height * 4);
	}

	tga_destroy(&tga);

	return out;
}

Texture texture_load_from_tga_path(const TextureLoadParams* p, const char* path, bool should_assert) {
	const Fd fd = fd_open_readonly(path, should_assert);
	if (!fd_is_valid(fd))
		return (Texture) {0};

	const Texture out = texture_load_from_tga_fd(p, fd);
	fd_close(fd);
	return out;
}

//
//
// System callbacks
//
//

volatile bool g_exit_requested = false;

int psp_exit_callback(int arg1, int arg2, void *common) {
	g_exit_requested = true;
	return 0;
}

int psp_callbacks_thread_main(SceSize args, void *argp) {
	const int cbid = sceKernelCreateCallback("Exit callback", psp_exit_callback, NULL);
	sceKernelRegisterExitCallback(cbid);
	sceKernelSleepThreadCB();
	return 0;
}

int psp_setup_callbacks(void) {
	const int thid = sceKernelCreateThread("Callbacks thread", psp_callbacks_thread_main, 0x18, 0xFA0, 0, 0);
	if (thid >= 0)
		sceKernelStartThread(thid, 0, 0);

	return thid;
}

//
//
// LUTs
//
//

typedef enum LUT {
	LUT_SRGB_TO_LINEAR = 0,
	LUT_IDENTITY,
	LUT_LINEAR_TO_SRGB,
	LUT_SEPIA,
	LUT_INVERT,
	LUT_COUNT // Keep last
} LUT;

static const char* lut_get_name(LUT lut) {
	switch (lut) {
	case LUT_IDENTITY: return "Identity";
	case LUT_INVERT: return "Invert";
	case LUT_SRGB_TO_LINEAR: return "sRGB to linear";
	case LUT_LINEAR_TO_SRGB: return "Linear to sRGB";
	case LUT_SEPIA: return "Sepia";
	default: return "???";
	}
}

typedef enum LUTMode {
	LUT_MODE_1_TO_1, // dst_color[channel] = func(src_color[channel])
	LUT_MODE_3_TO_3, // dst_color += func(src_color[channel])
} LUTMode;

//
//
// Main
//
//

PSP_MODULE_INFO("Experiment", 0, 1, 1);
PSP_MAIN_THREAD_ATTR(PSP_THREAD_ATTR_USER | PSP_THREAD_ATTR_VFPU);

u64 g_frame_counter = 0;

int main(int argc, char* argv[]) {
	const int fb_psm = GU_PSM_8888;
	const size_t fb_size = PSP_SCREEN_STRIDE * PSP_SCREEN_HEIGHT * gu_psm_get_bytes_per_pixel(fb_psm);
	const size_t zb_size = PSP_SCREEN_STRIDE * PSP_SCREEN_HEIGHT * 2;

	u8* vram_cursor = NULL;
	u8* fbp0 = vram_cursor;
	vram_cursor += fb_size;
	u8* fbp1 = vram_cursor;
	vram_cursor += fb_size;
	u8* fbp2 = vram_cursor;
	vram_cursor += fb_size;
	u8* zb = vram_cursor;
	vram_cursor += zb_size;
	assert((uintptr_t) vram_cursor <= 2 * 1024 * 1024);

	pspFpuSetEnable(0); // Disable exceptions
	pspFpuSetRoundmode(PSP_FPU_RN);
	pspFpuSetFS(1); // flush denormals to zero instead of causing an exception

	psp_setup_callbacks();

	pspDebugScreenInit();

	sceGuInit();
	sceGuStart(GU_DIRECT, g_gu_main_list);

	sceGuDrawBuffer(fb_psm, fbp0, PSP_SCREEN_STRIDE);
	sceGuDispBuffer(PSP_SCREEN_WIDTH, PSP_SCREEN_HEIGHT, fbp1, PSP_SCREEN_STRIDE);
	sceGuDepthBuffer(zb, PSP_SCREEN_STRIDE);

	gu_set_offset_and_viewport_and_scissor(PSP_SCREEN_WIDTH, PSP_SCREEN_HEIGHT);

	sceGuDepthFunc(GU_GEQUAL);
	sceGuDepthMask(0);
	sceGuDepthOffset(0);
	sceGuDepthRange(0xffff, 0x0000);

	sceGuFog(0.f, 0.f, 0);

	bool dither = false;
	sceGuSetAllStatus(0);
	sceGuDisable(GU_ALPHA_TEST);
	sceGuDisable(GU_DEPTH_TEST);
	sceGuDisable(GU_SCISSOR_TEST);
	sceGuDisable(GU_STENCIL_TEST);
	sceGuDisable(GU_BLEND);
	sceGuDisable(GU_CULL_FACE);
	sceGuDisable(GU_DITHER);
	sceGuDisable(GU_FOG);
	sceGuDisable(GU_CLIP_PLANES);
	sceGuDisable(GU_TEXTURE_2D);
	sceGuDisable(GU_LIGHTING);
	sceGuDisable(GU_LIGHT0);
	sceGuDisable(GU_LIGHT1);
	sceGuDisable(GU_LIGHT2);
	sceGuDisable(GU_LIGHT3);
	sceGuDisable(GU_LINE_SMOOTH);
	sceGuDisable(GU_PATCH_CULL_FACE);
	sceGuDisable(GU_COLOR_TEST);
	sceGuDisable(GU_COLOR_LOGIC_OP);
	sceGuDisable(GU_FACE_NORMAL_REVERSE);
	sceGuDisable(GU_PATCH_FACE);
	sceGuDisable(GU_FRAGMENT_2X);

	sceGuClearColor(0);
	sceGuClearDepth(0);
	sceGuClearStencil(0);
	sceGuPixelMask(0);

	sceGuColorMaterial(GU_AMBIENT | GU_DIFFUSE | GU_SPECULAR); // command 83
	// 84: model emissive (RGB)
	// 85: model ambient (RGB)
	// 86: model diffuse (RGB)
	// 87: model specular (RGB)
	// 88: model ambient alpha
	sceGuMaterial(GU_AMBIENT, 0xffffffff); // 1: 85,88 (RGBA). 2: 86. 4: 87
	// sceGuAmbientColor() // commands 85,88 (RGBA) => model ambient color
	// sceGuAmbient(); // commands 92,93 (RGBA) => global ambient light color
	sceGuModelColor(0, 0xffffffff, 0xffffffff, 0xffffffff); // emissive, ambient, diffuse, specular // commands 84, 85, 86, 87 respectively // RGB, no alpha
	sceGuSpecular(12.f);
	// sceGuShadeModel(GU_FLAT);
	sceGuShadeModel(GU_SMOOTH);

	// sceGuColor = sceGuMaterial(7, c);, so this sets the ambient, diffuse and specular, only the RGB components. The ambient's alpha is unchanged.
	sceGuColor(0xffffffff); // primitive color, overriden by vertex color

	sceGuAmbientColor(0xffffffffu);

	sceGuTexFunc(GU_TFX_REPLACE, GU_TCC_RGBA);
	sceGuTexFilter(GU_LINEAR, GU_LINEAR);
	sceGuTexWrap(GU_CLAMP, GU_CLAMP);

	sceGuEnable(GU_SCISSOR_TEST);
	// sceGuEnable(GU_TEXTURE_2D);
	sceGuEnable(GU_DEPTH_TEST);
	sceGuEnable(GU_CULL_FACE);

	sceGuFrontFace(GU_CW);

	sceGuSetCallback(GU_CALLBACK_SIGNAL, gu_on_signal);

	sceGuClear(GU_COLOR_BUFFER_BIT | GU_DEPTH_BUFFER_BIT);
	sceGuFinish();
	sceGuSync(GU_SYNC_FINISH, GU_SYNC_WHAT_DONE);

	sceDisplayWaitVblankStart();
	sceGuDisplay(GU_TRUE);

	assert(argc >= 1);
	chdir_to_assets_directory(argv[0]);

	SceCtrlData previous_pad = {0};
	sceCtrlSetSamplingCycle(0); // Sync input sampling to VSync
	sceCtrlSetSamplingMode(PSP_CTRL_MODE_ANALOG);

	u32* clut[4];
	for (size_t i = 0; i < countof(clut); ++i)
		clut[i] = malloc(256 * sizeof clut[i][0]);

	Texture uv_test_texture = {
		.psm = GU_PSM_8888,
		.size_px = { 256, 256 },
		.stride_px = 256,
		.nb_mipmap_levels = 1,
		.is_swizzled = false,
	};

	texture_allocate_buffers(&uv_test_texture);

	{
		Texture* m = &uv_test_texture;
		u32* pixels = m->data;
		for (int y = 0; y < m->size_px[1]; ++y)
			for (int x = 0; x < m->size_px[0]; ++x)
				pixels[y * m->stride_px + x] = GU_ABGR(0xff, 0xff, y, x);
		
		sceKernelDcacheWritebackRange(pixels, m->size_px[0] * m->size_px[1] * sizeof pixels[0]);
	}

	Texture horizon_gradient_texture = texture_load_from_tga_path((const TextureLoadParams[]) {{ .should_swizzle = true }}, "assets/horizon_gradient.tga", true);

	Mesh torus_mesh = {0};
	Mesh grid_mesh = {0};
	for (size_t i = 0; i < 2; ++i) {
		mesh_generate_torus(&torus_mesh, 48, 48, 0.5f, 0.3f);
		mesh_generate_grid(&grid_mesh, 16, 16);
		if (i == 0) {
			mesh_allocate_buffers(&torus_mesh);
			mesh_allocate_buffers(&grid_mesh);
		}
	}

	ScePspFVector3 up_vector = { 0.f, 1.f, 0.f };
	ScePspFVector3 eye_target_position = { 0.f, 10.f, 0.f };
	ScePspFVector3 eye_position = { 0.f, 10.f, 10.f };
	ScePspFVector3 torus_position = { 0.f, 10.f, -10.f };
	ScePspFVector3 torus_scale = { 10.f, 10.f, 10.f };
	ScePspFVector3 grid_scale = { 100.f, 100.f, 100.f };

	ScePspFMatrix4 grid_model_matrix;
	ScePspFMatrix4 grid_rotation_matrix;
	ScePspFMatrix4 torus_model_matrix;
	ScePspFMatrix4 torus_rotation_matrix;
	ScePspFMatrix4 view_matrix;
	ScePspFMatrix4 view_rotation_matrix;
	ScePspFMatrix4 projection_matrix;

	LUT lut = LUT_IDENTITY;
	LUTMode lut_mode = LUT_MODE_1_TO_1;

	bool use_framebuffer_as_texture = false;

	f32 last_frame_duration = 1.f / 60.f;
	f32 time_since_start = 0.f;
	TickRange current_frame_tick_range = {0};

	f32 fun = 0.f;
	f32 fun_speed = 0.5f;

	while (!g_exit_requested) {
		psp_rtc_get_current_tick_checked(&current_frame_tick_range.start);

		g_frame_stats = (FrameStats) {0};
		g_frame_stats.cpu.start = current_frame_tick_range.start;

		f32 vfpu_mmul_result = 0.f;
		if (false) {
			u8 matrices_buffer[63 + 3 * sizeof(m4)];
			m4* matrices = ptr_align(matrices_buffer, 64);
			for (size_t i = 0; i < 18000; ++i) {
				memset(&matrices[1], i, sizeof matrices[1]);
				memset(&matrices[2], i, sizeof matrices[2]);
				vfpu_m4_mul(&matrices[0], &matrices[1], &matrices[2]);
				vfpu_mmul_result += matrices[0].cols[0][0];
			}
		}

		SceCtrlData pad;
		if (sceCtrlPeekBufferPositive(&pad, 1)) {
			if (pad.Buttons != previous_pad.Buttons) {
				if (pad.Buttons & PSP_CTRL_LTRIGGER)
					lut = (lut + LUT_COUNT - 1) % LUT_COUNT;
				if (pad.Buttons & PSP_CTRL_RTRIGGER)
					lut = (lut + 1) % LUT_COUNT;
				if (pad.Buttons & PSP_CTRL_CROSS)
					use_framebuffer_as_texture ^= 1;
				if (pad.Buttons & PSP_CTRL_SQUARE)
					dither ^= 1;
			}

			if (pad.Buttons & PSP_CTRL_LEFT)
				fun -= last_frame_duration * fun_speed;
			if (pad.Buttons & PSP_CTRL_RIGHT)
				fun += last_frame_duration * fun_speed;

			previous_pad = pad;
		}

		u32* pcr = psp_uncached_ptr_non_null(clut[0]);
		u32* pcg = psp_uncached_ptr_non_null(clut[1]);
		u32* pcb = psp_uncached_ptr_non_null(clut[2]);
		lut_mode = LUT_MODE_1_TO_1;
		switch (lut) {
		case LUT_IDENTITY: 
			lut_mode = LUT_MODE_1_TO_1;
			for (int i = 0; i < 256; ++i) {
				pcr[i] = GU_ABGR(0xff, i, i, i);
			}
			break;
		case LUT_INVERT: 
			lut_mode = LUT_MODE_1_TO_1;
			for (int i = 0; i < 256; ++i) {
				int x = 256 - i;
				pcr[i] = GU_ABGR(0xff, x, x, x);
			}
			break;
		case LUT_SRGB_TO_LINEAR: 
			lut_mode = LUT_MODE_1_TO_1;
			for (int i = 0; i < 256; ++i) {
				const int x = 255 * powf(i / 255.f, 2.2f);
				pcr[i] = GU_ABGR(0xff, x, x, x);
			}
			break;
		case LUT_LINEAR_TO_SRGB: 
			lut_mode = LUT_MODE_1_TO_1;
			for (int i = 0; i < 256; ++i) {
				const int x = 255 * powf(i / 255.f, 1.f / 2.2f);
				pcr[i] = GU_ABGR(0xff, x, x, x);
			}
			break;
		case LUT_SEPIA:
			lut_mode = LUT_MODE_3_TO_3;
			for (int i = 0; i < 256; ++i) {
				// outputRed   = (inputRed * .393) + (inputGreen * .769) + (inputBlue * .189)
				// outputGreen = (inputRed * .349) + (inputGreen * .686) + (inputBlue * .168)
				// outputBlue  = (inputRed * .272) + (inputGreen * .534) + (inputBlue * .131)
				pcr[i] = GU_ABGR(0xff, (u32) (.272f * i), (u32) (.349f * i), (u32) (.393f * i));
				pcg[i] = GU_ABGR(0xff, (u32) (.534f * i), (u32) (.686f * i), (u32) (.769f * i));
				pcb[i] = GU_ABGR(0xff, (u32) (.131f * i), (u32) (.168f * i), (u32) (.189f * i));
			}
			break;
		default:
			break;
		}

		ScePspFMatrix4 lightMatrix;

		// orbiting light
		{
			ScePspFVector3 lightLookAt = eye_target_position;
			ScePspFVector3 rot1 = { 0, 1.f * 0.79f * (GU_PI / 180.0f), 0 };
			ScePspFVector3 rot2 = { -(GU_PI / 180.0f) * 60.0f, 0, 0 };
			ScePspFVector3 pos = {0, 0, 6.f };

			gumLoadIdentity(&lightMatrix);
			gumTranslate(&lightMatrix,&lightLookAt);
			gumRotateXYZ(&lightMatrix,&rot1);
			gumRotateXYZ(&lightMatrix,&rot2);
			gumTranslate(&lightMatrix,&pos);
		}

		ScePspFVector3 lightPos = { lightMatrix.w.x, lightMatrix.w.y, lightMatrix.w.z };
		ScePspFVector3 lightDir = { lightMatrix.z.x, lightMatrix.z.y, lightMatrix.z.z };

		// Object matrices
		gumLoadIdentity(&grid_model_matrix);
		gumScale(&grid_model_matrix, &grid_scale);

		gumLoadIdentity(&grid_rotation_matrix);

		gumLoadIdentity(&torus_rotation_matrix);
		gumRotateY(&torus_rotation_matrix, time_since_start * -1.8f);

		gumLoadIdentity(&torus_model_matrix);
		gumTranslate(&torus_model_matrix, &torus_position);
		gumScale(&torus_model_matrix, &torus_scale);
		gumMultMatrix(&torus_model_matrix, &torus_model_matrix, &torus_rotation_matrix);

		gumLoadIdentity(&view_matrix);
		gumLookAt(&view_matrix, &eye_position, &eye_target_position, &up_vector);

		view_rotation_matrix = view_matrix;
		view_rotation_matrix.w = (ScePspFVector4) { 0, 0, 0, 1 };

		gumLoadIdentity(&projection_matrix);
		gumPerspective(&projection_matrix, 60.f, PSP_SCREEN_WIDTH / (f32) PSP_SCREEN_HEIGHT, 0.5f, 1000.f);

		// Start drawing
		sceGuStart(GU_DIRECT, g_gu_main_list);
		gu_insert_clock_start_marker();

		sceGuClearColor(GU_ABGR(0, 0xff, 0, 0));
		sceGuClearDepth(0);
		sceGuClear(GU_COLOR_BUFFER_BIT | GU_DEPTH_BUFFER_BIT);

		if (dither)
			sceGuEnable(GU_DITHER);
		else
			sceGuDisable(GU_DITHER);

		{
			sceGuSetMatrix(GU_VIEW, &view_matrix);
			sceGuSetMatrix(GU_PROJECTION, &projection_matrix);

			sceGuEnable(GU_TEXTURE_2D);
			gu_set_texture(&horizon_gradient_texture);
			gu_draw_fullscreen_quad(horizon_gradient_texture.size_px[0], horizon_gradient_texture.size_px[1]);
			sceGuDisable(GU_TEXTURE_2D);

			sceGuLight(0, GU_DIRECTIONAL, GU_DIFFUSE_AND_SPECULAR, &lightDir);
			sceGuLightColor(0, GU_DIFFUSE, 0xffffffff);
			sceGuLightColor(0, GU_SPECULAR, 0xffffffff);
			sceGuLightAtt(0, 1.0f, 0.0f, 0.0f);
			sceGuAmbient(0x00202020);
			sceGuEnable(GU_LIGHTING);
			sceGuEnable(GU_LIGHT0);

			{
				sceGuEnable(GU_TEXTURE_2D);
				sceGuTexProjMapMode(GU_NORMALIZED_NORMAL);
				sceGuTexMapMode(GU_TEXTURE_MATRIX, 0, 0);
				gu_set_texture(&horizon_gradient_texture);

				ScePspFMatrix4 texture_matrix;
				gumLoadIdentity(&texture_matrix);
				gumTranslate(&texture_matrix, (const ScePspFVector3[]) {{ 0.5f, 0.5f, 1.f }});
				gumScale(&texture_matrix, (const ScePspFVector3[]) {{ 0.5f, -0.5f, 0.f }});
				gumMultMatrix(&texture_matrix, &texture_matrix, &view_rotation_matrix);
				gumMultMatrix(&texture_matrix, &texture_matrix, &grid_rotation_matrix);

				sceGuSetMatrix(GU_TEXTURE, &texture_matrix);

				sceGuAmbientColor(0);
				sceGuColor(GU_ABGR(0xff, 0xff, 0xff, 0x00));
				sceGuSetMatrix(GU_MODEL, &grid_model_matrix);
				mesh_draw_3d(&grid_mesh);

				sceGuDisable(GU_TEXTURE_2D);
				sceGuTexProjMapMode(GU_UV);
				sceGuTexMapMode(GU_TEXTURE_COORDS, 0, 0);
			}

			{
				sceGuEnable(GU_TEXTURE_2D);
				sceGuTexProjMapMode(GU_NORMALIZED_NORMAL);
				sceGuTexMapMode(GU_TEXTURE_MATRIX, 0, 0);
				gu_set_texture(&horizon_gradient_texture);

				ScePspFMatrix4 texture_matrix;
				gumLoadIdentity(&texture_matrix);
				gumTranslate(&texture_matrix, (const ScePspFVector3[]) {{ 0.5f, 0.5f, 1.f }});
				gumScale(&texture_matrix, (const ScePspFVector3[]) {{ 0.5f, -0.5f, 0.f }});
				gumMultMatrix(&texture_matrix, &texture_matrix, &view_rotation_matrix);
				gumMultMatrix(&texture_matrix, &texture_matrix, &torus_rotation_matrix);

				sceGuSetMatrix(GU_TEXTURE, &texture_matrix);

				sceGuAmbientColor(0);
				sceGuColor(GU_ABGR(0xff, 0x00, 0xff, 0xff));
				sceGuSetMatrix(GU_MODEL, &torus_model_matrix);
				mesh_draw_3d(&torus_mesh);

				sceGuDisable(GU_TEXTURE_2D);
				sceGuTexProjMapMode(GU_UV);
				sceGuTexMapMode(GU_TEXTURE_COORDS, 0, 0);
			}

			sceGuDisable(GU_LIGHTING);
			sceGuDisable(GU_LIGHT0);
		}

		if (false) {
			Texture test_texture_t32 = uv_test_texture;
			test_texture_t32.psm = GU_PSM_T32;
			gu_set_texture(&test_texture_t32);

			sceGuTexFunc(GU_TFX_REPLACE, GU_TCC_RGB);
			sceGuTexFilter(GU_LINEAR, GU_LINEAR);
			sceGuTexWrap(GU_CLAMP, GU_CLAMP);

			sceGuAmbientColor(0xffffffffu);
			sceGuColor(0xffffffffu);

			f32 fu = uv_test_texture.size_px[0];
			f32 fv = uv_test_texture.size_px[1];
			if (use_framebuffer_as_texture) {
				Texture rendertarget = {
					.psm = fb_psm,
					.nb_mipmap_levels = 1,
					.size_px = { PSP_SCREEN_WIDTH, PSP_SCREEN_HEIGHT },
					.stride_px = PSP_SCREEN_STRIDE,
					.data = (u8*) sceGeEdramGetAddr() + (uintptr_t) fbp2,
				};
				gu_set_rendertarget(&rendertarget);

				gu_set_texture(&uv_test_texture);
				gu_draw_fullscreen_quad(uv_test_texture.size_px[0], uv_test_texture.size_px[1]);

				rendertarget.data = (u8*) sceGeEdramGetAddr() + (uintptr_t) fbp0;
				gu_set_rendertarget(&rendertarget);

				const Texture src_texture = {
					.psm = GU_PSM_T32,
					.nb_mipmap_levels = 1,
					.size_px = { 512, 512 },
					.stride_px = PSP_SCREEN_STRIDE,
					.data = (u8*) sceGeEdramGetAddr() + (uintptr_t) fbp2,
				};
				gu_set_texture(&src_texture);

				fu = PSP_SCREEN_WIDTH;
				fv = PSP_SCREEN_HEIGHT;
			}

			switch (lut_mode) {
			case LUT_MODE_1_TO_1:
				sceGuClutLoad(256 / 8, clut[0]); // upload 32*8 entries (256)
				for (int i = 0; i < 3; ++i) {
					sceGuClutMode(GU_PSM_8888, i * 8, 0xff, 0);
					sceGuPixelMask(~(0xffu << (i*8)));
					gu_draw_fullscreen_quad(fu, fv);
				}
				sceGuPixelMask(0);
				break;
			case LUT_MODE_3_TO_3:
				sceGuEnable(GU_BLEND);
				sceGuBlendFunc(GU_ADD, GU_FIX, GU_FIX, 0xffffffff, 0xffffffff);
				for (int i = 0; i < 3; ++i) {
					sceGuClutMode(GU_PSM_8888, i * 8, 0xff, 0);
					sceGuClutLoad(256 / 8, clut[i]); // upload 32*8 entries (256)
					gu_draw_fullscreen_quad(fu, fv);
				}
				sceGuDisable(GU_BLEND);
				break;
			default:
				break;
			}
		}

		psp_rtc_get_current_tick_sync(&g_frame_stats.cpu.end);
		gu_insert_clock_end_marker();
		sceGuFinish();
		sceGuSync(GU_SYNC_FINISH, GU_SYNC_WHAT_DONE);

		int debug_screen_pos[2] = { 1, 1 };
		pspDebugScreenSetOffset((intptr_t)fbp0);
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("LUT (cycle via L/R): %s", lut_get_name(lut));
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("%s (toggle via X)", use_framebuffer_as_texture ? "Using FB as texture" : "Not using FB as texture");
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("Frame: %.3f ms", 1000.0 * (f64) last_frame_duration);
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("CPU: %.3f ms", 1000.0 * (f64) tick_range_get_duration(g_frame_stats.cpu));
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("GPU: %.3f ms", 1000.0 * (f64) tick_range_get_duration(g_frame_stats.gpu));
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("GPU: %" PRIu64 " elements", (u64) g_frame_stats.meshes.nb_elements);
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("GPU: %" PRIu64 " vertices", (u64) g_frame_stats.meshes.nb_vertices);
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("GPU: %" PRIu64 " faces", (u64) g_frame_stats.meshes.nb_faces);
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("VFPU MMUL result: %.3f", (f64) vfpu_mmul_result);
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("Dither: %s", dither ? "on" : "off");
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("Fun: %f", (f64) fun);

		sceDisplayWaitVblankStart();
		fbp1 = fbp0;
		fbp0 = sceGuSwapBuffers();

		g_frame_counter++;

		psp_rtc_get_current_tick_checked(&current_frame_tick_range.end);
		last_frame_duration = tick_range_get_duration(current_frame_tick_range);
		time_since_start += last_frame_duration;
	}

	mesh_destroy(&torus_mesh);
	mesh_destroy(&grid_mesh);

	texture_destroy(&uv_test_texture);
	texture_destroy(&horizon_gradient_texture);

	for (size_t i = 0; i < countof(clut); ++i)
		free(clut[i]);

	sceGuTerm();

	sceKernelExitGame();
	return 0;
}