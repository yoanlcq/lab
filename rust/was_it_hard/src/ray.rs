use math::Vec4f;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Vec4f,
    pub direction: Vec4f,
}
