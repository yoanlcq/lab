//! A basic (double)-buffer with linear allocation and resetting as only
//! freeing policy. The buffer's main purpose is to control all memory management
//! inside the GUI toolkit and still leave memory control as much as possible in
//! the hand of the user while also making sure the library is easy to use if
//! not as much control is needed.
//! In general all memory inside this library can be provided from the user in
//! three different ways.
//!
//! The first way and the one providing most control is by just passing a fixed
//! size memory block. In this case all control lies in the hand of the user
//! since he can exactly control where the memory comes from and how much memory
//! the library should consume. Of course using the fixed size API removes the
//! ability to automatically resize a buffer if not enough memory is provided so
//! you have to take over the resizing. While being a fixed sized buffer sounds
//! quite limiting, it is very effective in this library since the actual memory
//! consumption is quite stable and has a fixed upper bound for a lot of cases.
//!
//! If you don't want to think about how much memory the library should allocate
//! at all time or have a very dynamic UI with unpredictable memory consumption
//! habits but still want control over memory allocation you can use the dynamic
//! allocator based API. The allocator consists of two callbacks for allocating
//! and freeing memory and optional userdata so you can plugin your own allocator.

use nuke_sys::*;
use core::marker::PhantomData;
use core::mem::*;
use allocator::*;

#[derive(Debug, Clone)]
pub struct Buffer<'a> {
    raw: nk_buffer,
    _booh: PhantomData<&'a mut nk_buffer>,
}
#[derive(Debug, Clone, Copy)]
pub struct MemoryInfo<'a> {
    raw: nk_memory_status,
    _booh: PhantomData<&'a mut Buffer<'a>>,
}

impl<'a> Buffer<'a> {
    pub(crate) fn to_nk(&self) -> nk_buffer {
        self.raw
    }
    
    pub fn from_allocator(allocator: &'a mut &mut Allocator, size: usize) -> Self {
        unsafe {
            let mut raw: nk_buffer = uninitialized();
            let mut a = nk_allocator_from(allocator);
            nk_buffer_init(&mut raw, &mut a, size);
            Self { raw, _booh: PhantomData }
        }
    }
    pub fn from_fixed_mem(mem: &'a mut [u8]) -> Self {
        unsafe {
            let mut raw: nk_buffer = unsafe { uninitialized() };
            nk_buffer_init_fixed(&mut raw, mem.as_mut_ptr() as *mut _, mem.len());
            Self { raw, _booh: PhantomData }
        }
    }
    pub fn info(&self) -> MemoryInfo<'a> {
        unimplemented!()
        //nk_buffer_info()
    }
    pub fn push(&mut self,type_: nk_buffer_allocation_type,
                          memory: *const u8, 
                          size: usize,
                          align: usize)
    {
        unimplemented!()
        //nk_buffer_push()
    }
    pub fn mark(&mut self, type_: nk_buffer_allocation_type) {
        unimplemented!()
        //nk_buffer_mark()
    }
    pub fn reset(&mut self, type_: nk_buffer_allocation_type) {
        unimplemented!()
        //nk_buffer_reset()
    }
    pub fn clear(&mut self) {
        unimplemented!()
        //nk_buffer_clear()
    }
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        unimplemented!()
        //nk_buffer_memory()
    }
    pub fn as_ptr(&self) -> *const u8 {
        unimplemented!()
        //nk_buffer_memory_const()
    }
    pub fn total(&self) -> usize {
        unimplemented!()
        //nk_buffer_total()
    }
}

impl<'a> Drop for Buffer<'a> {
    fn drop(&mut self) {
        unsafe {
            nk_buffer_free(&mut self.raw);
        }
    }
}