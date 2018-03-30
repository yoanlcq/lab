use global::Global;

pub struct OnLoad<'a> {
    pub g: &'a Global,
    pub id: u32,
}
pub struct OnUnload<'a> {
    pub g: &'a Global,
}
pub struct OnDraw<'a> {
    pub g: &'a Global,
    pub frame_number: u64,
}


impl<'a> OnLoad<'a> {
    pub fn on_unload(&self, f: fn(&OnUnload)) {
        self.g.unload_subscribers.borrow_mut().insert(self.id, f);
    }
    pub fn on_draw(&self, f: fn(&OnDraw)) {
        self.g.draw_subscribers.borrow_mut().insert(self.id, f);
    }
}
