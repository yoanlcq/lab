#include "egi_demo.h"

#include "debug_ui.h"
#include "editor.h"
#include "camera.h"
#include "hf_gxf.h"

#include <vx/geom/xform.h>
#include <vx/gx/debugdraw.h>
#include <vx/gx/debugdraw_tangentspace.h>
#include <vx/gx/gx_mesh.h>
#include <vx/gx/gxf.h>
#include <vx/math/vmath.h>
#include <vx/mem/fixed.h>
#include <vx/mem/scratch.h>
#include <vx/misc/blender_conversions.h>
#include <vx/os/time.h>
#include <vx/res/stock.h>

// TODO: Divide by the sum of all view factors??
// TODO: specular with ggx

// # REFERENCE & FIDELITY
// - Have a reference. Possibly use the Sponza scene and compare results with Blender render.
// - Review the view factor approximation;
// - Support a PBR model for surfels;
// - Support extended material properties (e.g reflectivity, roughness, etc);
//
// # INTEGRATION
// - Integrate the GI result into final rendering; treat it as set of directional lights.
//   - Per-pixel GI (expensive but good-looking) ? i.e per fragment, perform one bounce of the simulation.
//   - Per fragment, interpolate the "light-received" value of relevant agents of that fragment's surface.
//
// # REAL TIME
// - Partially solve the simulation each frame (i.e progressivesolving);
// - Build a hierarchy of agents (possibly via mipmap generation);
// - Multi-thread the simulation;
// - Perform simulation on compute shaders;
// - Cache and reuse the result of static-vs-static interactions after N bounces.
// - Evaluate the "density" (LOD) of agents in a given world-space area, in order to control the LOD.
//
// # MISC
// - joint matrices: use UBOs instead of array.
// - Have special support for basic shapes, such as spheres and skyboxes;
// - Implement AI for agents;
// - Support "double-sided" meshes, with 1 agent per vertex instead of 2 due to both faces
// - Scale surfel areas when objects are scaled (i.e convert area to radius, scale radius, then convert radius back to area)

static void set_camera_to_match_reference() {
    const Xform xform = {
        .position = v3_blender_xyz_to_lh(v3_set(16.f, 3.5f, 7.f)),
        .orientation = quat_rotation_blender_xyz_euler_degrees_camera_to_lh(v3_set(75.5f, 0.f, 105.f)),
        .scale = v3_one(),
    };
    g_current_camera->xform = xform;
    g_current_camera->fov_y = blender_fov_degrees_to_fov_y(66.f, 1920.f / 1080.f);
}


static f32 compute_triangle_area(const v3* v) {
    const f32 a = v3_magnitude(v[0] - v[1]);
    const f32 b = v3_magnitude(v[1] - v[2]);
    const f32 c = v3_magnitude(v[2] - v[0]);
    const f32 s = (a + b + c) / 2.f;
    return sqrtf(s * (s-a) * (s-b) * (s-c));
}


// Result of:
//     m4_mul(m4_rotation_angle_axis(-PI_4, v3_010()), m4_rotation_angle_axis(PI_4, v3_100()));
// This matrix is hardcoded in EGI shaders.
static const m4 TANGENTSPACE_TO_IRRADIANCE_SAMPLING_DIRECTION_MATRIX = { .cols = {
    { 0.707106769, 0, 0.707106769, 0 },
    { -0.49999997, 0.707106769, 0.49999997, 0 },
    { -0.49999997, -0.707106769, 0.49999997, 0 },
    { 0, 0, 0, 1 },
} };

typedef struct EgiDemoAgent EgiDemoAgent;
typedef struct EgiDemoObject EgiDemoObject;

struct EgiDemoObject {
    const char* name;
    v4 base_color;
    v3 emissive;
    f32 metallic;
    f32 roughness;
    bool use_debug_color;
    bool skip_drawing;
    bool is_convex; // If true, then agents of that object are assumed to have zero influence on other agents of that object.
    const GxCpuMesh* cpu_mesh;
    GxGpuMesh gpu_mesh;
    GxBuffer extra_vbo;
    Xform xform;
    EgiDemoAgent* agents;
    u32 nb_agents;
};

struct EgiDemoAgent {
    EgiDemoObject* obj;
    v3 position; // world-space.
    v3 normal; // world-space.
    v4 tangent; // world-space.
    v4 base_color;
    v3 base_emissive;
    f32 metallic;
    f32 roughness;
    // NOTE: Be careful with how the area should scale depending on the model transform...
    f32 area; // Disk area, given by the sum of one-third of the area of the triangles that share the vertex. PERF: Should be stored as 4th component of a vector, for memory efficiency.
    v3 shadow_intensity;
    //
    v4 irradiance; // Irradiance received by the surfel, from the hemisphere around its normal. FIXME: Assumes we don't use normal maps...
    v4 irradiance_accumulator;
    //
    // Reflected light as seen by the camera, depending on the roughness. There are multiple issues:
    // - We don't support multiple bounces of reflection;
    // - We assume the roughness doesn't vary much per pixel.
    v4 reflected_light_for_camera_view;
    v4 reflected_light_for_camera_view_accumulator; // Not needed in the final implementation. Only useful for debugging when we re-upload colors
};

typedef struct  {
    f32 a_irradiance[3];
    f32 a_reflected_light_for_camera_view[3];
} AgentExtraVertexData;

static const GxVAttrib s_extra_vbo_attribs[] = {
    { .id = { .semantic = GX_VA_IRRADIANCE_SAMPLES, .i = 0 }, .layout = { .start = offsetof(AgentExtraVertexData, a_irradiance), .stride = sizeof(AgentExtraVertexData), .xtype = { .dim = 3, .type = TY_F32, .aggregate = TY_VECTOR } } },
    { .id = { .semantic = GX_VA_IRRADIANCE_SAMPLES, .i = 3 }, .layout = { .start = offsetof(AgentExtraVertexData, a_reflected_light_for_camera_view), .stride = sizeof(AgentExtraVertexData), .xtype = { .dim = 3, .type = TY_F32, .aggregate = TY_VECTOR } } },
};
static const GxVertexLayout s_extra_vbo_layout = {
    .attribs = s_extra_vbo_attribs,
    .nb_attribs = countof(s_extra_vbo_attribs),
};

typedef struct {
    GxBuffer gpu_buffer;
    const GxVertexLayout* layout;
    const GxVertexLayout* layout_notangent;
    bool should_draw;
    bool use_irradiance_sampling_transform;
    v3 gizmo_offset;
    v3 gizmo_scale;
} EgiTbnManager;

typedef struct {
    f32 position[3];
    f32 normal[3];
    f32 tangent[4];
} EgiAgentTbn;

//
static EgiDemoObject* s_objects = NULL;
static u32 s_nb_objects = 0;
static const u32 s_max_objects = 32;
//
static EgiDemoAgent* s_agents = NULL;
static u32 s_nb_agents = 0;
static const u32 s_max_agents = 100000;
//
static GxBuffer s_instanced_model_matrix_gpu_buffer = {0};
static GxBuffer s_instanced_color_gpu_buffer = {0};
//
static f32 s_disk_debugdraw_offset_from_surface = 0.01f;
static f32 s_disk_debugdraw_radius_scale = 1.f;
//
static bool s_skip_surfel_drawing = true;
static bool s_skip_object_drawing = false;
static bool s_use_debug_color = false;
//
static const f32 s_max_pos = 16.f;
static const f32 s_min_scale = 0.01f;
static const f32 s_max_scale = 40.0f;
//
static f32 s_debugdraw_color_factor = 1.f;
//
static f32 s_irradiance_viewfactor_exp = 1.4f;
static f32 s_focus_exp = 1.f;
static f32 s_refl_exp0 = 2.3f;
static f32 s_refl_exp1 = 1.f;
static bool s_should_average_irradiance = true;
static bool s_should_average_reflected = true;
static f32 s_irradiance_intensity_factor = 1.f;
static v3 s_shadow_intensity_factor = { 1.f, 1.f, 1.f };
//
static const GxVAttrib s_tbn_attribs[] = {
    { .id = { .semantic = GX_VA_POSITION, .i = 0 }, .layout = { .start = offsetof(EgiAgentTbn, position), .stride = sizeof(EgiAgentTbn), .xtype = { .type = TY_F32, .dim = 3, .aggregate = TY_VECTOR } } },
    { .id = { .semantic = GX_VA_NORMAL, .i = 0 }, .layout = { .start = offsetof(EgiAgentTbn, normal), .stride = sizeof(EgiAgentTbn), .xtype = { .type = TY_F32, .dim = 3, .aggregate = TY_VECTOR } } },
    { .id = { .semantic = GX_VA_TANGENT, .i = 0 }, .layout = { .start = offsetof(EgiAgentTbn, tangent), .stride = sizeof(EgiAgentTbn), .xtype = { .type = TY_F32, .dim = 4, .aggregate = TY_VECTOR } } },
};
static const GxVertexLayout s_tbn_layout = {
    .attribs = s_tbn_attribs,
    .nb_attribs = countof(s_tbn_attribs),
};
static const GxVertexLayout s_tbn_layout_notangent = {
    .attribs = s_tbn_attribs,
    .nb_attribs = countof(s_tbn_attribs)-1,
};
static EgiTbnManager s_tbn = {
    .layout = &s_tbn_layout,
    .layout_notangent = &s_tbn_layout_notangent,
    .should_draw = false,
    .gizmo_scale = { 1, 1, 1 },
    .gizmo_offset = { 0, 0, 0.02f },
};

//
//

typedef enum {
    AGENT_INVALID_COLOR_ID = 0,
    AGENT_BASE_COLOR,
    AGENT_BASE_EMISSIVE,
    AGENT_IRRADIANCE,
    AGENT_REFLECTED_LIGHT_FOR_CAMERA_VIEW,
    AGENT_COLOR_ID_COUNT, // Keep last
} AgentColorID;

static AgentColorID s_color_id = AGENT_IRRADIANCE;

static const char* agent_color_id_to_string(AgentColorID id) {
    switch (id) {
    case AGENT_BASE_COLOR: return "BASE_COLOR";
    case AGENT_BASE_EMISSIVE: return "BASE_EMISSIVE";
    case AGENT_IRRADIANCE: return "IRRADIANCE";
    case AGENT_REFLECTED_LIGHT_FOR_CAMERA_VIEW: return "REFLECTED_LIGHT_FOR_CAMERA_VIEW";
    default: break;
    }
    assert_soft(0);
    return NULL;
}

static v4 get_agent_color(const EgiDemoAgent* agent, AgentColorID id) {
    switch (id) {
    case AGENT_BASE_COLOR: return agent->base_color;
    case AGENT_BASE_EMISSIVE: return v4_point(agent->base_emissive);
    case AGENT_IRRADIANCE: return v4_point(agent->irradiance);
    case AGENT_REFLECTED_LIGHT_FOR_CAMERA_VIEW: return v4_point(agent->reflected_light_for_camera_view);
    default: break;
    }
    assert_soft(0);
    return v4_zero();
}

//
//
//

static m4 get_agent_model_matrix(const EgiDemoAgent* agent) {
    const Xform xform = {
        .position = agent->position + agent->normal * s_disk_debugdraw_offset_from_surface,
        .orientation = quat_rotation_from_to(-v3_001(), agent->normal),
        .scale = v3_broadcast(2.f * sqrtf(agent->area / PI) * s_disk_debugdraw_radius_scale), // Mul by 2 because our stock disk radius is 0.5f
    };
    const m4 model_matrix = xform_get_model_matrix(xform);
    return model_matrix;
}

