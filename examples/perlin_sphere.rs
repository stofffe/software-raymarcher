use glam::{vec3, Vec3};
use raymarching::{
    materials::Unlit,
    raymarcher::{Raymarcher, RED},
    surfaces::{PerlinSphere, Surface},
};

fn main() {
    #[rustfmt::skip]
    let surfaces: Vec<Box<dyn Surface>> = vec![
        // Box::new(PertrubedSphere::new(Vec3::ZERO, 10.0, 1.0, 1.0, Box::new(Unlit::new(RED)))),
        Box::new(PerlinSphere::new(Vec3::ZERO, 5.0, 0.0, Box::new(Unlit::new(RED)))),
    ];
    let light_pos = vec3(-15.0, 20.0, -30.0);
    let camera_pos = vec3(0.0, 0.0, -10.0);
    let app = Raymarcher::new(surfaces, light_pos, camera_pos);
    pixel_renderer::app::run(app)
}
