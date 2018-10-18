extern crate kiss3d;
extern crate nalgebra as na;
extern crate vek;
extern crate arrayvec;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate approx;

use std::rc::Rc;
use std::cell::RefCell;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use kiss3d::resource::Mesh;
use na::{Vector3, Point3, Translation3};

struct AppState {
    pub tri: Triangle,
    pub sphere: Sphere,
    pub tri_node: SceneNode,
    pub sphere_node: SceneNode,
    pub sphere_vel: Vec3<f32>,
    pub contact0_node: Option<SceneNode>,
    pub contact1_node: Option<SceneNode>,
}

impl State for AppState {
    fn step(&mut self, window: &mut Window) {
        {
            let v = self.sphere_vel;
            let c = self.sphere.c;
            let a = c - v * 1000.;
            let b = c + v * 1000.;
            window.draw_line(&Point3::new(a.x, a.y, a.z), &Point3::new(b.x, b.y, b.z), &Point3::new(1., 1., 0.));
        }
        // self.tri.prepend_to_local_rotation(&self.rot)
    }
}

fn main() {
    let mut window = Window::new("Moving sphere vs tri");

    let tri = Triangle::new(
        Vec3::new(10.0, 0.0, -5.0),
        Vec3::new(-10.0, 0.0, -5.0),
        Vec3::new(0.0, 0.0, 5.0),
    );
    let tri_vel = Vec3::zero();

    /*
    let sphere_vel = Vec3::new(0.0, -0.2, 1.0);
    let sphere = Sphere {
        c: Vec3::new(0.0, 1.4, 0.0),
        r: 0.5,
    };
    */
    /*
    let sphere_vel = Vec3::new(1., -0.1, 0.);
    let sphere = Sphere {
        c: Vec3::new(0.0, 1.4, -5.0),
        r: 0.5,
    };
    */
    let sphere_vel = Vec3::new(1., -0.5, 0.05);
    let mut sphere = Sphere {
        c: Vec3::new(0., 1.4, -5.4),
        r: 0.5,
    };

    let contacts = get_contact(&sphere, sphere_vel, &tri, tri_vel);
    println!("Contacts: {:#?}", contacts);

    let mut contact0_node = None;
    let mut contact1_node = None;
    if let Some(contacts) = contacts {
        let mut node = window.add_sphere(0.025);
        node.set_color(0., 1., 1.);
        let ct = contacts[0].contact;
        node.set_local_translation(Translation3::new(ct.x, ct.y, ct.z));
        contact0_node = Some(node);

        let mut node = window.add_sphere(0.025);
        node.set_color(0., 0., 1.);
        let ct = contacts[1].contact;
        node.set_local_translation(Translation3::new(ct.x, ct.y, ct.z));
        contact1_node = Some(node);

        sphere.c += sphere_vel * contacts[0].time;
    }

    let mesh = {
        let faces = vec![Point3::new(0, 1, 2)];
        let coords = vec![
            Point3::new(tri.p0.x, tri.p0.y, tri.p0.z),
            Point3::new(tri.p1.x, tri.p1.y, tri.p1.z),
            Point3::new(tri.p2.x, tri.p2.y, tri.p2.z),
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

    let mut sphere_node = window.add_sphere(sphere.r);
    let mut tri_node = window.add_mesh(mesh, Vector3::new(1.0, 1.0, 1.0) /* scale */);

    sphere_node.set_local_translation(Translation3::new(sphere.c.x, sphere.c.y, sphere.c.z));
    sphere_node.set_color(1.0, 0.0, 0.0);
    tri_node.set_local_translation(Translation3::new(0.0, 0.0, 0.0));
    tri_node.set_color(0.0, 1.0, 0.0);

    window.set_light(Light::StickToCamera);

    let state = AppState {
        sphere_node, tri_node, contact0_node, contact1_node,
        tri, sphere_vel, sphere,
    };

    window.render_loop(state)
}


use arrayvec::ArrayVec;
use vek::{Vec3, Vec2};

// https://www.geometrictools.com/Documentation/IntersectionMovingSphereTriangle.pdf

// The triangle is decomposed into 7 regions (Voronoi diagram):
bitflags! {
    struct Region: u8 {
        const R0   = 1 << 0;
        const R1   = 1 << 1;
        const R2   = 1 << 2;
        const R01  = 1 << 3;
        const R12  = 1 << 4;
        const R20  = 1 << 5;
        const R012 = 1 << 6;
    }
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

#[derive(Debug, Default, /* Copy, */Clone, PartialEq)]
struct Partition {
    pub elements: ArrayVec<[Element; 7]>,
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
    // n0 is the normal to the edge defined by q1-q0, pointing out of the triangle and along its plane.
    // same goes for n1 with q2-q1, and n2 with q0-q2.
    n0: Vec2<f32>,
    n1: Vec2<f32>,
    n2: Vec2<f32>,
}

use std::f32::INFINITY as INF;
use std::f32::NEG_INFINITY as NEG_INF;

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

        let n0 = Vec2::new(0., -1.);
        let n1 = Vec2::new(b, l-a) / (b*b + (l-a)*(l-a)).sqrt();
        let n2 = Vec2::new(-b, a) / (b*b + a*a).sqrt();

        Self {
            p0, p1, p2,
            u0, u1, u2,
            q0, q1, q2,
            n0, n1, n2,
        }
    }

    pub fn compute_partition(&self, sphere: &Sphere, v: Vec3<f32>) -> Partition {
        let p0_to_c = sphere.c - self.p0;
        let k = Vec2::new(self.u0.dot(p0_to_c), self.u1.dot(p0_to_c));
        let w = Vec2::new(self.u0.dot(v), self.u1.dot(v));

        let mut partition = Partition::default();

        if relative_eq!(w, Vec2::zero()) {
            partition.elements.push(Element { min: NEG_INF, max: INF, region: self.get_containing_region(k), });
            return partition;
        }

        let w_mag = w.magnitude();
        let w_norm = w / w_mag;

        let line  = Line2::new(k, k+w);
        let i01   = line.vs_seg(self.q0, self.q1).map(|i| (i, Region::R01 | Region::R012));
        let i12   = line.vs_seg(self.q1, self.q2).map(|i| (i, Region::R12 | Region::R012));
        let i20   = line.vs_seg(self.q2, self.q0).map(|i| (i, Region::R20 | Region::R012));
        let iq0n0 = line.vs_ray(self.q0, self.n0).map(|i| (i, Region::R0  | Region::R01 ));
        let iq0n2 = line.vs_ray(self.q0, self.n2).map(|i| (i, Region::R0  | Region::R20 ));
        let iq1n0 = line.vs_ray(self.q1, self.n0).map(|i| (i, Region::R1  | Region::R01 ));
        let iq1n1 = line.vs_ray(self.q1, self.n1).map(|i| (i, Region::R1  | Region::R12 ));
        let iq2n1 = line.vs_ray(self.q2, self.n1).map(|i| (i, Region::R2  | Region::R12 ));
        let iq2n2 = line.vs_ray(self.q2, self.n2).map(|i| (i, Region::R2  | Region::R20 ));

        let pts = [i01, i12, i20, iq0n0, iq0n2, iq1n0, iq1n1, iq2n1, iq2n2];
        let mut pts: Vec<_> = pts.iter().cloned().filter_map(|p| p).map(|(p, regs)| ((p-k).dot(w_norm) / w_mag, regs)).collect();
        assert!(pts.len() >= 2, "The partitioned line must have at least 2 points by now");
        pts.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        let mut i = 1;
        loop {
            if i >= pts.len() {
                break;
            }
            if relative_eq!(pts[i-1].0, pts[i].0) {
                pts[i-1].1 |= pts[i].1;
                pts.remove(i);
            } else {
                i += 1;
            }
        }

        println!("pts: {:#?}", &pts[..]);

        let mut elem = Element { min: NEG_INF, max: INF, region: Region::empty(), };

        for i in 0..pts.len() {
            elem.max = pts[i].0;
            elem.region = if i == 0 { pts[i].1 & !pts[i+1].1 } else { pts[i].1 & pts[i-1].1 };
            partition.elements.push(elem);
            elem.min = elem.max;
        }
        let i = pts.len();
        elem.max = INF;
        elem.region = pts[i-1].1 & !pts[i-2].1;
        partition.elements.push(elem);

        println!("Partition : {:#?}", &partition.elements[..]);

        assert!(partition.elements.len() >= 3, "The partition must have at least 3 elements in this case");

        partition
    }
    pub fn compute_roots(&self, sphere: &Sphere, v: Vec3<f32>, element: &Element) -> ArrayVec<[ContactInfo; 2]> {
        let mut contacts = ArrayVec::default();

        let radius_sq = sphere.r * sphere.r;
        let tmin = element.min;
        let tmax = element.max;

        if (element.region & (Region::R0 | Region::R1 | Region::R2)).bits() != 0 {
            let p = match element.region & (Region::R0 | Region::R1 | Region::R2) {
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
        } else if (element.region & (Region::R01 | Region::R12 | Region::R20)).bits() != 0 {
            let (pa, pb) = match element.region & (Region::R01 | Region::R12 | Region::R20) {
                Region::R01 => (self.p0, self.p1),
                Region::R12 => (self.p1, self.p2),
                Region::R20 => (self.p2, self.p0),
                _ => unreachable!(),
            };
            let diff = sphere.c - pa;
            let edge = pb - pa;
            let edge_mag_sq = edge.magnitude_squared();
            let s0 = edge.dot(diff) / edge_mag_sq;
            let s1 = edge.dot(v) / edge_mag_sq;
            let con_coeff = diff - edge * s0;
            let lin_coeff = v - edge * s1;
            let a0 = con_coeff.dot(con_coeff) - radius_sq;
            let a1 = con_coeff.dot(lin_coeff);
            let a2 = lin_coeff.dot(lin_coeff);
            let roots = solve_quadratic(tmin, tmax, a0, a1, a2);
            for root in roots {
                contacts.push(ContactInfo { time: root, contact: pa + edge * (s1 * root + s0), });
            }
        } else {
            assert_eq!(element.region, Region::R012);
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
        }

        contacts
    }
    // Point-in-convex-region test, or half-space tests
    pub fn get_containing_region(&self, k: Vec2<f32>) -> Region {
        let s01 = k.determine_side(self.q0, self.q1);
        let s12 = k.determine_side(self.q1, self.q2);
        let s20 = k.determine_side(self.q2, self.q0);

        if s01 >= 0. && s12 >= 0. && s20 >= 0. {
            return Region::R012;
        }

        let q0_n0_side = k.determine_side(self.q0, self.q0 + self.n0);
        let q0_n2_side = k.determine_side(self.q0, self.q0 + self.n2);
        let q1_n0_side = k.determine_side(self.q1, self.q1 + self.n0);
        let q1_n1_side = k.determine_side(self.q1, self.q1 + self.n1);
        let q2_n1_side = k.determine_side(self.q2, self.q2 + self.n1);
        let q2_n2_side = k.determine_side(self.q2, self.q2 + self.n2);

        if q0_n0_side <= 0. && q0_n2_side >= 0. {
            return Region::R0;
        }
        if q1_n1_side <= 0. && q1_n0_side >= 0. {
            return Region::R1;
        }
        if q2_n2_side <= 0. && q2_n1_side >= 0. {
            return Region::R2;
        }

        if q0_n0_side >= 0. && q1_n0_side <= 0. && s01 <= 0. {
            return Region::R01;
        }
        if q1_n1_side >= 0. && q2_n1_side <= 0. && s12 <= 0. {
            return Region::R12;
        }
        if q2_n2_side >= 0. && q0_n2_side <= 0. && s20 <= 0. {
            return Region::R20;
        }
        unreachable!()
    }
}

fn get_contact(sphere: &Sphere, sphere_vel: Vec3<f32>, tri: &Triangle, tri_vel: Vec3<f32>) -> Option<[ContactInfo; 2]> {
    let v = sphere_vel - tri_vel;
    let s = tri.compute_partition(sphere, v);
    let mut contacts = [
        ContactInfo { time: INF, contact: Vec3::zero(), },
        ContactInfo { time: NEG_INF, contact: Vec3::zero(), }
    ];

    // let mut i = 0;
    let mut nb = 0;

    for elem in s.elements.iter() {
        let roots = tri.compute_roots(sphere, v, elem);
        // PERF: Apparently we can exit when we detect 2 roots
        for root in roots {
            if root.time < contacts[0].time {
                contacts[0] = root;
                nb += 1;
            }
            if root.time > contacts[1].time {
                contacts[1] = root;
                nb += 1;
            }
            if nb > 2 {
                break;  // At least 2 roots found, we can leave the loop early. (nb == 2 when first root is found)
            }
        }
        //println!("i = {}, nb = {}, contacts = {:?}", i, nb, &contacts[..]);
        //i += 1;
    }
    if nb > 0 {
        Some(contacts)
    } else {
        None
    }
}

pub fn solve_quadratic(tmin: f32, tmax: f32, a0: f32, a1: f32, a2: f32) -> ArrayVec<[f32; 2]> {
    let mut roots = ArrayVec::default();
    if relative_ne!(a2, 0.) {
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
        } else if relative_eq!(discr, 0.) {
            let tmp = -a1 / a2;
            if tmin <= tmp && tmp <= tmax {
                roots.push(tmp);
                roots.push(tmp);
            }
        }
    } else if relative_ne!(a1, 0.) {
        let tmp = -a0 / a1;
        if tmin <= tmp && tmp <= tmax {
            roots.push(tmp);
        }
    } else if relative_eq!(a0, 0.) {
        roots.push(0.);
        roots.push(0.);
    }
    roots
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct Line2 {
    pub a: Vec2<f32>,
    pub b: Vec2<f32>,
}

impl Line2 {
    pub fn new(a: Vec2<f32>, b: Vec2<f32>) -> Self {
        Self {
            a, b
        }
    }
    pub fn ret_t(&self, c: Vec2<f32>, d: Vec2<f32>) -> Option<f32> {
        let &Self { a, b } = &self;
        let numtr = (a.x - b.x)*(a.y - c.y) - (a.y - b.y)*(a.x - c.x);
        let denom = (a.x - b.x)*(c.y - d.y) - (a.y - b.y)*(c.x - d.x);
        if relative_eq!(denom, 0.) { None } else { Some( - numtr / denom) }
    }
    pub fn vs_seg(&self, s0: Vec2<f32>, s1: Vec2<f32>) -> Option<Vec2<f32>> {
        let t = self.ret_t(s0, s1)?;
        if 0. <= t && t <= 1. {
            return Some(s0 + (s1 - s0) * t);
        }
        None
    }
    pub fn vs_ray(&self, origin: Vec2<f32>, dir: Vec2<f32>) -> Option<Vec2<f32>> {
        let t = self.ret_t(origin, origin + dir)?;
        if 0. <= t {
            return Some(origin + dir * t);
        }
        None
    }
}