static void upload_instanced_model_matrices(u32 first_agent, u32 nb) {
    void* vptr = gx_map_buffer(s_instanced_model_matrix_gpu_buffer, first_agent * sizeof(m4), nb * sizeof(m4), GX_MAP_WRITE_BIT | GX_MAP_INVALIDATE_RANGE_BIT);
    for (u32 i = 0; i < nb; ++i) {
        const EgiDemoAgent* agent = &s_agents[first_agent + i];
        const m4 model_matrix = get_agent_model_matrix(agent);
        memcpy(vptr + i * sizeof(m4), m4_cols_ptr_const(&model_matrix), sizeof(m4));
    }
    gx_unmap_buffer(s_instanced_model_matrix_gpu_buffer);
}

static void upload_instanced_colors(u32 first_agent, u32 nb) {
    void* vptr = gx_map_buffer(s_instanced_color_gpu_buffer, first_agent * sizeof(v4), nb * sizeof(v4), GX_MAP_WRITE_BIT | GX_MAP_INVALIDATE_RANGE_BIT);
    for (u32 i = 0; i < nb; ++i) {
        const EgiDemoAgent* agent = &s_agents[first_agent + i];
        const v4 rgba = get_agent_color(agent, s_color_id) * v4_point(v3_broadcast(s_debugdraw_color_factor));
        memcpy(vptr + i * sizeof(v4), v4_ptr_const(&rgba), sizeof(v4));
    }
    gx_unmap_buffer(s_instanced_color_gpu_buffer);
}

static void upload_tbn(u32 first_agent, u32 nb) {
    void* vptr = gx_map_buffer(s_tbn.gpu_buffer, first_agent * sizeof(EgiAgentTbn), nb * sizeof(EgiAgentTbn), GX_MAP_WRITE_BIT | GX_MAP_INVALIDATE_RANGE_BIT);
    for (u32 i = 0; i < nb; ++i) {
        const EgiDemoAgent* agent = &s_agents[first_agent + i];
        EgiAgentTbn data;
        v3_store(data.position, agent->position);
        v3_store(data.normal, agent->normal);
        v4_store(data.tangent, agent->tangent);
        memcpy(vptr + i * sizeof data, &data, sizeof data);
    }
    gx_unmap_buffer(s_tbn.gpu_buffer);
}

static void upload_object_light_received(const EgiDemoObject* obj) {
    assert_fatal(obj->nb_agents == obj->cpu_mesh->nb_vertices);
    void* vptr = gx_map_buffer(obj->extra_vbo, 0, obj->cpu_mesh->nb_vertices * sizeof(AgentExtraVertexData), GX_MAP_WRITE_BIT | GX_MAP_INVALIDATE_RANGE_BIT);
    for (u32 i = 0; i < obj->nb_agents; ++i) {
        const EgiDemoAgent* agent = &obj->agents[i];
        AgentExtraVertexData data;
        v3_store(data.a_irradiance, agent->irradiance);
        v3_store(data.a_reflected_light_for_camera_view, agent->reflected_light_for_camera_view);
        memcpy(vptr + i * sizeof data, &data, sizeof data);
    }
    gx_unmap_buffer(obj->extra_vbo);
}

//
//
//
//

static void draw_egi_demo_object(GxfiScene* s, const EgiDemoObject* obj) {
    if (s_skip_object_drawing || obj->skip_drawing)
        return;

    //
    //
    //
    const u32 nb_buffers = 1;
    GxStructuredBuffer* sbs = gxf_alloc(nb_buffers * sizeof(GxStructuredBuffer), alignof(GxStructuredBuffer), "");
    sbs[0].buffer = obj->extra_vbo;
    sbs[0].layout = gx_intern_vertex_layout(&s_extra_vbo_layout);

    //
    //
    //
    GxMaterial* material = gxf_alloc(sizeof(GxMaterial), alignof(GxMaterial), "");
    *material = GX_DEFAULT_MATERIAL;

    material->shading.shading = GX_EGI_PBR_SHADING;

    material->lighting.is_shadow_caster = false;
    material->lighting.is_shadow_receiver = false;
    material->lighting.lighting_enabled = false;

    v4_store(material->base_color.factor, obj->base_color);
    v3_store(material->emissive.factor, obj->emissive);

    material->metallic_roughness.metallic_factor = obj->metallic;
    material->metallic_roughness.roughness_factor = obj->roughness;

    const GxfMeshInstance mi = {
        .base = {
            .model_matrix = xform_get_model_matrix(obj->xform),
            .name = obj->name,
            .structured_buffers = sbs,
            .nb_structured_buffers = nb_buffers,
        },
        .material = material,
        .mesh = obj->gpu_mesh,
    };
    gxf_add_mesh(s, &mi);
}

static void draw_all_egi_demo_agents(GxfiScene* s, u32 nb_agents) {
    if (s_skip_surfel_drawing)
        return;

    static const GxAttribDivisor s_attrib_divisors[] = {
        { .id = { .semantic = GX_VA_MODEL_MATRIX_COL, .i = 0 }, .divisor = 1, },
        { .id = { .semantic = GX_VA_MODEL_MATRIX_COL, .i = 1 }, .divisor = 1, },
        { .id = { .semantic = GX_VA_MODEL_MATRIX_COL, .i = 2 }, .divisor = 1, },
        { .id = { .semantic = GX_VA_MODEL_MATRIX_COL, .i = 3 }, .divisor = 1, },
        { .id = { .semantic = GX_VA_COLOR_INSTANCED , .i = 0 }, .divisor = 1, },
    };
    const GxVAttrib instanced_model_matrix_attribs[] = {
        { .id = { .semantic = GX_VA_MODEL_MATRIX_COL, .i = 0 }, .layout = { .start = 0*sizeof(v4), .stride = 4*sizeof(v4), .xtype = { .dim = 4, .type = TY_F32, .aggregate = TY_VECTOR } } },
        { .id = { .semantic = GX_VA_MODEL_MATRIX_COL, .i = 1 }, .layout = { .start = 1*sizeof(v4), .stride = 4*sizeof(v4), .xtype = { .dim = 4, .type = TY_F32, .aggregate = TY_VECTOR } } },
        { .id = { .semantic = GX_VA_MODEL_MATRIX_COL, .i = 2 }, .layout = { .start = 2*sizeof(v4), .stride = 4*sizeof(v4), .xtype = { .dim = 4, .type = TY_F32, .aggregate = TY_VECTOR } } },
        { .id = { .semantic = GX_VA_MODEL_MATRIX_COL, .i = 3 }, .layout = { .start = 3*sizeof(v4), .stride = 4*sizeof(v4), .xtype = { .dim = 4, .type = TY_F32, .aggregate = TY_VECTOR } } },
    };
    const GxVertexLayout instanced_model_matrix_layout = {
        .attribs = instanced_model_matrix_attribs,
        .nb_attribs = countof(instanced_model_matrix_attribs),
    };

    const GxVAttrib instanced_color_attribs[] = {
        { .id = { .semantic = GX_VA_COLOR_INSTANCED , .i = 0 }, .layout = { .start = 0*sizeof(v4), .stride = 1*sizeof(v4), .xtype = { .dim = 4, .type = TY_F32, .aggregate = TY_VECTOR } } },
    };
    const GxVertexLayout instanced_color_layout = {
        .attribs = instanced_color_attribs,
        .nb_attribs = countof(instanced_color_attribs),
    };

    GxMaterial* material = gxf_alloc(sizeof(GxMaterial), alignof(GxMaterial), "");
    *material = *debugdraw_get_default_material();
    material->shading.shading = GX_DEBUGDRAW_INSTANCED_SHADING;

    const u32 nb_buffers = 2;
    GxStructuredBuffer* sbs = gxf_alloc(nb_buffers * sizeof(GxStructuredBuffer), alignof(GxStructuredBuffer), "");
    sbs[0].buffer = s_instanced_model_matrix_gpu_buffer;
    sbs[0].layout = gx_intern_vertex_layout(&instanced_model_matrix_layout);
    sbs[1].buffer = s_instanced_color_gpu_buffer;
    sbs[1].layout = gx_intern_vertex_layout(&instanced_color_layout);

    GxfMeshInstance mi = {0};
    mi.base.name = "EGI agents";
    mi.base.model_matrix = m4_identity();
    mi.base.nb_instances = nb_agents;
    mi.base.structured_buffers = sbs;
    mi.base.nb_structured_buffers = nb_buffers;
    mi.base.attrib_divisors = s_attrib_divisors;
    mi.base.nb_attrib_divisors = countof(s_attrib_divisors);
    mi.material = material;
    mi.mesh = get_stock_mesh(STOCK_MESH_DISK_NP_LOD0);
    gxf_add_mesh(s, &mi);
}

static void draw_tbn(GxfiScene* s, u32 nb_agents) {
    if (!s_tbn.should_draw)
        return;

    const m4 m = s_tbn.use_irradiance_sampling_transform ? TANGENTSPACE_TO_IRRADIANCE_SAMPLING_DIRECTION_MATRIX : m4_identity();

    const GxGpuMeshInterface iface = {
        .vertex_buffer = s_tbn.gpu_buffer,
        .vertex_layout = gx_intern_vertex_layout(s_tbn.layout),
        .nb_vertices = nb_agents,
    };
    const DebugDrawAddTangentSpaceBatchParams ddp = {
        .gpu_mesh_representation_for_normal = get_stock_mesh(STOCK_MESH_ZAXIS_CP),
        .gpu_mesh_representation_for_normal_tangent = get_stock_mesh(STOCK_MESH_XYZ_CP),
        .source_mesh_interface = &iface,
        .model_matrix = m4_identity(),
        .bind_shape_matrix = m4_mul(m4_translation(s_tbn.gizmo_offset), m4_mul(m4_scaling(s_tbn.gizmo_scale), m)),
        .name = "EGI: TBN batch",
    };
    debugdraw_add_tangentspace_batch(s, &ddp);
}

//
//
//
//
//

// The total number of vertices is given by (w+1) * (h+1).
// The total number of indices is given by w * h * 6.
static void build_2d_grid_gnp(u32 w, u32 h, VertexGNP* v, u16* indices) {
    u32 nb_vertices = 0;
    u32 nb_indices = 0;

    const v3 normal = -v3_001();
    const v4 tangent = v4_1000() - v4_0001();
    // const v3 bitangent = v3_cross(normal, tangent) * tangent[3];

    for (u32 yi = 0; yi <= h; ++yi)
    for (u32 xi = 0; xi <= w; ++xi) {
        const f32 x = lerpf(-0.5f, 0.5f, xi / (f32) w);
        const f32 y = lerpf(-0.5f, 0.5f, yi / (f32) h);

        v3_store(v[nb_vertices].position, v3_set(x, y, 0.f));
        v3_store(v[nb_vertices].normal, normal);
        v4_store(v[nb_vertices].tangent, tangent);
        nb_vertices++;
    }

    for (u32 yi = 0; yi < h; ++yi)
    for (u32 xi = 0; xi < w; ++xi) {
        const u16 tl = (yi+0) * (w+1) + (xi+0);
        const u16 tr = (yi+0) * (w+1) + (xi+1);
        const u16 bl = (yi+1) * (w+1) + (xi+0);
        const u16 br = (yi+1) * (w+1) + (xi+1);

        indices[nb_indices++] = bl;
        indices[nb_indices++] = tl;
        indices[nb_indices++] = tr;
        indices[nb_indices++] = tr;
        indices[nb_indices++] = br;
        indices[nb_indices++] = bl;
    }
}

static GxCpuMesh s_test_grid_cpu_mesh[7];
static GxGpuMesh s_test_grid_gpu_mesh[7];
static GxCpuMesh s_test_box_cpu_mesh[6];
static GxGpuMesh s_test_box_gpu_mesh[6];
static GxCpuMesh s_test_skybox_cpu_mesh[6];
static GxGpuMesh s_test_skybox_gpu_mesh[6];

