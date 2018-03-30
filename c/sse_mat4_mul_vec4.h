// Fast SSE-based mat4_mul_vec4() implementation.
//
// With GCC, compile with -DPREFER_INTRIN to use the intel intrinsics instead
// of GCC's __attribute__((vector_size)).
//
// This is mostly for illustrative (and teaching) purposes.
// In real-world, perf-demanding use cases,
// you'll probably need to think about how to lay out data such
// that performing this operation for a lot of items at once is efficient.
// It would mostly boil down to how you represent arrays of matrices.
//
// This helped me understand why one would prefer storing matrices as an array
// of column vectors, rather than an array of row vectors (which remains
// more convenient to use in my opinion though)

#pragma once

#if !(defined(__GNUC__) || defined(_MSC_VER))
	#error Unsupported compiler
#endif

#ifdef __GNUC__
	#if defined(__x86_64__) || defined(__SSE__)
		#define HAS_SSE
	#endif
#elif defined(_MSC_VER)
	#if defined(_M_X64) || (defined(_M_IX86) && _M_IX86_FP>=1)
		#define HAS_SSE
	#endif
#endif

#ifndef HAS_SSE
	#error Target CPU does not have SSE
#endif

#if defined(__FMA__)
	#define HAS_FMA3
#endif


#if defined(_MSC_VER) || (defined(__GNUC__) && defined(PREFER_INTRIN))
#define USE_INTRIN 1
#else
#define USE_INTRIN 0
#endif


#if USE_INTRIN

#include <stdint.h>
#ifdef __GNUC__
	#include <x86intrin.h>
#elif defined(_MSC_VER)
	#include <intrin.h>
#endif

typedef __m128 vec4;

typedef struct {
	vec4 cols[4];
} mat4;

#ifndef HAS_FMA3
	#ifdef _mm_fmadd_ps
		#undef _mm_fmadd_ps
	#endif
	#define _mm_fmadd_ps(a,b,c) _mm_add_ps(_mm_mul_ps(a,b),c)
#endif

static inline vec4 mat4_mul_vec4(mat4 m, vec4 v) {
	vec4 xxxx = _mm_shuffle_ps(v, v, _MM_SHUFFLE(0,0,0,0));
	vec4 yyyy = _mm_shuffle_ps(v, v, _MM_SHUFFLE(1,1,1,1));
	vec4 zzzz = _mm_shuffle_ps(v, v, _MM_SHUFFLE(2,2,2,2));
	vec4 wwww = _mm_shuffle_ps(v, v, _MM_SHUFFLE(3,3,3,3));
	
	vec4 outv = _mm_mul_ps(m.cols[0], xxxx);
	outv = _mm_fmadd_ps(m.cols[1], yyyy, outv);
	outv = _mm_fmadd_ps(m.cols[2], zzzz, outv);
	return _mm_fmadd_ps(m.cols[3], wwww, outv);
}

#else // USE_INTRIN

typedef __attribute__((vector_size(16))) float vec4;

typedef struct {
	vec4 cols[4];
} mat4;

static inline vec4 vec4_broadcast(float val) {
	return (vec4) {val,val,val,val};
}

static inline vec4 mat4_mul_vec4(mat4 m, vec4 v) {
	return m.cols[0] * vec4_broadcast(v[0])
		 + m.cols[1] * vec4_broadcast(v[1])
		 + m.cols[2] * vec4_broadcast(v[2])
		 + m.cols[3] * vec4_broadcast(v[3]);
}

#endif
