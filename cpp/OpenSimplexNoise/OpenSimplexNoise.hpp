#pragma once

/*
 * OpenSimplex Noise in Java.
 * by Kurt Spencer
 *
 * (C++ port by Yoan Lecoq)
 * 
 * v1.1 (October 5, 2014)
 * - Added 2D and 4D implementations.
 * - Proper gradient sets for all dimensions, from a
 *   dimensionally-generalizable scheme with an actual
 *   rhyme and reason behind it.
 * - Removed default permutation array in favor of
 *   default seed.
 * - Changed seed-based constructor to be independent
 *   of any particular randomization library, so results
 *   will be the same when ported to other languages.
 */
 
#include <stddef.h>
#include <stdint.h>

class OpenSimplexNoise {

	static const double  STRETCH_CONSTANT_2D ;
	static const double  SQUISH_CONSTANT_2D  ;
	static const double  STRETCH_CONSTANT_3D ;
	static const double  SQUISH_CONSTANT_3D  ;
	static const double  STRETCH_CONSTANT_4D ;
	static const double  SQUISH_CONSTANT_4D  ;
	static const double  NORM_CONSTANT_2D    ;
	static const double  NORM_CONSTANT_3D    ;
	static const double  NORM_CONSTANT_4D    ;
	static const int64_t DEFAULT_SEED        ;
	
	int16_t perm[256];
	int16_t permGradIndex3D[256];

public:
	OpenSimplexNoise();
    OpenSimplexNoise(int16_t perm[256]);
	//Initializes the class using a permutation array generated from a 64-bit seed.
	//Generates a proper permutation (i.e. doesn't merely perform N successive pair swaps on a base array)
	//Uses a simple 64-bit LCG.
	OpenSimplexNoise(int64_t seed);
	
	//2D OpenSimplex Noise.
	double eval(double x, double y);
	//3D OpenSimplex Noise.
	double eval(double x, double y, double z);
	//4D OpenSimplex Noise.
	double eval(double x, double y, double z, double w);

private:
	
	double extrapolate(int xsb, int ysb, double dx, double dy);
	double extrapolate(int xsb, int ysb, int zsb, double dx, double dy, double dz);
	double extrapolate(int xsb, int ysb, int zsb, int wsb, double dx, double dy, double dz, double dw);
	static int fastFloor(double x);
	
	//Gradients for 2D. They approximate the directions to the
	//vertices of an octagon from the center.
	static int8_t gradients2D[];
	
	static const size_t GRADIENTS3D_COUNT;

	//Gradients for 3D. They approximate the directions to the
	//vertices of a rhombicuboctahedron from the center, skewed so
	//that the triangular and square facets can be inscribed inside
	//circles of the same radius.
	static int8_t gradients3D[];

	//Gradients for 4D. They approximate the directions to the
	//vertices of a disprismatotesseractihexadecachoron from the center,
	//skewed so that the tetrahedral and cubic facets can be inscribed inside
	//spheres of the same radius.
	static int8_t gradients4D[];
};
