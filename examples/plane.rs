use glam::{vec3, Vec3};
use raymarching::{
    materials::{Normal, Shaded, Textured, Unlit},
    raymarcher::{Raymarcher, BLUE, GREEN, RED, YELLOW},
    surfaces::{Plane, Sphere, Surface},
};

fn main() {
    #[rustfmt::skip]
    let surfaces: Vec<Box<dyn Surface>> = vec![
        Box::new(Plane::new(vec3(0.0, -1.0, 0.0), -2.0, Box::new(Shaded::new(Box::new(Textured::new("assets/checkerboard.jpeg")))))),
        // Box::new(Plane::new(vec3(0.0,-1.0,0.0), -1.0, Box::new(Shaded::new(Box::new(Unlit::new(BLUE))))))
    ];
    let light_pos = vec3(-2.0, -1.0, -2.0);
    let app = Raymarcher::new(surfaces, light_pos);
    pixel_renderer::app::run(app)
}