static void init_test_grid_mesh() {
    for (u32 i = 0; i < countof(s_test_grid_cpu_mesh); ++i) {
        const u32 w = 1 << i;
        const u32 h = 1 << i;
        const u32 nb_vertices = (w+1) * (h+1);
        const u32 nb_indices = w * h * 6;
        assert_fatal(nb_indices <= UINT16_MAX);

        VertexGNP* vertices = fixed_alloc_uninitialized(nb_vertices * sizeof *vertices, 16, "");
        u16* indices = fixed_alloc_uninitialized(nb_indices * sizeof *indices, 16, "");

        build_2d_grid_gnp(w, h, vertices, indices);

        const GxCpuMesh cpu_mesh = {
            .name = "EGI: test 2D grid",
            .primitive_mode = GX_TRIANGLES,
            .vertex_layout = &GX_VERTEXGNP,
            .sizeof_vertex_buffer = nb_vertices * sizeof *vertices,
            .nb_vertices = nb_vertices,
            .vertices = vertices,
            .nb_indices = nb_indices,
            .indices = indices,
            .vertex_buffer_flags = GX_BUF_STATIC_BIT | GX_BUF_DRAW_BIT,
            .index_buffer_flags = GX_BUF_STATIC_BIT | GX_BUF_DRAW_BIT,
        };

        s_test_grid_cpu_mesh[i] = cpu_mesh;
        s_test_grid_gpu_mesh[i] = gx_upload_mesh(&cpu_mesh);
    }
}

static void init_test_box_mesh(GxCpuMesh* out_cpu_mesh, GxGpuMesh* out_gpu_mesh, u32 w, u32 h, bool skybox) {
    const u32 nb_vertices_per_face = (w+1) * (h+1);
    const u32 nb_indices_per_face = w * h * 6;
    const u32 nb_vertices_total = 6 * nb_vertices_per_face;
    const u32 nb_indices_total = 6 * nb_indices_per_face;
    assert_fatal(nb_indices_total <= UINT16_MAX);

    VertexGNP* vertices = fixed_alloc_uninitialized(6 * nb_vertices_per_face * sizeof *vertices, 16, "");
    u16* indices = fixed_alloc_uninitialized(6 * nb_indices_per_face * sizeof *indices, 16, "");

    const v3 face_centers[] = {
        { 0.5f, 0.0f, 0.0f },
        { 0.0f, 0.5f, 0.0f },
        { 0.0f, 0.0f, 0.5f },
        { -0.5f, 0.0f, 0.0f },
        { 0.0f, -0.5f, 0.0f },
        { 0.0f, 0.0f, -0.5f },
    };

    const quat face_orientations[] = {
        quat_rotation_angle_axis(-PI_2, v3_010()),
        quat_rotation_angle_axis(PI_2, v3_100()),
        quat_rotation_angle_axis(PI, v3_010()),
        quat_rotation_angle_axis(PI_2, v3_010()),
        quat_rotation_angle_axis(-PI_2, v3_100()),
        quat_rotation_angle_axis(0.f, v3_010()),
    };

    for (u32 face = 0; face < 6; ++face) {
        u16* indices_for_face = &indices[face * nb_indices_per_face];
        VertexGNP* vertices_for_face = &vertices[face * nb_vertices_per_face];

        build_2d_grid_gnp(w, h, vertices_for_face, indices_for_face);

        const Xform xform = {
            .position = face_centers[face] * (skybox ? -1.f : 1.f),
            .orientation = face_orientations[face],
            .scale = v3_one(),
        };
        const m4 model_matrix = xform_get_model_matrix(xform);

        for (u32 i = 0; i < nb_vertices_per_face; ++i) {
            f32* p = vertices_for_face[i].position;
            f32* n = vertices_for_face[i].normal;
            f32* t = vertices_for_face[i].tangent;
            v3_store(p, m4_mul_point(model_matrix, v3_load(p)));
            v3_store(n, m4_mul_direction(model_matrix, v3_load(n)));
            v3_store(t, m4_mul_direction(model_matrix, v3_load(t)));
        }

        for (u32 i = 0; i < nb_indices_per_face; ++i)
            indices_for_face[i] += face * nb_vertices_per_face;
    }

    const GxCpuMesh cpu_mesh = {
        .name = skybox ? "EGI: test skybox" : "EGI: test box",
        .primitive_mode = GX_TRIANGLES,
        .vertex_layout = &GX_VERTEXGNP,
        .sizeof_vertex_buffer = nb_vertices_total * sizeof *vertices,
        .nb_vertices = nb_vertices_total,
        .vertices = vertices,
        .nb_indices = nb_indices_total,
        .indices = indices,
        .vertex_buffer_flags = GX_BUF_STATIC_BIT | GX_BUF_DRAW_BIT,
        .index_buffer_flags = GX_BUF_STATIC_BIT | GX_BUF_DRAW_BIT,
    };

    *out_cpu_mesh = cpu_mesh;
    *out_gpu_mesh = gx_upload_mesh(&cpu_mesh);
}


//
//
//
//


typedef struct {
    EgiDemoObject* obj;
    const m4* model_matrix;
    const GxCpuMesh* cpu_mesh;
    const GxMeshView* view;
    const GxVLayout* pos_vl;
    const GxVLayout* texcoords_vl;
    f32* v_area;
    v3* v_tangent;
    v3* v_bitangent;
} TriangleVisitUserdata;

static void add_area_contribution_of_triangle(void* userdata, v3u elements) {
    const TriangleVisitUserdata* u = userdata;

    assert_fatal_rare(u->view->nb_indices); // Assume indexed mesh

    const v3u vindex = {
        u->view->indices[elements[0]],
        u->view->indices[elements[1]],
        u->view->indices[elements[2]],
    };

    // Get model-space positions
    v3 vpos[3];
    for (u32 i = 0; i < 3; ++i) {
        gx_read_vertex_attrib_f32(v3_ptr(&vpos[i]), 3, u->pos_vl, u->view->vertices, vindex[i]);
        vpos[i] = gx_transform_mesh_vertex(u->cpu_mesh, vpos[i]);
    }

    // PERF: could be VFPU-optimized
    for (u32 i = 0; i < 3; ++i) {
        vpos[i] = m4_mul_point(*u->model_matrix, vpos[i]);
    }

    const f32 area_third = compute_triangle_area(vpos) / 3.f;

    // Add areas
    for (u32 i = 0; i < 3; ++i) {
        u->v_area[vindex[i]] += area_third;
    }
}

// https://docs.google.com/viewer?url=http%3A%2F%2Ffoundationsofgameenginedev.com%2FFGED2-sample.pdf
static void calculate_triangle_tangent_and_bitangent(void* userdata, v3u elements) {
    const TriangleVisitUserdata* u = userdata;

    assert_fatal_rare(u->view->nb_indices); // Assume indexed mesh

    const v3u vindex = {
        u->view->indices[elements[0]],
        u->view->indices[elements[1]],
        u->view->indices[elements[2]],
    };

    // Get model-space positions
    v3 vpos[3];
    for (u32 i = 0; i < 3; ++i) {
        gx_read_vertex_attrib_f32(v3_ptr(&vpos[i]), 3, u->pos_vl, u->view->vertices, vindex[i]);
        vpos[i] = gx_transform_mesh_vertex(u->cpu_mesh, vpos[i]);
    }

    // Get UVs
    v2 vtexcoords[3];
    for (u32 i = 0; i < 3; ++i) {
        gx_read_vertex_attrib_f32(v2_ptr(&vtexcoords[i]), 2, u->texcoords_vl, u->view->vertices, vindex[i]);
    }

    const v3 p0 = vpos[vindex[0]];
    const v3 p1 = vpos[vindex[1]];
    const v3 p2 = vpos[vindex[2]];
    const v2 w0 = vtexcoords[vindex[0]];
    const v2 w1 = vtexcoords[vindex[1]];
    const v2 w2 = vtexcoords[vindex[2]];
    const v3 e1 = p1 - p0;
    const v3 e2 = p2 - p0;
    const f32 x1 = w1[0] - w0[0];
    const f32 x2 = w2[0] - w0[0];
    const f32 y1 = w1[1] - w0[1];
    const f32 y2 = w2[1] - w0[1];
    const f32 r = 1.f / (x1 * y2 - x2 * y1);
    const v3 t = (e1 * y2 - e2 * y1) * r;
    const v3 b = (e2 * x1 - e1 * x2) * r;
    u->v_tangent[vindex[0]] += t;
    u->v_tangent[vindex[1]] += t;
    u->v_tangent[vindex[2]] += t;
    u->v_bitangent[vindex[0]] += b;
    u->v_bitangent[vindex[1]] += b;
    u->v_bitangent[vindex[2]] += b;
}

static void compute_vertex_tangents_from_texcoords(v4* final_v_tangent, const GxCpuMesh* cpu_mesh, const GxMeshView* view, GxPrimitiveIterator prim_it, const GxVLayout* pos_vl, const GxVLayout* normal_vl, const GxVLayout* texcoords_vl) {
    const size_t prev_scratch_top = scratch_get_top();
    v3* v_tangent = scratch_alloc_zeroed(view->nb_vertices * sizeof(*v_tangent), 16, "v_tangent");
    v3* v_bitangent = scratch_alloc_zeroed(view->nb_vertices * sizeof(*v_bitangent), 16, "v_bitangent");

    // Userdata setup, then visit triangles
    TriangleVisitUserdata userdata = {
        .cpu_mesh = cpu_mesh,
        .view = view,
        .pos_vl = pos_vl,
        .texcoords_vl = texcoords_vl,
        .v_tangent = v_tangent,
        .v_bitangent = v_bitangent,
    };

    assert_fatal(view->nb_indices); // Require indexed mesh

    // For each triangle, compute tangent and bitangent
    gx_visit_primitive_triangles(prim_it, calculate_triangle_tangent_and_bitangent, &userdata);

    // https://docs.google.com/viewer?url=http%3A%2F%2Ffoundationsofgameenginedev.com%2FFGED2-sample.pdf
    for (u32 i = 0; i < view->nb_vertices; i++) {
        const v3 t = v_tangent[i];
        const v3 b = v_bitangent[i];

        v3 n;
        gx_read_vertex_attrib_f32(v3_ptr(&n), 3, normal_vl, view->vertices, i);

        v4* out_vtangent = &final_v_tangent[i];
        *out_vtangent = v3_normalize(t - v3_dot(t, n) * n);
        (*out_vtangent)[3] = v3_dot(v3_cross(t, b), b) > 0.f ? 1.f : -1.f;
    }

    scratch_set_top(prev_scratch_top);
}

