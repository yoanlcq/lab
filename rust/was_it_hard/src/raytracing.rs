use ndc;
use cmp::{self, ForceOrd};
use vec4::Vec4f;
use framebuffer::Framebuffer;
use ray::Ray;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub center: Vec4f,
    pub radius: f32,
    pub color: Vec4f,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RaycastHit {
    pub position: Vec4f,
    pub normal: Vec4f,
    pub color: Vec4f,
    pub depth: f32,
}

impl Ray {
    pub fn cast_on_sphere(&self, sphere: &Sphere) -> Option<RaycastHit> {
        let ray = self;
        let cr = ray.origin - sphere.center;

        let a = Vec4f::dot(ray.direction, ray.direction);
        let b = Vec4f::dot(cr, ray.direction) * 2.;
        let c = Vec4f::dot(cr, cr) - sphere.radius*sphere.radius;
        let delta = b*b - 4.*a*c;
        if delta < 0. {
            return None;
        }

        let t1 = (-b - delta.sqrt()) / (2.*a);
        let t2 = (-b + delta.sqrt()) / (2.*a);
        if t1 < 0. && t2 < 0. {
            return None;
        }

        let t = if t1 < 0. {
            t2
        } else {
            if t2 < 0. {
                t1
            } else {
                cmp::partial_min(t1, t2)
            }
        };

        let hit = {
            let color = sphere.color;
            let depth = t;
            let position = ray.origin + ray.direction * depth;
            let normal = (position - sphere.center).normalized();
            RaycastHit {
                color, depth, position, normal,
            }
        };

        Some(hit)
    }
    pub fn cast_on_spheres(&self, spheres: &[Sphere]) -> Vec<RaycastHit> {
        let mut hits = vec![];
        for sphere in spheres {
            if let Some(hit) = self.cast_on_sphere(sphere) {
                hits.push(hit);
            }
        }
        hits.sort_unstable_by_key(|hit| ForceOrd(hit.depth));
        hits
    }
}

impl Framebuffer {
    pub fn raytrace_spheres(&mut self, spheres: &[Sphere]) {
        for y in 0..self.h {
            for x in 0..self.w {
                self.pixel_raytrace_spheres(x, y, spheres);
            }
        }
    }
    fn pixel_raytrace_spheres(&mut self, x: u32, y: u32, spheres: &[Sphere]) {
        let i = (y * self.w + x) as usize;
        let ray = Ray {
            origin: ndc::from_pixel(x, y, self.w, self.h),
            direction: Vec4f::new(0., 0., 1., 0.),
        };
        let hits = ray.cast_on_spheres(spheres);
        let hit = match hits.get(0) {
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
