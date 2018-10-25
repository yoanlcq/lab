#include "cct.h"

CCT_Triangle triangle_compute(v3 p0, v3 p1, v3 p2) {
    const v3 u0 = v3_normalize(p1 - p0);
    const v3 u2 = v3_normalize(v3_cross(p1 - p0, p2 - p0));
    const v3 u1 = v3_cross(u2, u0);
    const float l = v3_magnitude(p1 - p0);
    const float a = v3_dot(u0, p2 - p0);
    const float b = v3_dot(u1, p2 - p0);
    const v2 q0 = { 0.f, 0.f };
    const v2 q1 = { l, 0.f };
    const v2 q2 = { a, b };
    const v2 n0 = { 0.f, -1.f };
    const v2 n1 = (v2) { b, l - a } / sqrtf(b*b + (l-a)*(l-a));
    const v2 n2 = (v2) { -b, a } / sqrtf(b*b + a*a);

    // TODO: Catch NaNs

    return (CCT_Triangle) {
        .p0 = p0, .p1 = p1, .p2 = p2,
        .u0 = u0, .u1 = u1, .u2 = u2,
        .q0 = q0, .q1 = q1, .q2 = q2,
        .n0 = n0, .n1 = n1, .n2 = n2,
    };
}

typedef struct QuadraticRoots {
    float roots[2];
    unsigned nb;
} QuadraticRoots;

static QuadraticRoots solve_quadratic(float tmin, float tmax, float a0, float a1, float a2) {
    QuadraticRoots roots;
    if (!relative_eq(a2, 0.f, 0.000001f)) {
        const float discr = a1 * a1 - a0 * a2;
        if (discr > 0.f) {
            const float root_discr = sqrtf(discr);
            const float tmp0 = (-a1 - root_discr) / a2;
            const float tmp1 = (-a1 + root_discr) / a2;
            roots.nb = 0;
            if (tmin <= tmp0 && tmp0 <= tmax)
                roots.roots[roots.nb++] = tmp0;
            if (tmin <= tmp1 && tmp1 <= tmax)
                roots.roots[roots.nb++] = tmp1;
        } else if (relative_eq(discr, 0.f, 0.000001f)) {
            const float tmp = -a1 / a2;
            if (tmin <= tmp && tmp <= tmax) {
                roots.roots[0] = tmp;
                roots.roots[1] = tmp;
                roots.nb = 2;
            }
        }
    } else if (!relative_eq(a1, 0.f, 0.000001f)) {
        const float tmp = -a0 / a1;
        if (tmin <= tmp && tmp <= tmax) {
            roots.roots[0] = tmp;
            roots.nb = 1;
        }
    } else if (relative_eq(a0, 0.f, 0.000001f)) {
        roots.roots[0] = 0.f;
        roots.roots[1] = 0.f;
        roots.nb = 2;
    }
    return roots;
}


enum Region {
    R0    = 1 << 0,
    R1    = 1 << 1,
    R2    = 1 << 2,
    R01   = 1 << 3,
    R12   = 1 << 4,
    R20   = 1 << 5,
    R012  = 1 << 6
};

// Point-in-convex-region test, or half-space tests
static unsigned get_containing_region(const CCT_Triangle* tri, v2 k) {
    const float s01 = v2_determine_side(k, tri->q0, tri->q1);
    const float s12 = v2_determine_side(k, tri->q1, tri->q2);
    const float s20 = v2_determine_side(k, tri->q2, tri->q0);

    if (s01 >= 0.f && s12 >= 0.f && s20 >= 0.f)
        return R012;

    const float q0_n0_side = v2_determine_side(k, tri->q0, tri->q0 + tri->n0);
    const float q0_n2_side = v2_determine_side(k, tri->q0, tri->q0 + tri->n2);
    const float q1_n0_side = v2_determine_side(k, tri->q1, tri->q1 + tri->n0);
    const float q1_n1_side = v2_determine_side(k, tri->q1, tri->q1 + tri->n1);
    const float q2_n1_side = v2_determine_side(k, tri->q2, tri->q2 + tri->n1);
    const float q2_n2_side = v2_determine_side(k, tri->q2, tri->q2 + tri->n2);

    if (q0_n0_side <= 0.f && q0_n2_side >= 0.f) return R0;
    if (q1_n1_side <= 0.f && q1_n0_side >= 0.f) return R1;
    if (q2_n2_side <= 0.f && q2_n1_side >= 0.f) return R2;
    if (q0_n0_side >= 0.f && q1_n0_side <= 0.f && s01 <= 0.f) return R01;
    if (q1_n1_side >= 0.f && q2_n1_side <= 0.f && s12 <= 0.f) return R12;
    if (q2_n2_side >= 0.f && q0_n2_side <= 0.f && s20 <= 0.f) return R20;

    assert(0);
}

