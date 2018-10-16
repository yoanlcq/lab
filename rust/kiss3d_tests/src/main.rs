extern crate kiss3d;
extern crate nalgebra as na;
extern crate vek;

use std::rc::Rc;
use std::cell::RefCell;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use kiss3d::resource::Mesh;
use na::{UnitQuaternion, Vector3, Point3, Translation3};

struct AppState {
    tri: SceneNode,
    sphere: SceneNode,
    rot: UnitQuaternion<f32>,
}

impl State for AppState {
    fn step(&mut self, _: &mut Window) {
        self.tri.prepend_to_local_rotation(&self.rot)
    }
}

fn main() {
    let mut window = Window::new("Moving sphere vs tri");

    let mesh = {
        let faces = vec![Point3::new(0, 1, 2)];
        let coords = vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
        ];
        let normals = Some(vec![
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        ]);
        let uvs = None;
        let dynamic_draw = false;
        Mesh::new(coords, faces, normals, uvs, dynamic_draw)
    };
    let mesh = Rc::new(RefCell::new(mesh));

    let mut sphere = window.add_sphere(0.5);
    let mut tri = window.add_mesh(mesh, Vector3::new(1.0, 1.0, 1.0));

    sphere.set_local_translation(Translation3::new(0.0, 2.0, 0.0));
    sphere.set_color(1.0, 0.0, 0.0);
    tri.set_local_translation(Translation3::new(0.0, -1.0, 0.0));
    tri.set_color(0.0, 1.0, 0.0);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    let state = AppState { sphere, tri, rot };

    window.render_loop(state)
}


use vek::{Vec3, Vec2};

// https://www.geometrictools.com/Documentation/IntersectionMovingSphereTriangle.pdf

