use glam::vec3;
use raymarching::{
    materials::{Textured, Unlit},
    raymarcher::{Raymarcher, RED},
    surfaces::{Fractal, Plane, Sphere, Surface},
};

fn main() {
    #[rustfmt::skip]
    let surfaces: Vec<Box<dyn Surface>> = vec![
        Box::new(Fractal::new(2.0, Box::new(Unlit::new(RED))))
    ];
    let light_pos = vec3(-2.0, 1.0, -2.0);
    let camera_pos = vec3(0.0, 1.0, -5.0);
    let app = Raymarcher::new(surfaces, light_pos, camera_pos);
    pixel_renderer::app::run(app)
}