typedef struct Element {
    float min, max;
    unsigned region;
} Element;

typedef struct Partition {
    Element elements[7];
    unsigned nb_elements;
} Partition;

typedef struct PartitionPoint {
    v2 p;
    unsigned region;
    float t;
} PartitionPoint;

static int compare_partition_point_t_values(const void* pa, const void* pb) {
    const PartitionPoint* a = pa;
    const PartitionPoint* b = pb;
    return a->t - b->t;
}

static Partition compute_partition(const CCT_Triangle* tri, const CCT_Sphere* sphere, v3 v) {
    const v3 p0_to_c = sphere->c - tri->p0;
    const v2 k = { v3_dot(tri->u0, p0_to_c), v3_dot(tri->u1, p0_to_c) };
    const v2 w = { v3_dot(tri->u0, v), v3_dot(tri->u1, v) };
    const float w_mag = v2_magnitude(w);

    if (relative_eq(w_mag, 0.f, 0.0001f)) {
        return (Partition) { .nb_elements = 1, .elements = {{ .min = -INFINITY, .max = INFINITY, .region = get_containing_region(tri, k) }} };
    }

    const v2 w_norm = w / w_mag;

    const PartitionPoint i01   = { .p = v2_line_vs_seg(k, k+w, tri->q0, tri->q1), .region = R01 | R012, .t = NAN, };
    const PartitionPoint i12   = { .p = v2_line_vs_seg(k, k+w, tri->q1, tri->q2), .region = R12 | R012, .t = NAN, };
    const PartitionPoint i20   = { .p = v2_line_vs_seg(k, k+w, tri->q2, tri->q0), .region = R20 | R012, .t = NAN, };
    const PartitionPoint iq0n0 = { .p = v2_line_vs_ray(k, k+w, tri->q0, tri->n0), .region = R0  | R01 , .t = NAN, };
    const PartitionPoint iq0n2 = { .p = v2_line_vs_ray(k, k+w, tri->q0, tri->n2), .region = R0  | R20 , .t = NAN, };
    const PartitionPoint iq1n0 = { .p = v2_line_vs_ray(k, k+w, tri->q1, tri->n0), .region = R1  | R01 , .t = NAN, };
    const PartitionPoint iq1n1 = { .p = v2_line_vs_ray(k, k+w, tri->q1, tri->n1), .region = R1  | R12 , .t = NAN, };
    const PartitionPoint iq2n1 = { .p = v2_line_vs_ray(k, k+w, tri->q2, tri->n1), .region = R2  | R12 , .t = NAN, };
    const PartitionPoint iq2n2 = { .p = v2_line_vs_ray(k, k+w, tri->q2, tri->n2), .region = R2  | R20 , .t = NAN, };
    PartitionPoint candidates[] = { i01, i12, i20, iq0n0, iq0n2, iq1n0, iq1n1, iq2n1, iq2n2 };

    PartitionPoint pts[countof(candidates)];
    unsigned nb = 0;
    for (unsigned i = 0; i < countof(candidates); ++i) {
        if (candidates[i].p[0] == NAN)
            continue;

        pts[nb] = candidates[i];
        pts[nb].t = v2_dot(pts[nb].p - k, w_norm) / w_mag;
        ++nb;
    }
    assert(nb >= 2 && "The partitioned line must have at least 2 points by now");
    qsort(pts, nb, sizeof pts[0], compare_partition_point_t_values);

    PartitionPoint orig_pts[countof(pts)];
    memcpy(orig_pts, pts, nb * sizeof pts[0]);

    for (unsigned i = 1; i < nb; /**/) {
        if (relative_eq(pts[i-1].t, pts[i].t, 0.0001f)) {
            pts[i-1].region |= pts[i].region;
            memmove(&pts[i], &pts[i+1], nb - 1 - i); // remove at i
        } else
            ++i;
    }

    Partition partition;
    partition.nb_elements = 0;
    Element elem = { .min = -INFINITY, .max = INFINITY, .region = 0 };

    for (unsigned i = 0; i < nb; ++i) {
        elem.region = i ? pts[i].region & pts[i-1].region : pts[i].region & !pts[i+1].region;
        if (!elem.region) // Seems to happen because of numeric errors. Quite rare.
            continue;

        elem.max = pts[i].t;
        partition.elements[(partition.nb_elements)++] = elem;
        elem.min = elem.max;
    }

    elem.max = INFINITY;
    elem.region = pts[nb - 1].region & !pts[nb - 2].region;
    partition.elements[(partition.nb_elements)++] = elem;

    // TODO: Perform plenty of checks

    return partition;
}


