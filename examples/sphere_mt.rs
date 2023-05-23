use std::sync::Arc;

use glam::{vec3, Vec3};
use raymarching::{
    materials::{Textured, Unlit},
    raymarcher::BLUE,
    raymarcher_mt::Raymarcher,
    surfaces_mt::{Plane, Sphere, SurfList},
};

fn main() {
    let camera_pos = vec3(0.0, 0.0, -3.0);
    let light_pos = vec3(1.0, 2.0, -4.0);
    #[rustfmt::skip]
    let surfaces: SurfList = Arc::new(vec![
        Arc::new(Sphere::new(Vec3::ZERO, 1.0, Arc::new(Unlit::new(BLUE)))),
        Arc::new(Plane::new(vec3(0.0,1.0,0.0), -2.0, Arc::new(Textured::new("assets/checkerboard.jpeg").with_scale(0.5))))
        // Arc::new(Plane::new(vec3(0.0,1.0,0.0), -2.0, Arc::new(Unlit::new(WHITE))))
        // Arc::new(Sphere::new(vec3(1.0,0.0,0.0), 1.0, Arc::new(Unlit::new(BLUE)))),
        // Arc::new(Sphere::new(vec3(2.0,0.0,0.0), 1.0, Arc::new(Unlit::new(BLUE)))),
        // Arc::new(Sphere::new(vec3(3.0,0.0,0.0), 1.0, Arc::new(Unlit::new(BLUE)))),
        // Arc::new(Sphere::new(vec3(4.0,0.0,0.0), 1.0, Arc::new(Unlit::new(BLUE)))),
        // Arc::new(Sphere::new(Vec3::ZERO, 1.0, Arc::new(data)))
        // Arc::new(Plane::new(vec3(0.0, 1.0, 0.0), -2.0, Arc::new(Unlit::new(RED)))),
        // Arc::new(Sphere::new(vec3(0.0,0.0,0.0), 1.0, Arc::new(Unlit::new(RED)))),
        // Box::new(Plane::new(vec3(0.0, -1.0, 0.0), -2.0, Box::new(Unlit::new(WHITE)))),
        // Box::new(BoxExact::new(vec3(1.0,1.0,1.0), Box::new(Unlit::new(GREEN))))
        // Box::new(Plane::new(vec3(0.0,-1.0,0.0), -1.0, Box::new(Shaded::new(Box::new(Unlit::new(BLUE))))))
    ]);
    let raymarcher = Raymarcher::new(surfaces, camera_pos, light_pos);
    pixel_renderer::app::run(raymarcher);
}