static void spawn_object_agents(EgiDemoObject* obj, EgiDemoAgent* agents, u32 max_agents) {
    const m4 model_matrix = xform_get_model_matrix(obj->xform);
    const GxCpuMesh* cpu_mesh = obj->cpu_mesh;

    // assert_fatal(obj->nb_agents == 0);
    obj->nb_agents = 0;
    obj->agents = agents;

    for (GxPartitionIterator it = gx_get_partition_iterator(cpu_mesh); gx_is_partition_iterator_valid(&it); gx_inc_partition_iterator(&it)) {
        const GxMeshView view = gx_deref_partition_iterator(&it);

        assert_fatal(gx_get_primitive_mode_base(view.primitive_mode) == GX_TRIANGLES);
        const u32 nb_elements = view.nb_indices ? view.nb_indices : view.nb_vertices;
        const GxPrimitiveIterator prim_it = gx_get_primitive_iterator(view.primitive_mode, nb_elements);

        // Look up the attributes we need
        const GxVertexLayout* vl = view.vertex_layout;
        const GxVLayout* pos_vl = NULL;
        const GxVLayout* normal_vl = NULL;
        const GxVLayout* tangent_vl = NULL;
        const GxVLayout* texcoords_vl = NULL;
        for (u32 i = 0; i < vl->nb_attribs && !(pos_vl && normal_vl && tangent_vl && texcoords_vl); ++i) {
            const GxVAttrib* va = &vl->attribs[i];
            if (va->id.semantic == GX_VA_POSITION && va->id.i == 0) pos_vl = &va->layout;
            if (va->id.semantic == GX_VA_NORMAL && va->id.i == 0) normal_vl = &va->layout;
            if (va->id.semantic == GX_VA_TANGENT && va->id.i == 0) tangent_vl = &va->layout;
            if (va->id.semantic == GX_VA_TEXCOORDS && va->id.i == 0) texcoords_vl = &va->layout;
        }

        assert_fatal_rare_fmt(pos_vl, "No position attribute");
        assert_fatal_rare_fmt(normal_vl, "No normal attribute");

        const size_t prev_scratch_top = scratch_get_top();
        v4* final_v_tangent = scratch_alloc_zeroed(view.nb_vertices * sizeof(*final_v_tangent), 16, "final_v_tangent");

        if (tangent_vl) {
            for (u32 i = 0; i < view.nb_vertices; i++) {
                gx_read_vertex_attrib_f32(v4_ptr(&final_v_tangent[i]), 4, tangent_vl, view.vertices, i);
            }
        } else { // tangent_vl == NULL
            if (texcoords_vl) {
                compute_vertex_tangents_from_texcoords(final_v_tangent, cpu_mesh, &view, prim_it, pos_vl, normal_vl, texcoords_vl);
            } else {
                assert_soft(0);
                // Project on sphere... Not great.
                const quat normal_to_tangent = quat_rotation_from_to(-v3_001(), v3_100());
                for (u32 i = 0; i < view.nb_vertices; i++) {
                    v3 vnorm;
                    gx_read_vertex_attrib_f32(v3_ptr(&vnorm), 3, normal_vl, view.vertices, i);
                    final_v_tangent[i] = quat_mul_v3(normal_to_tangent, vnorm);

                    // HACK: slight nudge
                    if (v3_dot(final_v_tangent[i], vnorm) >= 0.99f) {
                        final_v_tangent[i][0] -= 0.1f;
                    }

                    final_v_tangent[i] = v3_normalize(final_v_tangent[i] - v3_dot(final_v_tangent[i], vnorm) * vnorm);
                    final_v_tangent[i][3] = -1.f;
                }
            }
        }

        for (u32 i = 0; i < view.nb_vertices; i++) {
            const f32 w = final_v_tangent[i][3];
            assert_soft(w == -1.f || w == 1.f);
        }

        // "Per-vertex area" buffer alloc
        f32* v_area = scratch_alloc_zeroed(view.nb_vertices * sizeof(*v_area), 16, "v_area");

        // Userdata setup, then visit triangles to compute the area of each surfel
        TriangleVisitUserdata userdata = {
            .obj = obj,
            .cpu_mesh = cpu_mesh,
            .model_matrix = &model_matrix,
            .view = &view,
            .pos_vl = pos_vl,
            .v_area = v_area,
        };

        assert_fatal(view.nb_indices); // Require indexed mesh

        // For each triangle, add 1/3 of its area to the "area" of each of its vertices.
        gx_visit_primitive_triangles(prim_it, add_area_contribution_of_triangle, &userdata);

        // The areas are computed, now spawn an agent (surfel) for each vertex.
        const f32 min_tolerated_area = 0.00001f;
        for (u32 i = 0; i < view.nb_vertices; ++i) {
            v_area[i] = fmaxf(v_area[i], min_tolerated_area);
            //if (v_area[i] < min_area) {
                //assert_soft(v_area[i] != 0.f);
                //continue;
            //}

            v3 vnorm, vpos;
            gx_read_vertex_attrib_f32(v3_ptr(&vnorm), 3, normal_vl, view.vertices, i);
            gx_read_vertex_attrib_f32(v3_ptr(&vpos), 3, pos_vl, view.vertices, i);
            vpos = gx_transform_mesh_vertex(cpu_mesh, vpos);

            vpos = m4_mul_point(model_matrix, vpos);
            vnorm = v3_normalize(m4_mul_direction(model_matrix, vnorm));

            v3 tangent = final_v_tangent[i];
            const f32 tangent_w = tangent[3];
            tangent = v3_normalize(m4_mul_direction(model_matrix, tangent));
            tangent[3] = tangent_w;

            const v3 bitangent = v3_cross(vnorm, tangent) * tangent[3];
            const f32 l = v3_magnitude_squared(bitangent);
            assert_soft(abs_diff_eq(l, 1.f, 0.0001f));

            assert_fatal(obj->nb_agents < max_agents);
            obj->agents[(obj->nb_agents)++] = (EgiDemoAgent) {
                .obj = obj,
                .area = v_area[i],
                .position = vpos,
                .normal = vnorm,
                .tangent = tangent,
                .base_color = obj->base_color,
                .base_emissive = obj->emissive,
                .metallic = obj->metallic,
                .roughness = obj->roughness,
                .shadow_intensity = v3_one(),
            };
        }

        //
        scratch_set_top(prev_scratch_top);
    }
}

static void respawn_object_agents(EgiDemoObject* obj) {
    spawn_object_agents(obj, obj->agents, obj->nb_agents);
    upload_instanced_colors(obj->agents - s_agents, obj->nb_agents); // FIXME: ugly pointer subtraction
    upload_instanced_model_matrices(obj->agents - s_agents, obj->nb_agents); // FIXME: ugly pointer subtraction
    upload_tbn(obj->agents - s_agents, obj->nb_agents); // FIXME: ugly pointer subtraction
}

static void on_object_material_changed(EgiDemoObject* obj) {
    for (u32 i = 0; i < obj->nb_agents; ++i) {
        EgiDemoAgent* agent = &obj->agents[i];
        agent->base_color = obj->base_color;
        agent->base_emissive = obj->emissive;
    }

    upload_instanced_colors(obj->agents - s_agents, obj->nb_agents); // FIXME: ugly pointer subtraction
}

static void on_object_xform_changed(EgiDemoObject* obj) {
    respawn_object_agents(obj);
}


//
//
//
//
//
//
//
//

// To plot in academo.org:
// x*x / (3.14159265359 * ((y*y) * (x*x-1) + 1) * ((y*y) * (x*x-1) + 1))
float DistributionGGX(v3 N, v3 H, float roughness) {
    float a      = roughness*roughness;
    float a2     = a*a;
    float NdotH  = max(v3_dot(N, H), 0.0f);
    float NdotH2 = NdotH*NdotH;
	
    float num   = a2;
    float denom = (NdotH2 * (a2 - 1.0f) + 1.0f);
    denom = PI * denom * denom;
	
    float ggx = num / max(denom, 0.00001f);
    assert_fatal_rare(isfinite(ggx));
    return ggx;
}

float GeometrySchlickGGX(float NdotV, float roughness) {
    float r = (roughness + 1.0f);
    float k = (r*r) / 8.0f;

    float num   = NdotV;
    float denom = NdotV * (1.0f - k) + k;
	
    assert_fatal_rare(isfinite(num / denom));
    return num / denom;
}

float GeometrySmith(v3 N, v3 V, v3 L, float roughness) {
    float NdotV = max(v3_dot(N, V), 0.0f);
    float NdotL = max(v3_dot(N, L), 0.0f);
    float ggx2  = GeometrySchlickGGX(NdotV, roughness);
    float ggx1  = GeometrySchlickGGX(NdotL, roughness);
	
    return ggx1 * ggx2;
}

v3 fresnelSchlick(float cosTheta, v3 F0) {
    return F0 + (1.0f - F0) * powf(1.0f - cosTheta, 5.0f);
}

v3 fresnelSchlickRoughness(float cosTheta, v3 F0, float roughness) {
    return F0 + (v3_max(v3_broadcast(1.0f - roughness), F0) - F0) * powf(1.0f - cosTheta, 5.0f);
}

static v3 pbr_shader_3directional_lights(
    v3 base_color,
    f32 metallic,
    f32 roughness,
    v3 F0,
    v3 N, // Surface normal
    v3 V, // View vector, i.e normalized vector pointing from fragment position towards the eye.
    f32 NdotV,
    const v3* irradiance_samples_direction_normalized,
    const v3* irradiance_samples,
    u32 first_sample,
    u32 nb_samples
) {
    // Reflectance equation
    v3 Lo = v3_zero();
    for (u32 i = first_sample; i < first_sample+nb_samples; ++i) {
        //
        const v3 L = irradiance_samples_direction_normalized[i];
        const v3 H = v3_normalize(V + L);
        const float NdotL = max(v3_dot(N, L), 0.0f);

        //
        v3 radiance = irradiance_samples[i];

        // cook-torrance brdf
        float NDF = DistributionGGX(N, H, roughness);
        float G   = GeometrySmith(N, V, L, roughness);
        v3 F    = fresnelSchlick(max(v3_dot(H, V), 0.0f), F0);

        assert_fatal_rare(isfinite(NDF));
        assert_fatal_rare(isfinite(G));
        assert_fatal_rare(v3_isfinite(F));
        
        //
        v3 kS = F;
        v3 kD = v3_one() - kS;
        kD *= 1.0f - metallic;	  
        
        //
        v3 numerator    = NDF * G * F;
        float denominator = 4.0f * NdotV * NdotL;
        v3 l_specular   = numerator / max(denominator, 0.001f);  
            
        Lo += radiance * NdotL * (l_specular + kD * base_color / PI);
    }

    assert_fatal_rare(v3_isfinite(Lo));
    return Lo;
}

static v3 pbr_shader_ambient(
    v3 base_color,
    f32 metallic,
    f32 roughness,
    v3 F0,
    f32 NdotV,
    v3 irradiance
    // v3 reflected_color
) {
    //
    v3 F = fresnelSchlickRoughness(NdotV, F0, roughness);

    v3 kS = F;
    v3 kD = 1.0f - kS;
    kD *= 1.0f - metallic;

    v3 diffuse  = kD * irradiance * base_color;
        
    // v2 envBRDF  = texture(u_brdf_lut_texture, fma(vec2(p.NdotV, roughness), u_brdf_uv_mul, u_brdf_uv_add));
    // v3 specular = reflected_color * (F * envBRDF[0] + envBRDF[1]);

    return diffuse ;//+ specular;
}

static v3 agent_shader_fwd(const EgiDemoAgent* emitter, const EgiDemoAgent* receiver) {
    const v3 N = emitter->normal;
    const v3 V = v3_normalize(receiver->position - emitter->position);
    const float NdotV = max(v3_dot(N, V), 0.0f);

	// Surface reflection at zero incidence (for Fresnel)
    v3 F0 = v3_broadcast(0.04f);
    F0 = v3_lerp(F0, emitter->base_emissive, emitter->metallic);

    v3 Lo;
    switch (0) {
    case 0:
        Lo = emitter->irradiance * emitter->base_color;
        break;
    case 1:
        assert_fatal_fmt(0, "NULL argument is provided, with wrong number of elements as well");
        Lo = pbr_shader_3directional_lights(
            emitter->base_color, emitter->metallic, emitter->roughness,
            F0, N, V, NdotV,
            NULL,
            &emitter->irradiance,
            1, 3
        );
        break;
    case 2:
        Lo = pbr_shader_ambient(
            emitter->base_color, emitter->metallic, emitter->roughness,
            F0, NdotV,
            emitter->irradiance
        );
        break;
    }
    return (Lo + emitter->base_emissive) * s_irradiance_intensity_factor;
}

static v3 agent_shader_bck(const EgiDemoAgent* emitter) {
    return -emitter->irradiance * emitter->shadow_intensity * s_shadow_intensity_factor;
}

//
//
//
//
//
//
//

