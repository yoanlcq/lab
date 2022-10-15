#include <pspkernel.h>
#include <pspdisplay.h>
#include <pspdebug.h>
#include <pspctrl.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <math.h>
#include <string.h>
#include <time.h>

#include <pspgu.h>

PSP_MODULE_INFO("Postprocessing Sample", 0, 1, 1);
PSP_MAIN_THREAD_ATTR(THREAD_ATTR_USER);

static unsigned int __attribute__((aligned(16))) list[262144];

struct Vertex
{
	float u,v;
	float x,y,z;
};

int SetupCallbacks();

#define BUF_WIDTH (512)
#define SCR_WIDTH (480)
#define SCR_HEIGHT (272)
#define PIXEL_SIZE (4) /* change this if you change to another screenmode */
#define FRAME_SIZE (BUF_WIDTH * SCR_HEIGHT * PIXEL_SIZE)
#define ZBUF_SIZE (BUF_WIDTH * SCR_HEIGHT * 2) /* zbuffer seems to be 16-bit? */

static void draw_fullscreen_quad(float u, float v) {
	struct Vertex* vertices = (struct Vertex*)sceGuGetMemory(2 * sizeof(struct Vertex));
	vertices[0].u = 1; vertices[0].v = 1; // Avoid slight wraparound in the top-left corner; GU_CLAMP doesn't seem to fix it
	vertices[0].x = 0; vertices[0].y = 0; vertices[0].z = 0;
	vertices[1].u = u; vertices[1].v = v;
	vertices[1].x = SCR_WIDTH; vertices[1].y = SCR_HEIGHT; vertices[1].z = 0;
	sceGuDrawArray(GU_SPRITES,GU_TEXTURE_32BITF|GU_VERTEX_32BITF|GU_TRANSFORM_2D,2,0,vertices);
}

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

unsigned int __attribute__((aligned(16))) clut256[4][256];
unsigned int __attribute__((aligned(16))) tex256[256*256];

