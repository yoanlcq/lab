use ndc;
use vec4::Vec4f;
use framebuffer::Framebuffer;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Triangle {
    pub position: (Vec4f, Vec4f, Vec4f),
    pub color: (Vec4f, Vec4f, Vec4f),
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Aabb {
    pub min: Vec4f,
    pub max: Vec4f,
}

impl Triangle {
    pub fn aabb(&self) -> Aabb {
        let (a, b, c) = self.position;
        Aabb {
            min: Vec4f::min(Vec4f::min(a, b), c),
            max: Vec4f::max(Vec4f::max(a, b), c),
        }
    }
}

impl Framebuffer {
    pub fn rasterize_triangles(&mut self, triangles: &[Triangle]) {
        for tri in triangles {

            // Shave a lot of work by computing the bounding box of the triangle
            // and mapping it to framebuffer (pixel) coordinates.
            use ::std::cmp;
            let Aabb { min, max } = tri.aabb();
            let min = ndc::to_pixel(min, self.w, self.h);
            let max = ndc::to_pixel(max, self.w, self.h);
            // Avoid illegal indices; Also performs clipping on its own!
            let (minx, maxx) = (cmp::max(min.0, 0) as u32, cmp::min(max.0, self.w as _) as u32);
            let (miny, maxy) = (cmp::max(min.1, 0) as u32, cmp::min(max.1, self.h as _) as u32);

            for y in miny..maxy {
                for x in minx..maxx {
                    self.try_pixel(x, y, tri);
                }
            }
        }
    }
    fn try_pixel(&mut self, x: u32, y: u32, tri: &Triangle) {
        // Map this pixel coordinate to NDC (Normalized Device Coordinates)
        let p = ndc::from_pixel(x, y, self.w, self.h);

        let (a, b, c) = tri.position;
        // Half-space test (here, what 'h' is short for)
        let h = Vec4f::new(
            p.determine_side(b, c),
            p.determine_side(c, a),
            p.determine_side(a, b),
            0.
        );

        // Are we outside the triangle?
        if h.x < 0. || h.y < 0. || h.z < 0. {
            return;
        }

        // Compute barycentric coordinates (w stands for 'weight')
        let w = h / c.determine_side(a, b);

        // Interpolate vertex attributes using barycentric coordinates
        let depth = Vec4f::dot(w, Vec4f::new(a.z, b.z, c.z, 0.));
        let color = {
            let (ca, cb, cc) = tri.color;
            ca * w.x + cb * w.y + cc * w.z
        };

        // The usual convenient 1D index
        let i = (y*self.w + x) as usize;

        // Depth test
        if depth >= self.depth[i] {
            return;
        }

        self.depth[i] = depth;
        self.color[i] = color;
    }
}

fn rasterize(fb: &mut Framebuffer, tri: &Triangle) {
    let (a, b, c) = tri.position;
    // Shave a lot of work by computing the bounding box of the triangle
    // and mapping it to framebuffer (pixel) coordinates.
    let Aabb { min, max } = tri.aabb();
    // Map from NDC to pixel
    let wh = Vec4f::new((fb.w-1) as _, (fb.h-1) as _, 1., 1.);
    let min = Vec4f::max(wh * (min + 1.) / 2., Vec4f::zero());
    let max = Vec4f::min(wh * (max + 1.) / 2., wh);

    let mut w_row = Vec4f::new(min.side_2d(b, c), min.side_2d(c, a), min.side_2d(a, b), 0.);
    let w_step_x = Vec4f::new(b.y - c.y, c.y - a.y, a.y - b.y, 0.);
    let w_step_y = Vec4f::new(c.x - b.x, a.x - c.x, b.x - a.x, 0.);

    for y in (min.y as usize) .. (max.y as usize + 1) {
        let mut w = w_row;
        for x in (min.x as usize) .. (max.x as usize + 1) {
            if w.sign_mask() == 0 {
                let i = y*(fb.w as usize) + x;
                let zabc = Vec4f::new(a.z, b.z, c.z, 0.);
                let depth = Vec4f::dot(w, zabc);
                if depth < fb.depth[i] {
                    fb.depth[i] = depth;
                    fb.color[i] = {
                        let (mut ca, mut cb, mut cc) = tri.color;
                        let w = w / zabc;
                        (ca * w.x + cb * w.y + cc * w.z) / (w.x + w.y + w.z)
                    };
                }
            }
            w += w_step_x;
        }
        w_row += w_step_y;
    }
}
