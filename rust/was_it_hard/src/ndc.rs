// NDC = Normalized Device Coordinates.
use math::Vec4f;

pub fn from_pixel(x: u32, y: u32, w: u32, h: u32) -> Vec4f {
    let xy = Vec4f::new(x as _, y as _, 0., 1.);
    let wh = Vec4f::new(w as _, h as _, 1., 1.) - Vec4f::new(1., 1., 0., 0.);
    ((xy / wh) * 2.) - 1.
}

pub fn to_pixel(xy: Vec4f, w: u32, h: u32) -> (i32, i32) {
    let wh = Vec4f::new(w as _, h as _, 1., 1.) - Vec4f::new(1., 1., 0., 0.);
    let p = wh * ((xy + 1.) / 2.);
    (p.x as _, p.y as _)
}
