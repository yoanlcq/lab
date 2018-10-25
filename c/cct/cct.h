#pragma once

#include "vmath.h"

typedef struct CCT_ContactInfo {
    v3 contact;
    float time;
} CCT_ContactInfo;

typedef struct CCT_Contacts {
    CCT_ContactInfo contacts[2];
    unsigned nb;
} CCT_Contacts;

typedef struct CCT_Triangle {
    v3 p0, p1, p2;
    v3 u0, u1, u2;
    v2 q0, q1, q2;
    v2 n0, n1, n2;
} CCT_Triangle;

typedef struct CCT_Sphere {
    v3 c;
    float r;
} CCT_Sphere;

CCT_Triangle cct_bake_triangle(v3 p0, v3 p1, v3 p2);
CCT_Contacts cct_cast_sphere_on_triangle(const CCT_Sphere* sphere, v3 sphere_vel, const CCT_Triangle* tri, v3 tri_vel);
