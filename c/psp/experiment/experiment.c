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
// 2. Vertex_UVf32_XYZf32 format GU_NORMAL_8BIT | GU_VERTEX_8BIT
// 3. Vertex_UVf32_XYZf32 format GU_COLOR_8888 | GU_NORMAL_8BIT | GU_VERTEX_8BIT
//

#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#include <pspctrl.h>
#include <pspdebug.h>
#include <pspdisplay.h>
#include <pspgu.h>
#include <pspkernel.h>
#include <psprtc.h>

//
//
// Foundation
//
//

typedef float f32;
typedef double f64;

#define ALIGN_N(x) __attribute__((aligned(x)))
#define ALIGN16 ALIGN_N(16)

static inline void* psp_uncached_ptr(void* p) {
	return (void*) (((uintptr_t)p) | 0x40000000ul);
}

static inline u32 u32_popcount(u32 x) {
	return __builtin_popcount(x);
}

static inline bool u32_is_power_of_two_nonzero(u32 x) {
	return u32_popcount(x) == 1;
}

static inline bool u32_is_power_of_two_or_zero(u32 x) {
	return u32_popcount(x) <= 1;
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
	case GU_PSM_DXT1: return 0;
	case GU_PSM_DXT3: return 0;
	case GU_PSM_DXT5: return 0;
	default: return 0;
	}
}

size_t gu_psm_get_bytes_per_pixel(int psm) {
	return gu_psm_get_bits_per_pixel(psm) / 8;
}

typedef struct {
	TickRange cpu, gpu;
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

typedef struct {
	f32 uv[2];
	f32 position[3];
} Vertex_UVf32_XYZf32;

#define Vertex_UVf32_XYZf32_FORMAT (GU_TEXTURE_32BITF | GU_VERTEX_32BITF)

void gu_draw_fullscreen_quad(f32 uv0, f32 uv1) {
	Vertex_UVf32_XYZf32* v = sceGuGetMemory(2 * sizeof(Vertex_UVf32_XYZf32));
	v[0] = (Vertex_UVf32_XYZf32) {
		.uv = { 1, 1 }, // UV is 1 rather than 0; avoids slight wraparound in the top-left corner; GU_CLAMP doesn't seem to fix it
		.position = { 0, 0, 0 },
	};
	v[1] = (Vertex_UVf32_XYZf32) {
		.uv = { uv0, uv1 },
		.position = { PSP_SCREEN_WIDTH, PSP_SCREEN_HEIGHT, 0 },
	};
	sceGuDrawArray(GU_SPRITES, Vertex_UVf32_XYZf32_FORMAT | GU_TRANSFORM_2D, 2, 0, v);
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
		assert(u32_is_power_of_two_nonzero(m->stride_px));
		assert(m->data);
	} else {
		assert(u32_is_power_of_two_or_zero(m->stride_px));
	}
}

void texture_check_as_input(const Texture* m) {
	texture_check_common(m);
	assert(u32_is_power_of_two_or_zero(m->size_px[0]));
	assert(u32_is_power_of_two_or_zero(m->size_px[1]));
}

