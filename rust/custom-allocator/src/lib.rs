//! Wishlist:
//! - Logging feature;
//! - User-defined OOM handlers;
//! - Use jemalloc behind the scenes;

#![feature(allocator)]
#![allocator]
#![no_std]

#![feature(libc)]
extern crate libc;

use core::sync::atomic::*;

pub static TOTAL_USED_MEM: AtomicUsize = ATOMIC_USIZE_INIT;

#[no_mangle]
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    TOTAL_USED_MEM.fetch_add(size, Ordering::SeqCst);
    unsafe { libc::malloc(size as libc::size_t) as *mut u8  }
}
#[no_mangle]
pub extern fn __rust_allocate_zeroed(size: usize, _align: usize) -> *mut u8 {
    TOTAL_USED_MEM.fetch_add(size, Ordering::SeqCst);
    unsafe { libc::calloc(1, size as libc::size_t) as *mut u8  }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
    TOTAL_USED_MEM.fetch_sub(_old_size, Ordering::SeqCst);
    unsafe { libc::free(ptr as *mut libc::c_void)  }
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize, _align: usize) -> *mut u8 {
    TOTAL_USED_MEM.fetch_sub(_old_size, Ordering::SeqCst);
    TOTAL_USED_MEM.fetch_add(size, Ordering::SeqCst);
    unsafe {
        libc::realloc(ptr as *mut libc::c_void, size as libc::size_t) as *mut u8
    }
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize,
                                        _size: usize, _align: usize) -> usize {
    old_size // This api is not supported by libc.
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}

