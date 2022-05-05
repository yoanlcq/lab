// Compile with one of the following:
// gcc   -std=c11 -Wall -Wextra -pedantic -Wno-pedantic-ms-format auto_defrag_allocator.c
// clang -std=c11 -Wall -Wextra -pedantic auto_defrag_allocator.c

#include <stdlib.h> // malloc() for main()
#include <stdint.h>
#include <stdio.h> // snprintf()
#include <stdbool.h>
#include <stdalign.h>
#include <string.h> // memset(), memcpy()
#include <inttypes.h>
#include <assert.h> // static_assert()

#ifdef _WIN32
#  ifdef _WIN64
#    define PRI_SIZET PRIu64
#  else
#    define PRI_SIZET PRIu32
#  endif
#else
#  define PRI_SIZET "zu"
#endif

#define dfm_assert assert

#define countof(x) (sizeof(x) / sizeof((x)[0]))

//
// Helper macros
//

#define FORWARD_DECLARE_STRUCT(T) typedef struct T T
#define DECLARE_STRUCT(T) FORWARD_DECLARE_STRUCT(T); struct T

//
// Alignment utilities
//

// This must work at compile-time (because it is used in static initializers), hence the use of a builtin as a replacement for integer log2().
// This takes advantage of the fact that alignment values are power-of-two, so they can be stored as a number of bits to shift.
#define compress_alignment(a) (31 - __builtin_clz((uint32_t) a))

// This operation will never change. Feel free to write (1u << a) directly instead of using this macro.
#define decompress_alignment(a) (1ul << (a))

// Some tests, in case compress_alignment has to change, due to changing compilers or whatever...
static_assert((1u << compress_alignment(1)) == 1, "");
static_assert((1u << compress_alignment(2)) == 2, "");
static_assert((1u << compress_alignment(4)) == 4, "");
static_assert((1u << compress_alignment(8)) == 8, "");

static inline uintptr_t align_uintptr(uintptr_t x, uintptr_t a) {
	return (x + a - 1) & ~(a - 1);
}

static inline void* align_ptr(const void* x, uintptr_t a) {
	return (void*) align_uintptr((uintptr_t) x, a);
}

#if 0
static inline bool is_ptr_aligned(const void* p, uintptr_t a) {
	return ((uintptr_t) p) % a == 0;
}

static inline bool size_is_power_of_two(size_t x) {
	return x && !(x & (x - 1));
}

static inline void check_alignment(size_t a) {
	dfm_assert(a >= 1 && size_is_power_of_two(a));
}

static inline size_t compress_alignment_checked(size_t a) {
	check_alignment(a);
	return compress_alignment(a);
}
#endif

//
// Defrag allocator
//

FORWARD_DECLARE_STRUCT(DfmContext);

DECLARE_STRUCT(DfmArchetypeCallbackParams) {
    const void* item;
};

DECLARE_STRUCT(DfmArchetypeCallbackResult) {
    const size_t* offsets; // Must point to static const memory
    size_t nb_offsets;
};

typedef DfmArchetypeCallbackResult (*DfnArchetypeCallback)(const DfmArchetypeCallbackParams* p);

DECLARE_STRUCT(DfmCallerMetadata) {
    const char* name;
};

DECLARE_STRUCT(DfmAllocationParams) {
    DfmContext* cx;
    size_t nb_items;
    size_t sizeof_item;
    size_t alignof_item;
    DfnArchetypeCallback archetype_callback;
    DfmCallerMetadata metadata;
};

typedef enum DfmEventType {
    DFM_EVENT__INVALID = 0,
    DFM_EVENT__ALLOC_FAILED,
    DFM_EVENT__COUNT // Keep last
} DfmEventType;

DECLARE_STRUCT(DfmEvent) {
    DfmEventType event_type;
    const char* message;
    const DfmAllocationParams* allocation_params;
};

typedef void (*DfmEventHandlerFn)(const DfmEvent* e);

DECLARE_STRUCT(DfmSlice) {
    void* base;
    size_t size;
};

DECLARE_STRUCT(DfmSide) {
    size_t offset_from_base;
};

DECLARE_STRUCT(DfmBlockHeader) {
    uint8_t magic;
};

DECLARE_STRUCT(DfmFreeList) {
    DfmBlockHeader* first;
};

DECLARE_STRUCT(DfmContext) {
    DfmSlice mem;
    DfmSide left;
    DfmSide right;
    DfmFreeList free_list;
    DfmEventHandlerFn event_handler_fn;
};

