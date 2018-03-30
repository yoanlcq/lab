// Fast SSE-based mat4_mul_vec4() implementation.
// Go ahead and `gcc -Wall -O3 -S -masm=intel sse_mat4_mul_vec4.c` !
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


#include <stdint.h>
#ifdef __GNUC__
	#include <x86intrin.h>
#elif defined(_MSC_VER)
	#include <intrin.h>
#endif

typedef __m128 mm_vec4;

typedef struct { mm_vec4 col[4]; } mm_col4;
typedef struct { mm_vec4 row[4]; } mm_row4;

#ifndef HAS_FMA3
	#ifdef _mm_fmadd_ps
		#undef _mm_fmadd_ps
	#endif
	#define _mm_fmadd_ps(a,b,c) _mm_add_ps(_mm_mul_ps(a,b),c)
#endif

mm_row4 mm_row4_transpose(mm_row4 m) {
	_MM_TRANSPOSE4_PS(m.row[0], m.row[1], m.row[2], m.row[3]);
	return m;
}

mm_vec4 mm_col4_mul_vec4(mm_col4 m, mm_vec4 v) {
	mm_vec4 xxxx = _mm_shuffle_ps(v, v, _MM_SHUFFLE(0,0,0,0));
	mm_vec4 yyyy = _mm_shuffle_ps(v, v, _MM_SHUFFLE(1,1,1,1));
	mm_vec4 zzzz = _mm_shuffle_ps(v, v, _MM_SHUFFLE(2,2,2,2));
	mm_vec4 wwww = _mm_shuffle_ps(v, v, _MM_SHUFFLE(3,3,3,3));
	
	mm_vec4 outv = _mm_mul_ps(m.col[0], xxxx);
	outv = _mm_fmadd_ps(m.col[1], yyyy, outv);
	outv = _mm_fmadd_ps(m.col[2], zzzz, outv);
	return _mm_fmadd_ps(m.col[3], wwww, outv);
}



#ifdef __GNUC__

typedef __attribute__((vector_size(16))) float vec4;

typedef struct { vec4 col[4]; } col4;
typedef struct { vec4 row[4]; } row4;

vec4 vec4_broadcast(float val) {
	return (vec4) {val,val,val,val};
}

row4 row4_transpose(row4 m) {
	// From _MM_TRANSPOSE4_PS
	/*
	__m128 tmp3, tmp2, tmp1, tmp0;
	tmp0 = _mm_unpacklo_ps(row0, row1); 
	tmp2 = _mm_unpacklo_ps(row2, row3);
   	tmp1 = _mm_unpackhi_ps(row0, row1);
   	tmp3 = _mm_unpackhi_ps(row2, row3); 
	row0 = _mm_movelh_ps(tmp0, tmp2); 
	row1 = _mm_movehl_ps(tmp2, tmp0); 
	row2 = _mm_movelh_ps(tmp1, tmp3); 
	row3 = _mm_movehl_ps(tmp3, tmp1);
	*/
	vec4 row0 = m.row[0];
	vec4 row1 = m.row[1];
	vec4 row2 = m.row[2];
	vec4 row3 = m.row[3];
	vec4 tmp0 = {row0[0], row1[0], row0[1], row1[1]};
	vec4 tmp2 = {row2[0], row3[0], row2[1], row3[1]};
	vec4 tmp1 = {row0[2], row1[2], row0[3], row1[3]};
	vec4 tmp3 = {row2[2], row3[2], row2[3], row3[3]};
	return (row4) { .row = {
		{tmp0[0], tmp0[1], tmp2[0], tmp2[1]},
		{tmp0[2], tmp0[3], tmp2[2], tmp2[3]},
		{tmp1[0], tmp1[1], tmp3[0], tmp3[1]},
		{tmp1[2], tmp1[3], tmp3[2], tmp3[3]}
	}};
}
col4 row4_to_col4(row4 m) {
	row4 t = row4_transpose(m);
	return (col4) { .col = { 
		t.row[0],
		t.row[1],
		t.row[2],
		t.row[3]
	}};
}

vec4 col4_mul_vec4(col4 m, vec4 v) {
	return m.col[0] * v[0]
		 + m.col[1] * v[1]
		 + m.col[2] * v[2]
		 + m.col[3] * v[3];
}
vec4 vec4_mul_row4(vec4 v, row4 m) {
	return m.row[0] * v[0]
		 + m.row[1] * v[1]
		 + m.row[2] * v[2]
		 + m.row[3] * v[3];
}
// https://software.intel.com/en-us/articles/using-simd-technologies-on-intel-architecture-to-speed-up-game-code
row4 row4_mul_row4(row4 m1, row4 m2) {
	vec4 out0 = m2.row[0] * m1.row[0][0];
	vec4 out1 = m2.row[0] * m1.row[1][0];
	vec4 out2 = m2.row[0] * m1.row[2][0];
	vec4 out3 = m2.row[0] * m1.row[3][0];
	out0 += m2.row[1] * m1.row[0][1];
	out1 += m2.row[1] * m1.row[1][1];
	out2 += m2.row[1] * m1.row[2][1];
	out3 += m2.row[1] * m1.row[3][1];
	out0 += m2.row[2] * m1.row[0][2];
	out1 += m2.row[2] * m1.row[1][2];
	out2 += m2.row[2] * m1.row[2][2];
	out3 += m2.row[2] * m1.row[3][2];
	out0 += m2.row[3] * m1.row[0][3];
	out1 += m2.row[3] * m1.row[1][3];
	out2 += m2.row[3] * m1.row[2][3];
	out3 += m2.row[3] * m1.row[3][3];

	return (row4) { .row = {out0, out1, out2, out3} };
}
vec4 row4_mul_vec4(row4 m, vec4 v) {
	float out0 = m.row[0][0] * v[0];
	float out1 = m.row[1][0] * v[0];
	float out2 = m.row[2][0] * v[0];
	float out3 = m.row[3][0] * v[0];
	out0 += m.row[0][1] * v[1];
	out1 += m.row[1][1] * v[1];
	out2 += m.row[2][1] * v[1];
	out3 += m.row[3][1] * v[1];
	out0 += m.row[0][2] * v[2];
	out1 += m.row[1][2] * v[2];
	out2 += m.row[2][2] * v[2];
	out3 += m.row[3][2] * v[2];
	out0 += m.row[0][3] * v[3];
	out1 += m.row[1][3] * v[3];
	out2 += m.row[2][3] * v[3];
	out3 += m.row[3][3] * v[3];
	return (vec4) { out0, out1, out2, out3 };
}
vec4 row4_mul_vec4_by_transpose(row4 m, vec4 v) {
	return col4_mul_vec4(row4_to_col4(m), v);
}
// TODO
/*
vec4 vec4_mul_col4(vec4 v, col4 m) {
	return (vec4) {0,0,0,0};
}
*/


#endif
