use glam::{vec3, Vec3};
use raymarching::{
    materials::{Normal, PhongShaded, Textured, Unlit},
    raymarcher::{Raymarcher, GREEN, RED, YELLOW},
    surfaces::{SmoothUnion, Sphere, Surface},
};

fn main() {
    #[rustfmt::skip]
    let surfaces: Vec<Box<dyn Surface>> = vec![
        Box::new(SmoothUnion::new(
            Box::new(Sphere::new(vec3(2.0,1.0,-1.0), 1.0, Box::new(Textured::new("assets/dirt.jpeg")))),
            Box::new(Sphere::new(vec3(0.0,1.0,-1.0), 1.0, Box::new(Textured::new("assets/grass.jpeg")))),
            1.0,
        ))
    ];
    let light_pos = vec3(-2.0, -1.0, -2.0);
    let app = Raymarcher::new(surfaces, light_pos);
    pixel_renderer::app::run(app)
}