void get_closest_points_along_two_lines(v3 out[2], const v3 L1[2], const v3 L2[2]) {
    const v3 u = L1[1] - L1[0];
    const v3 v = L2[1] - L2[0];
    const v3 w = L1[0] - L2[0];
    const f32 a = v3_dot(u,u);         // always >= 0
    const f32 b = v3_dot(u,v);
    const f32 c = v3_dot(v,v);         // always >= 0
    const f32 d = v3_dot(u,w);
    const f32 e = v3_dot(v,w);
    const f32 D = a*c - b*b;        // always >= 0
    f32 sc, tc;

    // compute the line parameters of the two closest points
    if (D < 0.00000001f) {          // the lines are almost parallel
        sc = 0.f;
        tc = (b>c ? d/b : e/c);    // use the largest denominator
    } else {
        sc = (b*e - c*d) / D;
        tc = (a*e - b*d) / D;
    }

    // get the difference of the two closest points
    // Vector   dP = w + (sc * u) - (tc * v);  // =  L1(sc) - L2(tc)
    // return norm(dP);   // return the closest distance
    out[0] = L1[0] + u * sc;
    out[1] = L2[0] + v * tc;
}

// Chebyshev polynomial. Could be optimized later.
static f32 Tn(u32 n, f32 X) {
    if (n == 0) return 1.f;
    if (n == 1) return X;
    return 2.f * X * Tn(n-1, X) - Tn(n-2, X);
}

static f32 Cl2_sum(u32 N, f32 t) {
    const f32 b[] = {
        1.865555351433979e-1f,
        6.269948963579612e-2f,
        3.139559104552675e-4f,
        3.916780537368088e-6f,
        6.499672439854756e-8f,
        1.238143696612060e-9f,
        5.586505893753557e-13f,
    };
    assert_fatal_rare(N < countof(b));

    const f32 X = (t/PI) - 1.f;
    f32 sum = 0.f;
    for (u32 n = 0; n <= N; ++n) {
        sum += b[n] * Tn(2*n + 1, X);
    }
    return sum;
}

// Approximation of Clausen's integral
static f32 Cl2(f32 t) {
    assert_soft(t >= 0.f); // Otherwise, logf() returns NaN
    assert_soft(2.f*PI - t >= 0.f); // Otherwise, logf() returns NaN
    return (t - PI) * (2.f + logf(PI*PI/2.f)) + (2.f*PI - t) * logf(2.f*PI - t) - t * logf(t) + Cl2_sum(5, t);
}

// Imaginary part of dilogarithm. 'r' is the real part, 'a' the imaginary part.
// This computes Im(Li2(r*exp(i*a))); where i is the indeterminate satisfying i*i == -1.
static f32 Im_Li2(f32 r, f32 a, f32 cos_a, f32 sin_a) {
    assert_soft(a >= 0.f && a < PI);
    const f32 w = atan2f(r * sin_a, 1.f - r * cos_a); // XXX atan2f
    assert_soft(w >= 0.f && w < PI);
    return w * logf(r) + (Cl2(2.f*a) + Cl2(2.f*w) - Cl2(2.f*a + 2.f*w)) / 2.f;
}

// "An analytic expression for radiation view factor between two arbitrarily oriented planar polygons" by Arvind Narayanaswamy
// Formula (23)
static f32 f_tilde_when_a_very_close_to_0(f32 s, f32 l, f32 d, f32 d2) {
    const f32 l_s = l-s;
    const f32 s_l = s-l;
    const f32 s_l_2 = s_l * s_l;
    const f32 part0 = 0.5f * (s_l_2 - d2) * logf(s_l_2 + d2);
    const f32 part1 = s_l * d * atanf(d / s_l); // XXX atan2f
    const f32 part2 = l_s * d * atanf(d / l_s); // XXX atan2f
    return part0 - part1 - part2;
}

// "An analytic expression for radiation view factor between two arbitrarily oriented planar polygons" by Arvind Narayanaswamy
// Formula (22b)
static f32 f_tilde(f32 s, f32 l, f32 d, f32 d2, f32 a, f32 cos_a, f32 sin_a, f32 sin2_a) {
    if (abs_diff_eq(a, 0.f, 0.0001f))
        return f_tilde_when_a_very_close_to_0(s, l, d, d2);

    // Precalc
    const f32 ss = s * s;
    const f32 ll = l * l;
    const f32 sl = s * l;
    //
    const f32 S = s * sin_a / d;
    const f32 L = l * sin_a / d;
    const f32 W = S + sqrtf(S*S + 1.f);
    const f32 P = L + sqrtf(L*L + 1.f);
    //
    const f32 part0 = ((cos_a / 2.f) * (ss + ll) - sl) * logf(ss - 2.f*sl*cos_a + ll + d2);
    const f32 part1_sqrt = sqrtf(ss*sin2_a + d2);
    const f32 part2_sqrt = sqrtf(ll*sin2_a + d2);
    const f32 part1 = s * part1_sqrt * atanf(part1_sqrt / (l - s*cos_a)); // XXX atan2f
    const f32 part2 = l * part2_sqrt * atanf(part2_sqrt / (s - l*cos_a)); // XXX atan2f
    const f32 part3 = (d2 / (2.f*sin_a)) * (Im_Li2(W/P, a, cos_a, sin_a) + Im_Li2(P/W, a, cos_a, sin_a)); // XXX: decomposition of complex into sum?
    const f32 part4 = (d2 / sin_a) * Im_Li2((1.f / W) * (1.f / P), PI - a, -cos_a, sin_a);
    return part0 + part1 + part2 + part3 - part4;
}

// "An analytic expression for radiation view factor between two arbitrarily oriented planar polygons" by Arvind Narayanaswamy
// Formula (22a)
static f32 compute_a1_f12(const v3* r1, u32 r1_nb, const v3* r2, u32 r2_nb) {
    f32 sum = 0.f;
    for (u32 i = 0; i < r1_nb; ++i) {
        const u32 j = (i + 1) % r1_nb;

        for (u32 p = 0; p < r2_nb; ++p) {
            const u32 q = (p + 1) % r2_nb;

            const v3 L1[] = { r1[i], r1[j] };
            const v3 L2[] = { r2[p], r2[q] };
            v3 rT[2];
            get_closest_points_along_two_lines(rT, L1, L2); // XXX ?
            const v3 rT1 = rT[0];
            const v3 rT2 = rT[1];

            const v3 s_hat = v3_normalize(v3_magnitude_squared(r1[i] - rT1) > v3_magnitude_squared(r1[j] - rT1) ? (r1[i] - rT1) : (r1[j] - rT1));
            const v3 l_hat = v3_normalize(v3_magnitude_squared(r2[p] - rT2) > v3_magnitude_squared(r2[q] - rT2) ? (r2[p] - rT2) : (r2[q] - rT2));

            // XXX
            const f32 si = v3_dot(s_hat, r1[i] - rT1);
            const f32 sj = v3_dot(s_hat, r1[j] - rT1);
            const f32 lp = v3_dot(l_hat, r2[p] - rT2);
            const f32 lq = v3_dot(l_hat, r2[q] - rT2);

            const f32 d2 = v3_magnitude_squared(rT1 - rT2);
            const f32 d = sqrtf(d2);

            const f32 cos_a = v3_dot(s_hat, l_hat);
            const f32 sin2_a = v3_magnitude_squared(v3_cross(s_hat, l_hat));
            const f32 sin_a = sqrtf(sin2_a);
            const f32 a = atan2f(sin_a, cos_a);

            const f32 f_sj_lq = f_tilde(sj, lq, d, d2, a, cos_a, sin_a, sin2_a);
            const f32 f_sj_lp = f_tilde(sj, lp, d, d2, a, cos_a, sin_a, sin2_a);
            const f32 f_si_lp = f_tilde(si, lp, d, d2, a, cos_a, sin_a, sin2_a);

            sum += cos_a * (f_sj_lq - 2.f * f_sj_lp + f_si_lp);
        }
    }
    return fabsf(sum) / (4.f * PI);
}

static v2 compute_planar_polygons_view_factors(const v3* r1, u32 r1_nb, const v3* r2, u32 r2_nb) {
    assert_fatal_rare(r1_nb == 3 && r2_nb == 3); // Because we're using compute_triangle_area() for now...
    const f32 a1 = compute_triangle_area(r1);
    const f32 a2 = compute_triangle_area(r2);
    const f32 a1_f12 = compute_a1_f12(r1, r1_nb, r2, r2_nb);
    return v2_broadcast(a1_f12) / v2_set(a1, a2);
}

static void test_planar_polygon_view_factors() {
    return;

    if (0) {
        const v3 r1[] = {
            {0,0,0},
            {1,0,0},
            {1,1,0},
            {0,1,0},
        };
        const v3 r2[] = {
            {0,0,1},
            {1,0,1},
            {1,1,1},
            {0,1,1},
        };
        const v2 f = compute_planar_polygons_view_factors(r1, countof(r1), r2, countof(r2));
        logi("Test 1 : f12 = %g, f21 = %g\n", (f64) f[0], (f64) f[1]);
    }

    if (1) {
        const f32 e = 1e-4f;
        const v3 r1[] = {
            {0+e,0-e,0+e},
            {0+e,1+e,0-e},
            {1+e,1-e,0+e},
        };
        const v3 r2[] = {
            {1,0,1},
            {1,1,1},
            {0,1,1},
        };
        const v2 f = compute_planar_polygons_view_factors(r1, countof(r1), r2, countof(r2));
        logi("Test 1 : f12 = %g, f21 = %g\n", (f64) f[0], (f64) f[1]);
        assert_soft(abs_diff_eq(f[0], 0.099912f, 0.0001f));
    }

    if (1) {
        const f32 e = 1e-4f;
        const v3 r1[] = {
            {0+e,0-e,0+e},
            {1+e,0+e,0-e},
            {1+e,1-e,0+e},
        };
        const v3 r2[] = {
            {2,2,2},
            {4,4,4},
            {2,3,3},
        };
        const v2 f = compute_planar_polygons_view_factors(r1, countof(r1), r2, countof(r2));
        logi("Test 1 : f12 = %g, f21 = %g\n", (f64) f[0], (f64) f[1]);
        assert_soft(abs_diff_eq(f[0], 1.068e-3f, 1e-6f));
    }

    exit(0);
}

//
//
//
//
//
//
//
//
//
//


static v4 get_area_factor_v4_mbunnell(f32 a1, f32 a2, f32 distance, f32 distance_squared) {
    (void) distance;
    const v2 areas = { a1, a2 };
    const v2 area_factor = areas / (areas + PI * distance_squared);
    const v4 area_factor_v4 = { area_factor[0], area_factor[0], area_factor[1], area_factor[1] };
    return area_factor_v4;
}

static v4 get_area_factor_v4_principled(f32 a1, f32 a2, f32 distance, f32 distance_squared) {
    (void) distance_squared;
    const v4 areas = { a1, a2 };
    const v4 radii = _mm_sqrt_ps(areas / PI);
    const v4 rd = radii / distance;
    const v4 rds = rd * rd;
    const v4 m = {
        rds[1] / rds[0],
        rds[0] / rds[1],
    };
    const v4 x = v4_set(
        1.f / rds[0],
        1.f / rds[1],
        0,
        0
    ) + m + 1.f;
    const v4 y = _mm_sqrt_ps(x*x - m * 4.f);
    const v4 f = (x - y) / 2.f;
    const v4 area_factor_v4 = { f[0], f[0], f[1], f[1] };
    return area_factor_v4;
}

typedef v4 (*AreaFactorV4_Fn)(f32 a1, f32 a2, f32 distance, f32 distance_squared);
static AreaFactorV4_Fn get_area_factor_v4 = get_area_factor_v4_mbunnell;


