#include <stdint.h>

typedef __attribute__((vector_size(16))) int32_t i32v4;

inline i32v4 i32v4_dot4(i32v4 ax, i32v4 bx, i32v4 ay, i32v4 by, i32v4 az, i32v4 bz, i32v4 aw, i32v4 bw) {
    i32v4 dx = ax*bx;
    i32v4 dy = ay*by;
    i32v4 dz = az*bz;
    i32v4 dw = aw*bw;
    i32v4 a0 = dx+dy;
    i32v4 a1 = dz+dw;
    return a0 + a1;
}
inline i32v4 i32v4_4dots(i32v4 a[restrict static 4], i32v4 b[restrict static 4]) {
    i32v4 ax = {a[0][0], a[1][0], a[2][0], a[3][0]};
    i32v4 ay = {a[0][1], a[1][1], a[2][1], a[3][1]};
    i32v4 az = {a[0][2], a[1][2], a[2][2], a[3][2]};
    i32v4 aw = {a[0][3], a[1][3], a[2][3], a[3][3]};
    i32v4 bx = {b[0][0], b[1][0], b[2][0], b[3][0]};
    i32v4 by = {b[0][1], b[1][1], b[2][1], b[3][1]};
    i32v4 bz = {b[0][2], b[1][2], b[2][2], b[3][2]};
    i32v4 bw = {b[0][3], b[1][3], b[2][3], b[3][3]};
    return i32v4_dot4(ax, bx, ay, by, az, bz, aw, bw);
}

int main(void) {

}
