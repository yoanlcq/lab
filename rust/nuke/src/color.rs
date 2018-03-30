use nuke_sys::*;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Rgba<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl From<nk_color> for Rgba<u8> {
    fn from(c: nk_color) -> Self {
        let nk_color { r, g, b, a } = c;
        Self { r, g, b, a }
    }
}

impl From<nk_colorf> for Rgba<f32> {
    fn from(c: nk_colorf) -> Self {
        let nk_colorf { r, g, b, a } = c;
        Self { r, g, b, a }
    }
}