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

pub fn virtual_alloc(starting_address_hint: usize, size: usize, flags: u32, protection: u32) -> Result<NonNull<u8>, Error> {
    let p = unsafe { winapi::um::memoryapi::VirtualAlloc(starting_address_hint as *mut _, size, flags, protection) };
    if p.is_null() {
        Err(Error::last_os_error())
    } else {
        Ok(unsafe { NonNull::new_unchecked(p) }.cast())
    }
}

pub unsafe fn virtual_free(addr: usize, size: usize, flags: u32) -> Result<(), Error> {
    match unsafe { winapi::um::memoryapi::VirtualFree(addr as *mut _, size, flags) } {
        0 => Err(Error::last_os_error()),
        _ => Ok(())
    }
}

pub fn virtual_query(addr: usize) -> Result<winapi::um::winnt::MEMORY_BASIC_INFORMATION, Error> {
    let mut info = MaybeUninit::uninit();
    unsafe {
        match winapi::um::memoryapi::VirtualQuery(addr as *mut _, info.as_mut_ptr(), std::mem::size_of_val(&info)) {
            0 => Err(Error::last_os_error()),
            _ => Ok(info.assume_init()),
        }
    }
}

// Returns the old protection flags of the FIRST page in the range
pub fn virtual_protect(addr: usize, size: usize, flags: u32) -> Result<u32, Error> {
    let mut old_protect = 0;
    match unsafe { winapi::um::memoryapi::VirtualProtect(addr as *mut _, size, flags, &mut old_protect) } {
        0 => Err(Error::last_os_error()),
        _ => Ok(old_protect),
    }
}

pub fn virtual_lock(addr: usize, size: usize) -> Result<(), Error> {
    match unsafe { winapi::um::memoryapi::VirtualLock(addr as *mut _, size) } {
        0 => Err(Error::last_os_error()),
        _ => Ok(()),
    }
}

pub fn virtual_unlock(addr: usize, size: usize) -> Result<(), Error> {
    match unsafe { winapi::um::memoryapi::VirtualUnlock(addr as *mut _, size) } {
        0 => Err(Error::last_os_error()),
        _ => Ok(()),
    }
}