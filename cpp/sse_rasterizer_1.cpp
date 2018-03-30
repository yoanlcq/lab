#include <vector>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>
#include <assert.h>
#include <string.h>
#include <x86intrin.h>

class XmmVector {
    __m128* ptr; // Buffer
    size_t cap; // Capacity of allocated buffer
    size_t len; // Actually used number of elements

    void check_notnull() {
        if(ptr == nullptr)
            throw std::bad_alloc();
    }

public:
    ~XmmVector() {
        _mm_free(ptr);
    }

    XmmVector(): ptr(nullptr), cap(0), len(0) {}
    XmmVector(size_t cap): ptr((__m128*)_mm_malloc(cap*sizeof*ptr, 16)), cap(cap), len(0) {
        check_notnull();
    }

    // Copy constructor
    XmmVector(const XmmVector& v): cap(v.cap), len(v.len) {
        ptr = (__m128*) _mm_malloc(cap*sizeof*ptr, 16);
        check_notnull();
        memcpy(ptr, v.ptr, len*sizeof*ptr);
    }

    // Copy assignment
    XmmVector& operator=(const XmmVector& v) {
        if(this == &v)
            return *this;
        cap = v.cap;
        len = v.len;
        _mm_free(ptr);
        ptr = (__m128*) _mm_malloc(cap*sizeof*ptr, 16);
        check_notnull();
        memcpy(ptr, v.ptr, len*sizeof*ptr);
        return *this;
    }

    // Move constructor
    XmmVector(XmmVector&& v): ptr(v.ptr), cap(v.cap), len(v.len) {
        v.ptr = nullptr;
    }

    // Move assignment
    XmmVector& operator=(XmmVector&& v) {
        cap = v.cap;
        len = v.len;
        std::swap(ptr, v.ptr);
        return *this;
    }

    const __m128& operator[](size_t i) const { return ptr[i]; }
    __m128& operator[](size_t i) { return ptr[i]; }

    void push(__m128 v) {
        ++len;
        assert(len <= cap); // XXX because there's no _mm_realloc()
        ptr[len-1] = v;
    }

    size_t size() const { return len; }
    size_t capacity() const { return cap; }
};


/// A signed value which tells in which half-space of the line segment `ab` this point lies.
///
/// Returns:
///
/// - ` < 0`: This point lies in the half-space right of segment `ab`.
/// - `== 0`: This point lies in the infinite line along segment `ab`.
/// - ` > 0`: This point lies in the half-space left of segment `ab`.
float determine_side(__m128 c, __m128 a, __m128 b) {
    return (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
}
float signed_area(__m128 a, __m128 b, __m128 c) {
    return determine_side(c, a, b) / 2; 
};
float area(__m128 a, __m128 b, __m128 c) {
    return fabsf(signed_area(a, b, c));
};

struct VShaderInput {
    XmmVector position;
    XmmVector color;
    VShaderInput(size_t size):
        position(size),
        color(size)
        {}
};

struct Framebuffer {
    uint32_t w, h;
    XmmVector color;
    std::vector<float> depth;

    Framebuffer(size_t w, size_t h):
        w(w), h(h),
        color(w*h),
        depth(w*h)
        {}

    void clear_color(__m128 clear_color) {
        for(int i=0 ; i<w*h ; ++i)
            _mm_stream_ps((float*)&color[i], clear_color);
        _mm_sfence();
    }

    void clear_depth(float val = 1 / 0.f) {
        for(int i=0 ; i<w*h ; ++i)
            depth[i] = val;
    }

    void draw_triangles(const VShaderInput& in) {
        assert(in.position.size() % 3 == 0);
        assert(in.color.size() == in.position.size());

        for(int vi=0 ; vi < in.position.size() ; vi += 3) {
            __m128 pa = in.position[vi+0];
            __m128 pb = in.position[vi+1];
            __m128 pc = in.position[vi+2];

            // AABB of triangle
            __m128 pmin = _mm_min_ps(_mm_min_ps(pa, pb), pc);
            __m128 pmax = _mm_max_ps(_mm_max_ps(pa, pb), pc);
            pmin = _mm_setr_ps(w, h, 0, 1) * (pmin + 1) * 0.5;
            pmax = _mm_setr_ps(w, h, 0, 1) * (pmax + 1) * 0.5;
            // Clamp to framebuffer extents, to avoid segfaults
            pmin = _mm_max_ps(pmin, _mm_setzero_ps());
            pmax = _mm_min_ps(pmax, _mm_setr_ps(w, h, 0, 1));

            for(int y=pmin[1] ; y<pmax[1] ; ++y) for(int x=pmin[0] ; x<pmax[0] ; ++x) {
                __m128 p = -1 + 2 * _mm_setr_ps(x, y, 0, 1) / _mm_setr_ps(w-1, h-1, 1, 1);
                __m128 half_space_test = _mm_setr_ps(
                    determine_side(p, pb, pc),
                    determine_side(p, pc, pa),
                    determine_side(p, pa, pb),
                    0.f
                );

                // Are we outside the triangle?
                // test if: la < 0 && lb < 0 && lc < 0
                if(_mm_movemask_ps(half_space_test))
                    continue;

                // We're inside the triangle.
                // Compute barycentric coords (l standing for 'Lambda')
                __m128 l = half_space_test * 0.5f / signed_area(pa, pb, pc);

                // Depth test
                float z = l[0] * pa[2] + l[1] * pb[2] + l[2] * pc[2];
                if(z >= depth[y*w + x])
                    continue;

                depth[y*w + x] = z;
                __m128 ca = in.color[vi+0];
                __m128 cb = in.color[vi+1];
                __m128 cc = in.color[vi+2];
                _mm_stream_ps((float*)&color[y*w + x], l[0] * ca + l[1] * cb + l[2] * cc);
            }
        }
        _mm_sfence();
    }

    void present() const {
        for(int y=0 ; y<h ; ++y) {
            for(int x=0 ; x<w ; ++x) {
                if(depth[y*w + x] >= 1 / 0.f) 
                    putchar('-');
                else {
                    float r = color[y*w + x][0];
                    putchar('A' + r * 25);
                }
            }
            putchar('\n');
        }
    }
};

int main() {
    Framebuffer fb(80, 43);
    VShaderInput in(3);
    in.position.push(_mm_setr_ps(1, 0, 0, 1));
    in.position.push(_mm_setr_ps(-1,  1, 0, 1));
    in.position.push(_mm_setr_ps(-0.5f, -1, 0, 1));
    in.color.push(_mm_setr_ps(1, 0, 0, 1));
    in.color.push(_mm_setr_ps(0.5f, 0, 0, 1));
    in.color.push(_mm_setr_ps(0, 0, 0, 1));

    fb.clear_color(_mm_setr_ps(0, 0, 0, 1));
    fb.clear_depth();
    fb.draw_triangles(in);
    fb.present();

    return EXIT_SUCCESS;
}
