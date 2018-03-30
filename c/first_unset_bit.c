#include <stdint.h>
#include <stdio.h>

int first_unset_bit_linear(uint8_t b) {
    if(b==0xff) return -1;
    if(!(b&0x80)) return 0;
    if(!(b&0x40)) return 1;
    if(!(b&0x20)) return 2;
    if(!(b&0x10)) return 3;
    if(!(b&0x08)) return 4;
    if(!(b&0x04)) return 5;
    if(!(b&0x02)) return 6;
    return 7;
}

int first_unset_bit_binary(uint8_t b) {
    if(b==0xff) 
        return -1;
    if(b&0xf0 != 0xf0) {
        if(b&0xc0 != 0xc0)
            return b&0x80 ? 0 : 1;
        else
            return b&0x20 ? 2 : 3;
    } else {
        if(b&0x0c != 0x0c)
            return b&0x08 ? 4 : 5;
        else
            return b&0x02 ? 6 : 7;
    }
}

int first_unset_bit_oneop(uint64_t b) {
    int lookup[] = {
        -1, 63, 62, 61, 60, 61, 60, 59, 58, 57, 56, 55, 54, 53, 52, 51, 50,
        49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33,
        32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16,
        15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0
    };
    return lookup[~b & (b+1)];
}

int fub(uint64_t b) {
    if(b==0xffffffffffffffffULL) 
        return -1;
    if(b &0x8000000000000000ULL == 0)
        return 0;
#ifdef GCC
    return ++__builtin_clrsbll(b);
    /* Below is an alternative which should work as fast : */
    /* return __builtin_clz(~b); */
#elif defined (MSVC)
    /* FIXME TEST ME !!! */
    int res;
    _BitScanReverse64(&res, ~b);
    return res-1;
#endif
}


int main(void) {
    printf("%d\n", fub(0x4ffffffffffffff0ULL));
    return 0;
}
