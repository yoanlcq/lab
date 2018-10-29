typedef struct {} m4;

#define swap(a, b) { const __auto_type tmp = a; a = b; b = tmp; }

#define MAX 32

typedef struct {
    u16 nb;
    u16 parent[MAX];
    m4 local[MAX];
    m4 world[MAX];
} Tree;

void init_tree(Tree* h) {
    h->nb = 1;
    h->parent[0] = 0;
    h->local[0] = m4_identity();
    h->world[0] = m4_identity();
}

void update_world_transforms(Tree* h) {
    for (unsigned i = 1 /* start at 1 to skip root node */; i < h->nb; ++i) {
        h->world[i] = h->world[h->parent[i]] * h->local[i];
    }
}

u16 add_node(Tree* h) {
    h->parent[h->nb] = 0;
    h->local[h->nb] = m4_identity();
    h->world[h->nb] = m4_identity();
    h->nb += 1;
    return h->nb - 1;
}

void set_parent(Tree* h, u16 i, u16 p) {
    assert(i != p);
    h->parent[i] = p;
    fix_parent_ifn(i);
}

void fix_parent_ifn(Tree* h, u16 i) {
    const u16 p = h->parent[i];
    if (i < p) {
        swap_nodes(h, i, p);
        fix_parent_ifn(i);
    }
}

void swap_nodes(Tree* h, u16 a, u16 b) {
    swap(h->parent[a], h->parent[b]);
    swap(h->local[a], h->local[b]);
    swap(h->world[a], h->world[b]);
    // FIXME: Toutes les nodes qui référaient à a doivent référer b, et vice versa
    for (u16 i = min(a, b) + 1; i < h->nb; ++i) {
        if (i == a || i == b)
            continue;
        if (h->parent[i] == a)
            h->parent[i] = b;
        else if (h->parent[i] == b)
            h->parent[i] = a;
    }
}

void rm_node(Tree* h, u16 i) {
    assert(i >= 1); // Can't remove root node!!
    assert(i < h->nb);
    h->nb -= 1;
    swap_nodes(h, i, h->nb);
    fix_parent_ifn(h, i);
}