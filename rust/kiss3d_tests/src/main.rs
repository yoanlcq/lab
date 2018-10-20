#![feature(macro_literal_matcher)]

extern crate kiss3d;
extern crate nalgebra as na;
extern crate vek;
extern crate arrayvec;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate approx;

use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use kiss3d::event::Key;
use kiss3d::event::Action;
use na::{Vector3, Point3, Translation3};

struct AppState {
    pub sphere: Sphere,
    pub tri_node: SceneNode,
    pub sphere_node: SceneNode,
    pub sphere_vel: Vec3<f32>,
    pub sphere_move_vel: Vec3<f32>,
    pub sphere_gravity_vel: Vec3<f32>,
    pub is_grounded: bool,
}

#[derive(Debug)]
struct AllContacts {
    pub t: f32,
    pub ct: Vec3<f32>,
    pub highest_negative_contact_time: f32,
}

impl AllContacts {
    pub fn new(sphere: &Sphere, v: Vec3<f32>) -> Option<Self> {
        let tri_vel = Vec3::zero();
        let mut contacts: Option<[ContactInfo; 2]> = None;
        let mut highest_negative_contact_time = NEG_INF;
        let mut accum = Vec::with_capacity(8);

        for f in LV_FACES {
            // In OBJ, indices start at 1...
            let p0 = LV_TRIS[f.x as usize - 1];
            let p1 = LV_TRIS[f.y as usize - 1];
            let p2 = LV_TRIS[f.z as usize - 1];
            let tri = Triangle::new(p0, p1, p2).expect("Degenerate triangle");
            if let Some(candidates) = get_contact(sphere, v, &tri, tri_vel) {
                if candidates[0].time >= 0. {
                    if let Some(contacts_) = contacts {
                        if candidates[0].time < contacts_[0].time {
                            contacts = Some(candidates);
                            accum.insert(0, candidates[0].time);
                        }
                    } else {
                        contacts = Some(candidates);
                        accum.insert(0, candidates[0].time);
                    }
                } else if candidates[0].time > highest_negative_contact_time {
                    highest_negative_contact_time = candidates[0].time;
                }
            }
        }

        /*
        println!("accum: {:?}", if accum.len() >= 8 { &accum[0..8] } else { &accum[..] });
        if accum.len() >= 2 {
            println!("accum: ");
            let a = sphere.c.distance(sphere.c + v * accum[0]) - sphere.r;
            let b = sphere.c.distance(sphere.c + v * accum[1]) - sphere.r;
            println!("a: {}, b: {}, diff: {}", a, b, (a - b).abs());
        }
        */

        contacts.map(|c| Self { t: c[0].time, ct: c[0].contact, highest_negative_contact_time })
    }
}

fn proj_on_plane(v: Vec3<f32>, n: Vec3<f32>) -> Vec3<f32> {
    v - n * v.dot(n) / n.magnitude_squared()
}

fn plane_dist(plane: Plane, point: Vec3<f32>) -> f32 {
    plane.n.normalized().dot(point - plane.o)
}

#[derive(Debug, Copy, Clone)]
struct Plane {
    o: Vec3<f32>,
    n: Vec3<f32>,
}