// The triangle is decomposed into 7 regions (Voronoi diagram):
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Region {
    R0,
    R1,
    R2,
    R01,
    R12,
    R20,
    R012,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Sphere {
    pub c: Vec3<f32>,
    pub r: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Element {
    pub min: f32,
    pub max: f32,
    pub region: Region,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Partition {
    pub elements: Vec<Element>, // FIXME: Use some kind of smallvec[7] instead (and derive Copy)
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct ContactInfo {
    pub time: f32,
    pub contact: Vec3<f32>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Triangle {
    // p0, p1 and p2 are the triangle's vertices.
    p0: Vec3<f32>,
    p1: Vec3<f32>,
    p2: Vec3<f32>,

    // ---- Quantities derived from P0, P1, and P2 :

    // u0, u1 and u2 are unit-length vectors that define an orthonormal basis such that u0 = normalize(p1-p0), and u2 = normalize(cross(p1-p0, p2-p0)).
    u0: Vec3<f32>,
    u1: Vec3<f32>,
    u2: Vec3<f32>,
    // q0, q1 and q2 are the planar-coordinates counterparts to p0, p1 and p2.
    q0: Vec2<f32>,
    q1: Vec2<f32>,
    q2: Vec2<f32>,
    // Edge vectors (not normalized)
    e0: Vec2<f32>,
    e1: Vec2<f32>,
    e2: Vec2<f32>,
    // n0 is the normal to the edge defined by q1-q0, pointing out of the triangle and along its plane.
    // same goes for n1 with q2-q1, and n2 with q0-q2.
    n0: Vec2<f32>,
    n1: Vec2<f32>,
    n2: Vec2<f32>,
}

impl Triangle {
    pub fn new(p0: Vec3<f32>, p1: Vec3<f32>, p2: Vec3<f32>) -> Self {
        let u2 = Vec3::cross(p1-p0, p2-p0).normalized();
        let u0 = (p1-p0).normalized();
        let u1 = Vec3::cross(u2, u0); // XXX: was written cross(N, U0)

        let l = (p1-p0).magnitude();
        let a = u0.dot(p2-p0);
        let b = u1.dot(p2-p0);

        let q0 = Vec2::zero();
        let q1 = Vec2::new(l, 0.);
        let q2 = Vec2::new(a, b);

        let e0 = q1 - q0;
        let e1 = q2 - q1;
        let e2 = q0 - q2;

        let n0 = Vec2::new(0., -1.);
        let n1 = Vec2::new(b, l-a) / (b*b + (l-a)*(l-a)).sqrt();
        let n2 = Vec2::new(-b, a) / (b*b + a*a).sqrt();

        Self {
            p0, p1, p2,
            u0, u1, u2,
            q0, q1, q2,
            e0, e1, e2,
            n0, n1, n2,
        }
    }

    pub fn compute_partition(&self, sphere: &Sphere, v: Vec3<f32>) -> Partition {
        let mut s = Partition::default();

        let p0_to_c = sphere.c - self.p0;
        let k = Vec2::new(self.u0.dot(p0_to_c), self.u1.dot(p0_to_c));
        let w = Vec2::new(self.u0.dot(v), self.u1.dot(v));

        if w == Vec2::zero() {
            s.elements.push(Element { min: -1. / 0., max: 1. / 0., region: self.get_containing_region(k), });
        } else {
            // PERF: Brute-force, unoptimized
            for region in [Region::R0, Region::R1, Region::R2, Region::R01, Region::R12, Region::R20, Region::R012].iter().cloned() {
                if let Some((min, max)) = self.get_overlap_interval(k, w, region) {
                    s.elements.push(Element { min, max, region });
                }
            }
            s.sort_elements();
        }

        s
    }
    pub fn compute_roots(&self, sphere: &Sphere, v: Vec3<f32>, element: &Element) -> Vec<ContactInfo> { // FIXME: returns either zero, one, or two roots. Using a Vec is overkill.
        let mut contacts = vec![];

        let radius_sq = sphere.r * sphere.r;
        let tmin = element.min;
        let tmax = element.max;

        match element.region {
            Region::R0 | Region::R1 | Region::R2 => {
                let p = match element.region {
                    Region::R0 => self.p0,
                    Region::R1 => self.p1,
                    Region::R2 => self.p2,
                    _ => unreachable!(),
                };
                let diff = sphere.c - p;
                let a0 = diff.dot(diff) - radius_sq;
                let a1 = v.dot(diff);
                let a2 = v.dot(v);
                let roots = solve_quadratic(tmin, tmax, a0, a1, a2);
                for root in roots {
                    contacts.push(ContactInfo { time: root, contact: p, });
                }
            },
            Region::R01 | Region::R12 | Region::R20 => {
                let (pa, pb) = match element.region {
                    Region::R01 => (self.p0, self.p1),
                    Region::R12 => (self.p1, self.p2),
                    Region::R20 => (self.p2, self.p0),
                    _ => unreachable!(),
                };
                let diff = sphere.c - pa;
                let edge = pb - pa;
                let s0 = edge.dot(diff) / edge.magnitude_squared();
                let s1 = edge.dot(v) / edge.magnitude_squared();
                let con_coeff = diff - edge * s0;
                let lin_coeff = v - edge * s1;
                let a0 = con_coeff.dot(con_coeff) - radius_sq;
                let a1 = con_coeff.dot(lin_coeff);
                let a2 = lin_coeff.dot(lin_coeff);
                let roots = solve_quadratic(tmin, tmax, a0, a1, a2);
                for root in roots {
                    contacts.push(ContactInfo { time: root, contact: pa + edge * (s1 * root + s0), });
                }
            },
            Region::R012 => {
                let diff = sphere.c - self.p0;
                let s0 = self.u2.dot(diff);
                let s1 = self.u2.dot(v);
                let a0 = s0*s0 - radius_sq;
                let a1 = s0*s1;
                let a2 = s1*s1;
                let roots = solve_quadratic(tmin, tmax, a0, a1, a2);
                for root in roots {
                    contacts.push(ContactInfo { time: root, contact: sphere.c + v * root - self.u2 * (s1 * root + s0), });
                }
            },
        }

        contacts
    }
    // Point-in-convex-region test, or half-space tests
    pub fn get_containing_region(&self, k: Vec2<f32>) -> Region {
        unimplemented!()
    }
    // Clip the line defined by K + t*W for all t, against the given region. if there is overlap, return (min, max).
    pub fn get_overlap_interval(&self, k: Vec2<f32>, w: Vec2<f32>, region: Region) -> Option<(f32, f32)> {
        unimplemented!()
    }
}

impl Partition {
    /// Sort so that interval times are increasing
    pub fn sort_elements(&mut self) {
        unimplemented!()
    }
}

fn get_contact(sphere: &Sphere, sphere_vel: Vec3<f32>, tri: Triangle, tri_vel: Vec3<f32>) -> Option<[ContactInfo; 2]> {
    let v = sphere_vel - tri_vel;
    let s = tri.compute_partition(sphere, v);
    let mut contacts = [ContactInfo { time: 1. / 0., contact: Vec3::zero(), }, ContactInfo { time: -1. / 0., contact: Vec3::zero(), }];
    let mut has_roots = false;

    for elem in s.elements.iter() {
        let roots = tri.compute_roots(sphere, v, elem);
        // PERF: Apparently we can exit when we detect 2 roots
        for root in roots {
            if root.time < contacts[0].time {
                contacts[0] = root;
                has_roots = true;
            }
            if root.time > contacts[1].time {
                contacts[1] = root;
                has_roots = true;
            }
        }
    }
    if has_roots {
        Some(contacts)
    } else {
        None
    }
}

pub fn solve_quadratic(tmin: f32, tmax: f32, a0: f32, a1: f32, a2: f32) -> Vec<f32> { // FIXME: returns either zero, one, or two roots. Using a Vec is overkill.
    let mut roots = vec![];
    if a2 != 0. { // FIXME: "close to zero" tests
        let discr = a1 * a1 - a0 * a2;
        if discr > 0. {
            let root_discr = discr.sqrt();
            let tmp0 = (-a1 - root_discr) / a2;
            let tmp1 = (-a1 + root_discr) / a2;
            if tmin <= tmp0 && tmp0 <= tmax {
                roots.push(tmp0);
            }
            if tmin <= tmp1 && tmp1 <= tmax {
                roots.push(tmp1);
            }
        } else if discr == 0. {
            let tmp = -a1 / a2;
            if tmin <= tmp && tmp <= tmax {
                roots.push(tmp);
                roots.push(tmp);
            }
        }
    } else if a1 != 0. {
        let tmp = -a0 / a1;
        if tmin <= tmp && tmp <= tmax {
            roots.push(tmp);
        }
    } else if a0 == 0. {
        roots.push(0.);
        roots.push(0.);
    }
    roots
}