// Computes view factor approximations between two surfels, using multiple sampling directions for the viewer, as well as the emitter's no_tbnral, as well as its flipped normal.
// The flipped normal is useful for "negative emittance".
//
// Given a surfel "1" with parameters:
// - c1: center of the surfel, or surfel "position";
// - surface_n1: normal of the surfel;
// - sample_n1: normals to use for sampling irradiance (e.g you could sample the irradiance seen by surfel 1 from multiple directions, not just the normal).
// - a1: The area of the surfel;
// - nb_samples: the number of elements pointed to by sample_n1;
//
// (substitute "1" by "2" for parameters representing the second surfel).
//
// The following is computed, for all "i" in [0; nb_samples) range:
// - out[i][0]: View factor for radiance emitted by surfel 1 with normal "+surface_n1", as seen from surfel 2, using sample_n2[i] as the viewing direction.
// - out[i][1]: View factor for radiance emitted by surfel 1 with normal "-surface_n1", as seen from surfel 2, using sample_n2[i] as the viewing direction.
// - out[i][2]: View factor for radiance emitted by surfel 2 with normal "+surface_n2", as seen from surfel 1, using sample_n1[i] as the viewing direction.
// - out[i][3]: View factor for radiance emitted by surfel 2 with normal "-surface_n2", as seen from surfel 1, using sample_n1[i] as the viewing direction.
static void disk_to_disk_view_factor_approx_n(
    v4* restrict out, u32 nb_samples,
    v3 c1, v3 surface_n1, const v3* restrict sample_n1, f32 a1,
    v3 c2, v3 surface_n2, const v3* restrict sample_n2, f32 a2
) {
    const v3 p1_to_p2 = c2 - c1;
    const f32 distance_squared = fmaxf(v3_magnitude_squared(p1_to_p2), 0.00001f); // Prevent division by zero when c1 == c2.
    const f32 distance = sqrtf(distance_squared);
    const v3 p1_to_p2_normalized = p1_to_p2 / distance;

    const v4 area_factor_v4 = get_area_factor_v4(a1, a2, distance, distance_squared);

    const f32 dp_surface_n1 = v3_dot(surface_n1, +p1_to_p2_normalized);
    const f32 dp_surface_n2 = v3_dot(surface_n2, -p1_to_p2_normalized);
    const v4 dp_surface_nx_v4 = v4_max(v4_zero(), v4_set(+dp_surface_n1, -dp_surface_n1, +dp_surface_n2, -dp_surface_n2));

    for (u32 i = 0; i < nb_samples; ++i) {
        const f32 dp_sample_n1 = v3_dot(sample_n1[i], +p1_to_p2_normalized);
        const f32 dp_sample_n2 = v3_dot(sample_n2[i], -p1_to_p2_normalized);
        const v4 dp_sample_nx_v4 = v4_max(v4_zero(), v4_set(dp_sample_n2, dp_sample_n2, dp_sample_n1, dp_sample_n1));
        out[i] = area_factor_v4 * dp_sample_nx_v4 * dp_surface_nx_v4;
    }
}


//
//
//
//

typedef struct {
    u32 nb_calculations;
    f32 simulation_duration; // seconds
    f32 color_upload_duration; // seconds
} EgiDemoIterationStats;

static EgiDemoIterationStats s_last_iteration_stats = {0};

typedef struct {
    u32 state_id;
    u32 i;
    u32 j;
    bool paused;
} EgiDemoSim;

static EgiDemoSim s_sim = {
    .state_id = 0,
    .i = 0,
    .j = 1,
};

static u32 s_nb_iterations_done = 0;

static void restart_gi_simulation() {
    s_nb_iterations_done = 0;
    s_nb_agents = 0;
    for (u32 i = 0; i < s_nb_objects; ++i) {
        spawn_object_agents(&s_objects[i], &s_agents[s_nb_agents], s_max_agents - s_nb_agents);
        s_nb_agents += s_objects[i].nb_agents;
    }
    upload_instanced_colors(0, s_nb_agents);
    upload_instanced_model_matrices(0, s_nb_agents);
    upload_tbn(0, s_nb_agents);

    s_sim.state_id = 0;
    s_sim.i = 0;
    s_sim.j = 1;
}

static void transform_tbn(v3* directions, const EgiDemoAgent* a) {
    assert_soft(a->tangent[3] == -1.f || a->tangent[3] == 1.f);

    const v3 tangent = a->tangent;
    const v3 normal = a->normal;
    const v3 bitangent = v3_cross(normal, tangent) * tangent[3];

    const m4 tbn_change_of_basis = { .cols = {
        v4_direction(tangent),
        v4_direction(bitangent),
        v4_direction(normal),
        v4_0001(),
    } };

    const m4 m = m4_mul(tbn_change_of_basis, TANGENTSPACE_TO_IRRADIANCE_SAMPLING_DIRECTION_MATRIX);

    for (u32 i = 0; i < 3; ++i) {
        directions[i] = m.cols[i];
        const f32 l = v3_magnitude_squared(directions[i]);
        assert_soft(abs_diff_eq(l, 1.f, 0.0001f));
    }
}

static void step_gi_simulation_impl(EgiDemoSim* p, u32 max_iter) {
    if (p->paused)
        return;

    if (p->state_id == 0) {
        s_last_iteration_stats = (EgiDemoIterationStats){0};

        for (u32 i = 0; i < s_nb_agents; ++i) {
            EgiDemoAgent* agent = &s_agents[i];
            agent->irradiance_accumulator = v4_zero();
            agent->reflected_light_for_camera_view_accumulator = v4_zero();
        }

        p->state_id += 1;
    }

    const v3 cam_pos = g_current_camera->xform.position;

    if (p->state_id == 1) {
        u32 nb_iter = 0;
        for (u32 i = p->i; i < s_nb_agents; ++i) {
            EgiDemoAgent* agent0 = &s_agents[i];

            const v3 agent0_to_cam_pos = v3_normalize(cam_pos - agent0->position);
            const v3 agent0_R = v3_reflect(-agent0_to_cam_pos, agent0->normal);

            for (u32 j = nb_iter ? i+1 : p->j; j < s_nb_agents; ++j) {
                EgiDemoAgent* agent1 = &s_agents[j];

                // PERF: Agents should be sorted based on that
                if (agent0->obj == agent1->obj && agent0->obj->is_convex)
                    continue;

                if (nb_iter >= max_iter) {
                    p->i = i;
                    p->j = j;
                    return;
                }

                nb_iter++;

                const v4 a0_to_a1_normalized = v4_direction(v3_normalize(agent1->position - agent0->position));
                const v3 agent1_to_cam_pos = v3_normalize(cam_pos - agent1->position);
                const v3 agent1_R = v3_reflect(-agent1_to_cam_pos, agent1->normal);

                const f32 agent0_nd = clamp(DistributionGGX(agent0->normal, v3_normalize(agent0_to_cam_pos + a0_to_a1_normalized), agent0->roughness), 0.f, 1.f);
                const f32 agent1_nd = clamp(DistributionGGX(agent1->normal, v3_normalize(agent1_to_cam_pos - a0_to_a1_normalized), agent1->roughness), 0.f, 1.f);

#if 0
                v3 sample_n0[2];
                sample_n0[0] = agent0->normal;
                sample_n0[1] = a0_to_a1_normalized;

                v3 sample_n1[2];
                sample_n1[0] = agent1->normal;
                sample_n1[1] = -a0_to_a1_normalized;

                v4 view_factors[2];
                disk_to_disk_view_factor_approx_n(
                    view_factors, 2,
                    agent0->position, agent0->normal, sample_n0, agent0->area,
                    agent1->position, agent1->normal, sample_n1, agent1->area
                );

                const v4 agent0_emitted_fwd = v4_point(agent_shader_fwd(agent0, agent1));
                const v4 agent0_emitted_bck = v4_point(agent_shader_bck(agent0));
                const v4 agent1_emitted_fwd = v4_point(agent_shader_fwd(agent1, agent0));
                const v4 agent1_emitted_bck = v4_point(agent_shader_bck(agent1));

                agent1->irradiance_accumulator += agent0_emitted_fwd * powf(view_factors[0][0], s_irradiance_viewfactor_exp);
                agent1->irradiance_accumulator += agent0_emitted_bck * powf(view_factors[0][1], s_irradiance_viewfactor_exp);
                agent0->irradiance_accumulator += agent1_emitted_fwd * powf(view_factors[0][2], s_irradiance_viewfactor_exp);
                agent0->irradiance_accumulator += agent1_emitted_bck * powf(view_factors[0][3], s_irradiance_viewfactor_exp);

                agent1->reflected_light_for_camera_view_accumulator += agent0_emitted_fwd * powf(view_factors[1][0], lerpf(s_refl_exp0, s_refl_exp1, agent1_nd)) * fmaxf(0.f, floorf(1.f + v3_dot(agent1->normal, -a0_to_a1_normalized))); // * agent1_nd;
                agent1->reflected_light_for_camera_view_accumulator += agent0_emitted_bck * powf(view_factors[1][1], lerpf(s_refl_exp0, s_refl_exp1, agent1_nd)) * fmaxf(0.f, floorf(1.f + v3_dot(agent1->normal, -a0_to_a1_normalized))); // * agent1_nd;
                agent0->reflected_light_for_camera_view_accumulator += agent1_emitted_fwd * powf(view_factors[1][2], lerpf(s_refl_exp0, s_refl_exp1, agent0_nd)) * fmaxf(0.f, floorf(1.f + v3_dot(agent0->normal, +a0_to_a1_normalized))); // * agent0_nd;
                agent0->reflected_light_for_camera_view_accumulator += agent1_emitted_bck * powf(view_factors[1][3], lerpf(s_refl_exp0, s_refl_exp1, agent0_nd)) * fmaxf(0.f, floorf(1.f + v3_dot(agent0->normal, +a0_to_a1_normalized))); // * agent0_nd;
#else
                v3 sample_n0[2];
                sample_n0[0] = agent0->normal;
                sample_n0[1] = agent0_R;

                v3 sample_n1[2];
                sample_n1[0] = agent1->normal;
                sample_n1[1] = agent1_R;

                v4 view_factors[2];
                disk_to_disk_view_factor_approx_n(
                    view_factors, 2,
                    agent0->position, agent0->normal, sample_n0, agent0->area,
                    agent1->position, agent1->normal, sample_n1, agent1->area
                );

                const v4 agent0_emitted_fwd = v4_point(agent_shader_fwd(agent0, agent1));
                const v4 agent0_emitted_bck = v4_point(agent_shader_bck(agent0));
                const v4 agent1_emitted_fwd = v4_point(agent_shader_fwd(agent1, agent0));
                const v4 agent1_emitted_bck = v4_point(agent_shader_bck(agent1));

                agent1->irradiance_accumulator += agent0_emitted_fwd * powf(view_factors[0][0], s_irradiance_viewfactor_exp);
                agent1->irradiance_accumulator += agent0_emitted_bck * powf(view_factors[0][1], s_irradiance_viewfactor_exp);
                agent0->irradiance_accumulator += agent1_emitted_fwd * powf(view_factors[0][2], s_irradiance_viewfactor_exp);
                agent0->irradiance_accumulator += agent1_emitted_bck * powf(view_factors[0][3], s_irradiance_viewfactor_exp);

                agent1->reflected_light_for_camera_view_accumulator += agent0_emitted_fwd * view_factors[1][0] * powf(fmaxf(0.f, v3_dot(agent1_R, -a0_to_a1_normalized)), s_focus_exp);
                agent1->reflected_light_for_camera_view_accumulator += agent0_emitted_bck * view_factors[1][1] * powf(fmaxf(0.f, v3_dot(agent1_R, -a0_to_a1_normalized)), s_focus_exp);
                agent0->reflected_light_for_camera_view_accumulator += agent1_emitted_fwd * view_factors[1][2] * powf(fmaxf(0.f, v3_dot(agent0_R, +a0_to_a1_normalized)), s_focus_exp);
                agent0->reflected_light_for_camera_view_accumulator += agent1_emitted_bck * view_factors[1][3] * powf(fmaxf(0.f, v3_dot(agent0_R, +a0_to_a1_normalized)), s_focus_exp);
#endif

                s_last_iteration_stats.nb_calculations += 1;
            }
        }
        p->i = 0;
        p->j = 1;
        p->state_id += 1;
    }

    if (p->state_id == 2) {
        for (u32 i = 0; i < s_nb_agents; ++i) {
            EgiDemoAgent* agent = &s_agents[i];

            agent->irradiance = agent->irradiance_accumulator;
            agent->reflected_light_for_camera_view = agent->reflected_light_for_camera_view_accumulator;

            if (s_should_average_irradiance) {
                assert_soft(agent->irradiance[3] != 0.f);
                agent->irradiance /= agent->irradiance[3];
            }

            if (s_should_average_reflected) {
                if (agent->reflected_light_for_camera_view[3] != 0.f);
                    agent->reflected_light_for_camera_view /= agent->reflected_light_for_camera_view[3];
            }
        }

        const Instant color_upload_start_time = instant_now();
        upload_instanced_colors(0, s_nb_agents);
        const Instant color_upload_end_time = instant_now();
        s_last_iteration_stats.color_upload_duration = duration_since(color_upload_end_time, color_upload_start_time);

        for (u32 i = 0; i < s_nb_objects; ++i) {
            upload_object_light_received(&s_objects[i]);
        }

        p->state_id = 0;
        s_nb_iterations_done += 1;
    }
}