int main(int argc, char* argv[])
{
	//
	// Things I'd like to try:
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
	// 2. Vertex format GU_NORMAL_8BIT | GU_VERTEX_8BIT
	// 2. Vertex format GU_COLOR_8888 | GU_NORMAL_8BIT | GU_VERTEX_8BIT

	SetupCallbacks();

	// initialize texture

	for (int y = 0; y < 256; ++y)
	{
		for (int x = 0; x < 256; ++x)
		{
			tex256[y * 256 + x] = GU_ABGR(0xff, 0xff, y, x);
		}
	}

	sceKernelDcacheWritebackAll();

	pspDebugScreenInit();

	// setup GU

	sceGuInit();
	sceGuStart(GU_DIRECT,list);

	void* fbp0 = NULL;
	void* fbp1 = (void*) FRAME_SIZE;
	(void) fbp1;
	sceGuDrawBuffer(GU_PSM_8888,fbp0,BUF_WIDTH);
	sceGuDispBuffer(SCR_WIDTH,SCR_HEIGHT,(void*)FRAME_SIZE,BUF_WIDTH);
	sceGuDepthBuffer((void*)(FRAME_SIZE*2),BUF_WIDTH);
	sceGuOffset(2048 - (SCR_WIDTH/2),2048 - (SCR_HEIGHT/2));
	sceGuViewport(2048,2048,SCR_WIDTH,SCR_HEIGHT);
	sceGuDepthRange(0xffff,0x0000);
	sceGuScissor(0,0,SCR_WIDTH,SCR_HEIGHT);
	sceGuEnable(GU_SCISSOR_TEST);
	sceGuFrontFace(GU_CW);
	sceGuEnable(GU_TEXTURE_2D);
	sceGuClear(GU_COLOR_BUFFER_BIT|GU_DEPTH_BUFFER_BIT);
	sceGuFinish();
	sceGuSync(0,0);

	sceDisplayWaitVblankStart();
	sceGuDisplay(GU_TRUE);

	SceCtrlData oldPad;
	oldPad.Buttons = 0;
	sceCtrlSetSamplingCycle(0);
	sceCtrlSetSamplingMode(0);

	// run sample

	unsigned frame_counter = 0;
	LUT lut = LUT_IDENTITY;
	LUTMode lut_mode = LUT_MODE_1_TO_1;

	bool use_framebuffer_as_texture = false;

	for(;;)
	{
		SceCtrlData pad;
		if(sceCtrlPeekBufferPositive(&pad, 1))
		{
			if (pad.Buttons != oldPad.Buttons)
			{
				if (pad.Buttons & PSP_CTRL_LTRIGGER)
					lut = (lut + LUT_COUNT - 1) % LUT_COUNT;
				if (pad.Buttons & PSP_CTRL_RTRIGGER)
					lut = (lut + 1) % LUT_COUNT;
				if (pad.Buttons & PSP_CTRL_CROSS)
					use_framebuffer_as_texture ^= 1;
			}
			oldPad = pad;
		}

		sceGuStart(GU_DIRECT,list);

		// animate palette

		unsigned int* pcr = (unsigned int*)(((unsigned int)clut256[0])|0x40000000);
		unsigned int* pcg = (unsigned int*)(((unsigned int)clut256[1])|0x40000000);
		unsigned int* pcb = (unsigned int*)(((unsigned int)clut256[2])|0x40000000);
		lut_mode = LUT_MODE_1_TO_1;
		switch (lut) {
		case LUT_IDENTITY: 
			for (int i = 0; i < 256; ++i)
			{
				pcr[i] = GU_ABGR(0xff, i, i, i);
			}
			break;
		case LUT_INVERT: 
			for (int i = 0; i < 256; ++i)
			{
				int x = 256 - i;
				pcr[i] = GU_ABGR(0xff, x, x, x);
			}
			break;
		case LUT_SRGB_TO_LINEAR: 
			for (int i = 0; i < 256; ++i)
			{
				const int x = 255 * powf(i / 255.f, 2.2f);
				pcr[i] = GU_ABGR(0xff, x, x, x);
			}
			break;
		case LUT_LINEAR_TO_SRGB: 
			for (int i = 0; i < 256; ++i)
			{
				const int x = 255 * powf(i / 255.f, 1.f / 2.2f);
				pcr[i] = GU_ABGR(0xff, x, x, x);
			}
			break;
		case LUT_SEPIA:
			lut_mode = LUT_MODE_3_TO_3;
			for (int i = 0; i < 256; ++i)
			{
				// outputRed   = (inputRed * .393) + (inputGreen * .769) + (inputBlue * .189)
				// outputGreen = (inputRed * .349) + (inputGreen * .686) + (inputBlue * .168)
				// outputBlue  = (inputRed * .272) + (inputGreen * .534) + (inputBlue * .131)
				pcr[i] = GU_ABGR(0xff, (unsigned) (.272f * i), (unsigned) (.349f * i), (unsigned) (.393f * i));
				pcg[i] = GU_ABGR(0xff, (unsigned) (.534f * i), (unsigned) (.686f * i), (unsigned) (.769f * i));
				pcb[i] = GU_ABGR(0xff, (unsigned) (.131f * i), (unsigned) (.168f * i), (unsigned) (.189f * i));
			}
			break;
		default:
			break;
		}

		// clear screen

		sceGuClearColor(GU_ABGR(0,0,0,0));
		sceGuClear(GU_COLOR_BUFFER_BIT);

		// setup CLUT texture

		sceGuTexMode(GU_PSM_T32,0,0,0);
		sceGuTexImage(0,256,256,256,tex256);
		sceGuTexFunc(GU_TFX_REPLACE,GU_TCC_RGB);
		sceGuTexFilter(GU_LINEAR,GU_LINEAR);
		sceGuTexWrap(GU_CLAMP, GU_CLAMP);
		sceGuTexScale(1.0f,1.0f);
		sceGuTexOffset(0.0f,0.0f);
		sceGuAmbientColor(0xffffffff);

		// render sprite

		sceGuColor(0xffffffff);

		float fu = 256, fv = 256;
		if (use_framebuffer_as_texture) {
			sceGuTexMode(GU_PSM_8888,0,0,0);
			sceGuTexImage(0,256,256,256,tex256);
			draw_fullscreen_quad(256, 256);

			sceGuTexMode(GU_PSM_T32,0,0,0);
			// Size of textures must be power of two; if they're not, they'll be converted to the next lower power of two value.
			// So we have to pretend our texture is larger, it's the UVs that will be responsible for preventing invalid memory accesses.
			sceGuTexImage(0,512,512,BUF_WIDTH, (char*) sceGeEdramGetAddr() + (uintptr_t) fbp0);
			fu = SCR_WIDTH;
			fv = SCR_HEIGHT;
		}

		switch (lut_mode) {
		case LUT_MODE_1_TO_1:
			sceGuClutLoad((256/8),clut256[0]); // upload 32*8 entries (256)
			for (int i = 0; i < 3; ++i) {
				sceGuClutMode(GU_PSM_8888,i*8,0xff,0); // 32-bit palette
				sceGuPixelMask(~(0xffu << (i*8)));
				draw_fullscreen_quad(fu, fv);
			}
			sceGuPixelMask(0);
			break;
		case LUT_MODE_3_TO_3:
			// TODO: Requires ping-ponging if using the FB directly as source
			sceGuEnable(GU_BLEND);
			sceGuBlendFunc(GU_ADD, GU_FIX, GU_FIX, 0xffffffff, 0xffffffff);
			for (int i = 0; i < 3; ++i) {
				sceGuClutMode(GU_PSM_8888,i*8,0xff,0); // 32-bit palette
				sceGuClutLoad((256/8),clut256[i]); // upload 32*8 entries (256)
				draw_fullscreen_quad(fu, fv);
			}
			sceGuDisable(GU_BLEND);
			break;
		default:
			break;
		}

		// wait for next frame

		sceGuFinish();
		sceGuSync(0,0);

		pspDebugScreenSetOffset((int)fbp0);
		pspDebugScreenSetXY(4,16);
		pspDebugScreenPrintf("LUT (cycle via L/R): %s", lut_get_name(lut));
		pspDebugScreenSetXY(4,17);
		pspDebugScreenPrintf("%s (toggle via X)", use_framebuffer_as_texture ? "Using FB as texture" : "Not using FB as texture");

		sceDisplayWaitVblankStart();
		fbp1 = fbp0;
		fbp0 = sceGuSwapBuffers();

		frame_counter++;
	}

	sceGuTerm();

	sceKernelExitGame();
	return 0;
}

/* Exit callback */
int exit_callback(int arg1, int arg2, void *common)
{
	sceKernelExitGame();
	return 0;
}

/* Callback thread */
int CallbackThread(SceSize args, void *argp)
{
	int cbid;

	cbid = sceKernelCreateCallback("Exit Callback", exit_callback, NULL);
	sceKernelRegisterExitCallback(cbid);

	sceKernelSleepThreadCB();

	return 0;
}

/* Sets up the callback thread and returns its thread id */
int SetupCallbacks(void)
{
	int thid = 0;

	thid = sceKernelCreateThread("update_thread", CallbackThread, 0x18, 0xFA0, 0, 0);
	if(thid >= 0)
	{
		sceKernelStartThread(thid, 0, 0);
	}

	return thid;
}
