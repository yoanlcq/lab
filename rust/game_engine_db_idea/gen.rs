#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub const ID: u32 = 0xdead0000;