static void step_gi_simulation() {
    const u32 max_iter = (s_nb_agents * (s_nb_agents-1)) / 2;

    const Instant start_time = instant_now();
    step_gi_simulation_impl(&s_sim, max_iter);
    const Instant end_time = instant_now();
    const f32 duration = duration_since(end_time, start_time);

    s_last_iteration_stats.simulation_duration = duration;
}

//
//
//

static void step_gi_simulation_partial() {
    static u32 s_max_iter = UINT32_MAX;
    if (s_max_iter == UINT32_MAX)
        s_max_iter = (s_nb_agents * (s_nb_agents-1)) / 2;

    s_max_iter = 100000 / 7;

    const Instant start_time = instant_now();
    step_gi_simulation_impl(&s_sim, s_max_iter);
    const Instant end_time = instant_now();
    const f32 duration = duration_since(end_time, start_time);

    const f32 target_dt = 1.f / 144.f; // 60 FPS, but take into account other tasks to do per frame.
    const f32 target_step_rate = 1.f / (duration / target_dt);

    (void) target_step_rate;
    //s_max_iter *= target_step_rate;
    //logi("s_max_iter = %u\n", s_max_iter);
}

//
//
//

// computes F12
static f32 compute_disk_to_disk_vf_montecarlo(m4 mm1, f32 a1, m4 mm2, f32 a2, u32 NX, u32 NY) {
    const v3 n1 = v3_normalize(m4_mul_direction(mm1, -v3_001()));
    const v3 n2 = v3_normalize(m4_mul_direction(mm2, -v3_001()));

    const f32 da1 = a1 / (NX * NY);
    const f32 da2 = a2 / (NX * NY);

    f32 sum1 = 0.f;
    for (u32 iy1 = 0; iy1 < NY; ++iy1)
    for (u32 ix1 = 0; ix1 < NX; ++ix1)
    {
        const f32 x1 = cosf(2.f * PI * iy1 / (f32) NY) * ix1 / (f32) NX;
        const f32 y1 = sinf(2.f * PI * iy1 / (f32) NY) * ix1 / (f32) NX;
        const v3 c1 = m4_mul_point(mm1, v3_set(x1, y1, 0.f));

        f32 sum2 = 0.f;
        for (u32 iy2 = 0; iy2 < NY; ++iy2)
        for (u32 ix2 = 0; ix2 < NX; ++ix2)
        {
            const f32 x2 = cosf(2.f * PI * iy2 / (f32) NY) * ix2 / (f32) NX;
            const f32 y2 = sinf(2.f * PI * iy2 / (f32) NY) * ix2 / (f32) NX;
            const v3 c2 = m4_mul_point(mm2, v3_set(x2, y2, 0.f));

            const v3 c1_to_c2 = c2 - c1;
            const f32 distance_squared = v3_magnitude_squared(c1_to_c2);
            const v3 c1_to_c2_normalized = c1_to_c2 / sqrtf(distance_squared);

            const f32 cos_t1 = v3_dot(n1, +c1_to_c2_normalized);
            const f32 cos_t2 = v3_dot(n2, -c1_to_c2_normalized);

            sum2 += da1 * da2 * cos_t1 * cos_t2 / (PI * distance_squared * distance_squared);
        }
        sum1 += sum2;
    }
    return sum1;
}

static void test_view_factor_calcs() {
    // emitter
    const v3 c1 = v3_010() * 10.f;
    const v3 n1 = -v3_010();
    const f32 R1 = 1.f;
    const f32 a1 = PI * R1*R1;
    const Xform xform1 = { .position = c1, .orientation = quat_rotation_from_to(-v3_001(), n1), .scale = v3_broadcast(R1) };
    const m4 mm1 = xform_get_model_matrix(xform1);

    // receiver
    const v3 c2 = v3_zero();
    const v3 n2 = v3_010();
    const f32 R2 = 10.f;
    const f32 a2 = PI * R2*R2;
    const Xform xform2 = { .position = c2, .orientation = quat_rotation_from_to(-v3_001(), n2), .scale = v3_broadcast(R2) };
    const m4 mm2 = xform_get_model_matrix(xform2);

    // exact solution, for coaxial disks
    {
        const f32 H = v3_magnitude(c2 - c1);
        const f32 r1 = R1 / H;
        const f32 r2 = R2 / H;
        const f32 x = 1.f + 1.f / (r1*r1) + r2*r2 / (r1*r1);
        const f32 y = sqrtf(x*x - 4.f * r2*r2 / (r1*r1));
        const f32 f12 = (x - y) / 2.f;
        logi("f12 (exact) = %f\n", (f64) f12);
    }

    // monte carlo
    {
        const f32 f12 = compute_disk_to_disk_vf_montecarlo(mm1, a1, mm2, a2, 8, 8);
        logi("f12 (mtcrl) = %f\n", (f64) f12);
    }

    // our solution
    {
        const f32 ca = max(0.f, v3_dot(n1, +v3_normalize(c2 - c1)));
        const f32 cb = max(0.f, v3_dot(n2, -v3_normalize(c2 - c1)));
        const f32 f12 = 0.5f * R2 * R2 * a1 * ca * cb / (a1 + PI * v3_magnitude_squared(c2 - c1));
        logi("f12 (expmt) = %f\n", (f64) f12);
    }

    // our solution
    {
        const f32 ca = max(0.f, v3_dot(n1, +v3_normalize(c2 - c1)));
        const f32 cb = max(0.f, v3_dot(n2, -v3_normalize(c2 - c1)));
        const f32 f12 = a1 * ca * cb / (a1 + PI * v3_magnitude_squared(c2 - c1));
        logi("f12 (currt) = %f\n", (f64) f12);
    }

    // exit(0);
}

//
//
//
//

