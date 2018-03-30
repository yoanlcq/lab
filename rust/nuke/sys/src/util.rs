use nk_handle;
use core::mem::*;

impl<T> From<*mut T> for nk_handle {
    fn from(p: *mut T) -> Self {
        let mut h: nk_handle = unsafe { zeroed() };
        h.bindgen_union_field = p as usize as u64;
        h
    }
}
impl nk_handle {
    pub fn into<T>(self) -> *mut T {
        self.bindgen_union_field as usize as *mut T
    }
}
