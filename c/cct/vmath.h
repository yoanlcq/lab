#pragma once

#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <assert.h>

//
// DEFS
//

#define countof(x) (sizeof x / sizeof *x)

//
// FLOAT MATH
//

static inline bool relative_eq(float a, float b, float epsilon) {
    if (a == b) // Handle same infinites
        return true;

    if (isinf(a) || isinf(b)) // handle remaining infinites
        return false;

    const float abs_diff = fabs(a - b);
    return abs_diff <= epsilon;
}


//
// VECTOR TYPES
//

typedef float v4 __attribute__((vector_size(16)));
typedef float v3 __attribute__((vector_size(16)));
typedef float v2 __attribute__((vector_size(8)));
typedef int v4i __attribute__((vector_size(16)));
typedef int v3i __attribute__((vector_size(16)));
typedef int v2i __attribute__((vector_size(8)));

//
// VECTOR CONSTS
//

#define V2_NAN ((v2) { NAN, NAN })

//
// VECTOR OPS
//

static inline float v4_dot(v4 a, v4 b) { return a[0]*b[0] + a[1]*b[1] + a[2]*b[2] + a[3]*b[3]; }
static inline float v3_dot(v3 a, v3 b) { return a[0]*b[0] + a[1]*b[1] + a[2]*b[2]; }
static inline float v2_dot(v2 a, v2 b) { return a[0]*b[0] + a[1]*b[1]; }
static inline float v4_magnitude_squared(v4 v) { return v4_dot(v, v); }
static inline float v3_magnitude_squared(v3 v) { return v3_dot(v, v); }
static inline float v2_magnitude_squared(v2 v) { return v2_dot(v, v); }
static inline float v4_magnitude(v4 v) { return sqrtf(v4_magnitude_squared(v)); }
static inline float v3_magnitude(v3 v) { return sqrtf(v3_magnitude_squared(v)); }
static inline float v2_magnitude(v2 v) { return sqrtf(v2_magnitude_squared(v)); }
static inline v4 v4_normalize(v4 v) { return v / v4_magnitude(v); }
static inline v3 v3_normalize(v3 v) { return v / v3_magnitude(v); }
static inline v2 v2_normalize(v2 v) { return v / v2_magnitude(v); }

static inline v3 v3_cross(v3 a, v3 b) {
    return (v3) {
        a[1]*b[2] - a[2]*b[1],
        a[2]*b[0] - a[0]*b[2],
        a[0]*b[1] - a[1]*b[0]
    };
}

static inline float v2_determine_side(v2 c, v2 a, v2 b) {
    return (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
}

static inline float v2_line_vs_line(v2 a, v2 b, v2 c, v2 d) {
    const float numerator   = (a[0] - b[0])*(a[1] - c[1]) - (a[1] - b[1])*(a[0] - c[0]);
    const float denominator = (a[0] - b[0])*(c[1] - d[1]) - (a[1] - b[1])*(c[0] - d[0]);
    return relative_eq(denominator, 0.f, 0.00001f) ? NAN : numerator / denominator;
}

static inline v2 v2_line_vs_seg(v2 a, v2 b, v2 c, v2 d) {
    const float t = v2_line_vs_line(a, b, c, d);
    return t != NAN && 0.f <= t && t <= 1.f ? c + (d - c) * t : V2_NAN;
}

static inline v2 v2_line_vs_ray(v2 a, v2 b, v2 origin, v2 dir) {
    const float t = v2_line_vs_line(a, b, origin, origin + dir);
    return t != NAN && 0.f <= t ? origin + dir * t : V2_NAN;
}
