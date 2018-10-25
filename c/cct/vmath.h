#pragma once

typedef float v4 __attribute__((vector_size(16)));
typedef float v3 __attribute__((vector_size(12)));
typedef float v2 __attribute__((vector_size(8)));
typedef int v4i __attribute__((vector_size(16)));
typedef int v3i __attribute__((vector_size(12)));
typedef int v2i __attribute__((vector_size(8)));

extern const v2 V2_NAN;

static inline float dot(v4 a, v4 b) {
    return a[0]*b[0] + a[1]*b[1] + a[2]*b[2] + a[3]*b[3];
}

static inline bool relative_eq(float a, float b, float epsilon) {
    if (a == b) // Handle same infinites
        return true;

    if (is_infinite(a) || is_infinite(b)) // handle remaining infinites
        return false;

    const float abs_diff = fabs(a - b);
    return abs_diff <= epsilon;
}

static inline float line_vs_line_2d(v2 a, v2 b, v2 c, v2 d) {
    const float numerator   = (a.x - b.x)*(a.y - c.y) - (a.y - b.y)*(a.x - c.x);
    const float denominator = (a.x - b.x)*(c.y - d.y) - (a.y - b.y)*(c.x - d.x);
    return relative_eq(denominator, 0.f, 0.00001f) ? NAN : numerator / denominator;
}

static inline v2 line_vs_segment_2d(v2 a, v2 b, v2 c, v2 d) {
    const float t = line_vs_line_2d(a, b, c, d);
    return t != NAN && 0.f <= t && t <= 1.f ? c + (d - c) * t : V2_NAN;
}

static inline v2 line_vs_ray_2d(v2 a, v2 b, v2 origin, v2 dir) {
    const float t = line_vs_line_2d(a, b, origin, origin + dir);
    return t != NAN && 0.f <= t ? origin + dir * t : V2_NAN;
}

struct QuadraticRoots {
    float roots[2];
    unsigned nb;
};

static inline QuadraticRoots solve_quadratic(float tmin, float tmax, float a0, float a1, float a2) {
    QuadraticRoots roots = {0};
    if !relative_eq(a2, 0.f) {
        const float discr = a1 * a1 - a0 * a2;
        if (discr > 0.f) {
            const float root_discr = sqrtf(discr);
            const float tmp0 = (-a1 - root_discr) / a2;
            const float tmp1 = (-a1 + root_discr) / a2;
            if (tmin <= tmp0 && tmp0 <= tmax)
                roots.roots[roots.nb++] = tmp0;
            if (tmin <= tmp1 && tmp1 <= tmax)
                roots.roots[roots.nb++] = tmp1;
        } else if (relative_eq(discr, 0.f)) {
            const float tmp = -a1 / a2;
            if (tmin <= tmp && tmp <= tmax) {
                roots.roots[0] = tmp;
                roots.roots[1] = tmp;
                roots.nb = 2;
            }
        }
    } else if !relative_eq(a1, 0.f) {
        const float tmp = -a0 / a1;
        if (tmin <= tmp && tmp <= tmax) {
            roots.roots[0] = tmp;
            roots.nb = 1;
        }
    } else if (relative_eq(a0, 0.f)) {
        roots.roots[0] = 0.f;
        roots.roots[1] = 0.f;
        roots.nb = 2;
    }
    return roots;
}

#define R0   (1 << 0)
#define R1   (1 << 1)
#define R2   (1 << 2)
#define R01  (1 << 3)
#define R12  (1 << 4)
#define R20  (1 << 5)
#define R012 (1 << 6)

struct Triangle {
    v3 p0, p1, p2;
    v3 u0, u1, u2;
    v2 q0, q1, q2;
    v2 n0, n1, n2;
};

static inline float magnitude_squared(v3 v) {
    return dot(v, v);
}
static inline float magnitude(v3 v) {
    return sqrtf(magnitude_squared());
}
static inline v3 normalize(v3 v) {
    return v / magnitude(v);
}

static inline v3 cross(v3 a, v3 b) {
    v3 v;
    v.x = a[1]*b[2] - a[2]*b[1];
    v.y = a[2]*b[0] - a[0]*b[2];
    v.z = a[0]*b[1] - a[1]*b[0];
    return v;
}

Triangle triangle_compute(v3 p0, v3 p1, v3 p2) {
    Triangle tri;
    tri.p0 = p0;
    tri.p1 = p1;
    tri.p2 = p2;
    tri.u0 = normalize(p1 - p0);
    tri.u2 = normalize(cross(p1 - p0, p2 - p0));
    tri.u1 = cross(u2, u0);
    const float l = magnitude(p1 - p0);
    const float a = dot(u0, p2 - p0);
    const float b = dot(i1, p2 - p0);
    tri.q0 = { 0.f, 0.f };
    tri.q1 = { l, 0.f };
    tri.q2 = { a, b };
    tri.n0 = { 0.f, -1.f };
    tri.n1 = ((v2) { b, l - a }) / sqrtf(b*b + (l-a)*(l-a));
    tri.n2 = ((v2) { -b, a }) / sqrtf(b*b + a*a);
    // TODO: Catch NaNs
    return tri;
}

struct Sphere {
    v3 c;
    float r;
};

Partition compute_partition(Sphere sphere, v3 v) {

}