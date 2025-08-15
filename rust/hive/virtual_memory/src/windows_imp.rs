use super::Error;
use core::{mem::MaybeUninit, ptr::NonNull};

use windows::Win32::System::SystemInformation::*;
use windows::Win32::System::Memory::*;

pub fn get_system_info() -> SYSTEM_INFO {
    let mut info = MaybeUninit::uninit();
    unsafe {
        GetSystemInfo(info.as_mut_ptr());
        info.assume_init()
    }
}

pub fn virtual_alloc(starting_address_hint: usize, size: usize, flags: VIRTUAL_ALLOCATION_TYPE, protection: PAGE_PROTECTION_FLAGS) -> Result<NonNull<u8>, Error> {
    let p = unsafe { VirtualAlloc(Some(core::ptr::with_exposed_provenance_mut(starting_address_hint)), size, flags, protection) };
    if p.is_null() {
        Err(Error::last_os_error())
    } else {
        Ok(unsafe { NonNull::new_unchecked(p) }.cast())
    }
}

pub unsafe fn virtual_free(addr: usize, size: usize, flags: VIRTUAL_FREE_TYPE) -> Result<(), Error> {
    unsafe { VirtualFree(addr as *mut _, size, flags) }?;
    Ok(())
}

pub fn virtual_query(addr: usize) -> Result<MEMORY_BASIC_INFORMATION, Error> {
    let mut info = MaybeUninit::uninit();
    unsafe {
        match VirtualQuery(Some(addr as *mut _), info.as_mut_ptr(), size_of_val(&info)) {
            0 => Err(Error::last_os_error()),
            _ => Ok(info.assume_init()),
        }
    }
}

// Returns the old protection flags of the FIRST page in the range
pub fn virtual_protect(addr: usize, size: usize, flags: PAGE_PROTECTION_FLAGS) -> Result<PAGE_PROTECTION_FLAGS, Error> {
    let mut old_protect = MaybeUninit::zeroed();
    unsafe { VirtualProtect(addr as *mut _, size, flags, old_protect.as_mut_ptr()) }?;
    Ok(unsafe { old_protect.assume_init() })
}

pub fn virtual_lock(addr: usize, size: usize) -> Result<(), Error> {
    unsafe { VirtualLock(addr as *mut _, size) }?;
    Ok(())
}

pub fn virtual_unlock(addr: usize, size: usize) -> Result<(), Error> {
    unsafe { VirtualUnlock(addr as *mut _, size) }?;
    Ok(())
}