impl AppState {
    fn try_move(&mut self, window: &mut Window, mut v: Vec3<f32>) -> Vec3<f32> {
        let very_close = 0.001;
        let mut dst = self.sphere.c + v;
        let mut first_plane = Plane { o: Vec3::zero(), n: Vec3::zero() };

        for i in 0..3 {
            match AllContacts::new(&self.sphere, v) {
                None => {
                    self.sphere.c += v;
                    if i == 0 {
                        self.is_grounded = false;
                    }
                    return v;
                },
                Some(AllContacts { t, ct, .. }) => {
                    if t <= 0. || t > 1. {
                        self.sphere.c += v;
                        if i == 0 {
                            self.is_grounded = false;
                        }
                        return v;
                    }

                    let t = t.clamped01();

                    use ::vek::ops::Clamp;
                    let dist = v.magnitude() * t;
                    let short_dist = ::vek::partial_max(0., dist - very_close);

                    let touch_point = self.sphere.c + v * t;
                    let near_point = self.sphere.c + v.normalized() * short_dist;
                    let n = (touch_point - ct).normalized();
                    assert!(!n.x.is_nan(), "n.x is NaN, i = {}", i);
                    assert!(!n.y.is_nan(), "n.y is NaN, i = {}", i);
                    assert!(!n.z.is_nan(), "n.z is NaN, i = {}", i);

                    let sliding_plane = Plane { o: ct, n, };

                    self.is_grounded = n.dot(Vec3::new(0., 1., 0.)) >= 0.1;
                    let use_dead_zone = n.dot(Vec3::new(0., 1., 0.));

                    self.sphere.c += v.normalized() * short_dist;

                    if i == 0 {
                        let long_radius = self.sphere.r + very_close; // XXX
                        first_plane = sliding_plane;
                        dst -= first_plane.n * (plane_dist(first_plane, dst) - long_radius);
                        v = dst - self.sphere.c;
                        assert!(!v.x.is_nan(), "v.x is NaN");
                        assert!(!v.y.is_nan(), "v.y is NaN");
                        assert!(!v.z.is_nan(), "v.z is NaN");
                    } else if i == 1 {
                        let second_plane = sliding_plane;
                        let crease = first_plane.n.cross(second_plane.n).normalized();
                        assert!(!crease.x.is_nan(), "crease.x is NaN");
                        assert!(!crease.y.is_nan(), "crease.y is NaN");
                        assert!(!crease.z.is_nan(), "crease.z is NaN");
                        let signed_dist = (dst - self.sphere.c /* near_point*/).dot(crease);
                        v = crease * signed_dist;
                        assert!(!v.x.is_nan(), "v.x is NaN");
                        assert!(!v.y.is_nan(), "v.y is NaN");
                        assert!(!v.z.is_nan(), "v.z is NaN");
                        dst = self.sphere.c + v;
                    }

                    // println!("v = {}, v.mag = {}", v, v.magnitude());
                    if v.magnitude() < 0.01 * use_dead_zone * use_dead_zone { // Make the sphere stop at some point (dead zone)
                        v = Vec3::zero();
                        return v;
                    }
                }
            }
        }
        v
    }
    /*
    fn try_move(&mut self, window: &mut Window, v: Vec3<f32>) -> Vec3<f32> {
        self.try_move_recursive(window, v, 0)
    }
    fn try_move_recursive(&mut self, window: &mut Window, v: Vec3<f32>, recursion_level: u32) -> Vec3<f32> {
        let very_close = 0.002;

        match AllContacts::new(&self.sphere, v) {
            None => {
                self.sphere.c += v;
                self.is_grounded = false; // XXX: Works because overridden by gravity after
                v
            },
            Some(AllContacts { t, mut ct, .. }) => {
                if t <= 0. {  // At this point we're already penetrating a triangle, => too late!
                    self.sphere.c += v;
                    self.is_grounded = false; // XXX: Works because overridden by gravity after
                    return v;
                }

                use ::vek::ops::Clamp;
                let vt = v * t.clamped01();
                self.sphere.c += vt;

                // Now the sphere either exactly touches the plane, or is from some distance of it.

                let dist = self.sphere.c.distance(ct) - self.sphere.r;

                if dist > very_close {
                    self.is_grounded = false; // XXX: Works because overridden by gravity after
                    return /*v -*/ vt;
                }

                self.is_grounded = true;

                self.sphere.c -= v.normalized() * (very_close - dist);
                ct -= v.normalized() * very_close;
                let n = self.sphere.c - ct;
                let pv = proj_on_plane(v - vt, n);

                if pv.magnitude() <= very_close {
                    return pv;
                }

                if recursion_level < 20 {
                    self.try_move_recursive(window, pv, recursion_level + 1)
                } else {
                    pv
                }
            },
        }
    }