void init_egi_demo() {
    test_planar_polygon_view_factors();

    s_objects = fixed_alloc_uninitialized(s_max_objects * sizeof *s_objects, 16, "");
    s_agents = fixed_alloc_uninitialized(s_max_agents * sizeof *s_agents, 16, "");

    init_test_grid_mesh();

    for (u32 i = 0; i < countof(s_test_skybox_cpu_mesh); ++i)
        init_test_box_mesh(&s_test_skybox_cpu_mesh[i], &s_test_skybox_gpu_mesh[i], 1u << i, 1u << i, true);

    for (u32 i = 0; i < countof(s_test_box_cpu_mesh); ++i)
        init_test_box_mesh(&s_test_box_cpu_mesh[i], &s_test_box_gpu_mesh[i], 1u << i, 1u << i, false);

    const GxBufferParams normal_dbg_bp = {
        .name = "EGI demo: Normal debugger buffer",
        .size = s_max_agents * sizeof(EgiAgentTbn),
        .type = GX_VERTEX_BUFFER,
        .flags = GX_BUF_DYNAMIC_BIT | GX_BUF_DRAW_BIT,
    };
    s_tbn.gpu_buffer = gx_create_buffer(&normal_dbg_bp);

    const GxBufferParams mmbp = {
        .name = "EGI demo: Instanced model matrices",
        .size = s_max_agents * sizeof(m4),
        .type = GX_VERTEX_BUFFER,
        .flags = GX_BUF_DYNAMIC_BIT | GX_BUF_DRAW_BIT,
    };
    s_instanced_model_matrix_gpu_buffer = gx_create_buffer(&mmbp);

    const GxBufferParams cbp = {
        .name = "EGI demo: Instanced colors",
        .size = s_max_agents * sizeof(v4),
        .type = GX_VERTEX_BUFFER,
        .flags = GX_BUF_DYNAMIC_BIT | GX_BUF_DRAW_BIT,
    };
    s_instanced_color_gpu_buffer = gx_create_buffer(&cbp);

    const quat ground_rotation = quat_rotation_angle_axis(PI_2, v3_100());
    const quat ground_rotation_flip = quat_rotation_angle_axis(PI_2 + PI, v3_100());

    const f32 skybox_scale = 20.f;
    assert_soft(skybox_scale <= s_max_scale);

    const f32 purely_emissive_metallic = 0.f;
    const f32 purely_emissive_roughness = 1.f;
    const f32 test_metallic = 0.f;
    const f32 test_roughness = clamp(0.5f, 0.05f, 1.0f);

    // #define MANY_SURFELS

    assert_fatal(s_nb_objects < s_max_objects);
    s_objects[s_nb_objects++] = (EgiDemoObject) {
        .name = "egi_skybox",
        .base_color = { 0, 0, 0, 1 },
        .emissive = { 0.1f, 0.3f, 1.0f },
        .metallic = purely_emissive_metallic,
        .roughness = purely_emissive_roughness,
        #ifdef MANY_SURFELS
        .cpu_mesh = &s_test_skybox_cpu_mesh[3],
        .gpu_mesh = s_test_skybox_gpu_mesh[3],
        #else
        .cpu_mesh = &s_test_skybox_cpu_mesh[2],
        .gpu_mesh = s_test_skybox_gpu_mesh[2],
        #endif
        .xform = {
            .position = v3_zero(),
            .orientation = quat_identity_init(),
            .scale = v3_broadcast(skybox_scale) * 2.f,
        },
    };

    assert_fatal(s_nb_objects < s_max_objects);
    s_objects[s_nb_objects++] = (EgiDemoObject) {
        .name = "egi_sun",
        .base_color = { 0, 0, 0, 1 },
        .emissive = v3_set(1.f, 1.f, 1.f) * 1.f,
        .metallic = purely_emissive_metallic,
        .roughness = purely_emissive_roughness,
        .is_convex = true,
        .cpu_mesh = get_stock_cpu_mesh(STOCK_MESH_SPHERE_TGNP_LOD1),
        .gpu_mesh = get_stock_mesh(STOCK_MESH_SPHERE_TGNP_LOD1),
        .xform = {
            .position = v3_set(0, 5, 3),
            .orientation = quat_identity_init(),
            .scale = v3_one() * 2.f * 2.5f,
        },
    };

    const f32 obstacle_y = 2.f;
    const f32 obstacle_y_epsilon = 0.05f;

    assert_fatal(s_nb_objects < s_max_objects);
    s_objects[s_nb_objects++] = (EgiDemoObject) {
        .name = "egi_obstacle_facing_up",
        .base_color = { 0.8f, 0.0f, 0.f, 1 },
        .emissive = { 0, 0, 0 },
        .metallic = test_metallic,
        .roughness = test_roughness,
        .is_convex = true,
        .cpu_mesh = &s_test_grid_cpu_mesh[3],
        .gpu_mesh = s_test_grid_gpu_mesh[3],
        .xform = {
            .position = v3_set(0, obstacle_y + obstacle_y_epsilon, 4),
            .scale = v3_broadcast(3) * 2.f,
            .orientation = ground_rotation,
        },
    };

    assert_fatal(s_nb_objects < s_max_objects);
    s_objects[s_nb_objects++] = (EgiDemoObject) {
        .name = "egi_obstacle_facing_down",
        .base_color = { 0.8f, 0.0f, 0.f, 1 },
        .emissive = { 0, 0, 0 },
        .metallic = test_metallic,
        .roughness = test_roughness,
        .is_convex = true,
        .cpu_mesh = &s_test_grid_cpu_mesh[3],
        .gpu_mesh = s_test_grid_gpu_mesh[3],
        .xform = {
            .position = v3_set(0, obstacle_y - obstacle_y_epsilon, 4),
            .scale = v3_broadcast(3) * 2.f,
            .orientation = ground_rotation_flip,
        },
    };

    assert_fatal(s_nb_objects < s_max_objects);
    s_objects[s_nb_objects++] = (EgiDemoObject) {
        .name = "egi_ground_facing_up",
        .base_color = { 0.25f, 0.8f, 0.0f, 1 },
        .emissive = { 0, 0, 0 },
        .metallic = test_metallic,
        .roughness = test_roughness,
        .is_convex = true,
        #ifdef MANY_SURFELS
        .cpu_mesh = &s_test_grid_cpu_mesh[6],
        .gpu_mesh = s_test_grid_gpu_mesh[6],
        #else
        .cpu_mesh = &s_test_grid_cpu_mesh[5],
        .gpu_mesh = s_test_grid_gpu_mesh[5],
        #endif
        .xform = {
            .position = v3_zero(),
            .scale = v3_broadcast(19.f) * 2.f,
            .orientation = ground_rotation,
        },
    };

    s_objects[s_nb_objects++] = (EgiDemoObject) {
        .name = "egi_ground_facing_down",
        .base_color = { 0.25f, 0.8f, 0.0f, 1 },
        .emissive = { 0, 0, 0 },
        .metallic = test_metallic,
        .roughness = test_roughness,
        .is_convex = true,
        #ifdef MANY_SURFELS
        .cpu_mesh = &s_test_grid_cpu_mesh[4],
        .gpu_mesh = s_test_grid_gpu_mesh[4],
        #else
        .cpu_mesh = &s_test_grid_cpu_mesh[3],
        .gpu_mesh = s_test_grid_gpu_mesh[3],
        #endif
        .xform = {
            .position = -v3_010() * 0.05f,
            .scale = v3_broadcast(19.f) * 2.f,
            .orientation = ground_rotation_flip,
        },
    };

    assert_fatal(s_nb_objects < s_max_objects);
    s_objects[s_nb_objects++] = (EgiDemoObject) {
        .name = "egi_box",
        .base_color = { 0.8f, 0.4f, 0.0f, 1 },
        .emissive = { 0, 0, 0 },
        .metallic = test_metallic,
        .roughness = test_roughness,
        .is_convex = true,
        .cpu_mesh = &s_test_box_cpu_mesh[2],
        .gpu_mesh = s_test_box_gpu_mesh[2],
        .xform = {
            .position = v3_set(0, 1.3f, 0),
            .orientation = quat_identity_init(),
            .scale = v3_broadcast(1.f) * 2.f,
        },
    };

    for (u32 i = 0; i < s_nb_objects; ++i) {
        EgiDemoObject* obj = &s_objects[i];
        const GxBufferParams bp = {
            .name = "EGI demo: obj extra VBO",
            .size = obj->cpu_mesh->nb_vertices * sizeof(AgentExtraVertexData),
            .type = GX_VERTEX_BUFFER,
            .flags = GX_BUF_DYNAMIC_BIT | GX_BUF_DRAW_BIT,
        };
        obj->extra_vbo = gx_create_buffer(&bp);
    }

    // Set camera to reference, for now
    if (hf_get_editor_tab() == HF_EDITOR_TAB__MAIN_GAME) {
        hf_set_editor_mode(true);
        set_camera_to_match_reference();
    }

    restart_gi_simulation();

    test_view_factor_calcs();
}

void deinit_egi_demo() {
    // Nothing here yet...
}

//
//
//

static u32 debug_ui_v3(const char* name, v3* v, f32 min, f32 max) {
    u32 change_mask = 0;
    for (u32 i = 0; i < 3; ++i) {
        char label[128];
        snprintf(label, sizeof label, "%s[%u]", name, i);
        change_mask |= debug_ui_slider_f32(label, &v3_ptr(v)[i], min, max) << i;
    }
    return change_mask;
}

static void draw_egi_demo_object_debug_ui(EgiDemoObject* obj) {
    (void) obj;

    debug_ui_checkbox("Use debug color", &obj->use_debug_color);
    debug_ui_checkbox("Skip drawing", &obj->skip_drawing);

    const bool position_changed = !!debug_ui_v3("position", &obj->xform.position, -s_max_pos, s_max_pos);

    bool scale_changed = false;
    if (debug_ui_slider_f32("scale (uniform)", v3_ptr(&obj->xform.scale), s_min_scale, s_max_scale)) {
        obj->xform.scale = v3_broadcast(obj->xform.scale[0]);
        scale_changed = true;
    }

    scale_changed |= !!debug_ui_v3("scale", &obj->xform.scale, s_min_scale, s_max_scale);

    if (position_changed || scale_changed)
        on_object_xform_changed(obj);

    bool material_changed = false;
    material_changed |= !!debug_ui_v3("base_color", &obj->base_color, 0.f, 1.f);
    material_changed |= !!debug_ui_v3("emissive", &obj->emissive, 0.f, 3.f);
    if (material_changed)
        on_object_material_changed(obj);
}

static void draw_egi_demo_debug_ui() {
    const f64 fdt = 1. / 60.;

    debug_ui_text_fmt("Iterations: %u%s", s_nb_iterations_done, s_sim.paused ? " (PAUSED)" : "");
    debug_ui_text_fmt("real: %u xfers (~%u pts)", (s_nb_agents*s_nb_agents) / 2, s_nb_agents);
    debug_ui_text_fmt("sim : %u xfers (~%u pts)", s_last_iteration_stats.nb_calculations, (u32) sqrtf(2.f * s_last_iteration_stats.nb_calculations));
    debug_ui_text_fmt("sim duration : %.2f ms (%.2f frames at 60FPS)", (f64) s_last_iteration_stats.simulation_duration * 1000., (f64) s_last_iteration_stats.simulation_duration / fdt);
    debug_ui_text_fmt("color upload : %.2f ms (%.2f frames at 60FPS)", (f64) s_last_iteration_stats.color_upload_duration * 1000., (f64) s_last_iteration_stats.color_upload_duration / fdt);

    if (debug_ui_button("Set camera to match reference"))
        set_camera_to_match_reference();

    if (debug_ui_button("Restart simulation"))
        restart_gi_simulation();

    if (debug_ui_button("Run next iteration"))
        step_gi_simulation();

    /**/ if (!s_sim.paused && debug_ui_button("Pause")) s_sim.paused = true;
    else if ( s_sim.paused && debug_ui_button("Resume")) s_sim.paused = false;

    debug_ui_checkbox("Use debug color", &s_use_debug_color);
    debug_ui_checkbox("Skip object drawing", &s_skip_object_drawing);
    debug_ui_checkbox("Skip surfel drawing", &s_skip_surfel_drawing);

    debug_ui_checkbox("Draw TBN", &s_tbn.should_draw);
    if (s_tbn.should_draw) {
        debug_ui_checkbox("TBN: Irradiance transform", &s_tbn.use_irradiance_sampling_transform);
        debug_ui_v3("TBN: Gizmo offset", &s_tbn.gizmo_offset, 0.01f, 2.f);
        debug_ui_v3("TBN: Gizmo scale", &s_tbn.gizmo_scale, 0.01f, 2.f);
    }

    /**/ if (get_area_factor_v4 == get_area_factor_v4_mbunnell && debug_ui_button("ViewFactor: Use principled")) get_area_factor_v4 = get_area_factor_v4_principled;
    else if (get_area_factor_v4 == get_area_factor_v4_principled && debug_ui_button("ViewFactor: Use M. Bunnel approximation")) get_area_factor_v4 = get_area_factor_v4_mbunnell;

    debug_ui_v3("Shadow intensity factor", &s_shadow_intensity_factor, 0.f, 3.f);
    debug_ui_slider_f32("Irradiance intensity factor", &s_irradiance_intensity_factor, 0.f, 4.f);
    debug_ui_slider_f32("Irradiance view factor exponent", &s_irradiance_viewfactor_exp, 1.f, 20.f);
    debug_ui_slider_f32("Focus exp", &s_focus_exp, 0.2f, 30.f);
    debug_ui_slider_f32("Reflection exp0", &s_refl_exp0, 1.f, 20.f);
    debug_ui_slider_f32("Reflection exp1", &s_refl_exp1, 1.f, 20.f);
    debug_ui_checkbox("Should average irradiance", &s_should_average_irradiance);
    debug_ui_checkbox("Should average reflected", &s_should_average_reflected);

    if (debug_ui_slider_f32("Disk radius factor", &s_disk_debugdraw_radius_scale, 0.01f, 3.f))
        upload_instanced_model_matrices(0, s_nb_agents);

    if (debug_ui_slider_f32("Color factor", &s_debugdraw_color_factor, -10.f, 10.f))
        upload_instanced_colors(0, s_nb_agents);

    if (debug_ui_button("Set color factor to 1")) {
        s_debugdraw_color_factor = 1.f;
        upload_instanced_colors(0, s_nb_agents);
    }

    if (debug_ui_button("Set color factor to -1")) {
        s_debugdraw_color_factor = -1.f;
        upload_instanced_colors(0, s_nb_agents);
    }

    for (u32 i = 1; i < AGENT_COLOR_ID_COUNT; ++i) {
        const char* s = agent_color_id_to_string(i);
        if (s_color_id == (AgentColorID) i)
            debug_ui_text_fmt("%s", s);
        else if (debug_ui_button(s)) {
            s_color_id = i;
            upload_instanced_colors(0, s_nb_agents);
        }
    }

    for (u32 i = 0; i < s_nb_objects; ++i) {
        EgiDemoObject* obj = &s_objects[i];

        if (debug_ui_start_section(obj->name)) {
            draw_egi_demo_object_debug_ui(obj);
        }
        debug_ui_end_section();
    }
}

//
//
//
//
//

static bool s_draw = true;

void draw_egi_demo() {
    if (hf_get_editor_tab() != HF_EDITOR_TAB__MAIN_GAME)
        return;

    debug_ui_checkbox("Draw Experimental GI demo", &s_draw);
    if (!s_draw)
        return;

    if (debug_ui_start_section("Experimental GI demo")) {
        draw_egi_demo_debug_ui();
    }
    debug_ui_end_section();

    step_gi_simulation_partial();

    GxfiScene* s = g_hf_main_scene;

    for (u32 i = 0; i < s_nb_objects; ++i)
        draw_egi_demo_object(s, &s_objects[i]);

    draw_all_egi_demo_agents(s, s_nb_agents);
    draw_tbn(s, s_nb_agents);
}
