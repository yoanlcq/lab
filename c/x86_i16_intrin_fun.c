#include <stdint.h>
#include <x86intrin.h>

typedef __m128i fe_u16v4_xmm;
typedef __m128i fe_i16v4_xmm;

#if 0
/* The actual generated assembly has been observed on x86-64 Linux GCC (no optimizations).
 * Every part of this function (even the 'register'-qualified params)
 * is required. 
 * Couldn't get the generated assembly to be as nice using intrinsics
 * and registers variables. */
/* __attribute__((always_inline,artificial)) */
static inline fe_u16v4_xmm fe_u16v4_absdiff(register fe_u16v4_xmm xmm0, register fe_u16v4_xmm xmm1) {
    asm("movdqa  xmm2, %0\n\t"
        "psubusw %0, %1  \n\t" // _mm_subs_epu16()
        "psubusw %1, xmm2\n\t"
        "por     %0, %1  \n\t" // _mm_or_si128()
        : "+x" (xmm0) : "x" (xmm1) : "xmm2");
    return xmm0;
}
#else
/* This one generates the expected assembly with GCC's -O3, aside from
 * making that function indeed inline. */
static inline fe_u16v4_xmm fe_u16v4_absdiff(fe_u16v4_xmm a, fe_u16v4_xmm b) {
    return _mm_or_si128(_mm_subs_epu16(a,b), _mm_subs_epu16(b,a));
}
#endif
#if 0
/* __attribute__((always_inline,artificial)) */
static inline fe_u16v4_xmm fe_u16v4_abs(register fe_u16v4_xmm xmm0) {
    register fe_u16v4_xmm xmm1 asm("xmm1");
    asm("pxor   %0, %0\n\t"
        "psubw  %0, %1\n\t"
        "pmaxsw %0, %1\n\t"
        : "+x" (xmm1) : "x" (xmm0));
    return xmm1;
}
#else
static inline fe_i16v4_xmm fe_i16v4_abs(fe_i16v4_xmm x) {
    return _mm_max_epi16(_mm_sub_epi16(_mm_setzero_si128(), x), x);
}
#endif

static inline fe_i16v4_xmm fe_i16v4_clamp(fe_i16v4_xmm x, fe_i16v4_xmm lo, fe_i16v4_xmm hi) {
    return _mm_min_epi16(_mm_max_epi16(x, lo), hi);
}

static inline uint16_t u16_absdiff(uint16_t a, uint16_t b) {
    return _mm_cvtsi128_si32(fe_u16v4_absdiff(_mm_cvtsi64_si128(a), _mm_cvtsi64_si128(b)));
}
static inline int16_t i16_abs(int16_t a) {
    return _mm_cvtsi128_si32(fe_i16v4_abs(_mm_cvtsi64_si128(a)));
}
static inline int16_t i16_clamp(int16_t a, int16_t lo, int16_t hi) {
    return _mm_cvtsi128_si32(fe_i16v4_clamp(
        _mm_set_epi16( a, a, a, a, a, a, a, a),
        _mm_set_epi16(lo,lo,lo,lo,lo,lo,lo,lo),
        _mm_set_epi16(hi,hi,hi,hi,hi,hi,hi,hi)
    ));
}

/*
_mm_stream_pi    => MultiMedia_stream_PackedInteger
_mm_stream_pd    => MultiMedia_stream_PackedDoubleprecision
_mm_stream_si128 => MultiMedia_stream_SingleInteger128
_mm_sqrt_ps      => MultiMedia_sqrt_PackedSingleprecision
_mm_sqrt_ss      => MultiMedia_sqrt_SingleSingleprecision
_mm_mul_epu32    => Multimedia_mul_ExtractPackedUnsigned32
_mm_mul_su32     => Multimedia_mul_SingleUnsigned32
*/

#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <time.h>
int main(void) {
    srand(time(NULL)); 
    int16_t val = rand();
    printf("|%hd| = %hd\n", -val, i16_abs(-val));
    printf("| %hd| = %hd\n", val, i16_abs(val));
    uint16_t a = rand();
    uint16_t b = rand();
    printf("|%hd-%hd| = %hd\n", a, b, u16_absdiff(a, b));
    printf("|%hd-%hd| = %hd\n", b, a, u16_absdiff(b, a));
    return 0;
}
