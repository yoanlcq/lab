// Trivial malloc()-based std::vector alternative, to help
// me understand the rule of five.

#include <cstdlib> // malloc(), free(), realloc()
#include <cstring> // memcpy()
#include <utility> // swap(), move()
#include <new> // bad_alloc

using namespace std;

template<typename T>
class vec {
    T* ptr; // Buffer
    size_t cap; // Capacity of allocated buffer
    size_t len; // Actually used number of elements

    void check_notnull(T* p) {
        if(p == nullptr)
            throw std::bad_alloc();
    }

public:
    ~vec() {
        free(ptr);
    }

    vec(): ptr(nullptr), cap(0), len(0) {}

    // Copy constructor
    vec(const vec& v): cap(v.cap), len(v.len) {
        check_notnull(ptr = (T*) malloc(cap*sizeof*ptr));
        memcpy(ptr, v.ptr, len*sizeof*ptr);
    }

    // Copy assignment
    vec& operator=(const vec& v) {
        cap = v.cap;
        len = v.len;
        check_notnull(ptr = realloc(ptr, cap*sizeof*ptr));
        memcpy(ptr, v.ptr, len*sizeof*ptr);
        return *this;
    }

    // Move constructor
    vec(vec&& v): ptr(v.ptr), cap(v.cap), len(v.len) {
        v.ptr = nullptr;
    }

    // Move assignment
    vec& operator=(vec&& v) {
        cap = v.cap;
        len = v.len;
        std::swap(ptr, v.ptr);
        return *this;
    }

    void push(T v) {
        ++len;
        if(len > cap) {
            ++cap; // Make it not zero
            cap *= 2;
            check_notnull(ptr = (T*) realloc(ptr, cap*sizeof*ptr));
        }
        ptr[len-1] = v;
    }
};

int main() {
    vec<int> v;
    v.push(42);
    v.push(13);
    vec<int> b = std::move(v);
    return 0;
}
