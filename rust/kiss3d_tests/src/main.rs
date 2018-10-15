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

#[derive(Debug, Clone, PartialEq)]
struct Partition {
    pub elements: Vec<Element>, // FIXME: Use some kind of smallvec[7] instead (and derive Copy)
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
        unimplemented!()
    }
    pub fn get_overlap_interval(&self, k: Vec2<f32>, v: Vec2<f32>, region: Region) -> Option<(f32, f32)> {
        unimplemented!()
    }
    pub fn compute_roots(&self, sphere: &Sphere, v: Vec3<f32>, element: &Element) -> Vec<ContactInfo> { // FIXME: returns either zero, one, or two roots. Using a Vec is overkill.
        unimplemented!()
    }
    pub fn solve_quadratic(&self, tmin: f32, tmax: f32, a0: f32, a1: f32, a2: f32) -> Vec<f32> { // FIXME: returns either zero, one, or two roots. Using a Vec is overkill.
        unimplemented!()
    }
}

fn get_contact(sphere: &Sphere, sphere_vel: Vec3<f32>, tri: Triangle, tri_vel: Vec3<f32>) -> Option<(ContactInfo, ContactInfo)> {
    unimplemented!()
}