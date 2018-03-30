#![macro_use]

#[repr(packed)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgb24{
    pub r:u8,
    pub g:u8,
    pub b:u8,
}

macro_rules! hex24 {
    ($hex:expr) => {
        Rgb24 { 
            r: (($hex>>16)&0xff) as u8,
            g: (($hex>>8)&0xff) as u8,
            b: ($hex&0xff) as u8,
        }
    }
}

impl Rgb24 {
    #[inline(always)]
    pub fn new(r:u8, g:u8, b:u8) -> Self {
        Rgb24 { r:r, g:g, b:b }
    }
    pub fn is_near(&self, other: &Self, threshold: u8) -> bool {
           (self.r as i32 - other.r as i32).abs() <= threshold as i32
        && (self.g as i32 - other.g as i32).abs() <= threshold as i32
        && (self.b as i32 - other.b as i32).abs() <= threshold as i32
    }
    pub fn is_near_rgb(&self, other: &Self, threshold: &Self) -> bool {
           (self.r as i32 - other.r as i32).abs() <= threshold.r as i32
        && (self.g as i32 - other.g as i32).abs() <= threshold.g as i32
        && (self.b as i32 - other.b as i32).abs() <= threshold.b as i32
    }
}
impl From<u32> for Rgb24 {
    #[inline(always)]
    fn from(i : u32) -> Self {
        Self::new((i>>16) as u8, (i>>8) as u8, i as u8)
    }
}
