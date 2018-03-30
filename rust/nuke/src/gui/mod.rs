use nuke_sys::*;
use context::*;
use draw::*;

#[must_use = "this is the entry point to building the GUI's state"]
pub struct GuiStep<'a> {
    ctx: &'a mut Context<'a>
}

// We reference a Context that outlives us
pub struct GuiApi<'a, 'b:'a> {
    ctx: &'a mut Context<'b>,
}

impl<'a> GuiStep<'a> {
    pub(crate) fn new(ctx: &'a mut Context<'a>) -> Self {
        Self { ctx }
    }
    pub fn gui<F: FnMut(GuiApi)>(&mut self, f: F) -> DrawStep<'a> {
        unimplemented!()
    }
}