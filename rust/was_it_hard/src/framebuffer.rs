use std::fmt::{self, Display, Formatter};
use vec4::Vec4f;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Framebuffer {
    pub w: u32,
    pub h: u32,
    pub color: Vec<Vec4f>,
    pub depth: Vec<f32>,
}

impl Framebuffer {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            w, h,
            color: vec![Vec4f::new(0., 0., 0., 1.); (w*h) as _], 
            depth: vec![1. / 0. ; (w*h) as _],
        }
    }
}

// This actually only reads the red component of colors.
impl Display for Framebuffer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                let i = (y*self.w + x) as usize;
                let pixel = if self.depth[i] >= 1. / 0. {
                    '-'
                } else {
                    ('A' as u8 + (self.color[i].x * 25.) as u8) as char
                };
                write!(f, "{}", pixel)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

