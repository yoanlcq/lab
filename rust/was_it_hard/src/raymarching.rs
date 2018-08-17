// http://jamie-wong.com/2016/07/15/ray-marching-signed-distance-functions/
// http://iquilezles.org/www/articles/distfunctions/distfunctions.htm

use math::Vec4f;
use cmp;
use ndc;
use ray::Ray;
use std::f32::EPSILON;
use framebuffer::Framebuffer;

pub trait Sdf {
    fn sdf(&self, p: Vec4f) -> f32;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub radius: f32,
}

impl Sdf for Sphere {
    fn sdf(&self, p: Vec4f) -> f32 {
        p.magnitude()  - self.radius
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RaymarchHit {
    pub color: Vec4f,
    pub depth: f32,
    pub normal: Vec4f,
    pub position: Vec4f,
}

pub const MAX_MARCHING_STEPS: u32 = 128;

impl Framebuffer {
    pub fn raymarch_sdfs(&mut self, sdfs: &[&Sdf]) {
        for y in 0..self.h {
            for x in 0..self.w {
                self.pixel_raymarch_sdfs(x, y, sdfs);
            }
        }
    }

    fn pixel_raymarch_sdfs(&mut self, x: u32, y: u32, sdfs: &[&Sdf]) {
        let i = (y * self.w + x) as usize;
        let ray = Ray {
            origin: ndc::from_pixel(x, y, self.w, self.h),
            direction: Vec4f::new(0., 0., 1., 0.),
        };
        // FIXME: 2048 as a depth limit is arbitrary and buried in this code
        let hit = match ray.raymarch_sdfs(sdfs, 2048.) {
            Some(hit) => hit,
            None => return,
        };
        if hit.depth >= self.depth[i] {
            return;
        }
        self.depth[i] = hit.depth;
        self.color[i] = hit.color;
    }
}

impl Ray {
    pub fn raymarch_sdfs(&self, sdfs: &[&Sdf], limit: f32) -> Option<RaymarchHit> {
        let mut depth = 0.;
        for _ in 0..MAX_MARCHING_STEPS {
            let eye = self.origin; // XXX It the ray's origin really the 'eye' ?
            let position = eye + self.direction * depth;
            let dist = scene_sdf(sdfs, position);
            if dist < EPSILON {
                let color = Vec4f::new(1., 0., 0., 1.); // FIXME we assume red...
                let normal = estimate_normal(sdfs, position);
                return Some(RaymarchHit {
                    color, depth, position, normal,
                });
            }
            depth += dist;

            if depth >= limit {
                return None;
            }
        }
        None
    }
}

pub fn scene_sdf(sdfs: &[&Sdf], p: Vec4f) -> f32 {
    let mut d = 1. / 0.;
    for sdf in sdfs {
        d = union_sdf(d, sdf.sdf(p));
    }
    d
}

pub fn intersect_sdf(a: f32, b: f32) -> f32 {
    cmp::partial_max(a, b)
}
pub fn union_sdf(a: f32, b: f32) -> f32 {
    cmp::partial_min(a, b)
}
pub fn difference_sdf(a: f32, b: f32) -> f32 {
    cmp::partial_max(a, -b)
}

pub fn estimate_normal(sdfs: &[&Sdf], p: Vec4f) -> Vec4f {
    Vec4f::new(
        scene_sdf(sdfs, Vec4f::new(p.x + EPSILON, p.y, p.z, 0.)) - scene_sdf(sdfs, Vec4f::new(p.x - EPSILON, p.y, p.z, 0.)),
        scene_sdf(sdfs, Vec4f::new(p.x, p.y + EPSILON, p.z, 0.)) - scene_sdf(sdfs, Vec4f::new(p.x, p.y - EPSILON, p.z, 0.)),
        scene_sdf(sdfs, Vec4f::new(p.x, p.y, p.z + EPSILON, 0.)) - scene_sdf(sdfs, Vec4f::new(p.x, p.y, p.z - EPSILON, 0.)),
        0.
    ).normalized()
}