DECLARE_STRUCT(DfmContextParams) {
    DfmSlice mem;
    DfmEventHandlerFn event_handler_fn;
};

void dfm_event_handler_dummy(const DfmEvent* e) {
    (void) e;
}

void dfm_init(DfmContext* cx, const DfmContextParams* p) {
    dfm_assert(cx);
    dfm_assert(p);
    dfm_assert(p->event_handler_fn); // If you don't care, please pass dfm_event_handler_dummy. It allows us to avoid testing for NULL every time we want to call it.
    dfm_assert(cx->mem.size == 0 || cx->mem.base);
    *cx = (DfmContext) {
        .mem = p->mem,
        .event_handler_fn = p->event_handler_fn,
        .left = { .offset_from_base = 0 },
        .right = { .offset_from_base = p->mem.size },
    };
}

void dfm_deinit(DfmContext* cx) {
    (void) cx;
}

static void dfm_cx_notify_alloc_failed(const DfmAllocationParams* p, size_t required_offset_from_base) {
    char message[512];
    snprintf(message, sizeof message, "Allocation of size %" PRI_SIZET " x %" PRI_SIZET " would require a total capacity of %" PRI_SIZET ", but we only have %" PRI_SIZET " left\n", p->nb_items, p->sizeof_item, required_offset_from_base, p->cx->right.offset_from_base);

    const DfmEvent event = {
        .event_type = DFM_EVENT__ALLOC_FAILED,
        .message = message,
        .allocation_params = p,
    };
    p->cx->event_handler_fn(&event);
}

void* dfm_alloc_uninitialized(const DfmAllocationParams* p) {
    uint8_t* cursor = p->cx->mem.base;
    cursor += p->cx->left.offset_from_base;
    cursor = align_ptr(cursor, p->alignof_item);
    void* alloc_base = cursor;

    cursor += p->nb_items * p->sizeof_item;
    const void* alloc_end = cursor;

    const size_t new_offset_from_base = (uint8_t*)alloc_end - (uint8_t*)p->cx->mem.base;
    if (new_offset_from_base > p->cx->right.offset_from_base) {
        dfm_cx_notify_alloc_failed(p, new_offset_from_base);
        return NULL;
    }

    p->cx->left.offset_from_base = new_offset_from_base;
    return alloc_base;
}

void dfm_rename_ptr(DfmContext* cx, void* old_ptr, void* new_ptr) {
    // We want to find all pointers which value is equal to old_ptr, and set their value to new_ptr.
    // - Comfort+Safety: Soit on rebuild tout en mode bourrin (un gros O(N) mais va tout détecter);
    //   Could note that it only needs to happen once for the defragmentation, not every time we move a block.
    //   We could also reduce the cost by culling items which never have pointers.
    // - Perf: Soit le caller nous garantit l'équivalent d'un "on_ptr_changed", pour qu'on puisse updater une acceleration structure.
    //   Càd que toute modif sur un référenceur est notifiée au système.
    //   Du coup:
    //   - dfm_alloc_zeroed() (on ne permet pas _uninitialized(), car il faut que les pointeurs passent de NULL à non-NULL)
    //   - dfm_unregister_referencer(void** referencer_address) // should also set pointer value to NULL?
    //   - dfm_register_referencer(void** referencer_address)
    //   - dfm_replace_referencer(void** referencer_address, void* new_value) // unregister, set, then register
    //   - dfm_item_unregister_referencers(void* item, archetype)
    //   - dfm_item_register_referencers(void* item, archetype)
    //   - dfm_alloc_within(void** referencer_address, params); // Internally calls dfm_forget_referencer() and dfm_add_referencer()
}

// TODO:
// Move these notes aside, and work on a MVP.
// The main feature is pointer reachability, so this should be a priority (don't implement free() yet).
// Start with an explicitly-provided archetype_callback.

// What we want:
// - Ability to visit all blocks sequentially (rare, because it's for profiling/debugging)
// - Ability to quickly find the smallest free block that can fit a new allocation; (preferably O(log2 N))
// - Free block: ability to get the next (i.e not free) block (for defragmentation)
// - Magic number: detect if an address is a valid block

// Allocation params:
// - Strategy: Ordered set of instructions such as:
//   - Try to allocate directly by bumping the "top" pointer;
//   - Search in free blocks: condition may be:
//     - Find smallest matching;
//     - Find any matching;
// - Relocation callback IFN;
// - Userdata pointer IFN (is that pointer managed? Can know the answer by looking if it's within the arena's range)
// - Archetype definition IFN; Can be either of the following:
//   - Callback (useful if the item is a tagged union: receives pointer to item, returns static array of offsets to managed pointers)
//   - Static array of offsets to managed pointers

