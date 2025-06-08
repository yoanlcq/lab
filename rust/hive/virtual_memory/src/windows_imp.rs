use super::Error;
use std::{mem::MaybeUninit, ptr::NonNull};

#[inline(always)]
pub fn get_system_info() -> winapi::um::sysinfoapi::SYSTEM_INFO {
    let mut info = MaybeUninit::uninit();

    #[cfg_attr(not(miri), allow(unused_mut))]
    let mut info = unsafe {
        winapi::um::sysinfoapi::GetSystemInfo(info.as_mut_ptr());
        info.assume_init()
    };

    // Miri handles `GetSystemInfo()` specially by zeroing the struct; at the time of this writing it only preserves `dwPageSize` dans `dwNumberOfProcessors`
    // https://doc.rust-lang.org/nightly/nightly-rustc/src/miri/shims/windows/foreign_items.rs.html#430
    #[cfg(miri)]
    {
        info.dwAllocationGranularity = std::cmp::max(info.dwAllocationGranularity, info.dwPageSize); // Take the max() in case Miri ends up providing proper support later
    }

    info
}

pub fn virtual_alloc(starting_address_hint: usize, size: usize, flags: u32, protection: u32) -> Result<NonNull<u8>, Error> {
    let p = unsafe { winapi::um::memoryapi::VirtualAlloc(std::ptr::without_provenance_mut(starting_address_hint), size, flags, protection) };
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