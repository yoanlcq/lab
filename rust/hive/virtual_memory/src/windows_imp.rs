use super::Error;
use std::{mem::MaybeUninit, ptr::NonNull};

#[inline(always)]
pub fn get_system_info() -> winapi::um::sysinfoapi::SYSTEM_INFO {
    let mut info = MaybeUninit::uninit();
    unsafe {
        winapi::um::sysinfoapi::GetSystemInfo(info.as_mut_ptr());
        info.assume_init()
    }
}

pub fn virtual_alloc(starting_address_hint: *mut u8, size: usize, flags: u32, protection: u32) -> Result<NonNull<u8>, Error> {
    let p = unsafe { winapi::um::memoryapi::VirtualAlloc(starting_address_hint.cast(), size, flags, protection) };
    if p.is_null() {
        Err(Error::last_os_error())
    } else {
        Ok(unsafe { NonNull::new_unchecked(p) }.cast())
    }
}

pub unsafe fn virtual_free(ptr: *mut u8, size: usize, flags: u32) -> Result<(), Error> {
    match unsafe { winapi::um::memoryapi::VirtualFree(ptr.cast(), size, flags) } {
        0 => Err(Error::last_os_error()),
        _ => Ok(())
    }
}

pub fn virtual_query(ptr: *mut u8) -> Result<winapi::um::winnt::MEMORY_BASIC_INFORMATION, Error> {
    let mut info = MaybeUninit::uninit();
    unsafe {
        match winapi::um::memoryapi::VirtualQuery(ptr.cast(), info.as_mut_ptr(), std::mem::size_of_val(&info)) {
            0 => Err(Error::last_os_error()),
            _ => Ok(info.assume_init()),
        }
    }
}

// Returns the old protection flags of the FIRST page in the range
pub fn virtual_protect(ptr: *mut u8, size: usize, flags: u32) -> Result<u32, Error> {
    let mut old_protect = 0;
    match unsafe { winapi::um::memoryapi::VirtualProtect(ptr.cast(), size, flags, &mut old_protect) } {
        0 => Err(Error::last_os_error()),
        _ => Ok(old_protect),
    }
}

pub fn virtual_lock(ptr: *mut u8, size: usize) -> Result<(), Error> {
    match unsafe { winapi::um::memoryapi::VirtualLock(ptr.cast(), size) } {
        0 => Err(Error::last_os_error()),
        _ => Ok(()),
    }
}

pub fn virtual_unlock(ptr: *mut u8, size: usize) -> Result<(), Error> {
    match unsafe { winapi::um::memoryapi::VirtualUnlock(ptr.cast(), size) } {
        0 => Err(Error::last_os_error()),
        _ => Ok(()),
    }
}