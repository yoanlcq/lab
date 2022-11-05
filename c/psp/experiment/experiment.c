//
// TODO:
// - Ability to render 3D at half res
// - LUTs: get it to work with bloom as well
// - Render to vertex buffer
//   - First, proof-of-concept for rendering points with T16_XYZ8
// - Dither may be ugly in some postprocessing steps (e.g render to half-res buffer)
//
// TODO: Notes:
// - Coordinate space is +X right, +Y up, -Z forward (can be changed but the GU's T&L pipeline assumes that Z points towards the viewer, so matrices need to be adjusted)
// - Dithering is a bit ugly
// - Color materials
// - Bezier, Splines
//   TODO: How to determine facing?
// - Even if your mesh doesn't have UVs, if GU_TEXTURE_2D is enabled, the texture will still be sampled, seemingly with UV (0,0). If sceGuTexImage was never called, it seems to sample red (but that may be luck, or lack thereof).
// - Texture sizes must be power of two, due to the way sizes are encoded for GE commands. Non power-of-two literally can't be represented, so you'll get the next lower power of two instead.
// - How positions/normals are transformed to UVs (they start in model space, set W to 1, transform by texture matrix, divide by Z at the end)
// - sceGuTexProjMapMode() explained:
//   - GU_UV: UVs are specified in vertices, converted to a 4D vector like so : { u, v, 1, 1 }
//   - GU_POSITION/NORMAL/NORMALIZED_NORMAL: 3D vectors in model-space, specified from vertices, converted to a 4D vector like so: { x, y, z, 1 }
// - It appears that UVs are extended to 4D:
//   - Setting the W component to 1 allows it to be treated as a 3D point (rather than a direction), making translations effective.
//   - Before sampling, the 4D vector is divided by its Z component. I do not know if it is divided by W as well.
// - sceGuTexMapMode() explained:
//   - GU_TEXTURE_COORDS: UVs are taken as-is (TODO: does it ignore sceGuTexProjMapMode(), or doesn't it?)
//   - GU_TEXTURE_MATRIX: UVs are transformed via the texture matrix (set via sceGuSetMatrix(GU_TEXTURE, ...))
//   - GU_ENVIRONMENT_MAP: UVs are calculated as follows:
//     uv[0] = (dot(lights[light_indices[0]].position.normalized(), model_worldspace_normal) + 1) / 2;
//     uv[1] = (dot(lights[light_indices[1]].position.normalized(), model_worldspace_normal) + 1) / 2;
//     In essence, the value at each axis is "how much the world-space normal agrees with a fixed world-space direction".
//     The two "unknown" arguments to sceGuTexMapMode() are actually 2-bit values representing a light index, the position of which will be used as a column for the environment map matrix.
//
// An object's material is described as follows:
// - Feature flags: GU_AMBIENT | GU_DIFFUSE | GU_SPECULAR (set via sceGuColorMaterial())
// - Model emissive RGB (GU provides no API for setting it independently, however it is the 1st argument to sceGuModelColor() and can be set that way)
//   Question: is it only used when GU_LIGHTING is enabled?
// - Model ambient RGBA (RGB is set via command 85, alpha is set via command 88).
//   sceGuMaterial(GU_AMBIENT, ...) will set the complete RGBA value.
//   sceGuAmbientColor() will do the same. Not to be confused with sceGuAmbient(), which sets the *global* ambient light color (RGBA). Also note that each light may specify an ambient component.
//   sceGuModelColor() will only set the RGB part.
//   Question: What is the alpha used for?
// - Model diffuse RGB
//   May be set via sceGuMaterial() or sceGuModelColor().
// - Model specular RGB
//   May be set via sceGuMaterial() or sceGuModelColor().
// - Specular power (sceGuSpecular())
// - Shade model: smooth or flat (sceGuShadeModel())
// - Q: So what does sceGuColor() do?
//   A: It's equivalent to sceGuMaterial(GU_AMBIENT | GU_DIFFUSE | GU_SPECULAR, rgb), which is also equivalent to passing the same color to all arguments of sceGuModelColor(), except the emissive.
//      The ambient's alpha component is not set by this function.
//
// The global lighting model:
// - Either GU_SINGLE_COLOR or GU_SEPARATE_SPECULAR_COLOR
// 
// Each light (NOTE: uses the GE commands as reference, not the GU API):
// - Components, any of:
//   - GU_AMBIENT_AND_DIFFUSE
//   - GU_DIFFUSE_AND_SPECULAR
//   - GU_UNKNOWN_LIGHT_COMPONENT (diffuse color, affected by specular power)
// - Type: directional | spot | point.
// - Position
// - Direction
// - Attenuation (constant + linear + quadratic)
// - Spotlight exponent + cutoff (cosine of angle)
// - Ambient RGB
// - Diffuse RGB
// - Specular RGB
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
#include <pspsdk.h>

#include "me.h"

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

#define app_assert(x) do { if (!(x)) { fprintf(stderr, "Assertion failed: %s\n", #x); fflush(stderr); *(char*)NULL = 0; } } while (0)

typedef f32 __attribute__((vector_size(16))) v4;
typedef struct { v4 cols[4]; } m4;

#define ALIGN_N(x) __attribute__((aligned(x)))
#define ALIGN16 ALIGN_N(16)

#define countof(x) (sizeof((x)) / sizeof((x)[0]))

static inline void* psp_uncached_ptr_non_null(const void* p) {
	app_assert(p); // If you're passing NULL, you'll get an uncached NULL ptr but it won't evaluate to NULL, so that may trick conditionals and do bad stuff.
	return (void*) (((uintptr_t) p) | 0x40000000ul);
}

static inline void* psp_uncached_ptr_or_null(const void* p) {
	return p ? psp_uncached_ptr_non_null(p) : NULL;
}

static inline uintptr_t psp_ptr_actual_addr_u(const void* p) {
	return ((uintptr_t) p) & ~0x40000000ul;
}

static inline intptr_t psp_ptr_actual_addr_i(const void* p) {
	return ((intptr_t) p) & ~0x40000000ul;
}

static inline void* psp_ptr_actual_addr_p(const void* p) {
	return (void*) psp_ptr_actual_addr_u(p);
}

static inline bool size_is_power_of_two_nonzero(size_t x) {
	return x != 0 && !(x & (x - 1));
}

static inline bool size_is_power_of_two_or_zero(size_t x) {
	return x == 0 || !(x & (x - 1));
}

static inline void* ptr_align(const void* p, uintptr_t a) {
	app_assert(size_is_power_of_two_nonzero(a));
	return (void*) ((((uintptr_t) p) + a - 1) & ~(a - 1));
}

static inline bool ptr_is_aligned(const void* p, uintptr_t a) {
	app_assert(size_is_power_of_two_nonzero(a));
	return (((uintptr_t) p) & (a - 1)) == 0;
}

u32 u32_clz(u32 x) {
	return __builtin_clz(x);
}

u32 u32_next_power_of_two(u32 x) {
	app_assert(x > 1);
	return 1u << (32u - u32_clz(x - 1u));
}

// log2() of the given integer, result is undefined if x == 0
static inline u32 u32_log2(u32 x) {
	app_assert(x); // Undefined result if zero
	return ((sizeof(u32) * 8 - 1) - u32_clz(x));
}

// Returns log2(x) + 1, and has a well-defined result for x==0 (returns 0).
// This can be used for getting the number of mip levels for a given texture size, or the number of bits required for storing a value.
static inline u32 u32_log2_plus_1(u32 x) {
	return ((sizeof(u32) * 8) - u32_clz(x));
}

u32 u32_min(u32 a, u32 b) { return a < b ? a : b; }
u32 u32_max(u32 a, u32 b) { return a > b ? a : b; }


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
	app_assert(ptr_is_aligned(result, 64));
	app_assert(ptr_is_aligned(a, 64));
	app_assert(ptr_is_aligned(b, 64));
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
// Modules
//
//

typedef struct {
	int handle;
} Module;

Module module_load_and_start(const char* path) {
	const Module out = { .handle = pspSdkLoadStartModule(path, PSP_MEMORY_PARTITION_KERNEL) };
	if (out.handle < 0) {
		fprintf(stderr, "Failed to load `%s`: 0x%08x\n", path, out.handle);
	} else {
		printf("Successfully loaded `%s`: 0x%08x\n", path, out.handle);
	}
	return out;
}

bool module_stop_and_unload(Module* m) {
	app_assert(m->handle >= 0);

	int status;
	sceKernelStopModule(m->handle, 0, NULL, &status, NULL);

	const int ret = sceKernelUnloadModule(m->handle);
	if (ret < 0) {
		fprintf(stderr, "Couldn't unload module: 0x%08x\n", ret);
	}

	m->handle = -1;

	return ret >= 0;
}

//
//
// Media Engine
//
//

typedef struct {
	Module module;
	volatile struct me_struct* mei;
} MeManager;

bool me_manager_init(MeManager* m, const char* prx_path) {
	m->module = module_load_and_start(prx_path);
	if (m->module.handle < 0) {
		fprintf(stderr, "ME manager: Failed to load module\n");
		return false;
	}
	
	m->mei = psp_uncached_ptr_non_null(malloc(sizeof *m->mei));
	*m->mei = (struct me_struct) {0}; // Just in case
	sceKernelDcacheWritebackInvalidateAll();

	const int init_status = InitME(m->mei);
	if (init_status != 0) {
		fprintf(stderr, "ME manager: InitME() failed: 0x%08x\n", init_status);
		return false;
	}

	printf("ME manager: initialized\n");
	return true;
}

bool me_manager_deinit(MeManager* m) {
	printf("ME manager: wait\n");
	const int wait_status = WaitME(m->mei);
	printf("ME manager: WaitME() returned 0x%08x\n", wait_status);
	printf("ME manager: kill\n");
	KillME(m->mei);
	printf("ME manager: unload module\n");
	const bool unload_ok = module_stop_and_unload(&m->module);
	printf("ME manager: Module unload: %s\n", unload_ok ? "ok" : "failed");
	free((void*) m->mei);
	return unload_ok;
}

int me_test_job(void* x) {
	volatile int* val = psp_uncached_ptr_non_null(x);
	__atomic_store_n(&val[0], 42, __ATOMIC_SEQ_CST);
	__atomic_store_n(&val[50 * 1024 * 1024 / 4 - 1], 42, __ATOMIC_SEQ_CST);
	return 32;
}

void me_manager_test(MeManager* m) {
	volatile int* val = malloc(50 * 1024 * 1024);
	volatile int mem = -1;
	volatile int* sval = &mem;
	val[0] = -1;
	val[50 * 1024 * 1024 / 4 - 1] = -1;
	printf("mval addr: %p\n", val);
	printf("sval addr: %p\n", sval);
	sceKernelDcacheWritebackInvalidateAll();
	const int status = BeginME(m->mei, me_test_job, (void*) val, -1, (void*) val, -1, (void*) val);
	sceKernelDcacheWritebackInvalidateAll();
	printf("BeginME() returned 0x%08x\n", status);
	const int wait_status = WaitME(m->mei);
	printf("WaitME() returned %i\n", wait_status);
	printf("Value0 written by ME: %i\n", __atomic_load_n(&val[0], __ATOMIC_SEQ_CST));
	printf("Value1 written by ME: %i\n", __atomic_load_n(&val[50 * 1024 * 1024 / 4 - 1], __ATOMIC_SEQ_CST));
}

//
//
// Graphics
//
//

#define PSP_SCREEN_STRIDE 512
#define PSP_SCREEN_WIDTH  480
#define PSP_SCREEN_HEIGHT 272

// Make an absolute pointer relative to VRAM
void* psp_ptr_to_vram(const void* p) {
	app_assert(psp_ptr_actual_addr_i(p) >= (intptr_t) sceGeEdramGetAddr());
	app_assert(psp_ptr_actual_addr_i(p) <= (intptr_t) sceGeEdramGetAddr() + (intptr_t) sceGeEdramGetSize());
	return (void*) ((const u8*) p - (intptr_t) sceGeEdramGetAddr());
}

// Makes an absolute pointer from a pointer relative to VRAM
void* psp_ptr_from_vram(const void* p) {
	app_assert(psp_ptr_actual_addr_i(p) >= 0);
	app_assert(psp_ptr_actual_addr_i(p) <= (intptr_t) sceGeEdramGetSize());
	return (void*) ((const u8*) p + (intptr_t) sceGeEdramGetAddr());
}

typedef struct {
	u8 nb_bits;
} PsmChannelInfo;

typedef struct {
	u8 psm; // The psm itself, for convenience
	u8 nb_bits;
	PsmChannelInfo channels[4];
} PsmInfo;

PsmInfo gu_psm_get_info(u32 psm) {
	switch (psm) {
	case GU_PSM_5650: return (PsmInfo) { .psm = psm, .nb_bits = 16, .channels = { { .nb_bits = 5 }, { .nb_bits = 6 }, { .nb_bits = 5 }, { .nb_bits = 0 } } };
	case GU_PSM_5551: return (PsmInfo) { .psm = psm, .nb_bits = 16, .channels = { { .nb_bits = 5 }, { .nb_bits = 5 }, { .nb_bits = 5 }, { .nb_bits = 1 } } };
	case GU_PSM_4444: return (PsmInfo) { .psm = psm, .nb_bits = 16, .channels = { { .nb_bits = 4 }, { .nb_bits = 4 }, { .nb_bits = 4 }, { .nb_bits = 4 } } };
	case GU_PSM_8888: return (PsmInfo) { .psm = psm, .nb_bits = 32, .channels = { { .nb_bits = 8 }, { .nb_bits = 8 }, { .nb_bits = 8 }, { .nb_bits = 8 } } };
	case GU_PSM_T4  : return (PsmInfo) { .psm = psm, .nb_bits = 4 };
	case GU_PSM_T8  : return (PsmInfo) { .psm = psm, .nb_bits = 8 };
	case GU_PSM_T16 : return (PsmInfo) { .psm = psm, .nb_bits = 16 };
	case GU_PSM_T32 : return (PsmInfo) { .psm = psm, .nb_bits = 32 };
	case GU_PSM_DXT1:
	case GU_PSM_DXT3:
	case GU_PSM_DXT5:
		app_assert(0 && "DXT is not representable yet in PsmInfo and doesn't have bits-per-pixel since it operates on 4x4 blocks instead");
		return (PsmInfo) {0};
	default:
		app_assert(0 && "Unknown PSM");
		return (PsmInfo) {0};
	}
}

size_t gu_psm_get_bits_per_pixel(u32 psm) {
	return gu_psm_get_info(psm).nb_bits;
}

size_t gu_psm_get_bytes_per_pixel(u32 psm) {
	return gu_psm_get_bits_per_pixel(psm) / 8;
}

typedef struct {
	u64 nb_elements;
	u64 nb_vertices;
	u64 nb_faces;
} FrameMeshesStats;

