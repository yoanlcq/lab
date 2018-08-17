pub mod cmp;
pub mod ray;
pub mod ndc;
pub mod framebuffer;
pub mod rasterization;
pub mod raytracing;
pub mod raymarching;

pub mod math {
    extern crate vek;

    pub type Vec4f = self::vek::vec::repr_simd::Vec4<f32>;
}

use math::Vec4f;
use framebuffer::Framebuffer;

fn main() {
    let mut fb = Framebuffer::new(80, 43);
    test_rasterization(&mut fb);
    test_raytracing(&mut fb);
    println!("{}", &fb);
}

fn test_raytracing(fb: &mut Framebuffer) {
    use raytracing::Sphere;

    let radius = 0.8;
    let sphere = Sphere {
        radius,
        center: Vec4f::new(0., 0., radius*2.2, 1.),
        color: Vec4f::new(1.0, 0., 0., 1.),
    };
    fb.raytrace_spheres(&[sphere]);
}

fn test_rasterization(fb: &mut Framebuffer) {
    use rasterization::Triangle;

    let tri = Triangle {
        position: (
            Vec4f::new( 1.0,  0., 0., 1.),
            Vec4f::new(-1.0,  1., 0., 1.),
            Vec4f::new(-0.5, -1., 0., 1.),
        ),
        color: (
            Vec4f::new(1.0, 0., 0., 1.),
            Vec4f::new(0.5, 0., 0., 1.),
            Vec4f::new(0.0, 0., 0., 1.),
        ),
    };
    fb.rasterize_triangles(&[tri]);
}
