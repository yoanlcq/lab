use nuke_sys::*;
use context::*;

// NOT annotated with must_use, since it can safely be dropped.
pub struct ClearStep<'a> {
    ctx: &'a mut Context<'a>
}

impl<'a> ClearStep<'a> {
    pub fn clear(self) {
        drop(self)
    }
    pub unsafe fn leave_uncleared(self) {
        ::core::mem::forget(self)
    }
}

impl<'a> Drop for ClearStep<'a> {
    fn drop(&mut self) {
        unsafe {
            nk_clear(&mut self.ctx.raw)
        }
    }
}