*/
}


impl State for AppState {
    fn step(&mut self, window: &mut Window) {
        if self.sphere.c.y < -8.0 {
            self.sphere.c = Vec3::new(0.0, 2.0, 0.0);
        }

        let mut mv = Vec3::<f32>::zero();
        let speed = 0.1;
        if window.get_key(Key::Left) == Action::Press {
            mv.x -= speed;
        }
        if window.get_key(Key::Right) == Action::Press {
            mv.x += speed;
        }
        if window.get_key(Key::Up) == Action::Press {
            mv.z -= speed;
        }
        if window.get_key(Key::Down) == Action::Press {
            mv.z += speed;
        }
        if window.get_key(Key::Space) == Action::Press && self.is_grounded {
            self.sphere_gravity_vel.y += 0.3;
            self.is_grounded = false;
        }

        self.sphere_move_vel = self.try_move(window, mv);

        let gravity = Vec3::new(0.0, -0.008, 0.0) + self.sphere_gravity_vel;
        //println!("gravity: {}", gravity);
        self.sphere_gravity_vel = self.try_move(window, gravity);
        //println!("sphere_gravity_vel: {}", self.sphere_gravity_vel);
        
        // println!("grounded: {}", self.is_grounded);

        self.sphere_vel = self.sphere_move_vel + self.sphere_gravity_vel;

        self.sphere_node.set_local_translation(Translation3::new(self.sphere.c.x, self.sphere.c.y, self.sphere.c.z));

        {
            let v = self.sphere_vel;
            let c = self.sphere.c;
            let a = c - v * 1000.;
            let b = c + v * 1000.;
            window.draw_line(&Point3::new(a.x, a.y, a.z), &Point3::new(b.x, b.y, b.z), &Point3::new(1., 1., 0.));
        }
    }
}

// TODO: Detect slopes (to prevent climbing steep slopes)
// TODO: Control all of the epsilons
// TODO: "crease" effect is too strong (sphere gets stuck on edges)
// TODO: use soft_asserts (which also log the current CCT state). Replace all unreachable!() as well !!!
// TODO: No more Vec