typedef struct {
	TickRange cpu, gpu, cpu_with_gpu_sync;
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
typedef struct { i16 uv[2]; i16 position[3]; } Vertex_Ti16_Pi16;
typedef struct { i16 uv[2]; u32 color; i8 position[3]; } Vertex_Ti16_C8888_Pi8;
typedef struct { i16 uv[2]; i8 position[3]; } Vertex_Ti16_Pi8;
typedef struct { i8 uv[2]; i8 normal[3]; i8 position[3]; } Vertex_Ti8_Ni8_Pi8;
typedef struct { i8 normal[3]; i8 position[3]; } Vertex_Ni8_Pi8;
typedef struct { i8 normal[3]; i16 position[3]; } Vertex_Ni8_Pi16;
typedef struct { f32 normal[3]; f32 position[3]; } Vertex_Nf32_Pf32;

#define Vertex_Tf32_Pf32_FORMAT (GU_TEXTURE_32BITF | GU_VERTEX_32BITF)
#define Vertex_Ti16_Pi16_FORMAT (GU_TEXTURE_16BIT | GU_VERTEX_16BIT)
#define Vertex_Ti16_C8888_Pi8_FORMAT (GU_TEXTURE_16BIT | GU_COLOR_8888 | GU_VERTEX_8BIT)
#define Vertex_Ti16_Pi8_FORMAT (GU_TEXTURE_16BIT | GU_VERTEX_8BIT)
#define Vertex_Ti8_Ni8_Pi8_FORMAT (GU_TEXTURE_8BIT | GU_NORMAL_8BIT | GU_VERTEX_8BIT)
#define Vertex_Ni8_Pi8_FORMAT (GU_NORMAL_8BIT | GU_VERTEX_8BIT)
#define Vertex_Ni8_Pi16_FORMAT (GU_NORMAL_8BIT | GU_VERTEX_16BIT)
#define Vertex_Nf32_Pf32_FORMAT (GU_NORMAL_32BITF | GU_VERTEX_32BITF)

typedef struct {
	void* data;
	u16 size_px[2];
	u16 stride_px;
	u8 psm : 4;
	u8 nb_mipmap_levels : 4; // Must not be 0. Values range from 1 to 9
	u8 is_swizzled : 1;
	// NPOT = Non-Power-Of-Two;
	// This flags tells gu_set_texture() to round the size up to the next power of two (otherwise, it would be rounded down to the previous power of two by the GE when encoding the command).
	// This pretends that the texture is bigger than it actually is, meaning that there is a region of the texture that will be invalid to access.
	// So we rely on the UVs to prevent accesses to the invalid region.
	// In practice this is useful when binding framebuffer memory as an input texture.
	u8 is_non_power_of_two_on_purpose : 1;
} Texture;

size_t texture_get_level_size_in_bytes(const Texture* m, u32 level) {
	return (m->stride_px >> level) * (m->size_px[1] >> level) * gu_psm_get_bytes_per_pixel(m->psm);
}

void* texture_get_data_for_level(const Texture* m, u32 level) {
	u8* cursor = m->data;
	for (size_t i = 0; i < level; ++i)
		cursor = ptr_align(cursor + texture_get_level_size_in_bytes(m, i), 16);
	
	return cursor;
}

size_t texture_get_alloc_size(const Texture* m, u32 nb_levels) {
	return (u8*) texture_get_data_for_level(m, nb_levels) - (u8*) m->data;
}

void texture_check_common(const Texture* m) {
	app_assert(m->nb_mipmap_levels >= 1);
	// Question: is the stride allowed to be zero?
	// app_assert((m->stride_px * gu_psm_get_bits_per_pixel(m->psm)) % (16 * 8) == 0); // According to the comment on sceGuTexImage, stride must be "block-aligned"; I guess this refers to the { 16-byte wide, 8 px tall } blocks fetched by the GE.
	app_assert(m->stride_px <= UINT16_MAX); // Stride (buffer width) is sent directly as 16-bit to the GE
	if (m->size_px[0] && m->size_px[1])
		app_assert(m->data);
}

void texture_check_as_input(const Texture* m) {
	texture_check_common(m);
	for (size_t i = 0; i < 2; ++i) {
		app_assert(m->is_non_power_of_two_on_purpose || size_is_power_of_two_or_zero(m->size_px[i]));
		app_assert(m->size_px[i] <= 512); // Can't be represented in commands otherwise; size is sent as 8-bit exponent
	}
}

void texture_check_as_rendertarget(const Texture* m) {
	texture_check_common(m);
}

void texture_allocate_buffers(Texture* m) {
	const size_t size = texture_get_alloc_size(m, m->nb_mipmap_levels);
	m->data = malloc(size);
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
//
// NOTE: Stolen from samples/gu/blit/blit.c
void swizzle_fast(void* out, const void* in, uint32_t width_in_bytes, uint32_t height) {
	const u32 width_blocks = (width_in_bytes / 16);
	const u32 height_blocks = (height / 8);
	const u32 src_pitch = (width_in_bytes - 16) / 4;
	const u32 src_row = width_in_bytes * 8;

	app_assert(ptr_is_aligned(in, 4));
	app_assert(ptr_is_aligned(out, 4));
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

typedef enum {
	SKYBOX_FACE__NX = 0,
	SKYBOX_FACE__PX,
	SKYBOX_FACE__NY,
	SKYBOX_FACE__PY,
	SKYBOX_FACE__NZ,
	SKYBOX_FACE__PZ,
	SKYBOX_FACE__COUNT // Keep last, will always equal 6
} SkyboxFaceID;

typedef struct {
	Texture texture;
	u32 face_size_px[2];
} SkyboxTexture;

u32 skybox_face_id_get_color(SkyboxFaceID face_id) {
	switch (face_id) {
	case SKYBOX_FACE__NX: return GU_ABGR(0xff, 0xff, 0xff, 0x00);
	case SKYBOX_FACE__PX: return GU_ABGR(0xff, 0x00, 0x00, 0xff);
	case SKYBOX_FACE__NY: return GU_ABGR(0xff, 0xff, 0x00, 0xff);
	case SKYBOX_FACE__PY: return GU_ABGR(0xff, 0x00, 0xff, 0x00);
	case SKYBOX_FACE__NZ: return GU_ABGR(0xff, 0x00, 0xff, 0xff);
	case SKYBOX_FACE__PZ: return GU_ABGR(0xff, 0xff, 0x00, 0x00);
	default: app_assert(0 && "Unknown face ID"); break;
	}
	return 0;
}

ScePspIVector2 skybox_face_get_2d_index(SkyboxFaceID face_id) {
	// Face order is:
	// +X +Z -X
	// +Y -Y -Z
	u32 x = 0, y = 0;
	switch (face_id) {
	case SKYBOX_FACE__PX: x = 0; y = 0; break;
	case SKYBOX_FACE__PZ: x = 1; y = 0; break;
	case SKYBOX_FACE__NX: x = 2; y = 0; break;
	case SKYBOX_FACE__PY: x = 0; y = 1; break;
	case SKYBOX_FACE__NY: x = 1; y = 1; break;
	case SKYBOX_FACE__NZ: x = 2; y = 1; break;
	default: app_assert(0 && "Unknown face ID"); break;
	}
	return (ScePspIVector2) { x, y };
}

Texture skybox_texture_get_face_texture(const SkyboxTexture* m, SkyboxFaceID face_id) {
	const ScePspIVector2 i = skybox_face_get_2d_index(face_id);
	Texture out = m->texture;
	out.size_px[0] = m->face_size_px[0];
	out.size_px[1] = m->face_size_px[1];
	out.data = (u8*) out.data + (i.y * out.stride_px * out.size_px[1] + i.x * out.size_px[0]) * gu_psm_get_bytes_per_pixel(out.psm);
	return out;
}

// Face index is returned in the third component
ScePspFVector3 skybox_face_uvs_from_normal(ScePspFVector3 v) {
	ScePspFVector3 vabs = v;
	vabs.x = fabsf(v.x);
	vabs.y = fabsf(v.y);
	vabs.z = fabsf(v.z);

	ScePspFVector3 n;
	n.x = fminf(fmaxf(v.x * 0.5f + 0.5f, 0.f), 1.f);
	n.y = fminf(fmaxf(v.y * 0.5f + 0.5f, 0.f), 1.f);
	n.z = fminf(fmaxf(v.z * 0.5f + 0.5f, 0.f), 1.f);

	if (vabs.z >= vabs.x && vabs.z >= vabs.y) {
		return (ScePspFVector3) { v.z < 0 ? n.x : 1.f - n.x, 1.f - n.y, v.z < 0 ? SKYBOX_FACE__NZ : SKYBOX_FACE__PZ };
	} else if (vabs.y >= vabs.x) {
		return (ScePspFVector3) { n.x, v.y < 0 ? n.z : 1.f - n.z, v.y < 0 ? SKYBOX_FACE__NY : SKYBOX_FACE__PY };
	} else {
		return (ScePspFVector3) { v.x < 0 ? 1.f - n.z : n.z, 1.f - n.y, v.x < 0 ? SKYBOX_FACE__NX : SKYBOX_FACE__PX };
	}
}
// Face index is returned in the third component
ScePspFVector3 skybox_uvs_from_normal(ScePspFVector3 v) {
	ScePspFVector3 u = skybox_face_uvs_from_normal(v);
	const ScePspIVector2 i = skybox_face_get_2d_index(u.z);
	u.x = (u.x + i.x) / 3.f;
	u.y = (u.y + i.y) / 2.f;
	return u;
}

//
//
//
//

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
	sceGuDrawBufferList(m->psm, psp_ptr_to_vram(m->data), m->stride_px);
	gu_set_offset_and_viewport_and_scissor(m->size_px[0], m->size_px[1]);
}

void gu_set_texture_ex(const Texture* m, u32 first_level, u32 nb_levels_to_use) {
	texture_check_as_input(m);

	// u32_next_power_of_two() is important when binding framebuffers as input
	const u32 w = u32_next_power_of_two(m->size_px[0]);
	const u32 h = u32_next_power_of_two(m->size_px[1]);
	const u32 nb_mipmap_levels = u32_max(first_level + 1, u32_min(m->nb_mipmap_levels, first_level + nb_levels_to_use));

	sceGuTexMode(m->psm, (nb_mipmap_levels - first_level) - 1, 0, m->is_swizzled);

	u8* cursor = texture_get_data_for_level(m, first_level);
	for (size_t level = first_level; level < nb_mipmap_levels; ++level) {
		sceGuTexImage(level - first_level, w >> level, h >> level, m->stride_px >> level, cursor);
		cursor = ptr_align(cursor + texture_get_level_size_in_bytes(m, level), 16);
	}
}

void gu_set_texture(const Texture* m) {
	gu_set_texture_ex(m, 0, 9);
}

typedef enum {
	MESH_PATCH_MODE__NONE = 0,
	MESH_PATCH_MODE__BEZIER,
	MESH_PATCH_MODE__SPLINE,
	MESH_PATCH_MODE__COUNT // Keep last
} MeshPatchMode;

typedef struct {
	u8 count[2]; // Number of elements (vertices or indices) along the U and V direction
	u8 divide[2];
	u8 edge_mode[2]; // e.g GU_FILL_FILL
} MeshPatch;

typedef struct {
	size_t sizeof_vertex;
	void* vertices;
	size_t nb_vertices;
	u16* indices;
	size_t nb_indices;
	u32 gu_vertex_format : 24;
	u32 gu_topology : 3; // For patches, only GU_POINTS, GU_LINE_STRIP, GU_TRIANGLE_STRIP are valid
	u32 patch_mode : 2; // MeshPatchMode
	u32 draw_debug : 1;
	u32 reserved : 2;
	MeshPatch patch;
} Mesh;

typedef struct {
	Mesh mesh;
	size_t nb_rows;
	size_t nb_columns;	
} GridMesh;

void mesh_check_pointers(const Mesh* m) {
	app_assert(m->nb_vertices * m->sizeof_vertex == 0 || m->vertices);
	app_assert(m->nb_indices == 0 || m->indices);
	//app_assert(ptr_is_aligned(m->vertices, 16));
	//app_assert(ptr_is_aligned(m->indices, 16));
}

void mesh_allocate_buffers(Mesh* m) {
	m->vertices = malloc(m->nb_vertices * m->sizeof_vertex);
	m->indices = malloc(m->nb_indices * sizeof m->indices[0]);
	mesh_check_pointers(m);
}

void mesh_allocate_buffers_in_current_display_list(Mesh* m) {
	m->vertices = sceGuGetMemory(m->nb_vertices * m->sizeof_vertex);
	m->indices = sceGuGetMemory(m->nb_indices * sizeof m->indices[0]);
	mesh_check_pointers(m);
}

void mesh_destroy(Mesh* m) {
	free(m->vertices);
	free(m->indices);
	*m = (Mesh) {0};
}

u32 gu_topology_to_patch_topology(u32 x) {
	switch (x) {
	case GU_POINTS:
		return GU_POINTS;
	case GU_LINES:
	case GU_LINE_STRIP:
		return GU_LINE_STRIP;
	case GU_TRIANGLES:
	case GU_TRIANGLE_STRIP:
	case GU_TRIANGLE_FAN:
		return GU_TRIANGLE_STRIP;
	case GU_SPRITES:
		app_assert(0 && "Sprites are not supported for patches");
		break;
	default:
		app_assert(0 && "Unknown topology");
		break;
	}
	return 0;
}

void mesh_draw_impl(const Mesh* m, bool b2d) {
	u32 count = m->nb_vertices;
	u32 vtype = m->gu_vertex_format | (b2d ? GU_TRANSFORM_2D : GU_TRANSFORM_3D);

	if (m->nb_indices) {
		vtype |= GU_INDEX_16BIT;
		count = m->nb_indices;
	}

	app_assert(count <= UINT16_MAX); // The GE keeps only the lowest 16 bits. Consider splitting the draw call (or we could do it ourselves here?).

	if (m->patch_mode == MESH_PATCH_MODE__NONE) {
		sceGuDrawArray(m->gu_topology, vtype, count, m->indices, m->vertices);

		// Stats
		g_frame_stats.meshes.nb_elements += count;
		g_frame_stats.meshes.nb_vertices += m->nb_vertices;
		switch (m->gu_topology) {
		case GU_TRIANGLES:
			g_frame_stats.meshes.nb_faces += count / 3;
			break;
		case GU_SPRITES:
			g_frame_stats.meshes.nb_faces += count / 2;
			break;
		case GU_POINTS:
		case GU_LINES:
		case GU_LINE_STRIP:
			break;
		case GU_TRIANGLE_STRIP:
		case GU_TRIANGLE_FAN:
		default:
			app_assert(0 && "Calculating face number from this topology is not implemented yet");
			break;
		}
	} else {
		sceGuPatchDivide(m->patch.divide[0], m->patch.divide[1]);
		sceGuPatchPrim(gu_topology_to_patch_topology(m->gu_topology));

		sceGuDisable(GU_CULL_FACE); // TODO: Need to figure out why it's necessary (what mesh layout is expected for this to work?)

		if (m->patch_mode == MESH_PATCH_MODE__BEZIER)
			sceGuDrawBezier(vtype, m->patch.count[0], m->patch.count[1], m->indices, m->vertices);
		else if (m->patch_mode == MESH_PATCH_MODE__SPLINE)
			sceGuDrawSpline(vtype, m->patch.count[0], m->patch.count[1], m->patch.edge_mode[0], m->patch.edge_mode[1], m->indices, m->vertices);

		if (m->draw_debug)
			sceGuDrawArray(GU_POINTS, vtype, m->patch.count[0] * (size_t) m->patch.count[1], m->indices, m->vertices);

		sceGuEnable(GU_CULL_FACE); // TODO: see the disable above
	}
}

void mesh_draw_3d(const Mesh* m) {
	mesh_draw_impl(m, false);
}

void mesh_draw_2d(const Mesh* m) {
	mesh_draw_impl(m, true);
}

// Adapted from shadowprojection sample
void mesh_generate_grid(Mesh* m, size_t rows, size_t columns) {
	app_assert(rows >= 2);
	app_assert(columns >= 2);
	app_assert(rows * columns - 1 <= UINT16_MAX); // For indices

	const f32 columns_minus_one_inv = 1.f / (columns - 1.f);
	const f32 rows_minus_one_inv = 1.f / (rows - 1.f);

	m->gu_topology = GU_TRIANGLES;
	m->gu_vertex_format = Vertex_Ni8_Pi16_FORMAT;
	Vertex_Ni8_Pi16* vertices = m->vertices;
	m->sizeof_vertex = sizeof vertices[0];

	m->nb_vertices = rows * columns;
	if (vertices) {
		for (size_t y = 0; y < rows; ++y) {
			for (size_t x = 0; x < columns; ++x) {
				vertices[y * columns + x] = (Vertex_Ni8_Pi16) {
					.normal = { 0, INT8_MAX, 0 },
					.position = {
						(x * columns_minus_one_inv * 2.f - 1.f) * INT16_MAX,
						0,
						(y * rows_minus_one_inv * 2.f - 1.f) * INT16_MAX,
					},
				};
			}
		}
		sceKernelDcacheWritebackRange(vertices, m->nb_vertices * sizeof vertices[0]);
	}

	m->nb_indices = (rows - 1) * (columns - 1) * 6;
	if (m->indices) {
		u16* curr = m->indices;
		for (size_t y = 0; y < rows - 1; ++y) {
			for (size_t x = 0; x < columns - 1; ++x) {
				const u16 tl = (y + 0) * columns + (x + 0);
				const u16 tr = (y + 0) * columns + (x + 1);
				const u16 bl = (y + 1) * columns + (x + 0);
				const u16 br = (y + 1) * columns + (x + 1);

				*curr++ = tl;
				*curr++ = tr;
				*curr++ = br;

				*curr++ = br;
				*curr++ = bl;
				*curr++ = tl;
			}
		}
		sceKernelDcacheWritebackRange(m->indices, m->nb_indices * sizeof m->indices[0]);
	}
}

void mesh_generate_grid_bezier(Mesh* m, size_t rows, size_t columns) {
	app_assert((rows - 1) % 3 == 0);
	app_assert((columns - 1) % 3 == 0);
	m->indices = NULL;
	mesh_generate_grid(m, rows, columns);
	m->nb_indices = 0;
	m->gu_topology = GU_TRIANGLE_STRIP;
	m->patch_mode = MESH_PATCH_MODE__BEZIER;
	m->patch = (MeshPatch) {
		.count = { rows, columns },
		.divide = { 4, 4 },
		.edge_mode = { GU_FILL_FILL, GU_FILL_FILL },
	};
}

void gridmesh_generate(GridMesh* m) {
	mesh_generate_grid(&m->mesh, m->nb_rows, m->nb_columns);
}

void gridmesh_generate_bezier(GridMesh* m) {
	mesh_generate_grid_bezier(&m->mesh, m->nb_rows, m->nb_columns);
}

// Stolen from shadowprojection sample
void mesh_generate_torus(Mesh* m, size_t slices, size_t rows, f32 radius, f32 thickness) {
	// We're going to fit positions in a normalized integer format
	app_assert(radius + thickness <= 1.f);

	const f32 slices_inv = 1.f / slices;
	const f32 rows_inv = 1.f / rows;

	m->gu_topology = GU_TRIANGLES;
	m->gu_vertex_format = Vertex_Ni8_Pi16_FORMAT;
	Vertex_Ni8_Pi16* vertices = m->vertices;
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

// My notes from researching the optimal tile size by measuring (the timing is for some scene + 3 fullscreen quad draw calls):
// The time it takes to render the scene alone (without fullscreen quads) was 5.160 ms, so you can subtract that and divide by 3 to get the time for a single fullscreen quad.
//
// These are the fastest configurations I've found:
//
// Time (ms) ; Tile size X ; Tile size Y
// 8.625     ; 32          ; 272
// 8.860     ; 96          ; 272
// 8.888     ; 24          ; 272
// 8.888     ; 112         ; 272
// 8.890     ; 16          ; 272
// 8.930     ; 64          ; 272
//
// Testing with Y = 136 gives almost the same results as with Y = 272, but with ever so slightly less performance, so it's not worth it.
// Perf degrades noticeably as X moves away from the noted values, especially past the hundred.
// Perf also degrades as Y moves away from 136 and 272.
//
// I have no explanation for this. Perhaps it's because it makes better use of the 8K texture cache... Or is it, really? There is no pixel reuse in my use case.
// And why wouldn't the GE figure out the optimal "fetch" pattern when you send a single fullscreen-sized quad (especially since it's a sprite)?
// That looks like a design issue to me, and licensees were probably given recommendations accordingly.
// At least, if breaking up a large quad into smaller pieces improves perf from 27 ms to 8.6 ms for the same result, then there's no reason the engine shouldn't be able to figure that out and do the equivalent. But that's probably just the way it is for old hardware.
//
// I note that there's a similar idea with GU_FAST_CLEAR_BIT and that apparently drawing multiple vertical quads is the fast way to draw to the entire screen.
// For instance see the IsReallyAClear() function in PPSSPP (https://github.com/hrydgard/ppsspp/blob/17d807197d2da9e41dd6523bcbe94a92bbedb019/GPU/Common/SoftwareTransformCommon.cpp#L92)
//
// Also it doesn't make a difference at all whether the mesh is indexed or not; but I do find the indexed version more elegant, and it's good practice anyway (for vertex cache reuse in modern GPUs)

// These constants are mostly to make it easier to search for code that assumes these values; they don't have to be used in all cases.
// TODO: Is it faster yet when scaled proportionnaly to the input texture's size?
#define FULLSCREENQUAD_BEST_TILE_SIZE_X 32
#define FULLSCREENQUAD_BEST_TILE_SIZE_Y PSP_SCREEN_HEIGHT

void mesh_generate_fullscreen_quad_i16(Mesh* m, i16 screen_width, i16 screen_height, i16 uv0, i16 uv1, i16 tile_size_x_px, i16 tile_size_y_px) {
	if (tile_size_x_px <= 0)
		tile_size_x_px = FULLSCREENQUAD_BEST_TILE_SIZE_X;

	if (tile_size_y_px <= 0)
		tile_size_y_px = FULLSCREENQUAD_BEST_TILE_SIZE_Y;

	const u32 nb_tiles_y = (screen_height + tile_size_y_px - 1) / tile_size_y_px;
	const u32 nb_tiles_x = (screen_width + tile_size_x_px - 1) / tile_size_x_px;

	m->gu_topology = GU_SPRITES;
	m->gu_vertex_format = Vertex_Ti16_Pi16_FORMAT;
	Vertex_Ti16_Pi16* vertices = m->vertices;
	m->sizeof_vertex = sizeof vertices[0];

	m->nb_vertices = (nb_tiles_y + 1) * (nb_tiles_x + 1);
	if (vertices) {
		const f32 tile_size_x_percentage = tile_size_x_px / (f32) screen_width;
		const f32 tile_size_y_percentage = tile_size_y_px / (f32) screen_height;

		Vertex_Ti16_Pi16* current_vertex = vertices;
		for (size_t y = 0; y <= nb_tiles_y; ++y) {
			// For yt and xt, we min() with 1 to guarantee that we never sample outside of the given UV range.
			// This may be important for preventing invalid memory accesses; for instance we may use a framebuffer texture, pretending its width is larger than what it really is, in order to satisfy the power-of-two constraint.
			const f32 yt = fminf(1, y * tile_size_y_percentage);
			for (size_t x = 0; x <= nb_tiles_x; ++x) {
				const f32 xt = fminf(1, x * tile_size_x_percentage);

				// I use floorf() in order not to assume the default rounding mode
				*current_vertex++ = (Vertex_Ti16_Pi16) {
					.uv = { floorf(xt * uv0), floorf(yt * uv1) },
					.position = { floorf(xt * screen_width), floorf(yt * screen_height), 0 },
				};
			}
		}
		sceKernelDcacheWritebackRange(vertices, m->nb_vertices * sizeof vertices[0]);
	}

	m->nb_indices = nb_tiles_y * nb_tiles_x * 2;
	if (m->indices) {
		u16* current_index = m->indices;
		for (size_t y = 0; y < nb_tiles_y; ++y) {
			for (size_t x = 0; x < nb_tiles_x; ++x) {
				*current_index++ = (y + 0) * (nb_tiles_x + 1) + x + 0;
				*current_index++ = (y + 1) * (nb_tiles_x + 1) + x + 1;
			}
		}
		sceKernelDcacheWritebackRange(m->indices, m->nb_indices * sizeof m->indices[0]);
	}
}

void gu_draw_fullscreen_textured_quad_i16(i16 screen_width, i16 screen_height, i16 uv0, i16 uv1, i16 tile_size_x_px, i16 tile_size_y_px) {
	Mesh mesh = {0};
	for (size_t i = 0; i < 2; ++i) {
		mesh_generate_fullscreen_quad_i16(&mesh, screen_width, screen_height, uv0, uv1, tile_size_x_px, tile_size_y_px);
		if (i == 0)
			mesh_allocate_buffers_in_current_display_list(&mesh);
	}
	mesh_draw_2d(&mesh);
}

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

	app_assert(pathbuf[0] != '\0');
	printf("Assets path: `%s`\n", pathbuf);

	strcat(pathbuf, "/..");
	printf("Setting current directory: `%s`\n", pathbuf);

	// ----- Set res path as current dir
	const int chdir_status = sceIoChdir(pathbuf);
	app_assert(chdir_status >= 0);
}

typedef SceUID Fd;

bool fd_is_valid(Fd fd) {
	return fd >= 0;
}

Fd fd_open_readonly(const char* path, bool should_assert) {
	const Fd fd = sceIoOpen(path, PSP_O_RDONLY, 0777);
	if (!fd_is_valid(fd)) {
		fprintf(stderr, "Could not open `%s`\n", path);
		app_assert(!should_assert && "Failed to open file");
	}
	return fd;
}

void fd_close(Fd fd) {
	app_assert(fd_is_valid(fd));
	sceIoClose(fd);
}

ssize_t fd_read(Fd fd, void* data, ssize_t size) {
	app_assert(fd_is_valid(fd));
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
        app_assert(0 && "TGA Format isn't TGA32; make sure you saved it with an alpha channel"); // Format doesn't match
		return out;
    }

	if (hdr->id_length) {
		u8 image_id[255];
		nread = fd_read(fd, image_id, hdr->id_length);
		if (nread != hdr->id_length) {
			app_assert(0); // Invalid image ID
			return out;
		}
	}

	const ssize_t data_size = w * h * (hdr->bits_per_pixel / 8);
	out.pixel_data = malloc(data_size);

	nread = fd_read(fd, out.pixel_data, data_size);

	if (nread != data_size) {
		app_assert(0); // Not enough data for pixels?
		free(out.pixel_data);
		return out;
	}

	const bool is_origin_in_upper_left = !!(hdr->image_descriptor & (1 << 5));
	app_assert(is_origin_in_upper_left);

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
	u8 max_mipmap_levels;
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

	const u32 bytes_per_pixel = gu_psm_get_bytes_per_pixel(out.psm);

	// GE reads blocks of width and height { 16-bytes, 8 pixels }, so we cannot meaningfully go below that.
	const u32 nb_blocks_w = (out.size_px[0] * bytes_per_pixel) / 16;
	const u32 nb_blocks_h = out.size_px[1] / 8;
	const bool can_swizzle = nb_blocks_w && nb_blocks_h;

	const bool should_swizzle = p->should_swizzle && can_swizzle;

	if (p->max_mipmap_levels) {
		if (should_swizzle) {
			out.nb_mipmap_levels = u32_log2_plus_1(u32_min(nb_blocks_w, nb_blocks_h));
		} else {
			out.nb_mipmap_levels = u32_log2_plus_1(u32_min(out.size_px[0], out.size_px[1]));
		}
		out.nb_mipmap_levels = u32_min(out.nb_mipmap_levels, p->max_mipmap_levels);
	}

	texture_allocate_buffers(&out);

	void* data = texture_get_data_for_level(&out, 0);
	const size_t size = texture_get_level_size_in_bytes(&out, 0);

	if (should_swizzle) {
		swizzle_fast(data, tga.pixel_data, out.size_px[0] * bytes_per_pixel, out.size_px[1]);
		out.is_swizzled = true;
	} else {
		memcpy(data, tga.pixel_data, size);
	}

	sceKernelDcacheWritebackRange(data, size);

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
	LUT_R,
	LUT_G,
	LUT_B,
	LUT_COUNT // Keep last
} LUT;

typedef enum LUTMode {
	LUT_MODE_INVALID = 0,
	LUT_MODE_1_TO_1, // dst_color[channel] = func(src_color[channel])
	LUT_MODE_3_TO_3, // dst_color += func(src_color[channel])
	LUT_MODE_COUNT // Keep last
} LUTMode;

typedef struct {
	u32 psm;
	void* clut_per_channel[4];
} ColorLutsMemory;

static const char* lut_get_name(LUT lut) {
	switch (lut) {
	case LUT_IDENTITY: return "Identity";
	case LUT_INVERT: return "Invert";
	case LUT_SRGB_TO_LINEAR: return "sRGB to linear";
	case LUT_LINEAR_TO_SRGB: return "Linear to sRGB";
	case LUT_SEPIA: return "Sepia";
	case LUT_R: return "R";
	case LUT_G: return "G";
	case LUT_B: return "B";
	default: return "???";
	}
}

LUTMode lut_get_mode(LUT lut) {
	switch (lut) {
	case LUT_IDENTITY: return LUT_MODE_1_TO_1;
	case LUT_INVERT: return LUT_MODE_1_TO_1;
	case LUT_SRGB_TO_LINEAR: return LUT_MODE_1_TO_1;
	case LUT_LINEAR_TO_SRGB: return LUT_MODE_1_TO_1;
	case LUT_SEPIA: return LUT_MODE_3_TO_3;
	case LUT_R: return LUT_MODE_1_TO_1;
	case LUT_G: return LUT_MODE_1_TO_1;
	case LUT_B: return LUT_MODE_1_TO_1;
	default: return LUT_MODE_1_TO_1;
	}
}

static u32 safe_round_to_u32(f32 f, u32 max) {
	return roundf(fminf(fmaxf(f, 0), 1) * max);
}

// Color conversion functions.
// The 8888_to_* functions were stolen from here:
// https://github.com/pspdev/pspsdk/blob/d019dbc7ecc198102229d0cdfe02976b6bef4e4d/src/debug/scr_printf.c#L42
//
// For the 8888_from_* functions, I looked at how the GE converts from N bits to 8 bits.
// The function seems to be: min(255, (x * 256) / (max_val)).
// When x == max_val, this may evaluate to 256, hence the min(255, ...).
//
// Earlier I tested this by rendering a full-screen gradient texture, taking a screenshot via PSPLINK, and inspecting the color values in GIMP.
// PSPLINK seems to do a left-shift, but it doesn't appear to be how the GE operates.
//
// From my experiment (clear the screen then use the color test)
// "good" 8-bit values obtained from 4-bits: (guessing the calculation: output = 4bitvalue * 255 / 15, OR, output = 4bitvalue * 256 / 15)
// 255
// 238
// 221
// 204
// 187
// 170
// 153
// 136
// 119
// 102
// 85
// 68
// 51
// 34
// 17
// 0
//
// "good" 8-bit values obtained from 5-bits: (guessing the calculation: output = 5bitvalue * 256 / 31)
// 255 ?? (should be 256 according to the calculation)
// 247
// 239
// 231
// 222
// 214
// 206
// 198
// 189
// 181
// 173
// 165
// 156
// 148
// 140
// 132
// 123
// 115
// 107
// 99
// 90
// 82
// 74
// 66
// 57
// 49
// 41
// 33
// 24
// 16
// 8
// 0

u32 cvt_5bit_to_8bit(u32 x) {
	x = (x * 256u) / 31u;
	return x <= 255 ? x : 255;
}

u32 cvt_6bit_to_8bit(u32 x) {
	x = (x * 256u) / 63u;
	return x <= 255 ? x : 255;
}

u16 gu_color_8888_to_5650(u32 c) {
	const u32 b = (c >> 19) & 0x1F;
	const u32 g = (c >> 10) & 0x3F;
	const u32 r = (c >> 3) & 0x1F;
	return r | (g << 5) | (b << 11);
}

u32 gu_color_8888_from_5650(u16 c) {
	u32 b = (c >> 11) & 0x1F;
	u32 g = (c >> 5) & 0x3F;
	u32 r = c & 0x1F;
	b = cvt_5bit_to_8bit(b);
	g = cvt_6bit_to_8bit(g);
	r = cvt_5bit_to_8bit(r);
	return GU_ABGR(0, b, g, r);
}

u16 gu_color_8888_to_5551(u32 c) {
	const u32 a = (c >> 24) ? 0x8000 : 0;
	const u32 b = (c >> 19) & 0x1F;
	const u32 g = (c >> 11) & 0x1F;
	const u32 r = (c >> 3) & 0x1F;
	return a | r | (g << 5) | (b << 10);
}

u32 gu_color_8888_from_5551(u16 c) {
	u32 a = (c >> 15) & 0x1; 
	u32 b = (c >> 10) & 0x1F;
	u32 g = (c >> 5) & 0x1F;
	u32 r = c & 0x1F;
	a *= 0xff;
	b = cvt_5bit_to_8bit(b);
	g = cvt_5bit_to_8bit(g);
	r = cvt_5bit_to_8bit(r);
	return GU_ABGR(a, b, g, r);
}

u16 gu_color_8888_to_4444(u32 c) {
	const u32 a = (c >> 28) & 0xF; 
	const u32 b = (c >> 20) & 0xF;
	const u32 g = (c >> 12) & 0xF;
	const u32 r = (c >> 4) & 0xF;
	return (a << 12) | r | (g << 4) | (b << 8);
}

u32 gu_color_8888_from_4444(u16 c) {
	u32 a = (c >> 12) & 0xF; 
	u32 b = (c >> 8) & 0xF;
	u32 g = (c >> 4) & 0xF;
	u32 r = c & 0xF;
	a = (a * 255u) / 15u;
	b = (b * 255u) / 15u;
	g = (g * 255u) / 15u;
	r = (r * 255u) / 15u;
	return GU_ABGR(a, b, g, r);
}

u32 gu_color_8888_to_psm(u32 c, u32 psm) {
	switch (psm) {
	case GU_PSM_5650: return gu_color_8888_to_5650(c);
	case GU_PSM_5551: return gu_color_8888_to_5551(c);
	case GU_PSM_4444: return gu_color_8888_to_4444(c);
	case GU_PSM_8888: return c;
	default: app_assert(0); break;
	}
	return 0;
}

u32 gu_color_8888_from_psm(u32 c, u32 psm) {
	switch (psm) {
	case GU_PSM_5650: return gu_color_8888_from_5650(c);
	case GU_PSM_5551: return gu_color_8888_from_5551(c);
	case GU_PSM_4444: return gu_color_8888_from_4444(c);
	case GU_PSM_8888: return c;
	default: app_assert(0); break;
	}
	return 0;
}

u32 gu_color_8888_reduce_to_psm_quality(u32 c, u32 psm) {
	return gu_color_8888_from_psm(gu_color_8888_to_psm(c, psm), psm);
}

// Returns size stored
size_t gu_color_store_from_rgbaf(void* out, u32 psm, const f32* rgbaf) {
	const u32 rgba8888 = GU_ABGR(
		safe_round_to_u32(rgbaf[3], 0xff),
		safe_round_to_u32(rgbaf[2], 0xff),
		safe_round_to_u32(rgbaf[1], 0xff),
		safe_round_to_u32(rgbaf[0], 0xff)
	);
		
	switch (psm) {
	case GU_PSM_5650: *(u16*) out = gu_color_8888_to_5650(rgba8888); return 2;
	case GU_PSM_5551: *(u16*) out = gu_color_8888_to_5551(rgba8888); return 2;
	case GU_PSM_4444: *(u16*) out = gu_color_8888_to_4444(rgba8888); return 2;
	case GU_PSM_8888: *(u32*) out = rgba8888; return 4;
	default: app_assert(0); break;
	}
	return 0;
}

typedef void (*RgbafFromScalarFn)(f32*, f32);

void rgbaf_identity(f32* rgbaf, f32 x) { rgbaf[0] = rgbaf[1] = rgbaf[2] = x; }
void rgbaf_invert(f32* rgbaf, f32 x) { rgbaf[0] = rgbaf[1] = rgbaf[2] = 1.f - x; }
void rgbaf_srgb_to_linear(f32* rgbaf, f32 x) { rgbaf[0] = rgbaf[1] = rgbaf[2] = powf(x, 2.2f); }
void rgbaf_linear_to_srgb(f32* rgbaf, f32 x) { rgbaf[0] = rgbaf[1] = rgbaf[2] = powf(x, 1.f / 2.2f); }
void rgbaf_r(f32* rgbaf, f32 x) { rgbaf[0] = x; }
void rgbaf_g(f32* rgbaf, f32 x) { rgbaf[1] = x; }
void rgbaf_b(f32* rgbaf, f32 x) { rgbaf[2] = x; }
void rgbaf_0(f32* rgbaf, f32 x) { }

// outputRed   = (inputRed * .393) + (inputGreen * .769) + (inputBlue * .189)
// outputGreen = (inputRed * .349) + (inputGreen * .686) + (inputBlue * .168)
// outputBlue  = (inputRed * .272) + (inputGreen * .534) + (inputBlue * .131)
void sepia_func_r(f32* rgbaf, f32 x) { rgbaf[0] = x * .393f; rgbaf[1] = x * .349f; rgbaf[2] = x * .272f; }
void sepia_func_g(f32* rgbaf, f32 x) { rgbaf[0] = x * .769f; rgbaf[1] = x * .686f; rgbaf[2] = x * .534f; }
void sepia_func_b(f32* rgbaf, f32 x) { rgbaf[0] = x * .189f; rgbaf[1] = x * .168f; rgbaf[2] = x * .131f; }

const RgbafFromScalarFn sepia_funcs[] = { sepia_func_r, sepia_func_g, sepia_func_b };

void lut_fill_helper_3_funcs(ColorLutsMemory* m, const PsmInfo* psm_info, const RgbafFromScalarFn* f) {
	for (size_t channel = 0; channel < 3; ++channel) {
		const u32 channel_max = (1u << psm_info->channels[channel].nb_bits) - 1u;
		u8* clut_ptr = m->clut_per_channel[channel];
		for (u32 i = 0; i <= channel_max; ++i) {
			f32 rgbaf[4] = { 0, 0, 0, 1 };
			f[channel](rgbaf, i / (f32) channel_max);

			gu_color_store_from_rgbaf(clut_ptr, psm_info->psm, rgbaf);
			clut_ptr += psm_info->nb_bits / 8;
		}

		sceKernelDcacheWritebackRange(m->clut_per_channel[channel], clut_ptr - (u8*) m->clut_per_channel[channel]);
	}
}

void lut_fill_helper_1_func(ColorLutsMemory* m, const PsmInfo* psm_info, RgbafFromScalarFn f) {
	const RgbafFromScalarFn fa[] = { f, f, f };
	lut_fill_helper_3_funcs(m, psm_info, fa);
}

void lut_fill(ColorLutsMemory* m, LUT lut, u32 psm) {
	m->psm = psm;
	const PsmInfo psm_info = gu_psm_get_info(psm);
	switch (lut) {
	case LUT_IDENTITY: lut_fill_helper_1_func(m, &psm_info, rgbaf_identity); break;
	case LUT_INVERT: lut_fill_helper_1_func(m, &psm_info, rgbaf_invert); break;
	case LUT_SRGB_TO_LINEAR: lut_fill_helper_1_func(m, &psm_info, rgbaf_srgb_to_linear); break;
	case LUT_LINEAR_TO_SRGB: lut_fill_helper_1_func(m, &psm_info, rgbaf_linear_to_srgb); break;
	case LUT_SEPIA: lut_fill_helper_3_funcs(m, &psm_info, sepia_funcs); break;
	case LUT_R: lut_fill_helper_3_funcs(m, &psm_info, (const RgbafFromScalarFn[]) { rgbaf_r, rgbaf_0, rgbaf_0 }); break;
	case LUT_G: lut_fill_helper_3_funcs(m, &psm_info, (const RgbafFromScalarFn[]) { rgbaf_0, rgbaf_g, rgbaf_0 }); break;
	case LUT_B: lut_fill_helper_3_funcs(m, &psm_info, (const RgbafFromScalarFn[]) { rgbaf_0, rgbaf_0, rgbaf_b }); break;
	default:
		break;
	}
}

//
//
// Main
//
//

PSP_MODULE_INFO("Experiment", 0, 1, 1);
PSP_MAIN_THREAD_ATTR(PSP_THREAD_ATTR_USER | PSP_THREAD_ATTR_VFPU);

u32 ALIGN16 g_gu_main_list[256 * 1024] = {0}; // Zeroing should not be necessary, but samples declare it as static, which zeroes it, so...

typedef struct {
	u8* vram_cursor;
	Texture framebuffers[2];
	Texture z_buffer;
	Texture pingpong0_fb;
} AppGfx;

typedef struct {
	bool enabled;
	LUT lut;
} PostProcessingParams;

typedef struct {
	ColorLutsMemory color_luts_mem;
	Texture uv_test_texture;
	Texture lava_texture;
	Texture horizon_gradient_texture;
	Texture mountain_bg_texture;
	Texture lava_skybox_texture;
	Texture normal_to_color_texture;
	Texture reflection_texture;
	Texture rtvb_xyz_texture; // rtvb = Render-To-Vertex-Buffer, might rename later
	SkyboxTexture skybox_test_texture;
	Mesh torus_mesh;
	GridMesh grid_mesh;
	GridMesh bezier_grid_mesh;
	Mesh fullscreen_quad_2d_mesh;
} AppAssets;

typedef struct {
	ScePspFMatrix4 view_matrix;
	ScePspFMatrix4 view_matrix_r; // Only rotation
	ScePspFMatrix4 proj_matrix;
	PostProcessingParams post_processing;
} Camera;

typedef struct {
	ScePspFMatrix4 model_matrix;
} Light;

typedef struct {
	const Mesh* mesh;
	ScePspFMatrix4 model_matrix;
	ScePspFMatrix4 model_matrix_tr; // Only translation and rotation, no scale
} MeshInstance;

typedef struct {
	Camera camera;
	Light lights[4];
	MeshInstance torus;
	MeshInstance grid;
	MeshInstance grid_bezier;
	f32 lava_flow[2];
} AppScene;

typedef struct {
	f32 last_frame_duration;
	f32 time_since_start;
} AppTimeline;

typedef struct {
	u64 nb_frames;
	AppTimeline game_time;
	AppTimeline real_time;
} MainLoop;

typedef struct {
	SceCtrlData previous;
	SceCtrlData current;
} AppInput;

typedef enum {
	VAR_ID__INVALID = 0,
	VAR_ID__HACK,
	VAR_ID__RTVB_SPACING,
	VAR_ID__RTVB_RT_CONFIG_ID,
	VAR_ID__RTVB_RT_COLOR_MODE,
	VAR_ID__TIME_DILATION,
	VAR_ID__MIP_LEVEL,
	VAR_ID__FB_PSM,
	VAR_ID__DITHER_GLOBAL,
	VAR_ID__LIGHT_MODE,
	VAR_ID__LAVA_ATTENUATION,
	VAR_ID__SPECULAR,
	VAR_ID__BLOOM_THRESHOLD,
	VAR_ID__BLOOM_OPACITY,
	VAR_ID__COUNT // Keep last
} AppVariableID;

#define VAR_FLAG_ROUND           (1 << 0)
#define VAR_FLAG_SMOOTH_EDIT     (1 << 1)
#define VAR_FLAG_STEP_PER_SECOND (1 << 2)

typedef struct {
	const char* name;
	f32 value;
	f32 min;
	f32 max;
	f32 step;
	u32 flags; // VAR_FLAG_*
} AppVariable;

typedef struct {
	MainLoop loop;
	AppGfx gfx;
	AppAssets assets;
	AppScene scene;
	AppInput input;
	AppVariable vars[VAR_ID__COUNT];
	MeManager me_manager;
	size_t selected_var_index;
	bool debug_ui_enabled;
} App;

// Returned pointer is relative to VRAM base
void* app_gfx_vram_linear_alloc(AppGfx* m, size_t size, size_t alignment) {
	m->vram_cursor = ptr_align(m->vram_cursor, alignment);
	void* out = m->vram_cursor;
	m->vram_cursor += size;
	app_assert((uintptr_t) m->vram_cursor <= sceGeEdramGetSize());
	return out;
}

void app_gfx_allocate_vram_resources(AppGfx* m, u32 framebuffer_psm) {
	for (size_t i = 0; i < 4; ++i) {
		Texture* it = NULL;
		u32 psm = framebuffer_psm;

		if (i < 2) {
			it = &m->framebuffers[i];
		} else if (i == 2) {
		 	it = &m->pingpong0_fb;
		} else if (i == 3) {
		 	it = &m->z_buffer;
			psm = GU_PSM_4444; // Doesn't really matter as long as we pick a 16-bit format for the size calculation
		}

		app_assert(it);

		*it = (Texture) {
			.nb_mipmap_levels = 1,
			.psm = psm,
			.stride_px = PSP_SCREEN_STRIDE,
			.size_px = { PSP_SCREEN_WIDTH, PSP_SCREEN_HEIGHT },
			.is_non_power_of_two_on_purpose = true,
		};
		it->data = psp_ptr_from_vram(app_gfx_vram_linear_alloc(m, texture_get_level_size_in_bytes(it, 0), 16));
	}
}

void app_gfx_use_vram_resources(AppGfx* m) {
	const Texture* fb0 = &m->framebuffers[0];
	const Texture* fb1 = &m->framebuffers[1];
	const Texture* zb = &m->z_buffer;
	sceGuDrawBuffer(fb0->psm, psp_ptr_to_vram(fb0->data), fb0->stride_px);
	sceGuDispBuffer(fb1->size_px[0], fb1->size_px[1], psp_ptr_to_vram(fb1->data), fb1->stride_px);
	sceGuDepthBuffer(psp_ptr_to_vram(zb->data), zb->stride_px);
	gu_set_offset_and_viewport_and_scissor(fb0->size_px[0], fb1->size_px[1]);
}

void gu_reset_state_to_app_defaults(const App* app) {
	sceGuSetAllStatus(0);

	sceGuDepthFunc(GU_GEQUAL);
	sceGuDepthMask(0);
	sceGuDepthOffset(0);
	sceGuDepthRange(0xffff, 0x0000);

	sceGuClearColor(0);
	sceGuClearDepth(0);
	sceGuClearStencil(0);
	sceGuPixelMask(0);

	sceGuAmbient(0);
	sceGuFog(0.f, 0.f, 0);

	sceGuColorMaterial(GU_AMBIENT | GU_DIFFUSE | GU_SPECULAR);
	sceGuModelColor(0, 0, 0, 0);
	sceGuAmbientColor(0); // Just to set the model's ambient alpha
	sceGuColor(0xffffffffu);
	sceGuSpecular(app->vars[VAR_ID__SPECULAR].value);
	sceGuShadeModel(GU_SMOOTH);

	sceGuTexFunc(GU_TFX_MODULATE, GU_TCC_RGBA);
	sceGuTexFilter(GU_LINEAR_MIPMAP_LINEAR, GU_LINEAR_MIPMAP_LINEAR);
	sceGuTexWrap(GU_CLAMP, GU_CLAMP);

	sceGuEnable(GU_SCISSOR_TEST);
	sceGuEnable(GU_DEPTH_TEST);
	sceGuEnable(GU_CULL_FACE);
	sceGuEnable(GU_PATCH_CULL_FACE);
	sceGuEnable(GU_PATCH_FACE);
	sceGuEnable(GU_CLIP_PLANES);
	// sceGuEnable(GU_TEXTURE_2D);

	sceGuFrontFace(GU_CW);

	sceGuLightMode(app->vars[VAR_ID__LIGHT_MODE].value);
}

void app_gfx_init(AppGfx* m) {
	sceGuInit();
	sceGuStart(GU_DIRECT, g_gu_main_list);

	sceGuSetCallback(GU_CALLBACK_SIGNAL, gu_on_signal);

	app_gfx_use_vram_resources(m);

	sceGuClear(GU_COLOR_BUFFER_BIT);
	sceGuFinish();
	sceGuSync(GU_SYNC_FINISH, GU_SYNC_WHAT_DONE);
}

void app_gfx_swap_buffers(AppGfx* m) {
	m->framebuffers[1].data = m->framebuffers[0].data;
	m->framebuffers[0].data = psp_ptr_from_vram(sceGuSwapBuffers());
}

void gu_texture_generate_mipmaps(const Texture* m, u32 src_level, u32 nb_levels, void* vram_ptr) {
	const u32 end_level = u32_min(m->nb_mipmap_levels, nb_levels);
	for (u32 dst_level = src_level + 1; dst_level < end_level; ++dst_level) {
		Texture rt = *m;
		rt.data = vram_ptr;
		rt.stride_px  >>= dst_level;
		rt.size_px[0] >>= dst_level;
		rt.size_px[1] >>= dst_level;
		if (rt.stride_px == 0 || rt.size_px[0] == 0 || rt.size_px[1] == 0)
			break;

		void* dst = texture_get_data_for_level(m, dst_level);

		sceGuStart(GU_DIRECT, g_gu_main_list);

		sceGuSetAllStatus(0);
		sceGuEnable(GU_TEXTURE_2D);
		sceGuDepthMask(1);

		sceGuTexFilter(GU_LINEAR, GU_LINEAR);
		sceGuTexWrap(GU_CLAMP, GU_CLAMP);
		sceGuTexFunc(GU_TFX_REPLACE, GU_TCC_RGBA);

		gu_set_rendertarget(&rt);
		gu_set_texture_ex(m, dst_level - 1, 1);
		gu_draw_fullscreen_textured_quad_i16(rt.size_px[0], rt.size_px[1], rt.size_px[0] << 1, rt.size_px[1] << 1, -1, -1);

		sceGuFinish();
		sceGuSync(GU_SYNC_FINISH, GU_SYNC_WHAT_DONE);

		if (m->is_swizzled) {
			swizzle_fast(dst, rt.data, rt.stride_px * gu_psm_get_bytes_per_pixel(rt.psm), rt.size_px[1]);
			sceKernelDcacheWritebackInvalidateAll(); // No other variant does the trick...
		} else {
			sceGuStart(GU_DIRECT, g_gu_main_list);

			sceGuCopyImage(m->psm, 0, 0, rt.size_px[0], rt.size_px[1], rt.stride_px, rt.data, 0, 0, rt.stride_px, dst);
			sceGuTexSync();

			sceGuFinish();
			sceGuSync(GU_SYNC_FINISH, GU_SYNC_WHAT_DONE);
		}
	}
}

void app_assets_init(AppAssets* m) {
	{
		ColorLutsMemory* c = &m->color_luts_mem;
		for (size_t i = 0; i < countof(c->clut_per_channel); ++i) {
			c->clut_per_channel[i] = malloc(256 * 4);
			app_assert(ptr_is_aligned(c->clut_per_channel[i], 16));
		}
	}

	// UV test texture
	{
		Texture* t = &m->uv_test_texture;
		*t = (Texture) {
			.psm = GU_PSM_8888,
			.size_px = { 512, 512 },
			.stride_px = 512,
			.nb_mipmap_levels = 9,
			.is_swizzled = false,
		};

		texture_allocate_buffers(t);

		u32* pixels = texture_get_data_for_level(t, 0);
		for (u32 y = 0; y < t->size_px[1]; ++y)
			for (u32 x = 0; x < t->size_px[0]; ++x)
				pixels[y * t->stride_px + x] = GU_ABGR(0xff, 0x00, y, x);
		
		sceKernelDcacheWritebackRange(pixels, texture_get_level_size_in_bytes(t, 0));
	}

	// Skybox test texture
	{
		const u32 face_w = 42;
		const u32 face_h = 32;
		SkyboxTexture* st = &m->skybox_test_texture;
		*st = (SkyboxTexture) {
			.texture = {
				.psm = GU_PSM_8888,
				.size_px = { 128, 64 },
				.stride_px = 128,
				.nb_mipmap_levels = 1,
				.is_swizzled = false,
				// .is_non_power_of_two_on_purpose = true,
			},
			.face_size_px = { face_w, face_h },
		};

		texture_allocate_buffers(&st->texture);

		for (u32 i = 0; i < 6; ++i) {
			const u32 color = skybox_face_id_get_color(i);
			const Texture face_texture = skybox_texture_get_face_texture(st, i);
			u32* pixels = face_texture.data;
			for (u32 y = 0; y < face_texture.size_px[1]; ++y)
				for (u32 x = 0; x < face_texture.size_px[0]; ++x)
					pixels[y * face_texture.stride_px + x] = color;
		}

		// Make sure UV (0, 0) redirects to a black background
		*(u32*) st->texture.data = GU_ABGR(0, 0, 0, 0);
		
		sceKernelDcacheWritebackRange(st->texture.data, texture_get_level_size_in_bytes(&st->texture, 0));
	}

	// Normal-to-color texture
	{
		Texture* t = &m->normal_to_color_texture;
		*t = (Texture) {
			.psm = GU_PSM_8888,
			.size_px = { 32, 32 },
			.stride_px = 32,
			.nb_mipmap_levels = 1,
		};

		const size_t size = texture_get_level_size_in_bytes(t, 0);

		texture_allocate_buffers(t);

		u32* tmp_pixels = malloc(size);
		for (u32 y = 0; y < t->size_px[1]; ++y) {
			for (u32 x = 0; x < t->size_px[0]; ++x) {
				const f32 fx = 0.f + x / (t->size_px[0] - 1.f);
				const f32 fy = 1.f - y / (t->size_px[1] - 1.f);
				f32 nx = (fx - 0.5f) * 2.f;
				f32 ny = (fy - 0.5f) * 2.f;
				f32 nz = 0.f;
				const f32 nxy_magnitude_squared = nx*nx + ny*ny;
				if (nxy_magnitude_squared > 1.f) {
					const f32 nxy_magnitude = sqrtf(nxy_magnitude_squared);
					nx /= nxy_magnitude;
					ny /= nxy_magnitude;
					nz = 0.f;
				} else {
					nz = sqrtf(1.f - nxy_magnitude_squared);
				}
				tmp_pixels[y * t->stride_px + x] = GU_ABGR(0xff, (u32) (0xff * nz), (u32) (0xff * fmaxf(0, ny)), (u32) (0xff * fmaxf(0, nx)));
			}
		}
		
		u32* pixels = texture_get_data_for_level(t, 0);
		swizzle_fast(pixels, tmp_pixels, t->size_px[0] * gu_psm_get_bytes_per_pixel(t->psm), t->size_px[1]);
		t->is_swizzled = true;

		free(tmp_pixels);
		
		sceKernelDcacheWritebackRange(pixels, size);
	}

	// Reflection texture
	{
		Texture* t = &m->reflection_texture;
		*t = (Texture) {
			.psm = GU_PSM_8888,
			.size_px = { 128, 128 },
			.stride_px = 128,
			.nb_mipmap_levels = 1,
		};

		const size_t size = texture_get_level_size_in_bytes(t, 0);

		texture_allocate_buffers(t);

		u32* tmp_pixels = malloc(size);
		for (u32 y = 0; y < t->size_px[1]; ++y) {
			for (u32 x = 0; x < t->size_px[0]; ++x) {
				const f32 fx = 0.f + x / (t->size_px[0] - 1.f);
				const f32 fy = 1.f - y / (t->size_px[1] - 1.f);
				f32 nx = (fx - 0.5f) * 2.f;
				f32 ny = (fy - 0.5f) * 2.f;
				f32 nz = 0.f;
				const f32 nxy_magnitude_squared = nx*nx + ny*ny;
				if (nxy_magnitude_squared > 1.f) {
					const f32 nxy_magnitude = sqrtf(nxy_magnitude_squared);
					nx /= nxy_magnitude;
					ny /= nxy_magnitude;
					nz = 0.f;
				} else {
					nz = sqrtf(1.f - nxy_magnitude_squared);
				}
				// reflect(I, N) = I - 2.0 * dot(N, I) * N
				// When I = (0, 0, -1):
				//     R = (0, 0, -1) - 2 * -N.z * N
				//     R = (0, 0, -1) + 2 * N.z * N
				const f32 rx = 2.f * nz * nx;
				const f32 ry = 2.f * nz * ny;
				const f32 rz = 2.f * nz * nz - 1.f;
				tmp_pixels[y * t->stride_px + x] = GU_ABGR(0xff, (u32) (0xff * (rz * 0.5f + 0.5f)), (u32) (0xff * (ry * 0.5f + 0.5f)), (u32) (0xff * (rx * 0.5f + 0.5f)));

				if (true) {
					const ScePspFVector3 uvs = skybox_uvs_from_normal((ScePspFVector3) { rx, ry, rz });
					const i16 uvs_i16[] = { uvs.x * INT16_MAX, uvs.y * 255 }; // Because alpha is not written to the framebuffer, the V coordinate's 8 MSBs are dropped. We fix that with sceGuTexScale().
					memcpy(&tmp_pixels[y * t->stride_px + x], uvs_i16, sizeof uvs_i16);
				}
			}
		}
		
		u32* pixels = texture_get_data_for_level(t, 0);
		swizzle_fast(pixels, tmp_pixels, t->size_px[0] * gu_psm_get_bytes_per_pixel(t->psm), t->size_px[1]);
		t->is_swizzled = true;

		free(tmp_pixels);
		
		sceKernelDcacheWritebackRange(pixels, size);
	}

	// Render-to-vertex-buffer texture
	{
		Texture* t = &m->rtvb_xyz_texture;
		*t = (Texture) {
			.psm = GU_PSM_8888,
			.size_px = { 512, 256 },
			.stride_px = 512,
			.nb_mipmap_levels = 1,
		};

		const size_t size = texture_get_level_size_in_bytes(t, 0);

		texture_allocate_buffers(t);

		u32* tmp_pixels = malloc(size);
		for (u32 y = 0; y < t->size_px[1]; ++y) {
			for (u32 x = 0; x < t->size_px[0]; ++x) {
				// Figure out the "image-space" position of the pixel that comes before us in the linear buffer layout
				u32 px = (x + 1) / 2;
				u32 py = y;
				if (px >= 1) {
					px -= 1;
				} else if (py >= 1) {
					px = t->stride_px / 2 - 1;
					py -= 1;
				}

				const i8 ix = INT8_MIN + 1 + px;
				const i8 iy = INT8_MIN + 1 + py;
				i8 rgba[] = { ix, iy, 0, x & 1 };
				memcpy(&tmp_pixels[y * t->stride_px + x], rgba, 4);
			}
		}

		u32* pixels = texture_get_data_for_level(t, 0);
		swizzle_fast(pixels, tmp_pixels, t->size_px[0] * gu_psm_get_bytes_per_pixel(t->psm), t->size_px[1]);
		t->is_swizzled = true;

		free(tmp_pixels);
		
		sceKernelDcacheWritebackRange(pixels, texture_get_level_size_in_bytes(t, 0));
	}

	const bool should_swizzle = true;
	m->lava_texture = texture_load_from_tga_path((const TextureLoadParams[]) {{ .should_swizzle = should_swizzle, .max_mipmap_levels = 9 }}, "assets/lava.tga", true);
	m->horizon_gradient_texture = texture_load_from_tga_path((const TextureLoadParams[]) {{ .should_swizzle = should_swizzle, .max_mipmap_levels = 9 }}, "assets/horizon_gradient.tga", true);
	m->mountain_bg_texture = texture_load_from_tga_path((const TextureLoadParams[]) {{ .should_swizzle = should_swizzle, .max_mipmap_levels = 9 }}, "assets/mountain_bg.tga", true);
	m->lava_skybox_texture = texture_load_from_tga_path((const TextureLoadParams[]) {{ .should_swizzle = should_swizzle, .max_mipmap_levels = 1 }}, "assets/lava_skybox.tga", true);

	{
		void* vram_buffer = psp_ptr_from_vram(NULL);
		gu_texture_generate_mipmaps(&m->uv_test_texture, 0, 9, vram_buffer);
		gu_texture_generate_mipmaps(&m->lava_texture, 0, 9, vram_buffer);
		gu_texture_generate_mipmaps(&m->horizon_gradient_texture, 0, 9, vram_buffer);
		gu_texture_generate_mipmaps(&m->mountain_bg_texture, 0, 9, vram_buffer);
	}

	m->bezier_grid_mesh.nb_rows = 16;
	m->bezier_grid_mesh.nb_columns = 16;
	m->grid_mesh.nb_rows = 16;
	m->grid_mesh.nb_columns = 16;

	// Meshes. First pass is for determining size requirements then allocate buffers, second pass is for filling buffers
	for (size_t i = 0; i < 2; ++i) {
		mesh_generate_torus(&m->torus_mesh, 48, 48, 0.5f, 0.3f);
		gridmesh_generate(&m->grid_mesh);
		gridmesh_generate_bezier(&m->bezier_grid_mesh);
		mesh_generate_fullscreen_quad_i16(&m->fullscreen_quad_2d_mesh, PSP_SCREEN_WIDTH, PSP_SCREEN_HEIGHT, PSP_SCREEN_WIDTH, PSP_SCREEN_HEIGHT, -1, -1);
		if (i == 0) {
			mesh_allocate_buffers(&m->torus_mesh);
			mesh_allocate_buffers(&m->grid_mesh.mesh);
			mesh_allocate_buffers(&m->bezier_grid_mesh.mesh);
			mesh_allocate_buffers(&m->fullscreen_quad_2d_mesh);
		}
	}
}

void app_assets_deinit(AppAssets* m) {
	mesh_destroy(&m->torus_mesh);
	mesh_destroy(&m->grid_mesh.mesh);
	mesh_destroy(&m->bezier_grid_mesh.mesh);
	mesh_destroy(&m->fullscreen_quad_2d_mesh);

	texture_destroy(&m->uv_test_texture);
	texture_destroy(&m->horizon_gradient_texture);
	texture_destroy(&m->mountain_bg_texture);
	texture_destroy(&m->lava_skybox_texture);
	texture_destroy(&m->lava_texture);
	texture_destroy(&m->skybox_test_texture.texture);
	texture_destroy(&m->normal_to_color_texture);
	texture_destroy(&m->reflection_texture);
	texture_destroy(&m->rtvb_xyz_texture);

	{
		ColorLutsMemory* c = &m->color_luts_mem;
		for (size_t i = 0; i < countof(c->clut_per_channel); ++i) {
			free(c->clut_per_channel[i]);
		}
	}
}

void app_scene_update_lava_mesh_deformation(GridMesh* gm, f32 t) {
	const Mesh* m = &gm->mesh;
	app_assert(m->nb_vertices == gm->nb_rows * gm->nb_columns);
	app_assert(m->gu_vertex_format == Vertex_Ni8_Pi16_FORMAT);
	Vertex_Ni8_Pi16* vertices = m->vertices;

	const f32 rows_minus_one_inv = 1.f / (gm->nb_rows - 1.f);

	for (size_t j = 0; j < gm->nb_rows; ++j) {
		for (size_t i = 0; i < gm->nb_columns; ++i) {
			Vertex_Ni8_Pi16* it = &vertices[j * gm->nb_columns + i];
			const f32 dist = i * rows_minus_one_inv * 14.f;
			it->position[1] = (0.1f * cosf(t * 8.f + dist)) * INT16_MAX;
		}
	}
	sceKernelDcacheWritebackRange(vertices, m->nb_vertices * sizeof vertices[0]);
}

void app_scene_update(App* app) {
	ScePspFVector3 zero = { 0, 0, 0 };
	ScePspFVector3 up_vector = { 0.f, 1.f, 0.f };
	ScePspFVector3 eye_target_position = { 0.f, 10.f, 0.f };

	// Camera
	{
		ScePspFVector3 eye_position = { 0.f, 10.f, 25.f };

		ScePspFMatrix4 eye_transform_matrix;
		gumLoadIdentity(&eye_transform_matrix);
		// gumRotateY(&eye_transform_matrix, -1.f * app->loop.game_time.time_since_start);
		gumTranslate(&eye_transform_matrix, &eye_position);
		memcpy(&eye_position, &eye_transform_matrix.w.x, sizeof eye_position);

		Camera* c = &app->scene.camera;
		gumLoadIdentity(&c->view_matrix);
		gumLookAt(&c->view_matrix, &eye_position, &eye_target_position, &up_vector);

		c->view_matrix_r = c->view_matrix;
		c->view_matrix_r.w = (ScePspFVector4) { 0, 0, 0, 1 }; // Remove translation

		gumLoadIdentity(&c->proj_matrix);
		gumPerspective(&c->proj_matrix, 40.f, PSP_SCREEN_WIDTH / (f32) PSP_SCREEN_HEIGHT, 0.5f, 1000.f);
	}

	// Light
	{
		Light* l = &app->scene.lights[0];
		ScePspFVector3 pos = { 0, 20.f, 20.f };

		gumLoadIdentity(&l->model_matrix);
		gumLookAt(&l->model_matrix, &pos, &zero, &up_vector);
		gumFullInverse(&l->model_matrix, &l->model_matrix);
	}

	{
		Light* l = &app->scene.lights[1];
		ScePspFVector3 pos = { 0.f, -1.f, 0.01f };

		gumLoadIdentity(&l->model_matrix);
		gumLookAt(&l->model_matrix, &pos, &zero, &up_vector);
		gumFullInverse(&l->model_matrix, &l->model_matrix);
	}

	// Torus
	{
		ScePspFVector3 torus_position = { 0.f, 10.f, 0.f };
		ScePspFVector3 torus_scale = { 10.f, 10.f, 10.f };

		MeshInstance* mi = &app->scene.torus;
		mi->mesh = &app->assets.torus_mesh;

		gumLoadIdentity(&mi->model_matrix);
		gumTranslate(&mi->model_matrix, &torus_position);
		gumRotateY(&mi->model_matrix, -2.f * app->loop.game_time.time_since_start);
		mi->model_matrix_tr = mi->model_matrix;
		gumScale(&mi->model_matrix, &torus_scale);
	}

	// Grid
	{
		ScePspFVector3 grid_scale = { 100.f, 100.f, 100.f };

		MeshInstance* mi = &app->scene.grid;
		mi->mesh = &app->assets.grid_mesh.mesh;

		gumLoadIdentity(&mi->model_matrix);
		mi->model_matrix_tr = mi->model_matrix;
		gumScale(&mi->model_matrix, &grid_scale);
	}

	// Grid (Bezier)
	{
		ScePspFVector3 grid_scale = { 100.f, 10.f, 100.f };

		MeshInstance* mi = &app->scene.grid_bezier;
		mi->mesh = &app->assets.bezier_grid_mesh.mesh;

		gumLoadIdentity(&mi->model_matrix);
		mi->model_matrix_tr = mi->model_matrix;
		gumScale(&mi->model_matrix, &grid_scale);
	}

	app_scene_update_lava_mesh_deformation(&app->assets.bezier_grid_mesh, app->loop.game_time.time_since_start);

	const f32 lava_speed[2] = { 0.7f, 0.1f };
	for (size_t i = 0; i < 2; ++i)
		app->scene.lava_flow[i] = fmodf(app->scene.lava_flow[i] + lava_speed[i] * app->loop.game_time.last_frame_duration, 1.f);
}

void mesh_instance_draw(const MeshInstance* mi) {
	sceGuSetMatrix(GU_MODEL, &mi->model_matrix);
	mesh_draw_3d(mi->mesh);
}

void mesh_instance_draw_sampling_sky_texture_via_normals(const MeshInstance* mi, const Camera* camera) {
	sceGuTexProjMapMode(GU_NORMALIZED_NORMAL);
	sceGuTexMapMode(GU_TEXTURE_MATRIX, 0, 0);

	ScePspFMatrix4 model_matrix_r = mi->model_matrix_tr;
	model_matrix_r.w = (ScePspFVector4) { 0, 0, 0, 1 };

	ScePspFMatrix4 texture_matrix;
	gumLoadIdentity(&texture_matrix);
	gumTranslate(&texture_matrix, (const ScePspFVector3[]) {{ 0.5f, 0.5f, 1.f }});
	gumScale(&texture_matrix, (const ScePspFVector3[]) {{ 0.5f, -0.5f, 0.f }});
	gumMultMatrix(&texture_matrix, &texture_matrix, &camera->view_matrix_r);
	gumMultMatrix(&texture_matrix, &texture_matrix, &model_matrix_r);
	sceGuSetMatrix(GU_TEXTURE, &texture_matrix);

	mesh_instance_draw(mi);
}

void mesh_instance_draw_sampling_texture_via_positions_xz(const MeshInstance* mi, f32 sx, f32 sy, f32 tx, f32 ty) {
	sceGuTexProjMapMode(GU_POSITION);
	sceGuTexMapMode(GU_TEXTURE_MATRIX, 0, 0);

	ScePspFMatrix4 model_matrix_r = mi->model_matrix_tr;
	model_matrix_r.w = (ScePspFVector4) { 0, 0, 0, 1 };

	ScePspFMatrix4 xyz_to_xz0 = {
		{ 1, 0, 0, 0 },
		{ 0, 0, 0, 0 },
		{ 0, 1, 0, 0 },
		{ 0, 0, 0, 1 },
	};

	ScePspFMatrix4 texture_matrix;
	gumLoadIdentity(&texture_matrix);
	gumTranslate(&texture_matrix, (const ScePspFVector3[]) {{ tx, ty, 1 }});
	gumScale(&texture_matrix, (const ScePspFVector3[]) {{ sx, sy, 0 }});
	gumMultMatrix(&texture_matrix, &texture_matrix, &xyz_to_xz0);
	gumMultMatrix(&texture_matrix, &texture_matrix, &model_matrix_r);
	sceGuSetMatrix(GU_TEXTURE, &texture_matrix);

	mesh_instance_draw(mi);
}

void app_draw_postprocessing(App* app, const Texture* scene3d_fb) {
	Texture scene3d_fb_paletted = *scene3d_fb;
	scene3d_fb_paletted.psm = scene3d_fb->psm == GU_PSM_8888 ? GU_PSM_T32 : GU_PSM_T16;

	const Texture* fb0 = &app->gfx.framebuffers[0];

	// Disabling depth stuff is not just an optimization here, it's important for correctness.
	// We disable depth writes because we're reusing the Z-buffer as storage for temporary pingpong buffers.
	// We disable depth test because depth values are no longer meaningful and may cause incorrect rendering.
	sceGuDepthMask(1);
	sceGuDisable(GU_DEPTH_TEST);

	sceGuTexFilter(GU_NEAREST, GU_NEAREST);
	sceGuTexWrap(GU_CLAMP, GU_CLAMP);
	sceGuTexFunc(GU_TFX_REPLACE, GU_TCC_RGBA);

	// Bloom filter pass
	if (true) {
		// HRB = half-res buffer
		// We hijack the Z-buffer that isn't needed anymore.
		// The Z buffer's memory can hold exactly 2 32-bit color HRBs, or 4 16-bit color HRBs.
		Texture hrbs[2];
		for (size_t i = 0; i < countof(hrbs); ++i) {
			Texture* hrb = &hrbs[i];
			*hrb = *fb0;
			hrb->size_px[0] >>= 1;
			hrb->size_px[1] >>= 1;
			hrb->stride_px >>= 1;
			hrb->data = (u8*) app->gfx.z_buffer.data + i * texture_get_level_size_in_bytes(hrb, 0);
		}

		sceGuTexFilter(GU_LINEAR_MIPMAP_LINEAR, GU_LINEAR_MIPMAP_LINEAR);

		const u32 bloom_opacity = app->vars[VAR_ID__BLOOM_OPACITY].value;

		const u32 threshold = app->vars[VAR_ID__BLOOM_THRESHOLD].value;
		const u32 threshold_color = gu_color_8888_reduce_to_psm_quality(GU_ABGR(0xff, threshold, threshold, threshold), hrbs[0].psm);

		// The max() filter can be performed via GU_COLOR_TEST only if the following holds:
		// - Each channel of the threshold has the property "from MSB to LSB, start with all 1, then end with all 0".
		//   The list of these values is `colortest_threshold_values`.
		// 
		// This approach is "fast" as it only requires a single pass, however, it's not the same as the GU_MAX approach.
		// - GU_COLOR_TEST:
		//   Only the colors for which ALL channels are above the threshold will pass.
		//   The threshold is inclusive.
		// - GU_MAX:
		//   Only the channels above the threshold will pass.
		//   The threshold is exclusive.
		const bool bloom_filter_via_color_test_only = false;

		if (bloom_filter_via_color_test_only) {
			const u8 colortest_threshold_values[] = { 0x00, 0x80, 0xc0, 0xe0, 0xf0, 0xf8, 0xfc, 0xfe, 0xff };

			u32 tc = threshold_color;
			for (u32 i = 0; i < 3; ++i) {
				u8 c = (tc >> (i * 8)) & 0xff;
				if (c != 0) {
					for (u32 j = 1; j < countof(colortest_threshold_values); ++j) {
						if (c <= colortest_threshold_values[j]) {
							c = colortest_threshold_values[j];
							break;
						}
					}
					tc &= ~(0xffu << (i * 8));
					tc |= c << (i * 8);
				}
			}

			gu_set_rendertarget(&hrbs[0]);
			sceGuClearColor(0);
			sceGuClear(GU_COLOR_BUFFER_BIT | GU_FAST_CLEAR_BIT);

			sceGuEnable(GU_COLOR_TEST);
			sceGuColorFunc(GU_EQUAL, tc, tc);
			gu_set_texture(scene3d_fb);
			gu_draw_fullscreen_textured_quad_i16(hrbs[0].size_px[0], hrbs[0].size_px[1], scene3d_fb->size_px[0], scene3d_fb->size_px[1], -1, -1);
			sceGuDisable(GU_COLOR_TEST);
		} else {
			gu_set_rendertarget(&hrbs[1]);

			sceGuClearColor(threshold_color); // color is expressed as RGBA8888 regardless of color buffer format
			sceGuClear(GU_COLOR_BUFFER_BIT | GU_FAST_CLEAR_BIT);

			sceGuEnable(GU_BLEND);
			sceGuBlendFunc(GU_MAX, 0, 0, 0, 0); // The 4 arguments seem to be ignored for GU_MAX, GU_MIN and GU_ABS
			gu_set_texture(scene3d_fb);
			gu_draw_fullscreen_textured_quad_i16(hrbs[1].size_px[0], hrbs[1].size_px[1], scene3d_fb->size_px[0], scene3d_fb->size_px[1], -1, -1);
			sceGuDisable(GU_BLEND);

			gu_set_rendertarget(&hrbs[0]);
			sceGuClearColor(0);
			sceGuClear(GU_COLOR_BUFFER_BIT | GU_FAST_CLEAR_BIT);

			sceGuEnable(GU_COLOR_TEST);
			sceGuColorFunc(GU_NOTEQUAL, threshold_color, 0xffffffu); // color is expressed as RGBA8888 regardless of color buffer format
			gu_set_texture(&hrbs[1]);
			gu_draw_fullscreen_textured_quad_i16(hrbs[0].size_px[0], hrbs[0].size_px[1], hrbs[1].size_px[0], hrbs[1].size_px[1], -1, -1);
			sceGuDisable(GU_COLOR_TEST);
		}

		gu_set_rendertarget(fb0);
		gu_set_texture(scene3d_fb);
		mesh_draw_2d(&app->assets.fullscreen_quad_2d_mesh);
		sceGuEnable(GU_BLEND);
		sceGuBlendFunc(GU_ADD, GU_FIX, GU_FIX, GU_ABGR(0xff, bloom_opacity, bloom_opacity, bloom_opacity), 0xffffffff);
		gu_set_texture(&hrbs[0]);
		gu_draw_fullscreen_textured_quad_i16(fb0->size_px[0], fb0->size_px[1], hrbs[0].size_px[0], hrbs[0].size_px[1], -1, -1);
		sceGuDisable(GU_BLEND);

		sceGuTexFilter(GU_NEAREST, GU_NEAREST);
	}

	// LUTs
	if (false) {
		gu_set_texture(&scene3d_fb_paletted);

		const PsmInfo psm_info = gu_psm_get_info(app->assets.color_luts_mem.psm);
		u32 nb_bits_processed = 0;
		const LUTMode lut_mode = lut_get_mode(app->scene.camera.post_processing.lut);
		switch (lut_mode) {
		case LUT_MODE_1_TO_1:
			for (int i = 0; i < 3; ++i) {
				const u32 channel_max = (1u << psm_info.channels[i].nb_bits) - 1u;
				sceGuClutMode(app->assets.color_luts_mem.psm, nb_bits_processed, channel_max, 0);
				sceGuClutLoad((channel_max + 1) / 8, app->assets.color_luts_mem.clut_per_channel[i]);

				sceGuPixelMask(~(0xffu << (i*8))); // Format is RGBA8888
				mesh_draw_2d(&app->assets.fullscreen_quad_2d_mesh);

				nb_bits_processed += psm_info.channels[i].nb_bits;
			}
			sceGuPixelMask(0);
			break;
		case LUT_MODE_3_TO_3:
			for (int i = 0; i < 3; ++i) {
				const u32 channel_max = (1u << psm_info.channels[i].nb_bits) - 1u;
				sceGuClutMode(app->assets.color_luts_mem.psm, nb_bits_processed, channel_max, 0);
				sceGuClutLoad((channel_max + 1) / 8, app->assets.color_luts_mem.clut_per_channel[i]);

				if (i >= 1) {
					sceGuEnable(GU_BLEND);
					sceGuBlendFunc(GU_ADD, GU_FIX, GU_FIX, 0xffffffff, 0xffffffff);
				}

				mesh_draw_2d(&app->assets.fullscreen_quad_2d_mesh);

				nb_bits_processed += psm_info.channels[i].nb_bits;
			}
			sceGuDisable(GU_BLEND);
			break;
		default:
			break;
		}
	}
}

// The job of this matrix is, for any given viewport size:
// - Transform a grid of vertices which positions are 8-bit, from -127 to 127 (all inclusive; notice that -128 is not used)
// - Each vertex must land on a pixel-perfect on the viewport
// - Vertex position -127 must map to top-left of the viewport
// - An empty space must be inserted between each column
void get_pixel_perfect_matrix(ScePspFMatrix4* m, f32 obj_w_px, f32 obj_h_px, f32 viewport_w, f32 viewport_h, f32 offset_px_x, f32 offset_px_y, f32 scale_x) {
	const f32 half_viewport_w = viewport_w * 0.5f;
	const f32 half_viewport_h = viewport_h * 0.5f;

	// Ranges of "good values" are (empirically):
	// when scale_x == 1: 1.013 - 1.13
	// when scale_x == 2: 1.013 - 1.0667
	const f32 hack = 1.013f;

	const ScePspFVector3 t = {
		(offset_px_x - half_viewport_w + (obj_w_px + hack) * 0.5f * scale_x) / half_viewport_w,
		(offset_px_y + half_viewport_h - (obj_h_px + hack) * 0.5f) / half_viewport_h,
		0
	};

	const ScePspFVector3 s = {
		scale_x * +(obj_w_px + hack) / viewport_w,
		-(obj_h_px + hack) / viewport_h,
		1
	};

	gumLoadIdentity(m);
	gumTranslate(m, &t);
	gumScale(m, &s);
}

// We can't render 256x256 vertices in one go; the max number of vertices per draw call is UINT16_MAX, and 256*256 is just above that
#define QUAD128_W 255
#define QUAD128_H 255

void app_draw_scene(App* app) {
	// Light
	{
		const size_t light_index = 0;
		ScePspFVector3 lp = {
			app->scene.lights[light_index].model_matrix.z.x,
			app->scene.lights[light_index].model_matrix.z.y,
			app->scene.lights[light_index].model_matrix.z.z,
		};
		sceGuLight(light_index, GU_DIRECTIONAL, GU_DIFFUSE_AND_SPECULAR, &lp);
		sceGuLightColor(light_index, GU_DIFFUSE, GU_ABGR(0xff, 0xff, 0xff, 0xff));
		sceGuLightColor(light_index, GU_SPECULAR, GU_ABGR(0xff, 0xff, 0xff, 0xff));
		sceGuLightAtt(light_index, 0.0f, 1.0f, 0.0f);
	}

	{
		const u32 light_color = GU_ABGR(0xff, 110, 170, 255);
		const size_t light_index = 1;
		ScePspFVector3 lp = {
			app->scene.lights[light_index].model_matrix.z.x,
			app->scene.lights[light_index].model_matrix.z.y,
			app->scene.lights[light_index].model_matrix.z.z,
		};
		sceGuLight(light_index, GU_DIRECTIONAL, GU_DIFFUSE_AND_SPECULAR, &lp);
		sceGuLightColor(light_index, GU_DIFFUSE, light_color);
		sceGuLightColor(light_index, GU_SPECULAR, light_color);
		sceGuLightAtt(light_index, 0.0f, app->vars[VAR_ID__LAVA_ATTENUATION].value, 0.0f);
	}

	sceGuAmbient(GU_ABGR(0xff, 100, 100, 100));

	sceGuDisable(GU_LIGHTING);
	sceGuDisable(GU_LIGHT0);
	sceGuDisable(GU_LIGHT1);

	// Skybox
	{
		const Texture* t = &app->assets.mountain_bg_texture;
		sceGuEnable(GU_TEXTURE_2D);
		gu_set_texture(t);
		gu_draw_fullscreen_textured_quad_i16(PSP_SCREEN_WIDTH, PSP_SCREEN_HEIGHT, t->size_px[0], t->size_px[1], -1, -1);
	}

	sceGuEnable(GU_LIGHTING);
	// sceGuEnable(GU_LIGHT0);
	sceGuEnable(GU_LIGHT1);

	// Lava
	{
		sceGuEnable(GU_TEXTURE_2D);
		sceGuTexFunc(GU_TFX_REPLACE, GU_TCC_RGBA);
		sceGuTexFilter(GU_LINEAR_MIPMAP_LINEAR, GU_LINEAR_MIPMAP_LINEAR);
		sceGuTexWrap(GU_REPEAT, GU_REPEAT);

		gu_set_texture(&app->assets.lava_texture);
		// mesh_instance_draw_sampling_texture_via_positions_xz(&app->scene.grid, 2.f, 2.f, app->scene.lava_flow[0], app->scene.lava_flow[1]);
		mesh_instance_draw_sampling_texture_via_positions_xz(&app->scene.grid_bezier, 2.f, 2.f, app->scene.lava_flow[0], app->scene.lava_flow[1]);

		sceGuTexFunc(GU_TFX_MODULATE, GU_TCC_RGBA);
		sceGuTexFilter(GU_LINEAR_MIPMAP_LINEAR, GU_LINEAR_MIPMAP_LINEAR);
		sceGuTexWrap(GU_CLAMP, GU_CLAMP);

		sceGuAmbient(GU_ABGR(0xff, 1, 4, 20));
	}

	// Torus
	{
		sceGuEnable(GU_TEXTURE_2D);
		sceGuTexFunc(GU_TFX_MODULATE, GU_TCC_RGBA);
		sceGuTexFunc(GU_TFX_REPLACE, GU_TCC_RGBA);
		sceGuTexFunc(GU_TFX_ADD, GU_TCC_RGBA);
		sceGuModelColor(0, 0, 0xffffffu, 0xffffffu);
		sceGuAmbientColor(0xffffffffu);
		sceGuSpecular(app->vars[VAR_ID__SPECULAR].value); // 7 was a good value
		gu_set_texture_ex(&app->assets.mountain_bg_texture, app->vars[VAR_ID__MIP_LEVEL].value, 1);
		mesh_instance_draw_sampling_sky_texture_via_normals(&app->scene.torus, &app->scene.camera);
		sceGuModelColor(0, 0xffffffu, 0xffffffu, 0xffffffu);
	}

	sceGuDisable(GU_LIGHTING);
	sceGuDisable(GU_LIGHT0);
	sceGuDisable(GU_LIGHT1);

	sceGuEnable(GU_TEXTURE_2D);
	sceGuTexProjMapMode(GU_UV);
	sceGuTexMapMode(GU_TEXTURE_COORDS, 0, 0);

	sceGuTexFunc(GU_TFX_REPLACE, GU_TCC_RGBA);
	sceGuTexFilter(GU_NEAREST, GU_NEAREST);
	sceGuTexWrap(GU_CLAMP, GU_CLAMP);

	if (true) {
		Texture obj_rt = app->gfx.pingpong0_fb;
		obj_rt.stride_px  = 256;
		obj_rt.size_px[0] = 256;
		obj_rt.size_px[1] = 144; // 256 * 9 / 16

		Texture vb_rt = obj_rt;
		vb_rt.data = ptr_align((u8*) obj_rt.data + texture_get_level_size_in_bytes(&obj_rt, 0), 16);

		gu_set_rendertarget(&obj_rt);

		sceGuClearColor(0);
		sceGuClearDepth(0);
		sceGuClearStencil(0);
		sceGuClear(GU_COLOR_BUFFER_BIT | GU_DEPTH_BUFFER_BIT | GU_STENCIL_BUFFER_BIT | GU_FAST_CLEAR_BIT);

		const MeshInstance* mi = &app->scene.torus;
		const Camera* camera = &app->scene.camera;

		gu_set_texture(&app->assets.reflection_texture);
		sceGuTexFunc(GU_TFX_REPLACE, GU_TCC_RGBA);
		sceGuTexFilter(GU_NEAREST, GU_NEAREST);
		sceGuTexWrap(GU_CLAMP, GU_CLAMP);

		sceGuTexProjMapMode(GU_NORMALIZED_NORMAL);
		sceGuTexMapMode(GU_TEXTURE_MATRIX, 0, 0);

		ScePspFMatrix4 model_matrix_r = mi->model_matrix_tr;
		model_matrix_r.w = (ScePspFVector4) { 0, 0, 0, 1 };

		ScePspFMatrix4 texture_matrix;
		gumLoadIdentity(&texture_matrix);
		gumTranslate(&texture_matrix, (const ScePspFVector3[]) {{ 0.5f, 0.5f, 1.f }});
		gumScale(&texture_matrix, (const ScePspFVector3[]) {{ 0.5f, -0.5f, 0.f }});
		gumMultMatrix(&texture_matrix, &texture_matrix, &camera->view_matrix_r);
		gumMultMatrix(&texture_matrix, &texture_matrix, &model_matrix_r);
		sceGuSetMatrix(GU_TEXTURE, &texture_matrix);

		mesh_instance_draw(mi);

		sceGuTexProjMapMode(GU_UV);
		sceGuTexMapMode(GU_TEXTURE_COORDS, 0, 0);

		sceGuDisable(GU_DEPTH_TEST);
		sceGuDepthMask(1);

		ScePspFMatrix4 identity_matrix;
		gumLoadIdentity(&identity_matrix);
		sceGuSetMatrix(GU_VIEW, &identity_matrix);
		sceGuSetMatrix(GU_PROJECTION, &identity_matrix);

		for (i32 pass = 0; pass < 2; ++pass) {
			gu_set_rendertarget(&vb_rt);

			// We can't write to the alpha channel, so let's at least make sure it is zero.
			sceGuClearStencil(0);
			sceGuClear(GU_STENCIL_BUFFER_BIT | GU_FAST_CLEAR_BIT);

			gu_set_texture(&obj_rt);
			gu_draw_fullscreen_textured_quad_i16(vb_rt.size_px[0], vb_rt.size_px[1], obj_rt.size_px[0], obj_rt.size_px[1], -1, -1);

			sceGuEnable(GU_ALPHA_TEST);
			sceGuAlphaFunc(GU_EQUAL, (pass + 1) & 1, 1);
			const Texture* tp = &app->assets.rtvb_xyz_texture;
			gu_set_texture(tp);
			gu_draw_fullscreen_textured_quad_i16(tp->size_px[0], tp->size_px[1], tp->size_px[0], tp->size_px[1], -1, -1);
			sceGuDisable(GU_ALPHA_TEST);

			Mesh mesh = {
				.gu_topology = GU_POINTS,
				.gu_vertex_format = Vertex_Ti16_Pi8_FORMAT,
				.sizeof_vertex = sizeof(Vertex_Ti16_Pi8),
				.vertices = (u8*) vb_rt.data + pass * 4,
				.nb_vertices = vb_rt.size_px[1] * (vb_rt.stride_px / 2),
			};

			Texture fb0 = app->gfx.framebuffers[0];
			fb0.size_px[0] = vb_rt.size_px[0];
			fb0.size_px[1] = vb_rt.size_px[1];

			gu_set_rendertarget(&fb0);

			sceGuTexScale(1.f, INT16_MAX / 255);

			ScePspFMatrix4 model_matrix;
			get_pixel_perfect_matrix(&model_matrix, QUAD128_W, QUAD128_H, obj_rt.size_px[0], obj_rt.size_px[1], -(2 - pass), 1, 2);
			sceGuSetMatrix(GU_MODEL, &model_matrix);

			gu_set_texture(&app->assets.lava_skybox_texture);
			mesh_draw_3d(&mesh);

			// Flush some cache??
			// TODO: still not enough; there is one black pixel somewhere inside the image, which seems to be fixed by drawing the mesh twice
			// This happens when toggling post-processing. Timing issue?
			mesh.nb_vertices = vb_rt.stride_px / 2;
			mesh_draw_3d(&mesh);

			sceGuSetMatrix(GU_MODEL, &identity_matrix); // Should not be needed, but just in case

			sceGuTexScale(1.f, 1.f);
		}

		sceGuEnable(GU_DEPTH_TEST);
		sceGuDepthMask(0);
	}

	// Test vertex buffer
	if (false) {
		Texture obj_rt = app->gfx.pingpong0_fb;
		switch ((u32) app->vars[VAR_ID__RTVB_RT_CONFIG_ID].value) {
		case 1: obj_rt.stride_px = obj_rt.size_px[0] = obj_rt.size_px[1] = 256; break;
		case 2: obj_rt.stride_px = 256; obj_rt.size_px[0] = 256; obj_rt.size_px[1] = 144; break;
		case 3: obj_rt.stride_px = obj_rt.size_px[0] = obj_rt.size_px[1] = 128; break;
		case 4: obj_rt.stride_px = obj_rt.size_px[0] = obj_rt.size_px[1] = 64; break;
		case 5: obj_rt.stride_px = obj_rt.size_px[0] = obj_rt.size_px[1] = 32; break;
		case 6: obj_rt.stride_px = obj_rt.size_px[0] = obj_rt.size_px[1] = 16; break;
		default: break;
		}

		const u32 quad_w_px = QUAD128_W;
		const u32 quad_h_px = QUAD128_H;

		ScePspFMatrix4 identity_matrix;
		gumLoadIdentity(&identity_matrix);

		ScePspFMatrix4 model_matrix;
		get_pixel_perfect_matrix(&model_matrix, quad_w_px, quad_h_px, obj_rt.size_px[0], obj_rt.size_px[1], 0, 1, app->vars[VAR_ID__RTVB_SPACING].value);

		// NOTE: 8-bit vertex positions don't seem to be supported at all with GU_TRANSFORM_2D... But the same format does work with GU_TRANSFORM_3D.

		Vertex_Ti16_C8888_Pi8* vertices = NULL;
		Mesh mesh = {
			.gu_topology = GU_POINTS,
			.gu_vertex_format = Vertex_Ti16_C8888_Pi8_FORMAT,
			.sizeof_vertex = sizeof vertices[0],
			.vertices = vertices,
			.nb_vertices = quad_w_px * quad_h_px,
		};
		mesh_allocate_buffers_in_current_display_list(&mesh);
		vertices = mesh.vertices;

		for (u32 y = 0; y < quad_h_px; ++y)
		for (u32 x = 0; x < quad_w_px; ++x) {
			Vertex_Ti16_C8888_Pi8 v = {
				.uv = { 
					x * INT16_MAX / (quad_w_px - 1.f),
					y * INT16_MAX / (quad_h_px - 1.f)
				},
				.position = {
					x + INT8_MIN + 1,
					y + INT8_MIN + 1
				},
			};
			switch ((u32) app->vars[VAR_ID__RTVB_RT_COLOR_MODE].value) {
			case 0:
			default:
				v.color = GU_ABGR(0xff, 0x00, 0xff * (y & 1), 0xff * (x & 1));
				break;
			case 1:
				v.color = GU_ABGR(0xff, 0x00, y, x);
				break;
			}
			vertices[y * quad_w_px + x] = v;
		}
		sceKernelDcacheWritebackRange(vertices, mesh.nb_vertices * sizeof vertices[0]);

		sceGuDisable(GU_DEPTH_TEST);
		sceGuDepthMask(1);

		gu_set_rendertarget(&obj_rt);

		sceGuClearColor(0);
		sceGuClear(GU_COLOR_BUFFER_BIT | GU_FAST_CLEAR_BIT);

		sceGuDisable(GU_TEXTURE_2D);
		sceGuColor(0xffffffff);
		sceGuSetMatrix(GU_MODEL, &model_matrix);
		sceGuSetMatrix(GU_VIEW, &identity_matrix);
		sceGuSetMatrix(GU_PROJECTION, &identity_matrix);
		mesh_draw_3d(&mesh);
		sceGuEnable(GU_TEXTURE_2D);

		gu_set_rendertarget(&app->gfx.framebuffers[0]);
		gu_set_texture(&obj_rt);
		gu_draw_fullscreen_textured_quad_i16(obj_rt.size_px[0], obj_rt.size_px[1], obj_rt.size_px[0], obj_rt.size_px[1], -1, -1);

		sceGuEnable(GU_DEPTH_TEST);
	}

	sceGuDisable(GU_LIGHTING);
	sceGuDisable(GU_LIGHT0);
	sceGuDisable(GU_LIGHT1);

	sceGuEnable(GU_TEXTURE_2D);
	sceGuTexProjMapMode(GU_UV);
	sceGuTexMapMode(GU_TEXTURE_COORDS, 0, 0);
}

void app_draw(App* app) {
	if (app->vars[VAR_ID__FB_PSM].value != app->gfx.framebuffers[0].psm) {
		app->gfx.vram_cursor = NULL;
		app_gfx_allocate_vram_resources(&app->gfx, app->vars[VAR_ID__FB_PSM].value);
		app_gfx_use_vram_resources(&app->gfx);

		lut_fill(&app->assets.color_luts_mem, app->scene.camera.post_processing.lut, app->gfx.framebuffers[0].psm);

		pspDebugScreenSetColorMode(app->gfx.framebuffers[0].psm);
	}

	Texture* scene3d_fb = app->scene.camera.post_processing.enabled ? &app->gfx.pingpong0_fb : &app->gfx.framebuffers[0];
	gu_set_rendertarget(scene3d_fb);

	sceGuClearColor(GU_ABGR(0, 0xff, 0, 0));
	sceGuClearDepth(0);
	sceGuClear(GU_COLOR_BUFFER_BIT | GU_DEPTH_BUFFER_BIT | GU_FAST_CLEAR_BIT); // GU_FAST_CLEAR_BIT is really not that much faster, just very slightly. It consumes more memory in the display list but it's also negligible.

	// Camera
	sceGuSetMatrix(GU_VIEW, &app->scene.camera.view_matrix);
	sceGuSetMatrix(GU_PROJECTION, &app->scene.camera.proj_matrix);

	sceGuSetStatus(GU_DITHER, app->vars[VAR_ID__DITHER_GLOBAL].value);

	app_draw_scene(app);

	if (app->scene.camera.post_processing.enabled) {
		app_draw_postprocessing(app, scene3d_fb);
	}
}

void app_input_update(App* app) {
	LUT prev_lut = app->scene.camera.post_processing.lut;

	app->input.previous = app->input.current;
	if (sceCtrlReadBufferPositive(&app->input.current, 1)) {
		if (app->input.current.Buttons != app->input.previous.Buttons) {
			if (app->input.current.Buttons & PSP_CTRL_LTRIGGER)
				app->scene.camera.post_processing.lut = (app->scene.camera.post_processing.lut + LUT_COUNT - 1) % LUT_COUNT;

			if (app->input.current.Buttons & PSP_CTRL_RTRIGGER)
				app->scene.camera.post_processing.lut = (app->scene.camera.post_processing.lut + 1) % LUT_COUNT;

			if (app->input.current.Buttons & PSP_CTRL_CROSS)
				app->scene.camera.post_processing.enabled ^= 1;

			if (app->input.current.Buttons & PSP_CTRL_START)
				app->debug_ui_enabled ^= 1;

			if (app->debug_ui_enabled) {
				if (app->input.current.Buttons & PSP_CTRL_UP) {
					if (app->selected_var_index > 1)
						app->selected_var_index -= 1;
					else if (VAR_ID__COUNT >= 1)
						app->selected_var_index = VAR_ID__COUNT - 1;
				}

				if (app->input.current.Buttons & PSP_CTRL_DOWN) {
					app->selected_var_index = (app->selected_var_index + 1) % VAR_ID__COUNT;
					if (app->selected_var_index == 0 && VAR_ID__COUNT > 1)
						app->selected_var_index = 1;
				}
			}
		}

		if (app->debug_ui_enabled) {
			AppVariable* v = &app->vars[app->selected_var_index];
			i32 variable_edit_direction = 0;

			if ((v->flags & VAR_FLAG_SMOOTH_EDIT) || app->input.current.Buttons != app->input.previous.Buttons) {
				if (app->input.current.Buttons & PSP_CTRL_LEFT)
					variable_edit_direction = -1;

				if (app->input.current.Buttons & PSP_CTRL_RIGHT)
					variable_edit_direction = 1;
			}

			if (variable_edit_direction != 0 && v->step != 0) {
				f32 step = variable_edit_direction * v->step;
				if (v->flags & VAR_FLAG_STEP_PER_SECOND)
					step *= app->loop.real_time.last_frame_duration;

				v->value += step;

				if (v->flags & VAR_FLAG_ROUND) {
					const f32 prev_value = v->value;
					v->value = roundf(v->value);
					if (v->value != prev_value) {
						if (step > 0)
							v->value = roundf(v->value + 1);
						else
							v->value = roundf(v->value - 1);
					}
				}

				v->value = fminf(fmaxf(v->value, v->min), v->max);
			}
		}
	}

	if (app->loop.nb_frames == 0 || app->scene.camera.post_processing.lut != prev_lut)
		lut_fill(&app->assets.color_luts_mem, app->scene.camera.post_processing.lut, app->gfx.framebuffers[0].psm);
}

void app_draw_debug_overlay(App* app) {
	if (!app->debug_ui_enabled)
		return;

	int debug_screen_pos[2] = { 1, 1 };
	pspDebugScreenSetOffset((intptr_t) psp_ptr_to_vram(app->gfx.framebuffers[0].data));
	pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
	pspDebugScreenPrintf("Game Time: %.3f s", (f64) app->loop.game_time.time_since_start);
	pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
	pspDebugScreenPrintf("Frame: %.3f ms", 1000.0 * (f64) app->loop.real_time.last_frame_duration);
	pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
	pspDebugScreenPrintf("CPU with GPU sync: %.3f ms", 1000.0 * (f64) tick_range_get_duration(g_frame_stats.cpu_with_gpu_sync));
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

	if (app->selected_var_index) {
		const AppVariable* v = &app->vars[app->selected_var_index];
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("%s: %f", v->name, (f64) v->value);
	}

	pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
	pspDebugScreenPrintf("Post-processing (Toggle via X): %s", app->scene.camera.post_processing.enabled  ? "on" : "off");
	if (app->scene.camera.post_processing.enabled) {
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("LUT (cycle via L/R): %s", lut_get_name(app->scene.camera.post_processing.lut));
	}
}

void app_frame_inner(App* app) {
	app_input_update(app);

	app_scene_update(app);

	sceGuStart(GU_DIRECT, g_gu_main_list);
	gu_insert_clock_start_marker();
	gu_reset_state_to_app_defaults(app);
	app_draw(app);
	gu_insert_clock_end_marker();
	sceGuFinish();

	psp_rtc_get_current_tick_sync(&g_frame_stats.cpu.end); // End CPU timing now, don't count the sync with GPU
	sceGuSync(GU_SYNC_FINISH, GU_SYNC_WHAT_DONE);
	psp_rtc_get_current_tick_checked(&g_frame_stats.cpu_with_gpu_sync.end);

	app_draw_debug_overlay(app);
}

void app_frame(App* app) {
	TickRange current_frame_tick_range;
	psp_rtc_get_current_tick_checked(&current_frame_tick_range.start);

	g_frame_stats = (FrameStats) {0};
	g_frame_stats.cpu.start = current_frame_tick_range.start;
	g_frame_stats.cpu_with_gpu_sync.start = current_frame_tick_range.start;

	app_frame_inner(app);

	sceDisplayWaitVblankStart();
	app_gfx_swap_buffers(&app->gfx);

	psp_rtc_get_current_tick_checked(&current_frame_tick_range.end);
	app->loop.real_time.last_frame_duration = tick_range_get_duration(current_frame_tick_range);
	app->loop.real_time.time_since_start += app->loop.real_time.last_frame_duration;
	app->loop.game_time.last_frame_duration = app->loop.real_time.last_frame_duration * app->vars[VAR_ID__TIME_DILATION].value;
	app->loop.game_time.time_since_start += app->loop.game_time.last_frame_duration;
	app->loop.nb_frames += 1;
}

void app_init_fpu() {
	pspFpuSetEnable(0); // Disable exceptions
	pspFpuSetRoundmode(PSP_FPU_RN);
	pspFpuSetFS(1); // flush denormals to zero instead of causing an exception
}

// Assert at compile-time that they are equal, meaning we can safely pass GU_PSM_* constants to pspDebugScreenInitEx()
static_assert(GU_PSM_5650 == PSP_DISPLAY_PIXEL_FORMAT_565, "");
static_assert(GU_PSM_5551 == PSP_DISPLAY_PIXEL_FORMAT_5551, "");
static_assert(GU_PSM_4444 == PSP_DISPLAY_PIXEL_FORMAT_4444, "");
static_assert(GU_PSM_8888 == PSP_DISPLAY_PIXEL_FORMAT_8888, "");

int main(int argc, char* argv[]) {
	App app = {0};
	app.selected_var_index = 1;
	app.vars[VAR_ID__INVALID] = (AppVariable) { "Invalid var", 0, 0, 1, 1, VAR_FLAG_ROUND };
	app.vars[VAR_ID__HACK] = (AppVariable) { "Hack", 1.013f, -8, 8, 0.05f, VAR_FLAG_SMOOTH_EDIT | VAR_FLAG_STEP_PER_SECOND };
	app.vars[VAR_ID__RTVB_SPACING] = (AppVariable) { "RTVB Spacing", 1, 1, 2, 1, VAR_FLAG_ROUND };
	app.vars[VAR_ID__RTVB_RT_CONFIG_ID] = (AppVariable) { "RTVB RT Config ID", 0, 0, 6, 1, VAR_FLAG_ROUND };
	app.vars[VAR_ID__RTVB_RT_COLOR_MODE] = (AppVariable) { "RTVB RT Color Mode", 0, 0, 1, 1, VAR_FLAG_ROUND };
	app.vars[VAR_ID__TIME_DILATION] = (AppVariable) { "Time Dilation", 1, 0, 1, 0.5f, VAR_FLAG_SMOOTH_EDIT | VAR_FLAG_STEP_PER_SECOND };
	app.vars[VAR_ID__MIP_LEVEL] = (AppVariable) { "Mip Level", 0, 0, 8, 1, 0 };
	app.vars[VAR_ID__FB_PSM] = (AppVariable) { "FB Format", GU_PSM_8888, 0, 3, 1, VAR_FLAG_ROUND };
	app.vars[VAR_ID__DITHER_GLOBAL] = (AppVariable) { "Dither Global", 0, 0, 1, 1, VAR_FLAG_ROUND };
	app.vars[VAR_ID__LIGHT_MODE] = (AppVariable) { "Light Mode", 0, 0, 1, 1, VAR_FLAG_ROUND };
	app.vars[VAR_ID__LAVA_ATTENUATION] = (AppVariable) { "Lava Attenuation", 1, 0, 100, 2.f, VAR_FLAG_SMOOTH_EDIT | VAR_FLAG_STEP_PER_SECOND };
	app.vars[VAR_ID__SPECULAR] = (AppVariable) { "Specular", 7, 0.001f, 100.f, 1.5f, VAR_FLAG_SMOOTH_EDIT | VAR_FLAG_STEP_PER_SECOND };
	app.vars[VAR_ID__BLOOM_THRESHOLD] = (AppVariable) { "Bloom Threshold", 210, 0.f, 255.f, 1.f, VAR_FLAG_ROUND };
	app.vars[VAR_ID__BLOOM_OPACITY] = (AppVariable) { "Bloom Opacity", 60, 0.f, 255.f, 1.f, VAR_FLAG_ROUND };
	app.debug_ui_enabled = true;

	app_init_fpu();
	psp_setup_callbacks();

	gumInit();

	app_gfx_allocate_vram_resources(&app.gfx, app.vars[VAR_ID__FB_PSM].value);

	// Note: this initializes global variables then clears the screen
	pspDebugScreenInitEx(NULL, app.gfx.framebuffers[0].psm, true /* Call sceDisplaySetMode() and sceDisplaySetFrameBuf() */);

	app_gfx_init(&app.gfx);

	app_assert(argc >= 1);
	chdir_to_assets_directory(argv[0]);

	app_assets_init(&app.assets);

	me_manager_init(&app.me_manager, "assets/mediaengine.prx");

	sceCtrlSetSamplingCycle(0); // Sync input sampling to VSync
	sceCtrlSetSamplingMode(PSP_CTRL_MODE_ANALOG);

	sceDisplayWaitVblankStart();
	sceGuDisplay(GU_TRUE);

	me_manager_test(&app.me_manager);

	app.scene.camera.post_processing.lut = LUT_IDENTITY;

	app.loop.real_time.last_frame_duration = 1.f / 60.f;
	app.loop.game_time.last_frame_duration = 1.f / 60.f;
	while (!g_exit_requested)
		app_frame(&app);

	me_manager_deinit(&app.me_manager);

	app_assets_deinit(&app.assets);

	sceGuTerm();

	sceKernelExitGame();
	return 0;
}