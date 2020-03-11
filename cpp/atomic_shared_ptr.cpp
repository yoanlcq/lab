#include <stdint.h>
#include <assert.h>
#include <atomic>
#include <algorithm> // std::swap()?

template<typename T>
class CAtomicShaderPtr {
public:
    using TRefCount = intptr_t; // Signed to detect underflows, and pointer-sized to reduce wasted space caused by alignment requirements.

public:
    // Takes ownership of p_pObject, which must be allocated via new!
    CAtomicShaderPtr(T* p_pObject = nullptr) : m_pObject(p_pObject), m_pnAtomicRefCount(p_pObject ? new std::atomic<TRefCount>(1) : nullptr) {}
    ~CAtomicShaderPtr() { Unref(); }

    // Copy
    CAtomicShaderPtr(const CAtomicShaderPtr& p_pOther) { CopyFrom(p_pOther); }
    CAtomicShaderPtr& operator=(const CAtomicShaderPtr& p_pOther) { return CopyFrom(p_pOther); }

    // Move constructor
    CAtomicShaderPtr(CAtomicShaderPtr&& p_pOther) : CAtomicShaderPtr()
    {
        std::swap(m_pObject, p_pOther.m_pObject);
        std::swap(m_pnAtomicRefCount, p_pOther.m_pnAtomicRefCount);
    }

    // Move assignment
    CAtomicShaderPtr& operator=(CAtomicShaderPtr&& p_pOther)
    {
        std::swap(m_pObject, p_pOther.m_pObject);
        std::swap(m_pnAtomicRefCount, p_pOther.m_pnAtomicRefCount);
        return *this;
    }

    // Gets the current reference count. There's not much you can do with it, but it's there if you need it.
    TRefCount GetRefCount() const { return *m_pnAtomicRefCount; }

    // Dereference
    T& operator*() const { return *m_pObject; }
    T* operator->() const { return m_pObject; }

    // Disable these, there is a risk that the user extracts the pointer but forgets that it is managed by the refcount.
    operator T*() const = delete;
    operator T*() = delete;

    bool operator==(const T* pOther) const { return m_pObject == pOther; };
    bool operator!=(const T* pOther) const { return m_pObject != pOther; };

private:
    CAtomicShaderPtr& CopyFrom(const CAtomicShaderPtr& p_pOther)
    {
        m_pObject = p_pOther.m_pObject;
        m_pnAtomicRefCount = p_pOther.m_pnAtomicRefCount;
        AddRef();
        return *this;
    }

    void AddRef()
    {
        if (!m_pnAtomicRefCount)
            return;

        const TRefCount nPreviousValue = m_pnAtomicRefCount->fetch_add(1);
        assert(nPreviousValue >= 1); // AddRef() is only called in the context of a copy operation (and Unref() in destructor), so by definition, if we reach here, the refcount is always >= 1.
    }

    void Unref()
    {
        if (!m_pnAtomicRefCount)
            return;

        const TRefCount nPreviousValue = m_pnAtomicRefCount->fetch_sub(1);
        if (nPreviousValue == 1) // NOTE: If 2 threads perform the fetch_sub() "at the same time", the returned value will be 1 for the first thread, and 0 for the second thread. Avoid destroying the object twice in this case.
            Destroy();
    }

    void Destroy()
    {
        if (m_pObject)
            delete m_pObject;

        if (m_pnAtomicRefCount)
            delete m_pnAtomicRefCount;
    }

private:
    // Must be pointers on the heap, because the values are shared by all living copies of this CAtomicShaderPtr.
    T* m_pObject;
    std::atomic<TRefCount>* m_pnAtomicRefCount;
};

//
//
//
//
//
//
int main() {
    CAtomicShaderPtr<int> pFoo(new int);
    if (pFoo != nullptr) {
    }


    *pFoo;

    return 0;
}