// Impl:
// - Each block stores this struct, with a forced alignment of 1 (compressed/decompressed on-demand):
//   - Info for the current block (4 bytes):
//     - magic: 4; (for detecting if someone wrote past the previous allocation)
//     - typeof_nb_items : 2; (0 = u8, 1 = u16, 2 = u32, 3 = u64)
//     - typeof_sizeof_item : 2; (0 = u8, 1 = u16, 2 = u32, 3 = u64)
//     - compressed_alignof_item : 5; (alignment = (1 << compressed_alignof_item)).
//     - is_free : 1;
//     - has_name : 1; (If true: name is embedded in the alignment padding)
//     - has_userdata : 1;
//     - has_relocation_callback : 1; (If true: relocation callback is present in the variable length payload)
//     - archetype_handle: 16 (CANNOT be made variable-length, since the archetype may be determined progressively after allocation)
//   - Variable length payload (min 2 bytes, max 32 bytes):
//     - nb_items (compressed by choosing the smallest fitting unsigned type)
//     - sizeof_item (compressed by choosing the smallest fitting unsigned type)
//     - optional relocation callback
//     - optional userdata ptr (useful for passing to relocation callback, or storing the callstack (or callstack hash), or anything else)
//   - <unused space> (padding for the allocation's alignment, if necessary; some of these bytes are abused for storing an offset, see below)
//   - abused bytes: see below
//   - Sentinel just behind the allocation address (1 byte):
//     - typeof_abused_bytes : 3; (0 = none (perfectly aligned), 1 = u8, 2 = u16, 3 = u32, 4 = u64) (indicates a number of bytes just behind this byte, to read and cast to unsigned: gives an offset from this bytes, to the address of the info's end)
//     - payload_size_to_add : 5; ??
// - This is enough for any block to know where its allocation starts and ends
// - TODO: But not enough to jump to the previous block.
//   We need to access previous block when our thread is defragmenting (it knows its first block).
//   Solution: Whenever a free block is formed, it looks at its next block's thread ID, and sets itself as that thread's "free block before the first used one"

// TODO: Support for freeing and free space reuse
// TODO: Support for pointer reachability
// TODO: Support for defragmentation
// TODO: Support for multi-threading

//
// Main
//

DECLARE_STRUCT(TestItem) {
    TestItem* next;
    TestItem* prev;
    char value_char;
};

static void visit_testitem_linked_list(const char* msg, const TestItem* start) {
    printf("%s", msg);
    for (const TestItem* cur = start; cur; cur = cur->next)
        printf("%c", cur->value_char);

    printf("\n");
}

static DfmArchetypeCallbackResult TestItem_archetype_callback(const DfmArchetypeCallbackParams* p) {
    (void) p;
    static const size_t offsets[] = {
        offsetof(TestItem, next),
        offsetof(TestItem, prev),
    };
    return (DfmArchetypeCallbackResult) { offsets, countof(offsets) };
}

int main() {
    const size_t mem_capacity = 1024 * 1024;
    const DfmContextParams cx_params = {
        .mem = {
            .base = malloc(mem_capacity),
            .size = mem_capacity,
        },
        .event_handler_fn = dfm_event_handler_dummy,
    };

    DfmContext cx;
    dfm_init(&cx, &cx_params);

    const DfmAllocationParams params_to_alloc_one_item = {
        .cx = &cx,
        .nb_items = 1,
        .sizeof_item = sizeof(TestItem),
        .alignof_item = alignof(TestItem),
        .archetype_callback = TestItem_archetype_callback,
        .metadata = {0},
    };

    TestItem* head = dfm_alloc_uninitialized(&params_to_alloc_one_item);
    TestItem* mid = dfm_alloc_uninitialized(&params_to_alloc_one_item);
    TestItem* tail = dfm_alloc_uninitialized(&params_to_alloc_one_item);
    TestItem* newtail = dfm_alloc_uninitialized(&params_to_alloc_one_item);

    *head = (TestItem) { .next = mid, .prev = NULL, .value_char = 'A' };
    *mid = (TestItem) { .next = tail, .prev = head, .value_char = 'B' };
    *tail = (TestItem) { .next = NULL, .prev = mid, .value_char = 'C' };
    *newtail = (TestItem) { .next = NULL, .prev = mid, .value_char = 'D' };

    visit_testitem_linked_list("Before rename: ", head);
    dfm_rename_ptr(&cx, tail, newtail);
    visit_testitem_linked_list("After rename: ", head);

    return 0;
}
