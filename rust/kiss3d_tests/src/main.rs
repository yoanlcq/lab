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