static CCT_Contacts compute_roots(const CCT_Triangle* tri, const CCT_Sphere* sphere, v3 v, const Element* element) {
    CCT_Contacts out;
    out.nb = 0;

    const float radius_sq = sphere->r * sphere->r;
    const float tmin = element->min;
    const float tmax = element->max;

    if (element->region & (R0 | R1 | R2)) {
        v3 p;
        switch(element->region & (R0 | R1 | R2)) {
            case R0: p = tri->p0; break;
            case R1: p = tri->p1; break;
            case R2: p = tri->p2; break;
            default: assert(0); break;
        };
        const v3 diff = sphere->c - p;
        const float a0 = v3_dot(diff, diff) - radius_sq;
        const float a1 = v3_dot(v, diff);
        const float a2 = v3_dot(v, v);
        const QuadraticRoots roots = solve_quadratic(tmin, tmax, a0, a1, a2);
        for (unsigned i = 0; i < roots.nb; ++i)
            out.contacts[(out.nb)++] = (CCT_ContactInfo) { .time = roots.roots[i], .contact = p };

    } else if (element->region & (R01 | R12 | R20)) {
        v3 pa, pb;
        switch(element->region & (R01 | R12 | R20)) {
            case R01: pa = tri->p0; pb = tri->p1; break;
            case R12: pa = tri->p1; pb = tri->p2; break;
            case R20: pa = tri->p2; pb = tri->p0; break;
            default: assert(0); break;
        };
        const v3 diff = sphere->c - pa;
        const v3 edge = pb - pa;
        const float edge_mag_sq = v3_magnitude_squared(edge);
        const float s0 = v3_dot(edge, diff) / edge_mag_sq;
        const float s1 = v3_dot(edge, v) / edge_mag_sq;
        const v3 con_coeff = diff - edge * s0;
        const v3 lin_coeff = v - edge * s1;
        const float a0 = v3_dot(con_coeff, con_coeff) - radius_sq;
        const float a1 = v3_dot(con_coeff, lin_coeff);
        const float a2 = v3_dot(lin_coeff, lin_coeff);
        const QuadraticRoots roots = solve_quadratic(tmin, tmax, a0, a1, a2);
        for (unsigned i = 0; i < roots.nb; ++i)
            out.contacts[(out.nb)++] = (CCT_ContactInfo) { .time = roots.roots[i], .contact = pa + edge * (s1 * roots.roots[i] + s0) };

    } else {
        assert(element->region == R012);
        const v3 diff = sphere->c - tri->p0;
        const float s0 = v3_dot(tri->u2, diff);
        const float s1 = v3_dot(tri->u2, v);
        const float a0 = s0*s0 - radius_sq;
        const float a1 = s0*s1;
        const float a2 = s1*s1;
        const QuadraticRoots roots = solve_quadratic(tmin, tmax, a0, a1, a2);
        for (unsigned i = 0; i < roots.nb; ++i)
            out.contacts[(out.nb)++] = (CCT_ContactInfo) { .time = roots.roots[i], .contact = sphere->c + v * roots.roots[i] - tri->u2 * (s1 * roots.roots[i] + s0) };
    }

    return out;
}

CCT_Contacts get_contact(const CCT_Sphere* sphere, v3 sphere_vel, const CCT_Triangle* tri, v3 tri_vel) {
    const v3 v = sphere_vel - tri_vel;
    const Partition s = compute_partition(tri, sphere, v);

    CCT_Contacts contacts = {
        {   
            { .time = INFINITY },
            { .time = -INFINITY },
        },
        0
    };
    unsigned nb = 0;

    for (unsigned i = 0; i < s.nb_elements; ++i) {
        const CCT_Contacts roots = compute_roots(tri, sphere, v, &s.elements[i]);
        for (unsigned j = 0; j < roots.nb; ++j) {
            if (roots.contacts[j].time < contacts.contacts[0].time) {
                contacts.contacts[0] = roots.contacts[j];
                ++nb;
            }
            if (roots.contacts[j].time > contacts.contacts[1].time) {
                contacts.contacts[1] = roots.contacts[j];
                ++nb;
            }
            if (nb > 2) { // At least 2 roots found, we can leave the loop early. (nb == 2 when first root is found)
                i = s.nb_elements; // Force exit i-loop
                break;
            }
        }
    }
    contacts.nb = nb;
    return contacts;
}