void texture_check_as_rendertarget(const Texture* m) {
	texture_check_common(m);
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


u32 ALIGN16 g_gu_main_list[256 * 1024] = {0}; // Zeroing should not be necessary, but samples declare it as static, which zeroes it, so...

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
// Unsorted
//
//

u32 ALIGN16 g_clut[4][256];
u32 ALIGN16 g_test_texture_data[256 * 256];
Texture g_test_texture = {
	.psm = GU_PSM_8888,
	.data = g_test_texture_data,
	.size_px = { 256, 256 },
	.stride_px = 256,
	.nb_mipmap_levels = 1,
	.is_swizzled = false,
};

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

	psp_setup_callbacks();

	for (int y = 0; y < 256; ++y)
		for (int x = 0; x < 256; ++x)
			g_test_texture_data[y * 256 + x] = GU_ABGR(0xff, 0xff, y, x);

	sceKernelDcacheWritebackRange(g_test_texture_data, sizeof g_test_texture_data);

	pspDebugScreenInit();

	sceGuInit();
	sceGuStart(GU_DIRECT, g_gu_main_list);

	sceGuDrawBuffer(fb_psm, fbp0, PSP_SCREEN_STRIDE);
	sceGuDispBuffer(PSP_SCREEN_WIDTH, PSP_SCREEN_HEIGHT, fbp1, PSP_SCREEN_STRIDE);
	sceGuDepthBuffer(zb, PSP_SCREEN_STRIDE);

	gu_set_offset_and_viewport_and_scissor(PSP_SCREEN_WIDTH, PSP_SCREEN_HEIGHT);

	sceGuDepthRange(0xffff, 0x0000);

	sceGuEnable(GU_SCISSOR_TEST);
	sceGuEnable(GU_TEXTURE_2D);

	sceGuFrontFace(GU_CW); // TODO: use CCW instead? It's the OpenGL convention

	sceGuSetCallback(GU_CALLBACK_SIGNAL, gu_on_signal);

	sceGuClear(GU_COLOR_BUFFER_BIT | GU_DEPTH_BUFFER_BIT);
	sceGuFinish();
	sceGuSync(GU_SYNC_FINISH, GU_SYNC_WHAT_DONE);

	sceDisplayWaitVblankStart();
	sceGuDisplay(GU_TRUE);

	SceCtrlData previous_pad = {0};
	sceCtrlSetSamplingCycle(0); // Sync input sampling to VSync
	sceCtrlSetSamplingMode(PSP_CTRL_MODE_ANALOG);

	LUT lut = LUT_IDENTITY;
	LUTMode lut_mode = LUT_MODE_1_TO_1;

	bool use_framebuffer_as_texture = false;

	while (!g_exit_requested) {
		psp_rtc_get_current_tick_sync(&g_frame_stats.cpu.start);

		SceCtrlData pad;
		if (sceCtrlPeekBufferPositive(&pad, 1)) {
			if (pad.Buttons != previous_pad.Buttons) {
				if (pad.Buttons & PSP_CTRL_LTRIGGER)
					lut = (lut + LUT_COUNT - 1) % LUT_COUNT;
				if (pad.Buttons & PSP_CTRL_RTRIGGER)
					lut = (lut + 1) % LUT_COUNT;
				if (pad.Buttons & PSP_CTRL_CROSS)
					use_framebuffer_as_texture ^= 1;
			}
			previous_pad = pad;
		}

		u32* pcr = psp_uncached_ptr(g_clut[0]);
		u32* pcg = psp_uncached_ptr(g_clut[1]);
		u32* pcb = psp_uncached_ptr(g_clut[2]);
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

		sceGuStart(GU_DIRECT, g_gu_main_list);
		gu_insert_clock_start_marker();

		sceGuClearColor(GU_ABGR(0,0,0,0));
		sceGuClearDepth(0);
		sceGuClear(GU_COLOR_BUFFER_BIT | GU_DEPTH_BUFFER_BIT);

		Texture test_texture_t32 = g_test_texture;
		test_texture_t32.psm = GU_PSM_T32;
		gu_set_texture(&test_texture_t32);

		sceGuTexFunc(GU_TFX_REPLACE, GU_TCC_RGB);
		sceGuTexFilter(GU_LINEAR, GU_LINEAR);
		sceGuTexWrap(GU_CLAMP, GU_CLAMP);

		sceGuAmbientColor(0xffffffffu);
		sceGuColor(0xffffffffu);

		f32 fu = g_test_texture.size_px[0];
		f32 fv = g_test_texture.size_px[1];
		if (use_framebuffer_as_texture) {
			Texture rendertarget = {
				.psm = fb_psm,
				.nb_mipmap_levels = 1,
				.size_px = { PSP_SCREEN_WIDTH, PSP_SCREEN_HEIGHT },
				.stride_px = PSP_SCREEN_STRIDE,
				.data = (u8*) sceGeEdramGetAddr() + (uintptr_t) fbp2,
			};
			gu_set_rendertarget(&rendertarget);

			gu_set_texture(&g_test_texture);
			gu_draw_fullscreen_quad(g_test_texture.size_px[0], g_test_texture.size_px[1]);

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
			sceGuClutLoad(256 / 8, g_clut[0]); // upload 32*8 entries (256)
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
				sceGuClutLoad(256 / 8, g_clut[i]); // upload 32*8 entries (256)
				gu_draw_fullscreen_quad(fu, fv);
			}
			sceGuDisable(GU_BLEND);
			break;
		default:
			break;
		}

		psp_rtc_get_current_tick_sync(&g_frame_stats.cpu.end);
		gu_insert_clock_end_marker();
		sceGuFinish();
		sceGuSync(GU_SYNC_FINISH, GU_SYNC_WHAT_DONE);

		int debug_screen_pos[2] = { 4, 16 };
		pspDebugScreenSetOffset((intptr_t)fbp0);
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("LUT (cycle via L/R): %s", lut_get_name(lut));
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("%s (toggle via X)", use_framebuffer_as_texture ? "Using FB as texture" : "Not using FB as texture");
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("CPU: %.3f ms", 1000.0 * (f64) tick_range_get_duration(g_frame_stats.cpu));
		pspDebugScreenSetXY(debug_screen_pos[0], debug_screen_pos[1]++);
		pspDebugScreenPrintf("GPU: %.3f ms", 1000.0 * (f64) tick_range_get_duration(g_frame_stats.gpu));

		sceDisplayWaitVblankStart();
		fbp1 = fbp0;
		fbp0 = sceGuSwapBuffers();

		g_frame_counter++;
	}

	sceGuTerm();

	sceKernelExitGame();
	return 0;
}