fn main() {
    let mut window = Window::new("Moving sphere vs tri");

    let sphere = Sphere {
        c: Vec3::new(0., 1.4, 0.0),
        r: 0.5,
    };

    let mut sphere_node = window.add_sphere(sphere.r);
    let mut tri_node = window.add_obj("/home/yoon/level.obj".as_ref(), "/home/yoon/".as_ref(), Vector3::new(1.0, 1.0, 1.0) /* scale */);

    sphere_node.set_color(1.0, 0.0, 0.0);
    tri_node.set_local_translation(Translation3::new(0.0, 0.0, 0.0));
    tri_node.set_color(0.0, 1.0, 0.0);

    window.set_light(Light::StickToCamera);

    let state = AppState {
        sphere_node, tri_node,
        sphere,
        sphere_vel: Vec3::zero(),
        sphere_move_vel: Vec3::zero(),
        sphere_gravity_vel: Vec3::zero(),
        is_grounded: false,
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
    pub fn new(p0: Vec3<f32>, p1: Vec3<f32>, p2: Vec3<f32>) -> Option<Self> {
        let u0 = (p1-p0).normalized();
        let u2 = Vec3::cross(p1-p0, p2-p0).normalized();
        let u1 = Vec3::cross(u2, u0); // XXX: was written cross(N, U0)
        if (u0.map(|x| x.is_nan()) | u1.map(|x| x.is_nan()) | u2.map(|x| x.is_nan())).reduce_or() {
            return None; // Degenerate triangle
        }

        let l = (p1-p0).magnitude();
        let a = u0.dot(p2-p0);
        let b = u1.dot(p2-p0);

        let q0 = Vec2::zero();
        let q1 = Vec2::new(l, 0.);
        let q2 = Vec2::new(a, b);

        let n0 = Vec2::new(0., -1.);
        let n1 = Vec2::new(b, l-a) / (b*b + (l-a)*(l-a)).sqrt();
        let n2 = Vec2::new(-b, a) / (b*b + a*a).sqrt();

        Some(Self {
            p0, p1, p2,
            u0, u1, u2,
            q0, q1, q2,
            n0, n1, n2,
        })
    }

    pub fn compute_partition(&self, sphere: &Sphere, v: Vec3<f32>) -> Partition {
        let p0_to_c = sphere.c - self.p0;
        let k = Vec2::new(self.u0.dot(p0_to_c), self.u1.dot(p0_to_c));
        let w = Vec2::new(self.u0.dot(v), self.u1.dot(v));

        /*
        println!("p0 = {}", self.p0);
        println!("p1 = {}", self.p1);
        println!("p2 = {}", self.p2);
        println!("u0 = {}", self.u0);
        println!("u1 = {}", self.u1);
        println!("v  = {}", v);
        println!("w  = {}", w);
        */

        let mut partition = Partition::default();

        let w_mag = w.magnitude();

        if relative_eq!(w_mag, 0., epsilon = 0.0001) {
            partition.elements.push(Element { min: NEG_INF, max: INF, region: self.get_containing_region(k), });
            return partition;
        }

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
        let mut pts: Vec<_> = pts.iter().cloned().filter_map(|p| p).map(|(p, regs)| ((p-k).dot(w_norm) / w_mag, regs)).collect(); // PERF: Using Vec!!!
        assert!(pts.len() >= 2, "The partitioned line must have at least 2 points by now; Instead it is {:?}. The line is {:?}", &pts[..], line);
        pts.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        let orig_pts = pts.clone();

        let mut i = 1;
        loop {
            if i >= pts.len() {
                break;
            }
            if relative_eq!(pts[i-1].0, pts[i].0, epsilon = 0.0001) {
                pts[i-1].1 |= pts[i].1;
                pts.remove(i);
            } else {
                i += 1;
            }
        }

        // println!("pts: {:#?}", &pts[..]);

        let mut elem = Element { min: NEG_INF, max: INF, region: Region::empty(), };

        for i in 0..pts.len() {
            elem.region = if i == 0 { pts[i].1 & !pts[i+1].1 } else { pts[i].1 & pts[i-1].1 };
            if elem.region.is_empty() { // Seems to happen because of numeric errors. Quite rare.
                continue;
            }
            elem.max = pts[i].0;
            partition.elements.push(elem);
            elem.min = elem.max;
        }
        let i = pts.len();
        elem.max = INF;
        elem.region = pts[i-1].1 & !pts[i-2].1;
        partition.elements.push(elem);


        for element in partition.elements.iter() {
            assert!(element.min <= element.max, "min > max; {:#?}", partition);

            assert_ne!(element.region, Region::empty(), "Region of element was empty; {:#?}; orig_pts = {:#?}", partition, orig_pts);
            let r = element.region & (Region::R0 | Region::R1 | Region::R2);
            if r.bits() != 0 {
                assert_eq!(r.bits().count_ones(), 1, "Region of element was supposed be one of R0, R1 or R2, but was {:?}; {:#?}; orig_pts = {:#?}", element.region, partition, orig_pts);
            } else {
                let r = element.region & (Region::R01 | Region::R12 | Region::R20);
                if r.bits() != 0 {
                    assert_eq!(r.bits().count_ones(), 1, "Region of element was supposed be one of R01, R12 or R20, but was {:?}; {:#?}; orig_pts = {:#?}", element.region, partition, orig_pts);
                } else {
                    assert_eq!(element.region, Region::R012, "Region of element was supposed be R012, but was {:?}; {:#?}; orig_pts = {:#?}", element.region, partition, orig_pts);
                }
            } 
        }

        // println!("Partition : {:#?}", &partition.elements[..]);

        // assert!(partition.elements.len() >= 3, "The partition must have at least 3 elements in this case. Instead it is {:#?}", partition);

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

macro_rules! cat {
    ($($x:literal $y:literal $z:literal)+) => { [$(Vec3 { x: $x, y: $y, z: $z, },)+] };
}

static LV_TRIS: &[Vec3<f32>] = &cat![
    -7.312956 0.000000 7.312956
    7.312956 0.000000 7.312956
    -7.312956 0.000000 -7.312956
    7.312956 0.000000 -7.312956
    -7.312956 0.000000 0.000000
    0.000000 0.000000 7.312956
    7.312956 0.000000 0.000000
    0.000000 0.000000 -7.312956
    0.000000 0.000000 0.000000
    -7.312956 0.000000 3.656478
    3.656478 0.000000 7.312956
    7.312956 0.000000 -3.656478
    -3.656478 0.000000 -7.312956
    -7.312956 0.000000 -3.656478
    -3.656478 0.000000 7.312956
    7.312956 0.000000 3.656478
    3.656478 0.000000 -7.312956
    0.000000 0.000000 -3.656478
    0.000000 0.930497 3.656478
    -3.656478 0.000000 0.000000
    3.656478 0.000000 0.000000
    3.656478 0.000000 3.656478
    -3.656478 2.342503 3.656478
    -3.656478 0.000000 -3.656478
    3.656478 3.154645 -3.656478
    -7.312956 0.000000 5.484717
    5.484717 0.000000 7.312956
    7.312956 0.000000 -5.484717
    -5.484717 0.000000 -7.312956
    -7.312956 0.000000 -1.828239
    -1.828239 0.000000 7.312956
    7.312956 0.000000 1.828239
    1.828239 0.000000 -7.312956
    0.000000 0.000000 -5.484717
    0.000000 0.000000 1.828239
    -5.484717 0.000000 0.000000
    1.828239 0.000000 0.000000
    -7.312956 0.000000 1.828239
    1.828239 0.000000 7.312956
    7.312956 0.000000 -1.828239
    -1.828239 0.000000 -7.312956
    -7.312956 0.000000 -5.484717
    -5.484717 0.000000 7.312956
    7.312956 0.000000 5.484717
    5.484717 0.000000 -7.312956
    0.000000 0.000000 -1.828239
    0.000000 0.000000 5.484717
    -1.828239 0.000000 0.000000
    5.484717 0.000000 0.000000
    3.656478 0.000000 1.828239
    3.656478 0.000000 5.484717
    1.828239 1.844172 3.656478
    5.484717 -1.433271 3.656478
    -3.656478 0.000000 1.828239
    -3.656478 0.000000 5.484717
    -5.484717 2.342503 3.656478
    -1.828239 0.000000 3.656478
    -3.656478 0.000000 -5.484717
    -3.656478 0.000000 -1.828239
    -5.484717 0.000000 -3.656478
    -1.828239 0.000000 -3.656478
    3.816276 0.810387 -4.111725
    3.657956 0.065377 -3.104225
    2.942980 0.000000 -3.656478
    3.932896 0.772900 -3.635467
    4.638572 1.158739 -2.513397
    2.989855 -0.478099 -3.630000
    3.340670 0.432572 -3.918153
    -1.828239 4.002408 -1.828239
    -5.484717 0.000000 -1.828239
    -5.484717 0.000000 -5.484717
    -1.828239 0.000000 5.484717
    -5.484717 0.000000 5.484717
    -5.484717 0.000000 1.828239
    5.484717 0.000000 5.484717
    1.828239 0.000000 5.484717
    1.828239 0.000000 1.828239
    5.484717 -1.224619 1.828239
    -1.828239 0.000000 1.828239
    -1.828239 0.000000 -5.484717
    4.373277 1.140694 -3.848187
    -8.106224 0.000000 7.312956
    -8.106224 0.000000 -7.312956
    -12.300518 0.156372 0.000000
    -8.106224 0.000000 3.656478
    -9.888115 0.135124 -3.656478
    -8.106224 0.000000 5.484717
    -12.586838 0.156372 -1.828239
    -8.106224 0.000000 1.828239
    -8.106224 0.000000 -5.484717
    -8.106224 6.246957 7.312956
    -8.106224 2.063413 -11.382094
    -8.106224 2.932026 0.000000
    -8.106224 6.246957 3.656478
    -8.106224 6.246957 -3.656478
    -8.106224 6.246957 5.484717
    -8.106224 3.308950 -1.828239
    -8.106224 6.246957 1.828239
    -8.106224 6.246957 -5.484717
    -7.312956 0.000000 8.909842
    7.312956 0.000000 8.909842
    0.000000 0.000000 8.909842
    3.656478 0.000000 8.909842
    -3.656478 0.000000 8.909842
    5.484717 0.000000 8.909842
    -1.828239 0.000000 8.909842
    1.828239 0.000000 8.909842
    -5.484717 0.000000 8.909842
    -8.106224 0.000000 8.909842
    -7.312956 0.942591 11.966560
    7.312956 0.942591 11.966560
    0.000000 0.942591 11.966560
    3.656478 0.942591 11.966560
    -3.656478 0.942591 11.966560
    5.484717 0.942591 11.966560
    -1.828239 0.942591 11.966560
    1.828239 0.942591 11.966560
    -5.484717 0.942591 11.966560
    -8.106224 0.942591 11.966560
    -7.312956 2.383808 14.361155
    7.312956 2.383808 14.361155
    0.000000 2.383808 14.361155
    3.656478 2.383808 14.361155
    -3.656478 2.383808 14.361155
    5.484717 2.383808 14.361155
    -1.828239 2.383808 14.361155
    1.828239 2.383808 14.361155
    -5.484717 2.383808 14.361155
    -8.106224 2.383808 14.361155
    -7.312956 4.088459 16.115423
    7.312956 4.088459 16.115423
    0.000000 4.088459 16.115423
    3.656478 4.088459 16.115423
    -3.656478 4.088459 16.115423
    5.484717 4.088459 16.115423
    -1.828239 4.088459 16.115423
    1.828239 4.088459 16.115423
    -5.484717 4.088459 16.115423
    -8.106224 4.088459 16.115423
    -7.312956 7.148156 16.945248
    7.312956 7.148156 16.945248
    0.000000 7.148156 16.945248
    3.656478 7.148156 16.945248
    -3.656478 7.148156 16.945248
    5.484717 7.148156 16.945248
    -1.828239 7.148156 16.945248
    1.828239 7.148156 16.945248
    -5.484717 7.148156 16.945248
    -8.106224 7.148156 16.945248
    11.954430 0.000000 -7.312956
    11.954430 0.000000 0.000000
    11.954430 0.000000 -3.656478
    11.954430 0.000000 3.656478
    11.954430 0.000000 -5.484717
    11.954430 0.000000 1.828239
    11.954430 0.000000 -1.828239
];

static LV_FACES: &[Vec3<u32>] = &cat![
    28 45 81
    34 41 80
    35 48 79
    32 49 78
    50 37 77
    76 22 52
    75 16 53
    54 36 74
    55 56 73
    47 57 72
    58 29 71
    59 60 70
    46 61 69
    68 17 33
    63 64 67
    66 12 65
    63 65 25
    21 66 63
    49 40 66
    46 64 18
    37 46 9
    37 63 67
    34 33 8
    64 34 18
    25 68 64
    59 61 24
    48 59 20
    48 46 69
    70 14 30
    36 30 5
    20 70 36
    71 3 42
    60 42 14
    24 71 60
    55 57 23
    31 55 15
    6 72 31
    73 10 26
    43 26 1
    15 73 43
    74 5 38
    10 74 38
    23 74 56
    75 22 51
    27 51 11
    2 75 27
    76 19 47
    39 47 6
    11 76 39
    77 9 35
    19 77 35
    22 77 52
    50 49 21
    22 78 50
    16 78 53
    79 20 54
    57 54 23
    57 35 79
    80 13 58
    61 58 24
    18 80 61
    62 45 17
    25 81 62
    12 81 65
    10 87 26
    14 90 86
    3 90 42
    38 85 10
    5 89 38
    30 84 5
    30 86 88
    26 82 1
    89 93 98
    85 96 87
    89 94 85
    87 91 82
    83 99 90
    86 97 88
    90 95 86
    84 97 93
    82 100 1
    11 105 27
    15 106 31
    27 101 2
    31 102 6
    6 107 39
    1 108 43
    39 103 11
    43 104 15
    102 117 107
    108 114 104
    105 111 101
    100 118 108
    103 115 105
    106 112 102
    109 110 100
    104 116 106
    107 113 103
    113 125 115
    116 122 112
    119 120 110
    114 126 116
    117 123 113
    112 127 117
    118 124 114
    115 121 111
    110 128 118
    127 133 123
    122 137 127
    128 134 124
    125 131 121
    120 138 128
    123 135 125
    126 132 122
    129 130 120
    124 136 126
    135 141 131
    130 148 138
    133 145 135
    136 142 132
    139 140 130
    134 146 136
    137 143 133
    132 147 137
    138 144 134
    40 152 12
    7 156 40
    32 151 7
    16 155 32
    28 150 4
    12 154 28
    28 4 45
    34 8 41
    35 9 48
    32 7 49
    50 21 37
    76 51 22
    75 44 16
    54 20 36
    55 23 56
    47 19 57
    58 13 29
    59 24 60
    46 18 61
    68 62 17
    63 25 64
    66 40 12
    63 66 65
    21 49 66
    49 7 40
    46 67 64
    37 67 46
    37 21 63
    34 68 33
    64 68 34
    25 62 68
    59 69 61
    48 69 59
    48 9 46
    70 60 14
    36 70 30
    20 59 70
    71 29 3
    60 71 42
    24 58 71
    55 72 57
    31 72 55
    6 47 72
    73 56 10
    43 73 26
    15 55 73
    74 36 5
    10 56 74
    23 54 74
    75 53 22
    27 75 51
    2 44 75
    76 52 19
    39 76 47
    11 51 76
    77 37 9
    19 52 77
    22 50 77
    50 78 49
    22 53 78
    16 32 78
    79 48 20
    57 79 54
    57 19 35
    80 41 13
    61 80 58
    18 34 80
    62 81 45
    25 65 81
    12 28 81
    10 85 87
    14 42 90
    3 83 90
    38 89 85
    5 84 89
    30 88 84
    30 14 86
    26 87 82
    89 84 93
    85 94 96
    89 98 94
    87 96 91
    83 92 99
    86 95 97
    90 99 95
    84 88 97
    82 109 100
    11 103 105
    15 104 106
    27 105 101
    31 106 102
    6 102 107
    1 100 108
    39 107 103
    43 108 104
    102 112 117
    108 118 114
    105 115 111
    100 110 118
    103 113 115
    106 116 112
    109 119 110
    104 114 116
    107 117 113
    113 123 125
    116 126 122
    119 129 120
    114 124 126
    117 127 123
    112 122 127
    118 128 124
    115 125 121
    110 120 128
    127 137 133
    122 132 137
    128 138 134
    125 135 131
    120 130 138
    123 133 135
    126 136 132
    129 139 130
    124 134 136
    135 145 141
    130 140 148
    133 143 145
    136 146 142
    139 149 140
    134 144 146
    137 147 143
    132 142 147
    138 148 144
    40 156 152
    7 151 156
    32 155 151
    16 153 155
    28 154 150
    12 152 154
];
