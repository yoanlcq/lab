use nuke_sys::*;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)] // Required to build slices
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}
#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Rect<P,S> {
    pub x: P,
    pub y: P,
    pub w: S,
    pub h: S,
}
impl<P,S> Rect<P,S> {
    pub fn from_position_and_size(pos: Vec2<P>, size: Vec2<S>) -> Self {
        Self { x: pos.x, y: pos.y, w: size.x, h: size.y }
    }
}
impl Rect<f32,f32> {
    pub(crate) fn null() -> Self {
        let nk_rect { x, y, w, h } = unsafe { nk_get_null_rect() };
        Rect { x, y, w, h }
    }
}

impl From<nk_vec2> for Vec2<f32> {
    fn from(v: nk_vec2) -> Self {
        let nk_vec2 { x, y } = v;
        Self { x, y }
    }
}
impl From<nk_vec2i> for Vec2<i16> {
    fn from(v: nk_vec2i) -> Self {
        let nk_vec2i { x, y } = v;
        Self { x, y }
    }
}
