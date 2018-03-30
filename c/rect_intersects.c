#include <stdbool.h>
#include <math.h>

struct rect {
    float x, y, w, h;
};
typedef struct rect rect;

static inline bool rect_intersects_x(const rect *a, const rect *b) {
    float ah = a->w/2.0f;
    float bh = b->w/2.0f;
    float ac = a->x + ah;
    float bc = b->x + bh;
    return fabsf(bc-ac) <= ah+bh;
}
static inline bool rect_intersects_y(const rect *a, const rect *b) {
    float ah = a->h/2.0f;
    float bh = b->h/2.0f;
    float ac = a->y + ah;
    float bc = b->y + bh;
    return fabsf(bc-ac) <= ah+bh;
}
#define rect_intersects(a, b) rect_intersects_xfirst(a, b)
bool rect_intersects_xfirst(const rect *a, const rect *b) {
    return rect_intersects_x(a, b) ? rect_intersects_y(a, b) : false;
}
bool rect_intersects_yfirst(const rect *a, const rect *b) {
    return rect_intersects_y(a, b) ? rect_intersects_x(a, b) : false;
}
